use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use bridge_db::{Domain, SqlitePool};
use bridge_http::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

enum HistoryDomain {}

impl Domain for HistoryDomain {
    const NAME: &'static str = "history";
    const MIGRATIONS: &'static [&'static str] = &[
        include_str!("../migrations/0001_init.sql"),
        include_str!("../migrations/0002_fts.sql"),
    ];
}

#[derive(Serialize)]
pub struct HistoryEntry {
    id: i64,
    method: String,
    url: String,
    status: u16,
    request_headers: Vec<(String, String)>,
    request_body: Option<String>,
    response_headers: Vec<(String, String)>,
    response_body: String,
    elapsed_ms: u64,
    created_at: i64,
}

#[derive(Deserialize, Default)]
pub struct HistoryQuery {
    search: Option<String>,
    method: Option<String>,
    status: Option<u16>,
    since: Option<i64>,
    until: Option<i64>,
    limit: Option<u32>,
}

struct RequestRow {
    id: i64,
    method: String,
    url: String,
    status: i64,
    request_headers: String,
    request_body: Option<String>,
    response_headers: String,
    response_body: String,
    elapsed_ms: i64,
    created_at: i64,
}

impl RequestRow {
    fn into_entry(self) -> HistoryEntry {
        HistoryEntry {
            id: self.id,
            method: self.method,
            url: self.url,
            status: self.status as u16,
            request_headers: serde_json::from_str(&self.request_headers).unwrap_or_default(),
            request_body: self.request_body,
            response_headers: serde_json::from_str(&self.response_headers).unwrap_or_default(),
            response_body: self.response_body,
            elapsed_ms: self.elapsed_ms as u64,
            created_at: self.created_at,
        }
    }
}

pub struct History {
    pool: SqlitePool,
}

impl History {
    pub async fn open(path: &Path) -> Result<Self, sqlx::Error> {
        let pool = bridge_db::connect(path).await?;
        bridge_db::migrate::<HistoryDomain>(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn insert(&self, req: &HttpRequest, res: &HttpResponse) -> Result<i64, sqlx::Error> {
        let request_headers = serde_json::to_string(&req.headers).unwrap_or_default();
        let response_headers = serde_json::to_string(&res.headers).unwrap_or_default();
        let status = res.status as i64;
        let elapsed = res.elapsed_ms as i64;
        let created = now_millis();
        let id = sqlx::query!(
            "INSERT INTO requests
                (method, url, status, request_headers, request_body,
                 response_headers, response_body, elapsed_ms, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            req.method,
            req.url,
            status,
            request_headers,
            req.body,
            response_headers,
            res.body,
            elapsed,
            created,
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn query(&self, q: &HistoryQuery) -> Result<Vec<HistoryEntry>, sqlx::Error> {
        let limit = q.limit.unwrap_or(100).min(1000) as i64;
        let status = q.status.map(|s| s as i64);
        let fts = q.search.as_deref().map(fts_query).filter(|s| !s.is_empty());
        let rows = match fts {
            Some(m) => self.search_ranked(&m, q, status, limit).await?,
            None => self.browse(q, status, limit).await?,
        };
        Ok(rows.into_iter().map(RequestRow::into_entry).collect())
    }

    // Relevance-ranked search (bm25) over url + request/response bodies.
    async fn search_ranked(
        &self,
        match_query: &str,
        q: &HistoryQuery,
        status: Option<i64>,
        limit: i64,
    ) -> Result<Vec<RequestRow>, sqlx::Error> {
        sqlx::query_as!(
            RequestRow,
            r#"
            SELECT
                requests.id AS "id!: i64",
                requests.method AS "method!",
                requests.url AS "url!",
                requests.status AS "status!: i64",
                requests.request_headers AS "request_headers!",
                requests.request_body AS "request_body",
                requests.response_headers AS "response_headers!",
                requests.response_body AS "response_body!",
                requests.elapsed_ms AS "elapsed_ms!: i64",
                requests.created_at AS "created_at!: i64"
            FROM requests
            JOIN requests_fts ON requests_fts.rowid = requests.id
            WHERE requests_fts MATCH ?
              AND (CAST(? AS TEXT) IS NULL OR requests.method = ?)
              AND (CAST(? AS INTEGER) IS NULL OR requests.status = ?)
              AND (CAST(? AS INTEGER) IS NULL OR requests.created_at >= ?)
              AND (CAST(? AS INTEGER) IS NULL OR requests.created_at <= ?)
            ORDER BY bm25(requests_fts)
            LIMIT ?
            "#,
            match_query,
            q.method,
            q.method,
            status,
            status,
            q.since,
            q.since,
            q.until,
            q.until,
            limit,
        )
        .fetch_all(&self.pool)
        .await
    }

    // No search term: most-recent-first browse with optional filters.
    async fn browse(
        &self,
        q: &HistoryQuery,
        status: Option<i64>,
        limit: i64,
    ) -> Result<Vec<RequestRow>, sqlx::Error> {
        sqlx::query_as!(
            RequestRow,
            r#"
            SELECT
                id AS "id!: i64",
                method AS "method!",
                url AS "url!",
                status AS "status!: i64",
                request_headers AS "request_headers!",
                request_body AS "request_body",
                response_headers AS "response_headers!",
                response_body AS "response_body!",
                elapsed_ms AS "elapsed_ms!: i64",
                created_at AS "created_at!: i64"
            FROM requests
            WHERE (CAST(? AS TEXT) IS NULL OR method = ?)
              AND (CAST(? AS INTEGER) IS NULL OR status = ?)
              AND (CAST(? AS INTEGER) IS NULL OR created_at >= ?)
              AND (CAST(? AS INTEGER) IS NULL OR created_at <= ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
            q.method,
            q.method,
            status,
            status,
            q.since,
            q.since,
            q.until,
            q.until,
            limit,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get(&self, id: i64) -> Result<Option<HistoryEntry>, sqlx::Error> {
        let row = sqlx::query_as!(
            RequestRow,
            r#"
            SELECT
                id AS "id!: i64",
                method AS "method!",
                url AS "url!",
                status AS "status!: i64",
                request_headers AS "request_headers!",
                request_body AS "request_body",
                response_headers AS "response_headers!",
                response_body AS "response_body!",
                elapsed_ms AS "elapsed_ms!: i64",
                created_at AS "created_at!: i64"
            FROM requests
            WHERE id = ?
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(RequestRow::into_entry))
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

// Turn raw user input ("get github") into a safe FTS5 query: each word becomes a
// quoted prefix term, AND-ed together. Quoting neutralizes FTS5 syntax chars in
// URLs (':' '/' '-' '*' '"'); the trailing '*' makes partial words match
// ("git" -> "github"). Empty if the input has no usable tokens.
// ponytail: prefix-AND heuristic; swap for explicit query operators only if users ask.
fn fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .filter_map(|t| {
            let t = t.replace('"', "");
            (!t.is_empty()).then(|| format!("\"{t}\"*"))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::fts_query;

    #[test]
    fn builds_prefix_and_query() {
        assert_eq!(fts_query("get github"), "\"get\"* \"github\"*");
    }

    #[test]
    fn neutralizes_special_chars() {
        // URL punctuation must not leak into FTS5 syntax.
        assert_eq!(fts_query("https://api.x/users"), "\"https://api.x/users\"*");
        assert_eq!(fts_query("a\"b"), "\"ab\"*");
    }

    #[test]
    fn empty_for_blank_input() {
        assert_eq!(fts_query("   "), "");
        assert_eq!(fts_query(""), "");
    }
}

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use bridge_http::{HttpRequest, HttpResponse};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};

// ponytail: one Mutex<Connection> serializes all reads/writes.
// Swap for an r2d2 pool (or a read-only connection per query) if query
// throughput on history ever becomes the bottleneck.
pub struct History {
    conn: Mutex<Connection>,
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

impl History {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS requests (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                status INTEGER NOT NULL,
                request_headers TEXT NOT NULL,
                request_body TEXT,
                response_headers TEXT NOT NULL,
                response_body TEXT NOT NULL,
                elapsed_ms INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_requests_created_at ON requests(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_requests_url ON requests(url);
            CREATE INDEX IF NOT EXISTS idx_requests_method ON requests(method);
            CREATE INDEX IF NOT EXISTS idx_requests_status ON requests(status);",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn insert(&self, req: &HttpRequest, res: &HttpResponse) -> rusqlite::Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO requests
                (method, url, status, request_headers, request_body,
                 response_headers, response_body, elapsed_ms, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                req.method,
                req.url,
                res.status,
                serde_json::to_string(&req.headers).unwrap_or_default(),
                req.body,
                serde_json::to_string(&res.headers).unwrap_or_default(),
                res.body,
                res.elapsed_ms as i64,
                now_millis(),
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn query(&self, q: &HistoryQuery) -> rusqlite::Result<Vec<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let limit = q.limit.unwrap_or(100).min(1000);
        let mut stmt = conn.prepare(
            "SELECT id, method, url, status, request_headers, request_body,
                    response_headers, response_body, elapsed_ms, created_at
             FROM requests
             WHERE (?1 IS NULL OR url LIKE '%' || ?1 || '%')
               AND (?2 IS NULL OR method = ?2)
               AND (?3 IS NULL OR status = ?3)
               AND (?4 IS NULL OR created_at >= ?4)
               AND (?5 IS NULL OR created_at <= ?5)
             ORDER BY created_at DESC
             LIMIT ?6",
        )?;
        let rows = stmt.query_map(
            params![q.search, q.method, q.status, q.since, q.until, limit],
            map_entry,
        )?;
        rows.collect()
    }

    pub fn get(&self, id: i64) -> rusqlite::Result<Option<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, method, url, status, request_headers, request_body,
                    response_headers, response_body, elapsed_ms, created_at
             FROM requests WHERE id = ?1",
            params![id],
            map_entry,
        )
        .map(Some)
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other),
        })
    }
}

fn map_entry(row: &Row) -> rusqlite::Result<HistoryEntry> {
    let request_headers: String = row.get(4)?;
    let response_headers: String = row.get(6)?;
    Ok(HistoryEntry {
        id: row.get(0)?,
        method: row.get(1)?,
        url: row.get(2)?,
        status: row.get(3)?,
        request_headers: serde_json::from_str(&request_headers).unwrap_or_default(),
        request_body: row.get(5)?,
        response_headers: serde_json::from_str(&response_headers).unwrap_or_default(),
        response_body: row.get(7)?,
        elapsed_ms: row.get::<_, i64>(8)? as u64,
        created_at: row.get(9)?,
    })
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

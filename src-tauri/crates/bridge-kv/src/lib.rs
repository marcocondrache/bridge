use anyhow::Result;

use bridge_db::SqlitePool;

bridge_db::register_domain!("kv", [include_str!("../migrations/0001_init.sql")]);

pub struct Kv {
    pool: SqlitePool,
}

impl Kv {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        sqlx::query_scalar("SELECT value FROM kv WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO kv (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM kv WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // End-to-end: the `register_domain!` macro registers kv, `migrate_all`
    // creates its table, and set/get round-trips through it.
    #[tokio::test]
    async fn registered_kv_migrates_and_round_trips() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        bridge_db::migrate(&pool).await.unwrap();

        let kv = Kv::new(pool);
        kv.set("a", "b").await.unwrap();
        assert_eq!(kv.get("a").await.unwrap().as_deref(), Some("b"));
        kv.delete("a").await.unwrap();
        assert_eq!(kv.get("a").await.unwrap(), None);
    }
}

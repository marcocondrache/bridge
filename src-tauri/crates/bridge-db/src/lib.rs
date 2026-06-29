use std::borrow::Cow;
use std::future::Future;
use std::marker::PhantomData;
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;

use sqlx::error::BoxDynError;
use sqlx::migrate::{Migration, MigrationSource, MigrationType, Migrator};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlStr;

pub use sqlx::SqlitePool;

pub trait Domain {
    const NAME: &'static str;
    const MIGRATIONS: &'static [&'static str];
}

struct DomainSource<D>(PhantomData<D>);

impl<D> std::fmt::Debug for DomainSource<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DomainSource")
    }
}

impl<D: Domain + 'static> MigrationSource<'static> for DomainSource<D> {
    fn resolve(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Migration>, BoxDynError>> + Send + 'static>> {
        Box::pin(async move {
            let migrations = D::MIGRATIONS
                .iter()
                .enumerate()
                .map(|(i, sql)| {
                    Migration::new(
                        (i + 1) as i64,
                        Cow::Borrowed(D::NAME),
                        MigrationType::Simple,
                        SqlStr::from_static(sql),
                        false,
                    )
                })
                .collect();

            Ok(migrations)
        })
    }
}

pub async fn connect(path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_millis(500))
        .foreign_keys(true);

    SqlitePoolOptions::new().connect_with(options).await
}

pub async fn migrate<D: Domain + 'static>(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    Migrator::new(DomainSource::<D>(PhantomData))
        .await?
        .run(pool)
        .await
        .map_err(Into::into)
}

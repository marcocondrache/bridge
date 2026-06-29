use std::borrow::Cow;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;

use sqlx::error::BoxDynError;
use sqlx::migrate::{Migration, MigrationSource, MigrationType, Migrator};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlStr;

pub use inventory;
pub use sqlx::SqlitePool;

inventory::collect!(DomainSource);

#[macro_export]
macro_rules! register_domain {
    ($name:expr, [ $($migration:expr),+ $(,)? ]) => {
        $crate::inventory::submit! {
            $crate::DomainSource {
                name: $name,
                migrations: &[$($migration),+],
            }
        }
    };
}

pub struct DomainSource {
    pub name: &'static str,
    pub migrations: &'static [&'static str],
}

impl std::fmt::Debug for DomainSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DomainSource")
    }
}

impl MigrationSource<'static> for &'static DomainSource {
    fn resolve(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Migration>, BoxDynError>> + Send + 'static>> {
        Box::pin(async move {
            let migrations = self
                .migrations
                .iter()
                .enumerate()
                .map(|(i, sql)| {
                    Migration::new(
                        (i + 1) as i64,
                        Cow::Borrowed(self.name),
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

pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for source in inventory::iter::<DomainSource> {
        let mut migrator = Migrator::new(source).await?;
        migrator.dangerous_set_table_name(format!("_sqlx_migrations_{}", source.name));
        migrator.run(pool).await?;
    }

    Ok(())
}

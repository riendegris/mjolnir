use slog::{error, Logger};
use snafu::ResultExt;
use sqlx::postgres::PgPool;

pub mod error;
pub mod gql;
pub mod model;
pub mod utils;

pub async fn read_dotenv(_log: Logger) -> Result<(), error::Error> {
    dotenv::dotenv().context(error::EnvError {
        details: "Cannot find .env",
    })?;
    Ok(())
}

pub async fn get_connstr(_log: Logger) -> Result<String, error::Error> {
    dotenv::var("DATABASE_URL").context(error::EnvError {
        details: "Cannot find 'DATABASE_URL' in .env",
    })
}

pub async fn connect_db(connstr: String, _log: Logger) -> Result<PgPool, error::Error> {
    PgPool::builder()
        .max_size(5)
        .build(&connstr)
        .await
        .context(error::DBError {
            details: format!("Could not create a PgPool with connection {}", connstr),
        })
}

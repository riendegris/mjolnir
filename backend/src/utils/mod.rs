use crate::error;
use slog::Logger;
use snafu::ResultExt;

pub async fn get_workdir(_log: Logger) -> Result<String, error::Error> {
    dotenv::var("WORK_DIR").context(error::EnvError {
        details: "Cannot find 'WORK_DIR' in .env",
    })
}

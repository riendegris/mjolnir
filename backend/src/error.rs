use juniper::{graphql_value, FieldError, IntoFieldError};
use snafu::{Backtrace, Snafu};
use std::io;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("User Error: {}", details))]
    #[snafu(visibility(pub))]
    UserError { details: String },

    #[snafu(display("Environment Variable Error: {} => {}", details, source))]
    #[snafu(visibility(pub))]
    EnvError {
        details: String,
        source: dotenv::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("IO Error: {}", source))]
    #[snafu(visibility(pub))]
    IOError {
        source: io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Tokio IO Error: {}", source))]
    #[snafu(visibility(pub))]
    TokioIOError {
        source: tokio::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("DB Connection Error: {} => {}", details, source))]
    #[snafu(visibility(pub))]
    DBError {
        details: String,
        source: sqlx::error::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Json Error: {} => {}", details, source))]
    #[snafu(visibility(pub))]
    SerdeJsonError {
        details: String,
        source: serde_json::error::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Reqwest Error: {} => {}", details, source))]
    #[snafu(visibility(pub))]
    ReqwestError {
        details: String,
        source: reqwest::Error,
        backtrace: Backtrace,
    },
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Error::UserError { details, .. } => {
                FieldError::new("User Error", graphql_value!({ "internal_error": details }))
            }
            Error::DBError { details, .. } => {
                FieldError::new("User Error", graphql_value!({ "internal_error": details }))
            }
            _ => FieldError::new("Go Figure!", graphql_value!({ "internal_error": "sol" })),
        }
    }
}

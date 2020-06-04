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

    #[snafu(display("DB Error: {} => {}", details, source))]
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

    #[snafu(display("Gherkin Parser Error: {} => {}", details, source))]
    #[snafu(visibility(pub))]
    GherkinError {
        details: String,
        source: gherkin_rust::ParseError<gherkin_rust::LineCol>,
        backtrace: Backtrace,
    },
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            err @ Error::UserError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new("User Error", graphql_value!({ "internal_error": errmsg }))
            }
            err @ Error::EnvError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new(
                    "Environment Error",
                    graphql_value!({ "internal_error": errmsg }),
                )
            }
            err @ Error::IOError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new("IO Error", graphql_value!({ "internal_error": errmsg }))
            }
            err @ Error::TokioIOError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new(
                    "Tokio IO Error",
                    graphql_value!({ "internal_error": errmsg }),
                )
            }
            err @ Error::DBError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new(
                    "Database Error",
                    graphql_value!({ "internal_error": errmsg }),
                )
            }
            err @ Error::SerdeJsonError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new("Serde Error", graphql_value!({ "internal_error": errmsg }))
            }
            err @ Error::ReqwestError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new(
                    "Reqwest Error",
                    graphql_value!({ "internal_error": errmsg }),
                )
            }
            err @ Error::GherkinError { .. } => {
                let errmsg = format!("{}", err);
                FieldError::new(
                    "Gherkin Error",
                    graphql_value!({ "internal_error": errmsg }),
                )
            }
        }
    }
}

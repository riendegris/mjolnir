use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

pub mod env;
pub mod features;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, GraphQLEnum)]
#[sqlx(rename = "file_status")]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    #[sqlx(rename = "not_available")]
    NotAvailable,
    #[sqlx(rename = "download_in_progress")]
    DownloadInProgress,
    #[sqlx(rename = "available")]
    Available,
    #[sqlx(rename = "download_error")]
    DownloadError,
}

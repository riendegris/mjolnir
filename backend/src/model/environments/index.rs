// use super::scenario::{self, Scenario};
use crate::{error, gql};
use chrono::prelude::*;
// use futures::stream::{self, TryStreamExt};
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use slog::debug;
use snafu::ResultExt;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, GraphQLEnum)]
#[sqlx(rename = "index_status")]
#[serde(rename_all = "lowercase")]
pub enum IndexStatus {
    #[sqlx(rename = "not_available")]
    NotAvailable,
    #[sqlx(rename = "download_in_progress")]
    DownloadInProgress,
    #[sqlx(rename = "download_error")]
    DownloadError,
    #[sqlx(rename = "downloaded")]
    Downloaded,
    #[sqlx(rename = "indexing_in_progress")]
    IndexingInProgress,
    #[sqlx(rename = "indexing_error")]
    IndexingError,
    #[sqlx(rename = "indexed")]
    Indexed,
    #[sqlx(rename = "validation_in_progress")]
    ValidationInProgress,
    #[sqlx(rename = "validation_error")]
    ValidationError,
    #[sqlx(rename = "available")]
    Available,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Index {
    pub id: Uuid,
    pub signature: String,
    pub index_type: String,
    pub data_source: String,
    pub regions: Vec<String>,
    pub filepath: Option<String>,
    pub status: IndexStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Index {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Index {
            id: row.get(0),
            signature: row.get(1),
            index_type: row.get(2),
            data_source: row.get(3),
            regions: row.get(4),
            filepath: row.get(5),
            status: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        })
    }
}

pub async fn fetch_indexes_by_environment_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Vec<Index>, error::Error> {
    debug!(context.logger, "Fetching indexes from environment '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT i.id, i.signature, i.index_type, i.data_source, i.regions, i.filepath, i.status, i.created_at, i.updated_at FROM main.indexes AS i
        INNER JOIN main.environment_index_map AS m ON m.index_id = i.id
        WHERE m.environment = $1",
    )
    .bind(id)
    .fetch_all(&context.pool)
    .await
    .map(Into::<Vec<Index>>::into)
    .context(error::DBError {
        details: "Could not retrieve indexes",
    })
}

pub async fn validate_index_type(
    index_type: &str,
    context: &gql::Context,
) -> Result<(), error::Error> {
    // To validate the index type, we just check it exists in the database
    let _res = sqlx::query("SELECT id FROM main.index_types WHERE id = $1")
        .bind(index_type)
        .try_map(|row: PgRow| row.try_get::<String, _>(0))
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not retrieve index type '{}'", index_type),
        })?;
    Ok(())
}

pub async fn validate_data_source(
    data_source: &str,
    context: &gql::Context,
) -> Result<(), error::Error> {
    // To validate the data source, we just check it exists in the database
    let _res = sqlx::query("SELECT id FROM main.data_sources WHERE id = $1")
        .bind(data_source)
        .try_map(|row: PgRow| row.try_get::<String, _>(0))
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not retrieve data source '{}'", data_source),
        })?;
    Ok(())
}

// We check that the data_source is compatible with the index_type
pub async fn validate_data_source_with_index_type(
    data_source: &str,
    index_type: &str,
    context: &gql::Context,
) -> Result<(), error::Error> {
    // To validate the index type, we just check it exists in the database
    let _res = sqlx::query("SELECT data_source FROM main.index_type_data_source WHERE index_type = $1 AND data_source = $2")
        .bind(index_type)
        .bind(data_source)
        .try_map(|row: PgRow| row.try_get::<String, _>(0))
        .fetch_one(&context.pool)
        .await
        //.map(Into::<Vec<Environment>>::into)
        .context(error::DBError {
            details: "Could not validate",
        })?;
    Ok(())
}

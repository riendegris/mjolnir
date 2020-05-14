use crate::{error, gql};
use chrono::prelude::*;
// use futures::stream::{self, TryStreamExt};
use juniper::GraphQLObject;
// use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use slog::{debug, info, Logger};
use snafu::ResultExt;
use uuid::Uuid;
// use sqlx::postgres::PgPool;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
// use std::path::PathBuf;
// use tokio::fs;
// use tokio::prelude::*;

/// A Bano Environment consists in several BanoItem.
/// It can be identified by its label, and contains a description.
#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Feature {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Feature {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Feature {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            tags: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        })
    }
}

/* Retrieve All features */
pub async fn fetch_all_features(context: &gql::Context) -> Result<Vec<Feature>, error::Error> {
    sqlx::query_as("SELECT id, title, description, tags, created_at, updated_at FROM main.features")
        .fetch_all(&context.pool)
        .await
        .map(|rows| {
            let features: Vec<Feature> = rows.into();
            features
        })
        .context(error::DBError {
            details: "Could not retrieve features",
        })
}

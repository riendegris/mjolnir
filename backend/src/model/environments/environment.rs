// use super::scenario::{self, Scenario};
use super::index::IndexStatus;
use crate::{error, gql};
use chrono::prelude::*;
// use futures::stream::{self, TryStreamExt};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use slog::debug;
use snafu::ResultExt;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Environment {
    pub id: Uuid,
    pub signature: String,
    pub status: IndexStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Environment {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Environment {
            id: row.get(0),
            signature: row.get(1),
            status: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
    }
}

pub async fn fetch_all_environments(
    context: &gql::Context,
) -> Result<Vec<Environment>, error::Error> {
    debug!(context.logger, "Retrieving all environments");
    sqlx::query_as("SELECT id, signature, status, created_at, updated_at FROM main.features")
        .fetch_all(&context.pool)
        .await
        .map(Into::<Vec<Environment>>::into)
        .context(error::DBError {
            details: "Could not retrieve environments",
        })
}

pub async fn fetch_environment_by_id(
    id: Uuid,
    context: &gql::Context,
) -> Result<Environment, error::Error> {
    debug!(context.logger, "Fetching environment with id '{}'", id);
    sqlx::query_as(
        "SELECT id, signature, status, created_at, updated_at FROM main.environments WHERE id=$1",
    )
    .bind(id)
    .fetch_one(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve environment with id '{}', id",
    })
}

pub async fn create_or_replace_environment(
    id: Uuid,
    signature: String,
    status: IndexStatus,
    context: &gql::Context,
) -> Result<Environment, error::Error> {
    debug!(
        context.logger,
        "Creating or Updating Environment '{}'", signature
    );
    sqlx::query_as("SELECT * FROM main.create_or_replace_environment($1, $2, $3)")
        .bind(id)
        .bind(signature.clone())
        .bind(status)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not create or replace environment '{}'", signature),
        })
}

pub async fn delete_enviroment_by_id(
    id: Uuid,
    context: &gql::Context,
) -> Result<Environment, error::Error> {
    debug!(context.logger, "Deleting environment with id '{}'", id);
    sqlx::query_as("SELECT * FROM main.delete_environment($1)")
        .bind(id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not delete environment",
        })
}

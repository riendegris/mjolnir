use super::IdTimestamp;
use crate::{error, gql};
use chrono::prelude::*;
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
pub struct Scenario {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Scenario {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Scenario {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            tags: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        })
    }
}

pub async fn fetch_scenario_by_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Scenario, error::Error> {
    debug!(context.logger, "Fetching scenario '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT id, name, description, tags, created_at, updated_at FROM main.scenarios WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve features",
    })
}

pub async fn fetch_scenarios_by_feature_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Vec<Scenario>, error::Error> {
    debug!(context.logger, "Fetching scenarios from feature '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT id, name, description, tags, created_at, updated_at FROM main.scenarios WHERE feature = $1"
    )
    .bind(id)
    .fetch_all(&context.pool)
    .await
    .map(Into::<Vec<Scenario>>::into)
    .context(error::DBError {
        details: "Could not retrieve features",
    })
}

pub async fn create_or_replace_scenario(
    scenario: &Scenario,
    feature_id: &Uuid, // owns the scenario
    context: &gql::Context,
) -> Result<IdTimestamp, error::Error> {
    sqlx::query_as("SELECT * FROM main.create_or_replace_scenario($1, $2, $3, $4, $5)")
        .bind(scenario.id)
        .bind(scenario.name.clone())
        .bind(scenario.description.clone())
        .bind(scenario.tags.join(","))
        .bind(feature_id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not insert or update feature {}", scenario.id),
        })
}

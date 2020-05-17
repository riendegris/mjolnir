use super::IdTimestamp;
use crate::{error, gql};
use chrono::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use slog::debug;
use snafu::ResultExt;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, GraphQLEnum)]
#[sqlx(rename = "step_type")]
#[serde(rename_all = "lowercase")]
pub enum StepType {
    #[sqlx(rename = "given")]
    Given,
    #[sqlx(rename = "when")]
    When,
    #[sqlx(rename = "then")]
    Then,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Step {
    pub id: Uuid,
    pub step_type: StepType,
    pub value: String,
    pub docstring: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Step {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Step {
            id: row.get(0),
            step_type: row.get(1),
            value: row.get(2),
            docstring: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        })
    }
}

pub async fn fetch_step_by_id(id: &Uuid, context: &gql::Context) -> Result<Step, error::Error> {
    debug!(context.logger, "Fetching step '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT id, step_type, value, docstring, created_at, updated_at FROM main.steps WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve features",
    })
}

pub async fn fetch_step_by_scenario_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Vec<Step>, error::Error> {
    debug!(context.logger, "Fetching scenarios from feature '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT st.id, st.step_type, st.value, st.docstring, st.created_at, st.updated_at FROM main.steps AS st
         INNER JOIN main.scenario_step_map AS map ON map.step = st.id
         INNER JOIN main.scenario as sc ON map.scenario = sc.id
         WHERE sc.id = $1"
    )
    .bind(id)
    .fetch_all(&context.pool)
    .await
    .map(Into::<Vec<Step>>::into)
    .context(error::DBError {
        details: "Could not retrieve features",
    })
}

pub async fn create_or_replace_step(
    step: &Step,
    context: &gql::Context,
) -> Result<IdTimestamp, error::Error> {
    sqlx::query_as("SELECT * FROM main.create_or_replace_step($1, $2, $3, $4)")
        .bind(step.id)
        .bind(step.step_type.clone())
        .bind(step.value.clone())
        .bind(step.docstring.clone())
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not insert or update feature {}", step.id),
        })
}

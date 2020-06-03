use super::{IdTimestamp, SourceType};
use crate::{error, gql};
use chrono::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use slog::{debug, info};
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

impl From<gherkin_rust::StepType> for StepType {
    fn from(step_type: gherkin_rust::StepType) -> Self {
        match step_type {
            gherkin_rust::StepType::Given => StepType::Given,
            gherkin_rust::StepType::When => StepType::When,
            gherkin_rust::StepType::Then => StepType::Then,
        }
    }
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

pub async fn fetch_steps_by_scenario_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Vec<Step>, error::Error> {
    debug!(context.logger, "Fetching steps from feature '{}'", id);
    sqlx::query_as(
        "SELECT st.id, st.step_type, st.value, st.docstring, st.created_at, st.updated_at FROM main.steps AS st
         INNER JOIN main.scenario_step_map AS map ON map.step = st.id
         INNER JOIN main.scenarios as sc ON map.scenario = sc.id
         WHERE sc.id = $1"
    )
    .bind(id)
    .fetch_all(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve steps",
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

pub async fn create_or_replace_step_from_gherkin(
    step: gherkin_rust::Step,
    id: &Uuid,          // id of the source
    source: SourceType, // type of the source
    context: &gql::Context,
) -> Result<Step, error::Error> {
    info!(
        context.logger,
        "Creating or Updating Step from gherkin: {} / {:?} / {:?}",
        step.value,
        step.ty,
        step.docstring
    );

    let step_id = Uuid::new_v4();

    let res: Step = sqlx::query_as("SELECT * FROM main.create_or_replace_step($1, $2, $3, $4)")
        .bind(step_id)
        .bind(StepType::from(step.ty))
        .bind(step.value.clone())
        .bind(step.docstring.unwrap_or(String::from("")))
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not insert or update step '{}'", step.value),
        })?;

    info!(context.logger, "Inserted step '{}'", step.value);

    // TODO There is an opportunity to make the code more generic below...
    match source {
        SourceType::Scenario => {
            let _idts: IdTimestamp =
                sqlx::query_as("SELECT * FROM main.add_step_to_scenario($1, $2)")
                    .bind(id)
                    .bind(step_id)
                    .fetch_one(&context.pool)
                    .await
                    .context(error::DBError {
                        details: format!(
                            "Could not associate step '{}' to scenario '{}'",
                            step_id, id
                        ),
                    })?;
        }
        SourceType::Background => {
            info!(
                context.logger,
                "adding step '{}' to background '{}'", step_id, id
            );
            let _idts: IdTimestamp =
                sqlx::query_as("SELECT * FROM main.add_step_to_background($1, $2)")
                    .bind(id)
                    .bind(step_id)
                    .fetch_one(&context.pool)
                    .await
                    .context(error::DBError {
                        details: format!(
                            "Could not associate step '{}' to background '{}'",
                            step_id, id
                        ),
                    })?;
        }
    }
    Ok(res)
}

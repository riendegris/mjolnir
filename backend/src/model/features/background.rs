use super::{
    step::{self},
    IdTimestamp, SourceType,
};
use crate::{error, gql};
use chrono::prelude::*;
use futures::stream::{self, TryStreamExt};
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
pub struct Background {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// This should match the main.return_background_typek
impl<'c> FromRow<'c, PgRow<'c>> for Background {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Background {
            id: row.get(0),
            created_at: row.get(1),
            updated_at: row.get(2),
        })
    }
}

// Retrieve the Background given its Id.
// This is not very useful, because the background type does not carry much information.
pub async fn fetch_background_by_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Background, error::Error> {
    debug!(context.logger, "Fetching background '{}'", id);
    sqlx::query_as("SELECT id, created_at, updated_at FROM main.backgrounds WHERE id = $1")
        .bind(id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not retrieve background",
        })
}

// Return a background if there is one for that feature.
pub async fn fetch_background_by_feature_id(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Option<Background>, error::Error> {
    debug!(context.logger, "Fetching background from feature '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as("SELECT id, created_at, updated_at FROM main.backgrounds WHERE feature = $1")
        .bind(id)
        .fetch_optional(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not retrieve backgrounds",
        })
}

pub async fn create_or_replace_background_from_gherkin(
    background: gherkin_rust::Background,
    feature: &Uuid,
    context: &gql::Context,
) -> Result<Background, error::Error> {
    debug!(context.logger, "Creating background from gherkin");

    let res: Background = sqlx::query_as("SELECT * FROM main.create_background($1)")
        .bind(feature)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not create background"),
        })?;

    let id = res.id;

    // Here we're turning the background's steps into a stream of Result<Step, _>, on
    // which we can use try_for_each and insert them in the database
    stream::iter(background.steps.into_iter().map(|step| Ok(step)))
        .try_for_each(|step| async {
            let _step = step::create_or_replace_step_from_gherkin(
                step,
                &id,
                SourceType::Background,
                context,
            )
            .await?;
            Ok(())
        })
        .await?;

    Ok(res)
}

use super::{step, SourceType};
use crate::{error, gql, model::environments};
use chrono::prelude::*;
use futures::stream::{self, TryStreamExt};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use slog::{debug, info};
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

// Return a list of steps
pub async fn fetch_background_steps(
    id: &Uuid,
    context: &gql::Context,
) -> Result<Vec<step::Step>, error::Error> {
    debug!(context.logger, "Fetching steps for background '{}'", id);

    sqlx::query_as("SELECT s.id, s.step_type, s.value, s.docstring, s.created_at, s.updated_at FROM main.steps AS s
        INNER JOIN main.background_step_map AS m ON m.step = s.id
        INNER JOIN main.backgrounds AS b ON b.id = m.background
        WHERE b.id = $1")
        .bind(id)
        .fetch_all(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not retrieve steps",
        })
}

// This function returns the environment that correspond to the background specified by 'id'
// If the environment doesn't exist, it is created, along with all the indexes that compose it.
// TODO we could implement this function in the database, using Postgresql Regex syntax.
pub async fn fetch_background_environment(
    id: &Uuid, // background id
    context: &gql::Context,
) -> Result<environments::environment::Environment, error::Error> {
    debug!(context.logger, "Retrieving Background Environment '{}'", id);

    // We retrieve the list of steps that constitute the background.
    // Then, for each step,
    // - we parse it to extract the keywords
    // - we do some validation on the data
    // - we use 'create_or_replace_index' to create the index in the database
    // - we use 'add_index_to_background' to associate that index to the current background.
    // Finally, we retrieve the environment corresponding to that background.

    let steps = fetch_background_steps(&id, &context).await?;

    stream::iter(steps.into_iter().map(|step| Ok(step)))
        .try_for_each(|step| async {
            info!(context.logger, "Working on step {}", step.value);
            // FIXME Do something if step_type is not 'GIVEN'
            // FIXME Do something if step does not parse like we expect
            let (index_type, data_source, regions) =
                step::extract_index_from_step(step.value).unwrap();
            info!(
                context.logger,
                "Parsed into {} - {} - {:?}", index_type, data_source, regions
            );
            environments::index::validate_index_type(&index_type, context).await?;
            // FIXME These lines have been commented out because the rules for validating must be
            // thought through a bit more.
            // environments::index::validate_data_source(&data_source, context).await?;
            // environments::index::validate_data_source_with_index_type(
            //     &data_source,
            //     &index_type,
            //     context,
            // )
            // .await?;

            info!(context.logger, "last check");
            let index: environments::index::Index =
                sqlx::query_as("SELECT * FROM main.create_or_replace_index($1, $2, $3)")
                    .bind(index_type)
                    .bind(data_source)
                    .bind(regions)
                    .fetch_one(&context.pool)
                    .await
                    .context(error::DBError {
                        details: "Could not create or replace index",
                    })?;

            info!(context.logger, "Created index {:?}", index);

            let _environment: environments::environment::Environment =
                sqlx::query_as("SELECT * FROM main.add_index_to_background($1, $2)")
                    .bind(index.id)
                    .bind(id)
                    .fetch_one(&context.pool)
                    .await
                    .context(error::DBError {
                        details: "Could not add index to background",
                    })?;

            info!(context.logger, "Added index to background");
            Ok(())
        })
        .await?;

    sqlx::query_as("SELECT * FROM main.fetch_background_environment($1)")
        .bind(id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not fetch environment for background '{}'", id),
        })
}

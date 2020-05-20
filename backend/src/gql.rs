use super::model::{environments, features};
use juniper::{FieldResult, IntoFieldError};
use slog::{debug, Logger};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Context {
    pub pool: PgPool,
    pub logger: Logger,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Return a list of all features
    async fn features(&self, context: &Context) -> FieldResult<Vec<features::feature::Feature>> {
        debug!(context.logger, "Fetching All Features");
        features::feature::fetch_all_features(&context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return the feature corresponding to the given id.
    async fn feature(
        &self,
        id: Uuid,
        context: &Context,
    ) -> FieldResult<features::feature::Feature> {
        debug!(context.logger, "Fetching Feature with id '{}'", id);
        features::feature::fetch_feature_by_id(id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return the scenarios belonging to the feature specified by the given id.
    async fn scenarios(
        &self,
        id: Uuid,
        context: &Context,
    ) -> FieldResult<Vec<features::scenario::Scenario>> {
        debug!(
            context.logger,
            "Fetching scenarios from feature id '{}'", id
        );
        features::scenario::fetch_scenarios_by_feature_id(&id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return the steps belonging to the scenario specified by the given id.
    async fn steps(&self, id: Uuid, context: &Context) -> FieldResult<Vec<features::step::Step>> {
        debug!(context.logger, "Fetching steps from feature id '{}'", id);
        features::step::fetch_steps_by_scenario_id(&id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return a list of all environments
    async fn environments(
        &self,
        context: &Context,
    ) -> FieldResult<Vec<environments::environment::Environment>> {
        debug!(context.logger, "Fetching All Environments");
        environments::environment::fetch_all_environments(&context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn add_feature(
        name: String,
        description: String,
        tags: Vec<String>,
        context: &Context,
    ) -> FieldResult<features::feature::Feature> {
        debug!(context.logger, "Adding Feature {}", name);

        features::feature::create_or_replace_feature(
            uuid::Uuid::new_v4(),
            name,
            description,
            tags,
            &context,
        )
        .await
        .map_err(IntoFieldError::into_field_error)
    }

    async fn load_feature(
        feature: String,
        context: &Context,
    ) -> FieldResult<features::feature::Feature> {
        debug!(context.logger, "Loading Feature from string");

        features::feature::create_or_replace_feature_from_string(feature, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn delete_feature(
        id: Uuid,
        context: &Context,
    ) -> FieldResult<features::feature::Feature> {
        debug!(context.logger, "Dropping Feature '{}'", id);

        features::feature::delete_feature_by_id(id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}

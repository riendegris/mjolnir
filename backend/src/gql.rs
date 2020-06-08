use super::model::{environments, features};
use crate::get_connstr;
use futures::Stream;
use juniper::{FieldError, FieldResult, IntoFieldError, RootNode};
use slog::{debug, info, Logger};
use sqlx::postgres::{PgListener, PgPool};
use std::pin::Pin;
use uuid::Uuid;

#[derive(Debug, Clone)]
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
    // Temporarily disabled
    // async fn feature(
    //     &self,
    //     id: Uuid,
    //     context: &Context,
    // ) -> FieldResult<features::feature::Feature> {
    //     debug!(context.logger, "Fetching Feature with id '{}'", id);
    //     features::feature::fetch_feature_by_id(id, &context)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)
    // }

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

    /// Return the background belonging to the feature specified by the given id.
    async fn background(
        &self,
        id: Uuid,
        context: &Context,
    ) -> FieldResult<Option<features::background::Background>> {
        debug!(
            context.logger,
            "Fetching background from feature id '{}'", id
        );
        features::background::fetch_background_by_feature_id(&id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return the steps belonging to the scenario or the background specified by the given id.
    async fn steps(
        &self,
        id: Uuid,
        src: features::SourceType,
        context: &Context,
    ) -> FieldResult<Vec<features::step::Step>> {
        match src {
            features::SourceType::Scenario => {
                debug!(context.logger, "Fetching steps from scenario id '{}'", id);
                features::step::fetch_steps_by_scenario_id(&id, &context)
                    .await
                    .map_err(IntoFieldError::into_field_error)
            }
            features::SourceType::Background => {
                debug!(context.logger, "Fetching steps from background id '{}'", id);
                features::step::fetch_steps_by_background_id(&id, &context)
                    .await
                    .map_err(IntoFieldError::into_field_error)
            }
        }
    }

    /// Return the environment belonging to the scenario or the background specified by the given id.
    async fn environment(
        &self,
        id: Uuid,
        src: features::SourceType,
        context: &Context,
    ) -> FieldResult<Option<environments::environment::Environment>> {
        match src {
            features::SourceType::Scenario => {
                debug!(
                    context.logger,
                    "Fetching environment from scenario id '{}'", id
                );
                // FIXME Not implemented
                Ok(None)
            }
            features::SourceType::Background => {
                debug!(
                    context.logger,
                    "Fetching environment from background id '{}'", id
                );
                features::background::fetch_background_environment(&id, &context)
                    .await
                    .map_err(IntoFieldError::into_field_error)
                    .map(|e| Some(e))
            }
        }
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

    /// Return the indexes belonging to the environment specified by the given id.
    async fn indexes(
        &self,
        id: Uuid,
        context: &Context,
    ) -> FieldResult<Vec<environments::index::Index>> {
        debug!(
            context.logger,
            "Fetching indexes from environment id '{}'", id
        );
        environments::index::fetch_indexes_by_environment_id(&id, &context)
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

        features::feature::create_or_replace_feature(name, description, tags, &context)
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

    // This function returns the environment that correspond to the background specified by 'id'
    // If the environment doesn't exist, it is created, along with all the indexes that compose it.
    async fn background_environment(
        id: Uuid, // background id
        context: &Context,
    ) -> FieldResult<environments::environment::Environment> {
        debug!(context.logger, "Retrieving Background Environment '{}'", id);
        features::background::fetch_background_environment(&id, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}

type PayloadStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

pub struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    async fn notifications(context: &Context) -> PayloadStream {
        info!(context.logger, "Subscribing to database notifications");
        let connstr = get_connstr(context.logger.clone())
            .await
            .expect("connection");
        let mut listener = PgListener::new(&connstr).await.unwrap();
        listener.listen("notifications").await.unwrap();
        let logger = context.logger.clone();
        let stream = listener.into_stream().map(move |i| {
            let n = i.unwrap(); // FIXME
            info!(logger, "Received Postgres Notification {}", n.payload());
            // let feature: features::feature::Feature = serde_json::from_str(n.payload())
            //     .expect("Pg Notification should send deseriazable message");
            Ok(String::from(n.payload()))
        });

        Box::pin(stream)
    }
}

type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

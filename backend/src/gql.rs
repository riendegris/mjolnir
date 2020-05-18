use super::model::{env, features};
use juniper::{FieldResult, GraphQLObject, IntoFieldError};
use slog::{debug, Logger};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Context {
    pub pool: PgPool,
    pub logger: Logger,
}

impl juniper::Context for Context {}

#[derive(GraphQLObject, Debug)]
pub struct EnvBanosResp {
    pub error: Option<String>,
    pub data: Option<Vec<env::bano::Bano>>,
}

#[derive(GraphQLObject, Debug)]
pub struct EnvBanoResp {
    pub error: Option<String>,
    pub data: Option<env::bano::Bano>,
}

#[derive(GraphQLObject, Debug)]
pub struct EnvBanoItemResp {
    pub error: Option<String>,
    pub data: Option<env::bano::Item>,
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Return a list of Bano environments.
    async fn banos(&self, context: &Context) -> FieldResult<EnvBanosResp> {
        debug!(context.logger, "Querying BANO environments");

        match env::bano::fetch_banos(&context).await {
            Ok(banos) => Ok(EnvBanosResp {
                error: None,
                data: Some(banos),
            }),
            Err(err) => Ok(EnvBanosResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }

    /// Return a list of features environments.
    async fn features(&self, context: &Context) -> FieldResult<Vec<features::feature::Feature>> {
        debug!(context.logger, "Fetching All Features");
        features::feature::fetch_all_features(&context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    /// Return a list of features environments.
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
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn add_bano(
        bano_id: String,
        description: String,
        context: &Context,
    ) -> FieldResult<EnvBanoResp> {
        debug!(context.logger, "Querying BANO environments");

        match env::bano::check_and_insert_bano(&bano_id, &description, &context).await {
            Ok(item) => Ok(EnvBanoResp {
                error: None,
                data: Some(item),
            }),
            Err(err) => Ok(EnvBanoResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }
    async fn remove_bano(bano_id: String, context: &Context) -> FieldResult<EnvBanoResp> {
        debug!(context.logger, "Removing BANO environments");

        match env::bano::remove_bano(&bano_id, &context).await {
            Ok(item) => Ok(EnvBanoResp {
                error: None,
                data: Some(item),
            }),
            Err(err) => Ok(EnvBanoResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }
    async fn add_bano_item(
        bano_id: String,
        item_id: String,
        context: &Context,
    ) -> FieldResult<EnvBanoItemResp> {
        debug!(context.logger, "Querying BANO environments");

        match env::bano::check_and_insert_bano_item(&bano_id, &item_id, &context).await {
            Ok(item) => Ok(EnvBanoItemResp {
                error: None,
                data: Some(item),
            }),
            Err(err) => Ok(EnvBanoItemResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }
    async fn remove_bano_item(
        bano_id: String,
        item_id: String,
        context: &Context,
    ) -> FieldResult<EnvBanoItemResp> {
        debug!(context.logger, "Removing BANO environments");

        match env::bano::remove_bano_item(&bano_id, &item_id, &context).await {
            Ok(()) => Ok(EnvBanoItemResp {
                error: None,
                data: None,
            }),
            Err(err) => Ok(EnvBanoItemResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }
    async fn download_bano_item(
        bano_id: String,
        item_id: String,
        context: &Context,
    ) -> FieldResult<EnvBanoItemResp> {
        debug!(context.logger, "Downloading BANO item");

        match env::bano::download_bano_item(&bano_id, &item_id, &context).await {
            Ok(item) => Ok(EnvBanoItemResp {
                error: None,
                data: Some(item),
            }),
            Err(err) => Ok(EnvBanoItemResp {
                error: Some(format!("Bano Environments Error: {}", err)),
                data: None,
            }),
        }
    }

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
        debug!(context.logger, "Loading Feature");

        features::feature::create_or_replace_feature_from_string(feature, &context)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}

use chrono::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    row::{FromRow, Row},
};
use uuid::Uuid;

pub mod background;
pub mod feature;
pub mod scenario;
pub mod step;

// This structure is sometime returned by the database.
#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct IdTimestamp {
    pub id: Uuid,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for IdTimestamp {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(IdTimestamp {
            id: row.try_get(0)?,
            updated_at: row.try_get(1)?,
        })
    }
}

// A step can be part of a scenario, or a background.
// This enum helps differentiate between the two cases.
// FIXME The name, SourceType, is too general
#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, GraphQLEnum)]
#[sqlx(rename = "source_type")]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    #[sqlx(rename = "background")]
    Background,
    #[sqlx(rename = "scenario")]
    Scenario,
}

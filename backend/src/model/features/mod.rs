use chrono::prelude::*;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    row::{FromRow, Row},
};
use uuid::Uuid;

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
            id: row.get(0),
            updated_at: row.get(1),
        })
    }
}

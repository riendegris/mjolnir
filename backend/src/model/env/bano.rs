use crate::{error, gql, model::FileStatus, utils};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::prelude::*;
use futures::stream::{self, TryStreamExt};
use juniper::GraphQLObject;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use slog::{debug, info, Logger};
use snafu::ResultExt;
use sqlx::postgres::PgPool;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
use std::path::PathBuf;
use tokio::fs;
use tokio::prelude::*;

/// A Bano Environment consists in several BanoItem.
/// It can be identified by its label, and contains a description.
#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Bano {
    pub id: String,
    pub description: String,
    pub items: Vec<Item>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Bano {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Bano {
            id: row.get(0),
            description: row.get(1),
            items: vec![],
        })
    }
}

/// This is a single source file for Bano.
#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Item {
    pub id: String,
    pub filename: String,
    pub md5: String,   // hex formatted digest
    pub filesize: f64, // file size in Kilobytes (this is because GraphQLType only supports f64 or i32)
    pub filestatus: FileStatus,
    pub updated_at: DateTime<Utc>,
}

impl Item {
    pub fn new(id: String) -> Self {
        Item {
            id,
            filename: String::from(""),
            md5: String::from(""),
            filesize: 0.0,
            filestatus: FileStatus::NotAvailable,
            updated_at: Utc::now(),
        }
    }
}

pub async fn download_bano_item(
    bano_id: &str,
    item_id: &str,
    context: &gql::Context,
) -> Result<Item, error::Error> {
    // spawn a download task, and update the database to say its in progress
    let id = String::from(bano_id);
    let iid = String::from(item_id);
    let pool = context.pool.clone();
    let logger = context.logger.clone();
    // tokio::spawn(download_bano_item_task(id, iid, pool, logger));
    sqlx::query_as(
        "UPDATE main.env_bano_item
        SET (filestatus, updated_at) = ($1, $2)
        WHERE id = $3
        RETURNING *",
    )
    .bind(FileStatus::DownloadInProgress)
    .bind(Utc::now())
    .bind(item_id)
    .fetch_one(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not update BANO item",
    })
}

pub async fn download_bano_item_task(
    bano_id: String,
    item_id: String,
    pool: PgPool,
    logger: Logger,
) -> Result<Item, error::Error> {
    // Before downloading the file, make sure we have a place to store it.
    let workdir = utils::get_workdir(logger.clone()).await?;
    let mut path: PathBuf = [&workdir, "bano", &bano_id].iter().collect();
    fs::create_dir_all(path.clone())
        .await
        .context(error::TokioIOError)?;

    let filename = format!("bano-{}.csv", item_id);
    path.push(filename.clone());
    info!(logger, "Downloading bano to {}", path.display());

    // FIXME lot's of hardcoded values...
    let url = format!("http://bano.openstreetmap.fr/data/{}", filename);
    let content = reqwest::get(&url)
        .await
        .context(error::ReqwestError {
            details: format!("Could not download {} from {}", filename, url),
        })?
        .text()
        .await
        .context(error::ReqwestError {
            details: format!("Could not download {} from {}", filename, url),
        })?;

    info!(logger, "... download ok");
    let mut file = fs::File::create(path).await.context(error::TokioIOError)?;
    file.write_all(content.as_bytes())
        .await
        .context(error::TokioIOError)?;
    info!(logger, "... save ok");
    let digest = format!("{:x}", &Md5::digest(content.as_bytes()));
    // What follows is not very pretty.... Since GraphQL only supports f64, I take
    // the size in bytes, turn it into a bigdecimal which facilitates conversion to f64
    // divide by 1024
    let filesize =
        BigDecimal::from_u64(file.metadata().await.context(error::TokioIOError)?.len()).unwrap();
    let filesize = filesize / BigDecimal::from_u64(1024).unwrap();
    let filesize: f64 = filesize.to_f64().unwrap();
    sqlx::query_as(
        "UPDATE main.env_bano_item
            SET (filename, md5, filesize, filestatus, updated_at) = ($1, $2, $3, $4, $5)
            WHERE id = $6
            RETURNING *",
    )
    .bind(filename)
    .bind(digest)
    .bind(filesize)
    .bind(FileStatus::Available)
    .bind(Utc::now())
    .bind(item_id)
    .fetch_one(&pool)
    .await
    .context(error::DBError {
        details: "Could not update BANO item",
    })
}

impl<'c> FromRow<'c, PgRow<'c>> for Item {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Item {
            id: row.get(0),
            filename: row.get(1),
            md5: row.get(2),
            filesize: row.get(3),
            filestatus: row.get(4),
            updated_at: row.get(5),
        })
    }
}

/// Return a BANO, or None if not found.
pub async fn fetch_bano(id: &str, context: &gql::Context) -> Result<Option<Bano>, error::Error> {
    sqlx::query_as("SELECT * FROM main.env_bano WHERE id = $1")
        .bind(id)
        .fetch_optional(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not retrieve BANO environment",
        })
}

/// Return a BanoItem identified by its Bano Environment Id, and its id.
pub async fn fetch_bano_item(
    bano_id: &str,
    item_id: &str,
    context: &gql::Context,
) -> Result<Option<Item>, error::Error> {
    sqlx::query_as(
        "SELECT item.* FROM main.env_bano AS bano
            INNER JOIN main.env_bano_map AS map ON bano.id = map.env
            INNER JOIN main.env_bano_item AS item ON map.item = item.id
            WHERE bano.id = $1 AND item.id = $2",
    )
    .bind(bano_id)
    .bind(item_id)
    .fetch_optional(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve BANO item",
    })
}

/// Return a BanoItem identified by its Bano Environment Id, and its id.
pub async fn fetch_bano_items(
    bano_id: &str,
    context: &gql::Context,
) -> Result<Vec<Item>, error::Error> {
    sqlx::query_as(
        "SELECT item.* FROM main.env_bano AS bano
            INNER JOIN main.env_bano_map AS map ON bano.id = map.env
            INNER JOIN main.env_bano_item AS item ON map.item = item.id
            WHERE bano.id = $1",
    )
    .bind(bano_id)
    .fetch_all(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve BANO item",
    })
}

/// Insert a BanoItem identified by its Bano Environment Id, and its id.
pub async fn insert_bano_item(
    bano_id: &str,
    item_id: &str,
    context: &gql::Context,
) -> Result<Item, error::Error> {
    // FIXME This should be a transaction grouping both actions
    let item = sqlx::query_as("INSERT INTO main.env_bano_item (id) VALUES ($1) RETURNING *")
        .bind(item_id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not insert BANO item {}", item_id),
        })?;
    let conn = context.pool.acquire().await.context(error::DBError {
        details: "Could not get conn",
    })?;
    // FIXME should not use this method to build a query.
    sqlx::query(&format!(
        "INSERT INTO main.env_bano_map VALUES ('{}', '{}')",
        bano_id, item_id
    ))
    .execute(conn)
    .await
    .context(error::DBError {
        details: "Could not retrieve BANO item",
    })?;
    Ok(item)
}

/// Remove a BanoItem identified by its Bano Environment Id, and its id.
pub async fn remove_bano_item(
    bano_id: &str,
    item_id: &str,
    context: &gql::Context,
) -> Result<(), error::Error> {
    // FIXME This should be a transaction grouping both actions

    let conn = context.pool.acquire().await.context(error::DBError {
        details: "Could not get conn",
    })?;
    // FIXME should not use this method to build a query.
    sqlx::query(&format!(
        "DELETE FROM main.env_bano_map WHERE env = '{}' AND item = '{}'",
        bano_id, item_id
    ))
    .execute(conn)
    .await
    .context(error::DBError {
        details: "Could not remove BANO item",
    })?;
    let conn = context.pool.acquire().await.context(error::DBError {
        details: "Could not get conn",
    })?;
    // FIXME should not use this method to build a query.
    sqlx::query(&format!(
        "DELETE FROM main.env_bano_item AS item
        WHERE NOT EXISTS (
            SELECT FROM main.env_bano_map AS map
            WHERE map.item = item.id
            )"
    ))
    .execute(conn)
    .await
    .context(error::DBError {
        details: "Could not remove BANO item",
    })?;
    Ok(())
}

/// Insert a BanoItem identified by its Bano Environment Id, and its id.
pub async fn insert_bano(
    id: &str,
    description: &str,
    context: &gql::Context,
) -> Result<Bano, error::Error> {
    sqlx::query_as("INSERT INTO main.env_bano (id, description) VALUES ($1, $2) RETURNING *")
        .bind(id)
        .bind(description)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not insert BANO item",
        })
}

/// Remove a BanoItem identified by its Bano Environment Id, and its id.
pub async fn remove_bano(id: &str, context: &gql::Context) -> Result<Bano, error::Error> {
    let conn = context.pool.acquire().await.context(error::DBError {
        details: "Could not get conn",
    })?;
    // FIXME should not use this method to build a query.
    sqlx::query(&format!(
        "DELETE FROM main.env_bano_map WHERE env = '{}'",
        id
    ))
    .execute(conn)
    .await
    .context(error::DBError {
        details: "Could not remove BANO item",
    })?;
    sqlx::query_as("DELETE FROM main.env_bano WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not insert BANO item",
        })
}

pub async fn fetch_banos(context: &gql::Context) -> Result<Vec<Bano>, error::Error> {
    let banos = sqlx::query_as("SELECT * FROM main.env_bano")
        .fetch_all(&context.pool)
        .await
        .map(|rows| {
            let banos: Vec<Bano> = rows.into();
            // We need to wrap vector elements in a Result to make it compatible with try_fold
            // used below.
            banos.into_iter().map(|bano| Ok(bano))
        })
        .context(error::DBError {
            details: "Could not retrieve BANO item",
        })?;

    stream::iter(banos)
        .try_fold(vec![], |mut acc, bano| async move {
            let items = fetch_bano_items(&bano.id, &context).await?;
            acc.push(Bano {
                id: bano.id,
                description: bano.description,
                items,
            });
            Ok(acc)
        })
        .await
}

pub async fn check_and_insert_bano_item(
    bano_id: &str,
    item_id: &str,
    context: &gql::Context,
) -> Result<Item, error::Error> {
    // make sure there is a bano with that id
    if fetch_bano(&bano_id, context).await?.is_none() {
        info!(context.logger, "unknown id {}", bano_id);
        return Err(error::Error::UserError {
            details: String::from("Unknown bano id"),
        });
    }

    // see if there is a preexisting item
    if fetch_bano_item(&bano_id, &item_id, context)
        .await?
        .is_some()
    {
        return Err(error::Error::UserError {
            details: String::from("already a bano item with that id"),
        });
    }

    insert_bano_item(bano_id, item_id, context).await
}

pub async fn check_and_insert_bano(
    bano_id: &str,
    description: &str,
    context: &gql::Context,
) -> Result<Bano, error::Error> {
    // make sure there is a bano with that id
    if fetch_bano(&bano_id, context).await?.is_some() {
        info!(context.logger, "bano already exists id {}", bano_id);
        return Err(error::Error::UserError {
            details: String::from("duplicate bano id"),
        });
    }

    insert_bano(bano_id, description, context).await
}

use futures::{Stream, StreamExt};
use slog::{error, Logger};
use snafu::ResultExt;
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{self, filters::sse::ServerSentEvent};

pub mod error;
pub mod gql;
pub mod model;
pub mod utils;

pub type Schema =
    juniper::RootNode<'static, gql::Query, gql::Mutation, juniper::EmptySubscription<gql::Context>>;

pub async fn read_dotenv(_log: Logger) -> Result<(), error::Error> {
    dotenv::dotenv().context(error::EnvError {
        details: "Cannot find .env",
    })?;
    Ok(())
}

pub async fn get_connstr(_log: Logger) -> Result<String, error::Error> {
    dotenv::var("DATABASE_URL").context(error::EnvError {
        details: "Cannot find 'DATABASE_URL' in .env",
    })
}

pub async fn connect_db(connstr: String, _log: Logger) -> Result<PgPool, error::Error> {
    PgPool::builder()
        .max_size(5)
        .build(&connstr)
        .await
        .context(error::DBError {
            details: format!("Could not create a PgPool with connection {}", connstr),
        })
}

pub fn schema() -> Schema {
    Schema::new(
        gql::Query,
        gql::Mutation,
        juniper::EmptySubscription::<gql::Context>::new(),
    )
}

pub async fn db_notifications() -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    // let mut listener = PgListener::new("postgresql://odin:secret@postgres/mjolnir")
    //     .await
    //     .unwrap();
    // listener.listen("banos").await.unwrap();
    // let stream = listener.into_stream();
    // stream.map(|i| {
    //     let n = i.unwrap();
    //     Ok((
    //         sse::event(String::from(n.channel())),
    //         sse::data(String::from(n.payload())),
    //     ))
    // })
    let mut counter: u64 = 0;
    // create server event source
    interval(Duration::from_secs(1)).map(move |_| {
        counter += 1;
        sse_counter(counter)
    })
}

// create server-sent event
fn sse_counter(counter: u64) -> Result<impl ServerSentEvent, Infallible> {
    let msg = format!("{{ \"counter\": {} }}", counter);
    Ok((warp::sse::event("TEST_COUNTER"), warp::sse::data(msg)))
}

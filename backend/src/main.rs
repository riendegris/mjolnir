use futures::{future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use slog::{error, info, o, Drain, Logger};
use snafu::{futures::TryStreamExt as STSE, ResultExt};
use sqlx::postgres::{PgListener, PgNotification, PgPool};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{
    self,
    filters::sse::{self, ServerSentEvent},
    Filter,
};

mod error;
mod gql;
mod model;
mod utils;

type Schema =
    juniper::RootNode<'static, gql::Query, gql::Mutation, juniper::EmptySubscription<gql::Context>>;

#[tokio::main]
async fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    run(log).await;
}

async fn run(log: Logger) {
    run_error(log.clone()).await.unwrap_or_else(|err| {
        error!(log, "{}", err);
    })
}

async fn run_error(log: Logger) -> Result<(), error::Error> {
    let root_logger = log.new(o!());

    read_dotenv(log).await?;

    let pool = get_connstr(root_logger.clone())
        .and_then(|connstr| connect_db(connstr, root_logger.clone()))
        .await?;

    let gql_logger = root_logger.clone();
    let state = warp::any().map(move || gql::Context {
        pool: pool.clone(),
        logger: gql_logger.clone(),
    });

    /* This is ApiRoutes.EventSource */
    let events = warp::path("events")
        .and(warp::get())
        .and_then(|| async move {
            let stream: Result<_, Infallible> = Ok(warp::sse::reply(db_notifications().await));
            stream
        });

    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    let graphql_index = warp::path("graphiql")
        .and(warp::path::end())
        .and(warp::get())
        .and(juniper_warp::graphiql_filter("/graphql", None));

    /* This is ApiRoutes.Base */
    let graphql_query = warp::path!("graphql" / "v1").and(graphql_filter);

    let index = warp::fs::file("dist/index.html");

    let dir = warp::fs::dir("dist");

    let routes = graphql_index.or(graphql_query).or(events).or(dir).or(index);

    info!(root_logger.clone(), "Serving Mjolnir on 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn read_dotenv(_log: Logger) -> Result<(), error::Error> {
    dotenv::dotenv().context(error::EnvError {
        details: "Cannot find .env",
    })?;
    Ok(())
}

async fn get_connstr(_log: Logger) -> Result<String, error::Error> {
    dotenv::var("DATABASE_URL").context(error::EnvError {
        details: "Cannot find 'DATABASE_URL' in .env",
    })
}

async fn connect_db(connstr: String, _log: Logger) -> Result<PgPool, error::Error> {
    PgPool::builder()
        .max_size(5)
        .build(&connstr)
        .await
        .context(error::DBError {
            details: format!("Could not create a PgPool with connection {}", connstr),
        })
}

fn schema() -> Schema {
    Schema::new(
        gql::Query,
        gql::Mutation,
        juniper::EmptySubscription::<gql::Context>::new(),
    )
}

async fn db_notifications() -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
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

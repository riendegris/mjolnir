use futures::TryFutureExt;
use slog::{error, info, o, Drain, Logger};
use std::convert::Infallible;
use warp::{self, Filter};

use mjolnir::{self, error};

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

    mjolnir::read_dotenv(log).await?;

    let pool = mjolnir::get_connstr(root_logger.clone())
        .and_then(|connstr| mjolnir::connect_db(connstr, root_logger.clone()))
        .await?;

    let gql_logger = root_logger.clone();
    let state = warp::any().map(move || mjolnir::gql::Context {
        pool: pool.clone(),
        logger: gql_logger.clone(),
    });

    /* This is ApiRoutes.EventSource */
    let events = warp::path("events")
        .and(warp::get())
        .and_then(|| async move {
            let stream: Result<_, Infallible> =
                Ok(warp::sse::reply(mjolnir::db_notifications().await));
            stream
        });

    let graphql_filter = juniper_warp::make_graphql_filter(mjolnir::schema(), state.boxed());

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

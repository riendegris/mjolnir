use futures::{Future, FutureExt, TryFutureExt};
use juniper_subscriptions::Coordinator;
use juniper_warp::subscriptions::graphql_subscriptions;
use slog::{error, info, o, Drain, Logger};
use std::{pin::Pin, sync::Arc};
use warp::{self, Filter};

use mjolnir::{self, error, gql};

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

    let logger1 = root_logger.clone();
    let pool1 = pool.clone();
    let state = warp::any().map(move || gql::Context {
        pool: pool1.clone(),
        logger: logger1.clone(),
    });

    let graphiql = warp::path("graphiql")
        .and(warp::path::end())
        .and(warp::get())
        .and(juniper_warp::graphiql_filter("/graphql", None));

    let graphql_filter = juniper_warp::make_graphql_filter(gql::schema(), state.boxed());
    /* This is ApiRoutes.Base */
    let graphql = warp::path!("graphql").and(graphql_filter);

    let logger2 = root_logger.clone();
    let pool2 = pool.clone();
    let substate = warp::any().map(move || gql::Context {
        pool: pool2.clone(),
        logger: logger2.clone(),
    });

    let coordinator = Arc::new(juniper_subscriptions::Coordinator::new(gql::schema()));

    let notifications = (warp::path("notifications")
        .and(warp::ws())
        .and(substate.clone())
        .and(warp::any().map(move || Arc::clone(&coordinator)))
        .map(
            |ws: warp::ws::Ws,
             context: gql::Context,
             coordinator: Arc<Coordinator<'static, _, _, _, _, _>>| {
                ws.on_upgrade(|websocket| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                    println!("On upgrade");
                    graphql_subscriptions(websocket, coordinator, context)
                        .map(|r| {
                            println!("r: {:?}", r);
                            if let Err(err) = r {
                                println!("Websocket Error: {}", err);
                            }
                        })
                        .boxed()
                })
            },
        ))
    .map(|reply| warp::reply::with_header(reply, "Sec-Websocket-Protocol", "graphql-ws"));

    let index = warp::fs::file("dist/index.html");

    let dir = warp::fs::dir("dist");

    let routes = graphiql.or(graphql).or(notifications).or(dir).or(index);

    info!(root_logger.clone(), "Serving Mjolnir on 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

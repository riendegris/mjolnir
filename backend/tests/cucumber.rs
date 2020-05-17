use cucumber_rust::{after, before, cucumber};
use futures::TryFutureExt;
use gherkin_rust::Feature;
use slog::{o, Drain};
use tokio::runtime::Runtime;
use uuid::Uuid;

pub struct MyWorld {
    context: mjolnir::gql::Context,
    feature: Feature, // feature, as read from file.
    id: Uuid,         // id of the feature returned by the loading operation.
    name: String,     // name of the feature returned by the fetching operation.
}

impl cucumber_rust::World for MyWorld {}

impl std::default::Default for MyWorld {
    /// This function is called every time a new scenario is started
    fn default() -> MyWorld {
        let context = get_gql_context();
        MyWorld {
            feature: Feature {
                name: String::from(""),
                description: None,
                background: None,
                scenarios: Vec::new(),
                rules: Vec::new(),
                tags: None,
                position: (0, 0),
            },
            context,
            id: Uuid::new_v4(), // this value will get overwritten
            name: String::new(),
        }
    }
}

// #[tokio::main]
// async fn main() {
//     let output = DebugOutput::new();
//     let instance = {
//         let mut instance = CucumberBuilder::new(output);
//
//         instance
//             .features(vec![PathBuf::from("./tests/data")])
//             .steps(Steps::combine([example_steps::steps].iter().map(|f| f())));
//
//         instance
//     };
//
//     let res = instance.command_line();
//
//     if !res {
//         std::process::exit(1);
//     }
// }

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./tests/data",
    world: ::MyWorld,
    steps: &[
        example_steps::steps
    ],
    setup: setup,
    before: &[a_before_fn],
    after: &[an_after_fn]
}

mod example_steps {
    use cucumber_rust::steps;
    use gherkin_rust::Feature;
    use slog::info;
    use std::convert::TryFrom;
    use std::path::PathBuf;

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given regex r#"^I am loading a feature from file '(.*)'$"# (String) |world, filename, _step| {
            // We read the feature from file, use gherkin to turn it into a feature.
            let filepath = PathBuf::from(filename);
            world.feature = Feature::try_from(filepath.as_path()).unwrap();
            // Then we extract a few values from that feature, and prepare a GraphQL statement.
            let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
            variables.insert(String::from("name"), juniper::InputValue::scalar(world.feature.name.clone()));
            variables.insert(String::from("description"), juniper::InputValue::scalar(world.feature.description.clone().unwrap_or(String::from(""))));
            variables.insert(String::from("tags"), juniper::InputValue::scalar(world.feature.tags.clone().map_or_else(
                        || String::from(""),
                        | tags | tags.join(", ")
                        )));
            // Using GraphQL is in the async world, so we need a runtime...
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // execute the sql statement.
                let (res, errs) = juniper::execute(
                    r#"mutation($name: String!, $description: String!, $tags: String!) {
                        addFeature(name: $name, description: $description, tags: $tags) {
                            id, updatedAt
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();

                assert!(errs.is_empty());

                // and extract the id of the feature to store it in the world
                info!(world.context.logger, "{}", res);
                world.id = uuid::Uuid::parse_str(
                    res.as_object_value().unwrap()
                    .get_field_value("addFeature").unwrap()
                    .as_object_value().unwrap()
                    .get_field_value("id").unwrap()
                    .as_string_value().unwrap()
                    ).unwrap();

            });

        };

        when r#"I search for the feature by id"# |world, _step| {
            // Now we use the id we got in step one as a key for searching for a feature
            let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
            variables.insert(String::from("id"), juniper::InputValue::scalar(world.id.to_string()));
            // Again we use a runtime for running async code
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (res, errs) = juniper::execute(
                    r#"query($id: Uuid!) {
                        feature(id: $id) {
                            id, name
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();
                assert!(errs.is_empty());

                // this time we store the name of the feature
                world.name = String::from(res.as_object_value().unwrap()
                    .get_field_value("feature").unwrap()
                    .as_object_value().unwrap()
                    .get_field_value("name").unwrap()
                    .as_string_value().unwrap());

            });
        };

        then r#"I can find that feature and verify its name"# |world, __step| {
            assert_eq!(world.name, world.feature.name);
        };
    });
}

fn get_gql_context() -> mjolnir::gql::Context {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());

        let logger = log.new(o!());

        mjolnir::read_dotenv(logger.clone()).await.unwrap();

        let pool = mjolnir::get_connstr(logger.clone())
            .and_then(|connstr| mjolnir::connect_db(connstr, logger.clone()))
            .await
            .unwrap();

        mjolnir::gql::Context {
            pool: pool.clone(),
            logger: logger.clone(),
        }
    })
}

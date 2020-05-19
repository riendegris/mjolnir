use cucumber_rust::{after, before, cucumber};
use futures::TryFutureExt;
use gherkin_rust::Feature;
use slog::{o, Drain};
use tokio::runtime::Runtime;
use uuid::Uuid;

pub struct MyWorld {
    context: mjolnir::gql::Context,
    feature: Feature,      // feature, as read from file.
    id: Uuid,              // id of the feature returned by the loading operation.
    name: String,          // name of the feature returned by fetching the feature back.
    scenario_count: usize, // count of scenarios returned by fetching scenarios.
    step_count: usize,
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
                tags: Vec::new(),
                span: (0, 0),
            },
            context,
            id: Uuid::new_v4(), // this value will get overwritten
            name: String::new(),
            scenario_count: 0,
            step_count: 0,
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
    features: "./features",
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
    use slog::{info, warn};
    //use std::convert::TryFrom;
    use std::path::PathBuf;

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given regex r#"^I am loading a feature from file '(.*)'$"# (String) |world, filename, _step| {
            // We read the feature from file, use gherkin to turn it into a feature.
            let filepath = PathBuf::from(filename);
            world.feature = Feature::parse_path(filepath.as_path()).unwrap();

            let feature = std::fs::read_to_string(filepath).unwrap();
            // Then we extract a few values from that feature, and prepare a GraphQL statement.
            let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
            variables.insert(String::from("feature"), juniper::InputValue::scalar(feature));

            // Using GraphQL is in the async world, so we need a runtime...
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // execute the sql statement.
                let (res, errs) = juniper::execute(
                    r#"mutation($feature: String!) {
                        loadFeature(feature: $feature) {
                            id, updatedAt
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();

                if !errs.is_empty() {
                    for err in errs {
                      warn!(world.context.logger, "{:?}", err);
                    }
                    assert!(false, "errors occured while executing a graphql statement for adding a feature")
                }

                // and extract the id of the feature to store it in the world
                info!(world.context.logger, "{}", res);
                world.id = uuid::Uuid::parse_str(
                    res.as_object_value().unwrap()
                    .get_field_value("loadFeature").unwrap()
                    .as_object_value().unwrap()
                    .get_field_value("id").unwrap()
                    .as_string_value().unwrap()
                    ).unwrap();

                // assert!(false);
            });

        };

        when r#"I search for the feature by id"# |world, _step| {
            // Now we use the id we got in the 'given' step as a key for searching the feature
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

                if !errs.is_empty() {
                    for err in errs {
                      warn!(world.context.logger, "{:?}", err);
                    }
                    assert!(false, "errors occured while executing a graphql statement for searching a feature")
                }

                // this time we store the name of the feature
                world.name = String::from(res.as_object_value().unwrap()
                    .get_field_value("feature").unwrap()
                    .as_object_value().unwrap()
                    .get_field_value("name").unwrap()
                    .as_string_value().unwrap());

            });
        };

        when r#"I search for the scenarios by id"# |world, _step| {
            // Now we use the id we got in the 'given' step as a key for searching the feature
            let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
            variables.insert(String::from("id"), juniper::InputValue::scalar(world.id.to_string()));
            // Again we use a runtime for running async code
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (res, errs) = juniper::execute(
                    r#"query($id: Uuid!) {
                        scenarios(id: $id) {
                            id, name
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();

                if !errs.is_empty() {
                    for err in errs {
                      warn!(world.context.logger, "{:?}", err);
                    }
                    assert!(false, "errors occured while executing a graphql statement for searching scenarios")
                }

                // this time we store the name of the feature
                world.scenario_count = res.as_object_value().unwrap()
                    .get_field_value("scenarios").unwrap()
                    .as_list_value().unwrap()
                    .len();
            });
        };

        when r#"I search for the steps belonging to the first scenario"# |world, _step| {

            // Now we use the id we got in the 'given' step as a key for searching the feature
            let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
            variables.insert(String::from("id"), juniper::InputValue::scalar(world.id.to_string()));
            // Again we use a runtime for running async code
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (res, errs) = juniper::execute(
                    r#"query($id: Uuid!) {
                        scenarios(id: $id) {
                            id, name
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();

                if !errs.is_empty() {
                    for err in errs {
                      warn!(world.context.logger, "{:?}", err);
                    }
                    assert!(false, "errors occured while executing a graphql statement for searching scenarios")
                }

                // Here we extract the id of the first scenario.
                let id = res.as_object_value().unwrap()
                    .get_field_value("scenarios").unwrap()
                    .as_list_value().unwrap()[0]
                    .as_object_value().unwrap()
                    .get_field_value("id").unwrap()
                    .as_string_value().unwrap();

                // Now we call graphql to get the list of steps belonging to the scenario with that
                // id.
                let mut variables: juniper::Variables<juniper::DefaultScalarValue> = juniper::Variables::new();
                variables.insert(String::from("id"), juniper::InputValue::scalar(id));

                let (res, errs) = juniper::execute(
                    r#"query($id: Uuid!) {
                        steps(id: $id) {
                            id, value
                        }
                    }"#,
                    None,
                    &mjolnir::schema(),
                    &variables,
                    &world.context
                    ).await.unwrap();

                if !errs.is_empty() {
                    for err in errs {
                      warn!(world.context.logger, "{:?}", err);
                    }
                    assert!(false, "errors occured while executing a graphql statement for searching steps")
                }

                // this time we store the count of steps
                world.step_count = res.as_object_value().unwrap()
                    .get_field_value("steps").unwrap()
                    .as_list_value().unwrap()
                    .len();

            });
        };

        then r#"I find that feature and verify its name"# |world, __step| {
            assert_eq!(world.name, world.feature.name);
        };

        then r#"I find that I have the correct number of scenarios"# |world, __step| {
            assert_eq!(world.scenario_count, world.feature.scenarios.len());
        };

        then r#"I find that I have the correct number of steps"# |world, __step| {
            assert_eq!(world.step_count, world.feature.scenarios[0].steps.len());
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

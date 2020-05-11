# Mjolnir

A set of tools to test [mimmirsbrunn](https://github.com/canaltp/mimirsbrunn) (or bring developpers
to their tears).

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for
development and testing purposes. See deployment for notes on how to deploy the project on a live
system.

### Prerequisites

The project has 3 components:

* The frontend is a web application, which may be eventually combined with a command line tool.
* The backend is a web server
* The database, for storing the information stored by the web server.

Each of these components may be built on the same machine, or in three different ones. They
interact through a network.

The frontend is a Vue.js application, and you need [npm](https://www.npmjs.com) to build it.
The backend is a rust application, and your need [rust](https://www.rust-lang.org/) to build it.
The database is [PostgreSQL](https://www.postgresql.org)

Installing these components heavily depends on your OS / distribution.

### Installing

We'll build each component in turn, and ensure each stage is built correctly before moving on.

#### 1. Create the database

```sh
cd database
./provision.sh
```

#### 2. Create the backend

```sh
cd backend
cargo build --release
```

This results in a binary under `backend/target/release/mjolnir`, which is the server to which the
frontend will connect.

#### 3. Create the frontend

```sh
cd frontend
npm run build
```

This results in a directory `frontend/dist`, which will be served by the backend.

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Billie Thompson** - *Initial work* - [PurpleBooth](https://github.com/PurpleBooth)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc



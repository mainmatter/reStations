# reStations

This is an implementation of the [OSDM API spec](https://osdm.io)'s [Places endpoint](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/UnionInternationalCheminsdeFer/OSDM/master/specification/v3.3/OSDM-online-api-v3.3.0.yml&nocors#tag/Places), backed by [Trainline EU's stations dataset](https://github.com/trainline-eu/stations).

The easiest way to use reStations is as a Docker container using the [image that we published including the dataset](https://hub.docker.com/r/mainmatter/restations):

```
docker run -p 3000:3000 --rm mainmatter/restations
curl localhost:3000/places
```

New versions of the image are published regularly as the dataset is updated.

## Endpoints supported

### GET /places

Request all places:
```
curl localhost:3000/places
```

### GET /places/{id}

Fetch Lisboa Santa Apolónia station with its UIC:
```
curl localhost:3000/places/8721428
```

### POST /places (search)

Search stations with `Lisboa` in its name.

(Works with both Portuguese and English versions, and in [other languages](https://github.com/trainline-eu/stations/blob/master/src/main/resources/languages.json) as well):
```
curl -X POST -H "Content-Type: application/json" \
-d '{"placeInput": {"name": "Lisboa"}}' \
localhost:3000/places
```

By geo position:
```
curl -X POST -H "Content-Type: application/json" \
-d '{"placeInput": {"geoPosition": {"latitude": 38.109417, "longitude": 13.367472}}}' \
localhost:3000/places
```

(`restrictions` request fields support coming soon for POST /places)

## Working with reStations

`reStations` can also be used directly as a Rust project. To run the project, import the data into a local SQLite database first:

```
./scripts/sync-data
```

Next, export an environment variable to point to the database:

```
export DATABASE_URL=sqlite:stations.sqlite.db
```

This is important since reStations uses sqlx which does compile-time checks on the database schema and needs the `DATABASE_URL` environment variable to be able to connect to the database.

Then run the applications from the project root:

```
cargo run
```

### Prerequisites

* Rust (install via [rustup](https://rustup.rs))
* sqlite3 (see [SQLite](https://www.sqlite.org))

### Project Structure

Distinct parts of the project are separated into separate crates:

```
.
├── cli    // CLI tools for generating project files
├── config // Defines the `Config` struct and handles building the configuration from environment-specific TOML files and environment variables
├── macros // Contains macros for application tests
└── web    // The web interface as well as tests for it
```

#### Environment

The project uses `.env` and `.env.test` files to store configuration settings for the development and test environments respectively. Those files are read automatically by the parts of the application that require configuration settings to be present.

### Commands

Running the application in development mode:

```
cargo run
```

Running the application tests:

```
cargo test
```

Generating project files like entities, controllers, tests, etc. (see the [CLI create](./cli/README.md) for detailed documentation):

```
cargo generate
```

Building the project's docs:

### Building documentation

Build the project's documentation with:

```
cargo doc --workspace --all-features
```

# License

Copyright © 2025- Mainmatter GmbH (https://mainmatter.com), released under the
[MIT](./LICENSE-MIT) and [Apache](./LICENSE-APACHE) licenses.

[Trainline EU's stations dataset](https://github.com/trainline-eu/stations) is released under the [Open Data Commons Open Database License v1.0 license](https://github.com/trainline-eu/stations/blob/master/LICENCE.txt) which allows private and commercial use.

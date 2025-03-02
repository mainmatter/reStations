# reStations

This is an implementation of the [OSDM API spec](https://osdm.io)'s [Places endpoint](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/UnionInternationalCheminsdeFer/OSDM/master/specification/v3.3/OSDM-online-api-v3.3.0.yml&nocors#tag/Places), backed by [Trainline EU's stations dataset](https://github.com/trainline-eu/stations).

The easiest way to use reStations is as a Docker container that comes pre-published including the dataset:

```
docker run -it -p 3000:3000 mainmatter/reStations
```

New versions of the image are published regularly as the dataset is updated.

## Working with reStations

reStations can also be used directly as a Rust project. To run the project, import the data into a local SQLite database first:

```
./scripts/sync-data
```

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

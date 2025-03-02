# restations

This README explains how to collaborate on this Gerust application.

<add a description of the project here>

## Prerequisites

* Rust (install via [rustup](https://rustup.rs))

## Project Structure

Distinct parts of the project are separated into separate crates:

```
.
├── cli    // CLI tools for generating project files
├── config // Defines the `Config` struct and handles building the configuration from environment-specific TOML files and environment variables
├── macros // Contains macros for application tests
└── web    // The web interface as well as tests for it
```

### Environment

The project uses `.env` and `.env.test` files to store configuration settings for the development and test environments respectively. Those files are read automatically by the parts of the application that require configuration settings to be present.

## Commands

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

## Building documentation

Build the project's documentation with:

```
cargo doc --workspace --all-features
```

# License

Copyright © 2025- Mainmatter GmbH (https://mainmatter.com), released under the
[MIT](./LICENSE-MIT) and [Apache](./LICENSE-APACHE) licenses.

[Trainline EU's stations dataset](https://github.com/trainline-eu/stations) is released under the [Open Data Commons Open Database License v1.0 license](https://github.com/trainline-eu/stations/blob/master/LICENCE.txt) which allows private and commercial use.

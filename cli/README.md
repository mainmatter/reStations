# restations-cli

This crate contains a CLI for managing the database and creating project files like controllers, entities, middleware, or tests.

_You should not need to make any changes to this crate._

## Managing the database

Preparing a schema file for offline compile-time query checking as SQLx does:

```
cargo db prepare
```

## Generating project files

Project files are generated with the

```
cargo generate
```

command. The CLI comes with commands for generating middlewares, controllers, controller tests, CRUD controllers and tests for those, migrations, and entities. To get help for each of the controllers, use the `-h` flag, e.g.:

```
cargo generate controller -h
```

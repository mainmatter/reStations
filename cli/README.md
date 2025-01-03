# restations-cli

This crate contains a CLI for creating project files like controllers, middleware, or tests.

_You should not need to make any changes to this crate._

## Generating project files

Project files are generated with the

```
cargo generate
```

command. The CLI comes with commands for generating middlewares, controllers, controller tests, CRUD controllers and tests for those. To get help for each of the controllers, use the `-h` flag, e.g.:

```
cargo generate controller -h
```

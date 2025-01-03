# restations-web

This crate implements the application's web interface. It contains controllers and middleware and is responsible for booting up the application which includes setting up tracing.

## Application state

The code for defining the application state and creating a fresh state when the application boots, is located in `state.rs`.

```rs
#[derive(Clone)]
pub struct AppState {
    
}
```

The `AppState` struct can be freely extended with custom fields.

## Routing

Routes are defined in `src/routes.rs`, e.g.:

```rs
pub fn init_routes(app_state: AppState) -> Router {
Router::new()
    .route("/tasks", post(create_task))
    .route("/tasks", get(get_tasks))
    .route("/tasks/:id", get(get_task))
```

## Controllers and Middlewares

Controllers and middlewares are kept in the respectively named directories. Controllers export axum request handlers. Middlewares are standard Tower middlewares.

## Tests

Gerust follows a full stack testing approach. The application's endpoint including database access are tested via tests in the `web` crate. Using Gerust's test macros, tests receive a fully configured and booted up instance of the application that requests can be made against:.

#[test]
async fn test_hello(context: &TestContext) {
    let response = context.app.request("/greet").send().await;

    let greeting: Greeting = response.into_body().into_json().await;
    assert_that!(greeting.hello, eq(String::from("world")));
}
### Test helpers

The restations-web crate includes test helpers in `src/test_helpers` that add a number of convience functions for easier issuing of requests and parsing of responses. Those helpers depend on the `test-helpers` feature flag which is automatically enabled when running tests but not for production builds. _You should not need to make any changes to these helpers._

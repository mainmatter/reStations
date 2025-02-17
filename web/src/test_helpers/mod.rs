use std::sync::Arc;

use crate::routes::init_routes;
use crate::state::AppState;
use axum::{
    body::{Body, Bytes},
    http::{Method, Request},
    response::Response,
    Router,
};
use hyper::header::{HeaderMap, HeaderName};
use restations_config::{load_config, Config, Environment};
use std::cell::OnceCell;
use tower::ServiceExt;

use crate::db;

/// A request that a test sends to the application.
///
/// TestRequests are constructed via the test context (see[`TestContext`]).
///
/// Example:
/// ```
/// let response = context
///     .app
///     .request("/greet")
///     .method(Method::GET)
///     .send()
///     .await;
/// ```
pub struct TestRequest {
    router: Router,
    uri: String,
    method: Method,
    headers: HeaderMap,
    body: Body,
}

impl TestRequest {
    fn new(router: Router, uri: &str) -> Self {
        Self {
            router,
            uri: String::from(uri),
            headers: HeaderMap::new(),
            body: Body::empty(),
            method: Method::GET,
        }
    }

    /// Sets the HTTP method for the request, e.g. GET or POST, see [`axum::http::Method`].
    #[allow(unused)]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Adds an HTTP header to the request.
    ///
    /// Header names must be passed as [`hyper::header::HeaderName`] while values can be passed as [`&str`]s.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/greet")
    ///     .method(Method::GET)
    ///     .header(.header(http::header::CONTENT_TYPE, "application/json"))
    ///     .await;
    /// ```
    #[allow(unused)]
    pub fn header(mut self, name: HeaderName, value: &str) -> Self {
        self.headers.insert(name, value.parse().unwrap());
        self
    }

    /// Sets the body for the request.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/tasks")
    ///     .method(Method::POST)
    ///     .body(Body::from(json!({
    ///         "description": "get milk!",
    ///     }).to_string()))
    /// ```
    #[allow(unused)]
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    /// Sends the request to the application under test.
    #[allow(unused)]
    pub async fn send(self) -> Response {
        let mut request_builder = Request::builder().uri(&self.uri);

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }

        request_builder = request_builder.method(&self.method);

        let request = request_builder.body(self.body);

        self.router.oneshot(request.unwrap()).await.unwrap()
    }
}

/// Testing convenience functions for [`axum::Router`].
pub trait RouterExt {
    /// Creates a [`TestRequest`] pointed at the application under test.
    #[allow(unused)]
    fn request(&self, uri: &str) -> TestRequest;
}

impl RouterExt for Router {
    #[allow(unused)]
    fn request(&self, uri: &str) -> TestRequest {
        TestRequest::new(self.clone(), uri)
    }
}

/// Testing convenience functions for [`axum::body::Body`].
pub trait BodyExt {
    /// Returns the body as raw bytes.
    #[allow(unused, async_fn_in_trait)]
    async fn into_bytes(self) -> Bytes;

    /// Returns the body as parsed JSON.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/tasks")
    ///     .method(Method::GET)
    ///     .send()
    ///     .await;
    ///
    /// let tasks: Vec<Task> = response.into_body().into_json::<Vec<Task>>().await;
    /// ```
    #[allow(unused, async_fn_in_trait)]
    async fn into_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned;
}

impl BodyExt for Body {
    #[allow(unused)]
    async fn into_bytes(self) -> Bytes {
        // We don't care about the size limit in tests.
        axum::body::to_bytes(self, usize::MAX)
            .await
            .expect("Failed to read response body")
    }

    #[allow(unused)]
    async fn into_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let body = self.into_bytes().await;
        serde_json::from_slice::<T>(&body).expect("Failed to deserialize JSON body")
    }
}
#[allow(clippy::test_attr_in_doctest)]
/// Provides context information for application tests.
///
/// A `TestContext` is passed as an argument to tests marked with the [`restations_macros::test`] attribute macro. It is used to access the application under test.
///
/// Example:
/// ```
/// #[test]
/// async fn test_hello(context: &TestContext) {
///     let response = context.app.request("/greet").send().await;
///
///     let greeting: Greeting = response.into_body().into_json().await;
///     assert_that!(greeting.hello, eq(String::from("world")));
/// }
/// ```
pub struct TestContext {
    /// The application that is being tested.
    pub app: Router,
    pub pool: Arc<db::Pool>,
}

/// Sets up a test and returns a [`TestContext`].
///
/// This function initializes a new instance of the application under test using the configuration for [`restations_config::Environment::Test`].
///
/// This function is not invoked directly but used inside of the [`restations_macros::test`] attribute macro. The test context is automatically passed to test cases marked with that macro as an argument.
pub async fn setup() -> TestContext {
    let init_config: OnceCell<Config> = OnceCell::new();
    let _config = init_config.get_or_init(|| load_config(&Environment::Test).unwrap());

    let pool = Arc::new(db::create_pool());
    let app = init_routes(AppState { pool: pool.clone() });

    TestContext { app, pool }
}

use googletest::prelude::{assert_that,eq};
use restations_macros::test;
use restations_web::test_helpers::{BodyExt, RouterExt, TestContext};

use restations_web::controllers::stations::StationsListResponse;

#[test]
async fn test_list_empty(context: &TestContext) {
    let response = context.app.request("/stations").send().await;
    let stations: StationsListResponse = response.into_body().into_json().await;

    assert_that!(stations, eq(&StationsListResponse::new()));
}

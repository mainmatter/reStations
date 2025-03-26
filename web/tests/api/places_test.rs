use axum::{
    body::Body,
    http::{self, Method},
};
use fake::{Fake, Faker};
use googletest::prelude::{assert_that, eq};
use restations_db::{entities::stations, test_helpers};
use restations_macros::db_test;
use restations_web::osdm::{
    OsdmInitialPlaceInput, OsdmPlaceRequest, OsdmPlaceResponse, OsdmPlaceRestrictions, OsdmProblem,
};
use restations_web::test_helpers::{BodyExt, DbTestContext, RouterExt};
use serde_json::json;

// GET /places
//
#[db_test]
async fn test_list_empty(context: &DbTestContext) {
    let response = context.app.request("/places").send().await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(0));
}

#[db_test]
async fn test_list_ok(context: &DbTestContext) {
    let changeset: stations::StationChangeset = Faker.fake();
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();
    let response = context.app.request("/places").send().await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(1));
}

// POST /places
// Search by name
//
#[db_test]
async fn test_search_by_name_ok(context: &DbTestContext) {
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Berlin");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Bremen");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(OsdmPlaceRequest {
        restrictions: None,
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Berlin")),
            geo_position: None,
        }),
    });
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(1));

    assert_that!(&response_body.places[0].name, eq("Berlin"));
}

#[db_test]
async fn test_search_by_name_with_results_limit_ok(context: &DbTestContext) {
    // Create two stations so we can test the results limit
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Überlingen");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Berlin-Lichtenberg");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Bremen");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(OsdmPlaceRequest {
        restrictions: Some(OsdmPlaceRestrictions {
            number_of_results: Some(1)
        }),
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Berlin")),
            geo_position: None,
        }),
    });
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(1));

    assert_that!(&response_body.places[0].name, eq("Überlingen"));
}

#[db_test]
async fn test_search_other_languages(context: &DbTestContext) {
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Sevilla");
    changeset.info_fr = Some(String::from("Seville"));
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Berlin");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(OsdmPlaceRequest {
        restrictions: None,
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Seville")),
            geo_position: None,
        })
    });
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(1));
}

// POST /places
// Search by geo position
//
#[db_test]
async fn test_search_geo_position(context: &DbTestContext) {
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("London Charing Cross");
    changeset.latitude = Some(51.507);
    changeset.longitude = Some(-0.123);
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("London Waterloo");
    changeset.latitude = Some(51.503);
    changeset.longitude = Some(-0.113);
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    // London Charing Cross
    //
    // Note: we're posting json here to assert camelcasing of request structs
    // is in place
    let payload = r#"
        {
            "placeInput": {
                "geoPosition": {
                    "latitude": 51.508362,
                    "longitude": -0.123835
                }
            }
        }
    "#;
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    // 20 is the limit on the results
    assert_that!(response_body.places.len(), eq(2));

    let first = &response_body.places[0];
    assert_that!(first.name, eq("London Charing Cross"));

    let second = &response_body.places[1];
    assert_that!(second.name, eq("London Waterloo"));
}

#[db_test]
async fn test_search_geo_position_with_results_limit(context: &DbTestContext) {
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("London Charing Cross");
    changeset.latitude = Some(51.507);
    changeset.longitude = Some(-0.123);
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("London Waterloo");
    changeset.latitude = Some(51.503);
    changeset.longitude = Some(-0.113);
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    // Note: we're posting json here to assert camelcasing of request structs
    // is in place
    //
    // London Charing Cross
    let payload = r#"
        {
        "restrictions": { "numberOfResults": 1 },
        "placeInput": {
                "geoPosition": {
                    "latitude": 51.508362,
                    "longitude": -0.123835
                }
            }
        }
    "#;
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    // 20 is the default limit on the results
    assert_that!(response_body.places.len(), eq(1));

    let first = &response_body.places[0];
    assert_that!(first.name, eq("London Charing Cross"));
}

// POST /places
// Weird requests that we still gracefully handle
//

#[db_test]
async fn test_search_unknown_parameters(context: &DbTestContext) {
    // Note: we're posting json here to assert camelcasing of request structs
    // is in place
    let payload = r#"
        {
            "placeInput": {
                "name": "Lisbon"
            },
            "unknown": {
                "parameter": "here"
            }
        }
    "#;
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));
}

#[db_test]
async fn test_search_missing_parameters(context: &DbTestContext) {
    let payload = r#"
        {
            "unknown": {
                "parameter": "here"
            }
        }
    "#;
    let response = context
        .app
        .request("/places")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;
    assert_that!(response.status(), eq(200));
}

// GET /places/{id}
//
#[db_test]
async fn test_show_ok(context: &DbTestContext) {
    let mut changeset: stations::StationChangeset = Faker.fake();
    changeset.name = String::from("Test Station");
    changeset.uic = String::from("9430007");
    test_helpers::stations::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context.app.request("/places/9430007").send().await;
    assert_that!(response.status(), eq(200));

    let response_body: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(response_body.places.len(), eq(1));
    let place = &response_body.places[0];
    assert_that!(place.id, eq("urn:uic:stn:9430007"));
    assert_that!(place.object_type, eq("StopPlace"));
    assert_that!(place.name, eq("Test Station"));
}

#[db_test]
async fn test_show_not_found(context: &DbTestContext) {
    let response = context.app.request("/places/1").send().await;
    assert_that!(response.status(), eq(404));

    let problem: OsdmProblem = response.into_body().into_json::<OsdmProblem>().await;

    assert_that!(problem.code, eq("not-found"));
    assert_that!(problem.title, eq("Could not find place!"));
}

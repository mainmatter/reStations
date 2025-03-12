use axum::{
    body::Body,
    http::{self, Method},
};
use googletest::prelude::{assert_that, eq, gt};
use restations_macros::test;
use restations_web::controllers::places::*;
use restations_web::test_helpers::{BodyExt, RouterExt, TestContext};
use serde_json::json;

// GET /places
//
#[test]
async fn test_list_ok(context: &TestContext) {
    let response = context.app.request("/places").send().await;
    assert_that!(response.status(), eq(200));

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1));
}

#[test]
async fn test_search_ok(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1));
}

#[test]
async fn test_search_other_languages(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1));
}

#[test]
async fn test_search_geo_position(context: &TestContext) {
    // Palermo Centrale
    let payload = json!(OsdmPlaceRequest {
        place_input: Some(OsdmInitialPlaceInput {
            name: None,
            geo_position: Some(OsdmGeoPosition {
                latitude: 38.109417,
                longitude: 13.367472,
            })
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    // 20 is the limit on the results
    assert_that!(api_place.places.len(), eq(20));
}

// TODO test that Palermo Centrale is the first result

// TODO test when either lat or lon is missing

#[test]
async fn test_search_unknown_parameters(context: &TestContext) {
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1));
}

#[test]
async fn test_search_missing_parameters(context: &TestContext) {
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1000));
}

// GET /places/{id}
//
#[test]
async fn test_show_ok(context: &TestContext) {
    let response = context.app.request("/places/9430007").send().await;
    assert_that!(response.status(), eq(200));

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), eq(1));
    let place = &api_place.places[0];
    assert_that!(place.id, eq("urn:uic:stn:9430007"));
    assert_that!(place.object_type, eq("StopPlace"));
    assert_that!(
        place.geo_position.as_ref().unwrap(),
        eq(&OsdmGeoPosition {
            latitude: 38.71387,
            longitude: -9.122271
        })
    );
}

#[test]
async fn test_show_not_found(context: &TestContext) {
    let response = context.app.request("/places/1").send().await;
    assert_that!(response.status(), eq(404));

    let problem: OsdmProblem = response.into_body().into_json::<OsdmProblem>().await;

    assert_that!(problem.code, eq("not-found"));
    assert_that!(problem.title, eq("Could not find place with id #1"));
}

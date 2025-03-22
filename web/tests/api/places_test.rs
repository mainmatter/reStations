use axum::{
    body::Body,
    http::{self, Method},
};
use googletest::prelude::{assert_that, eq, gt};
use restations_macros::db_test;
use restations_web::osdm::{
    OsdmGeoPosition, OsdmInitialPlaceInput, OsdmPlaceRequest, OsdmPlaceResponse, OsdmProblem,
};
use restations_web::test_helpers::{BodyExt, DbTestContext, RouterExt};
use serde_json::json;

// GET /places
//
#[db_test]
async fn test_list_ok(context: &DbTestContext) {
    let response = context.app.request("/places").send().await;
    assert_that!(response.status(), eq(200));

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1));
}

#[db_test]
async fn test_search_ok(context: &DbTestContext) {
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

#[db_test]
async fn test_search_other_languages(context: &DbTestContext) {
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

#[db_test]
async fn test_search_geo_position(context: &DbTestContext) {
    // London Charing Cross
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    // 20 is the limit on the results
    assert_that!(api_place.places.len(), eq(20));

    let first = &api_place.places[0];
    assert_that!(first.name, eq("London Charing Cross"));

    let second = &api_place.places[1];
    assert_that!(second.name, eq("London Waterloo"));
}

// TODO test when either lat or lon is missing

#[db_test]
async fn test_search_unknown_parameters(context: &DbTestContext) {
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

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), gt(1000));
}

// GET /places/{id}
//
#[db_test]
async fn test_show_ok(context: &DbTestContext) {
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

#[db_test]
async fn test_show_not_found(context: &DbTestContext) {
    let response = context.app.request("/places/1").send().await;
    assert_that!(response.status(), eq(404));

    let problem: OsdmProblem = response.into_body().into_json::<OsdmProblem>().await;

    assert_that!(problem.code, eq("not-found"));
    assert_that!(problem.title, eq("Could not find place!"));
}

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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), gt(1));
}

#[test]
async fn test_search_ok(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Berlin")),
            geo_position: None,
        }),
        restrictions: Some(OsdmPlaceRestrictions {
            number_of_results: Some(1),
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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), eq(1));
}

#[test]
async fn test_search_other_languages(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Seville")),
            geo_position: None,
        }),
        restrictions: Some(OsdmPlaceRestrictions {
            number_of_results: Some(1),
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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), eq(1));
}

#[test]
async fn test_search_geo_position(context: &TestContext) {
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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    // 20 is the limit on the results
    assert_that!(places_response.places.len(), eq(20));
    // Validate the order of search results
    let expected_places = vec![
        "London Charing Cross",
        "London Waterloo",
        "London Waterloo (East)",
        "London Blackfriars",
        "City Thameslink",
        "Farringdon",
        "London Victoria",
        "London Cannon Street",
        "London Euston",
        "Elephant & Castle",
        "London Kings Cross",
        "Vauxhall",
        "Moorgate",
        "London St Pancras International",
        "London Bridge",
        "London Fenchurch Street",
        "Old Street",
        "London",
        "London Marylebone",
        "London Liverpool Street",
    ];
    assert_that!(
        places_response
            .places
            .into_iter()
            .map(|place| place.name)
            .collect::<Vec<String>>(),
        eq(&expected_places)
    );
}

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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), gt(1));
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

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), eq(20));
}

// GET /places/{id}
//
#[test]
async fn test_show_ok(context: &TestContext) {
    let response = context.app.request("/places/9430007").send().await;
    assert_that!(response.status(), eq(200));

    let places_response: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(places_response.places.len(), eq(1));
    let place = &places_response.places[0];
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

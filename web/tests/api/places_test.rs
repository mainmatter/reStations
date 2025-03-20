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

    // 23722 was the number of places when last checked, but this may vary with each build.
    // So we just check that there are more than 1 place.
    assert_that!(places_response.places.len(), gt(1));
}

#[test]
async fn test_search_by_name(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Berlin")),
            geo_position: None,
        }),
        restrictions: None,
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

    assert_that!(places_response.places.len(), eq(20));
    let expected_places = vec![
        "Überlingen",
        "Berlin-Lichtenberg",
        "Berlin Flughafen BER - Terminal 5 (Schönefeld)",
        "Berlin Zoologischer Garten",
        "Berlin Südkreuz",
        "Berlin-Charlottenburg",
        "Berlin Ostbahnhof",
        "Berlin-Spandau",
        "Berlin Hbf",
        "Berlin Wannsee",
        "Hamburg Berliner Tor",
        "Oberlindhart",
        "Oberlinxweiler",
        "Überlingen Therme",
        "Überlingen-Nußdorf",
        "Berlin-Karlshorst",
        "Berlin-Schöneweide",
        "Berlin-Lichterfelde Ost",
        "Berlin-Karow",
        "Berlin Gesundbrunnen",
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
async fn test_search_by_name_with_restriction(context: &TestContext) {
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
    assert_that!(places_response.places[0].name, eq("Überlingen"));
}

#[test]
async fn test_search_name_in_another_language(context: &TestContext) {
    let payload = json!(OsdmPlaceRequest {
        place_input: Some(OsdmInitialPlaceInput {
            name: Some(String::from("Seville")),
            geo_position: None,
        }),
        restrictions: None,
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

    assert_that!(places_response.places.len(), eq(4));
    let expected_places = vec![
        "Sevilla San Bernardo",
        "Sevilla Plaza de Armas",
        "Sevilla Santa Justa",
        "Sevilla-Virgen del Roció",
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
async fn test_search_other_languages_with_restrictions(context: &TestContext) {
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

    // Default limit on results is 20
    assert_that!(places_response.places.len(), eq(20));
    // Validate the order of search results
    let expected_places = vec![
        "London Charing Cross",
        "London Waterloo",
        "London Waterloo (East)",
        "London Blackfriars",
        "City Thameslink",
        "Farringdon",
        "London Kings Cross",
        "London Euston",
        "Vauxhall",
        "London St Pancras International",
        "London Victoria",
        "Elephant & Castle",
        "London Cannon Street",
        "Moorgate",
        "Caledonian Road & Barnsbury",
        "Camden Road",
        "London Bridge",
        "Old Street",
        "Battersea Park",
        "Wandsworth Road",
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
async fn test_search_geo_position_with_restrictions(context: &TestContext) {
    // London Charing Cross
    let payload = r#"
        {
            "placeInput": {
                "geoPosition": {
                    "latitude": 51.508362,
                    "longitude": -0.123835
                }
            },
            "restrictions": {
                "numberOfResults": 3
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

    // Default limit on results is 20
    assert_that!(places_response.places.len(), eq(3));
    // Validate the order of search results
    let expected_places = vec![
        "London Charing Cross",
        "London Waterloo",
        "London Waterloo (East)",
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
async fn test_search_name_and_geo_position(context: &TestContext) {
    // London Charing Cross
    let payload = r#"
        {
            "placeInput": {
                "name": "London Charing Cross",
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

    assert_that!(places_response.places.len(), eq(1));
    assert_that!(places_response.places[0].name, eq("London Charing Cross"));
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

    // When no params are posted, we just search for all places
    // and return the first 20
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

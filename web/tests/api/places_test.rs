use googletest::prelude::{assert_that, eq};
use restations_macros::test;
use restations_web::test_helpers::{BodyExt, RouterExt, TestContext};

use restations_web::db;
use restations_web::types::osdm::*;

use restations_web::types::station_record::StationRecord;

#[test]
async fn test_list_ok(context: &TestContext) {
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn).expect("Could not create DB tables");
    // Lisbon Santa Apolónia
    let station1 = StationRecord {
        uic: String::from("9430007"),
        latitude: Some(38.71387),
        longitude: Some(-9.122271),
        ..Default::default()
    };
    // Sevilla Santa Justa
    let station2 = StationRecord {
        uic: String::from("7151003"),
        latitude: Some(37.391925),
        longitude: Some(-5.975264),
        ..Default::default()
    };
    let _ = db::insert_station(&dbconn, &station1).expect("Could not insert station in DB");
    let _ = db::insert_station(&dbconn, &station2).expect("Could not insert station in DB");

    let response = context.app.request("/places").send().await;
    assert_that!(response.status(), eq(200));

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), eq(2));
    let place = &api_place.places[0];
    assert_that!(place.id, eq("9430007"));
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
async fn test_show_ok(context: &TestContext) {
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn).expect("Could not create DB tables");
    // Lisbon Santa Apolónia
    let test_station = StationRecord {
        uic: String::from("9430007"),
        latitude: Some(38.71387),
        longitude: Some(-9.122271),
        ..Default::default()
    };
    let _ = db::insert_station(&dbconn, &test_station).expect("Could not insert station in DB");

    let response = context.app.request("/places/9430007").send().await;
    assert_that!(response.status(), eq(200));

    let api_place: OsdmPlaceResponse = response.into_body().into_json().await;

    assert_that!(api_place.places.len(), eq(1));
    let place = &api_place.places[0];
    assert_that!(place.id, eq("9430007"));
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
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn).expect("Could not create DB tables");

    let response = context.app.request("/places/1").send().await;
    assert_that!(response.status(), eq(404));

    let problem: OsdmProblem = response.into_body().into_json::<OsdmProblem>().await;

    assert_that!(problem.code, eq("not-found"));
    assert_that!(problem.title, eq("Could not find place with id #1"));
}

use googletest::prelude::{assert_that, eq};
use restations_macros::test;
use restations_web::test_helpers::{BodyExt, RouterExt, TestContext};

use restations_web::controllers::places::{ApiPlaceResponse, ApiProblem};
use restations_web::db;

use restations_web::types::station_record::StationRecord;

#[test]
async fn test_show_ok(context: &TestContext) {
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn).expect("Could not create DB tables");
    let test_station = StationRecord {
        id: 1,
        latitude: String::from("40.416729"),
        longitude: String::from("-3.703339"),
        ..Default::default()
    };
    let _ = db::insert_station(&dbconn, &test_station).expect("Could not insert station in DB");

    let response = context.app.request("/places/1").send().await;
    let response: ApiPlaceResponse = response.into_body().into_json::<ApiPlaceResponse>().await;

    assert_that!(response.places.len(), eq(1));
    let place = &response.places[0];
    assert_that!(place.id, eq(1));
    assert_that!(place.object_type, eq("StopPlace"));
    assert_that!(place.geo_position.latitude, eq(40.416729));
    assert_that!(place.geo_position.longitude, eq(-3.703339));
}

#[test]
async fn test_show_not_found(context: &TestContext) {
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn).expect("Could not create DB tables");

    let response = context.app.request("/places/1").send().await;
    let problem: ApiProblem = response.into_body().into_json::<ApiProblem>().await;

    assert_that!(problem.code, eq("not-found"));
    assert_that!(problem.title, eq("Could not find place with id #1"));
}

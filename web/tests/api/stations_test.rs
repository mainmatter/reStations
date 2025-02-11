use googletest::prelude::{assert_that,eq};
use restations_macros::test;
use restations_web::test_helpers::{BodyExt, RouterExt, TestContext};

use restations_web::db;
use restations_web::controllers::stations::StationsListResponse;

use restations_web::types::station_record::StationRecord;

#[test]
async fn test_list_empty(context: &TestContext) {
    let response = context.app.request("/stations").send().await;
    let stations: StationsListResponse = response.into_body().into_json().await;

    assert_that!(stations, eq(&StationsListResponse::new()));
}

#[test]
async fn test_list_single_record(context: &TestContext) {
    let dbconn = context.pool.get().unwrap();
    let _ = db::create_tables(&dbconn)
        .expect("Could not create DB tables");
    let test_station = StationRecord {
        id: 1,
        name: String::from("Test Station"),
        slug: String::from("test-station"),
        ..Default::default()
    };
    let _ = db::insert_station(&dbconn, &test_station)
        .expect("Could not insert station in DB");

    let response = context.app.request("/stations").send().await;
    let stations: StationsListResponse = response.into_body().into_json::<StationsListResponse>().await;

    assert_that!(stations.len(), eq(1));
    let station = &stations[0];
    assert_that!(station.id, eq(1));
    assert_that!(station.name, eq("Test Station"));
    assert_that!(station.slug, eq("test-station"));
}

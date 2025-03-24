#[cfg(feature = "test-helpers")]
use fake::{
    faker::{address::en::*, boolean::en::Boolean},
    Dummy,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Sqlite;
use std::f64::consts::PI;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Station {
    pub id: i64,
    pub name: String,
    pub uic: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country: Option<String>,
    pub country_hint: bool,
    pub info_de: Option<String>,
    pub info_en: Option<String>,
    pub info_es: Option<String>,
    pub info_fr: Option<String>,
    pub info_it: Option<String>,
    pub info_nb: Option<String>,
    pub info_nl: Option<String>,
    pub info_cs: Option<String>,
    pub info_da: Option<String>,
    pub info_hu: Option<String>,
    pub info_ja: Option<String>,
    pub info_ko: Option<String>,
    pub info_pl: Option<String>,
    pub info_pt: Option<String>,
    pub info_ru: Option<String>,
    pub info_sv: Option<String>,
    pub info_tr: Option<String>,
    pub info_zh: Option<String>,
}

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct StationChangeset {
    #[cfg_attr(feature = "test-helpers", dummy(faker = "100..1000000"))]
    pub id: i64,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "CityName()"))]
    #[validate(length(min = 1))]
    pub name: String,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "8000000..10000000"))]
    pub uic: String,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "Latitude()"))]
    pub latitude: Option<f64>,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "Longitude()"))]
    pub longitude: Option<f64>,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "CountryName()"))]
    pub country: Option<String>,
    #[cfg_attr(feature = "test-helpers", dummy(faker = "Boolean(1)"))]
    pub country_hint: bool,
    pub info_de: Option<String>,
    pub info_en: Option<String>,
    pub info_es: Option<String>,
    pub info_fr: Option<String>,
    pub info_it: Option<String>,
    pub info_nb: Option<String>,
    pub info_nl: Option<String>,
    pub info_cs: Option<String>,
    pub info_da: Option<String>,
    pub info_hu: Option<String>,
    pub info_ja: Option<String>,
    pub info_ko: Option<String>,
    pub info_pl: Option<String>,
    pub info_pt: Option<String>,
    pub info_ru: Option<String>,
    pub info_sv: Option<String>,
    pub info_tr: Option<String>,
    pub info_zh: Option<String>,
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Sqlite>,
) -> Result<Vec<Station>, crate::Error> {
    let stations = sqlx::query_as!(Station, "SELECT id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations")
        .fetch_all(executor)
        .await?;
    Ok(stations)
}

pub async fn load(
    id: i64,
    executor: impl sqlx::Executor<'_, Database = Sqlite>,
) -> Result<Station, crate::Error> {
    match sqlx::query_as!(
        Station,
        "SELECT id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic = ?",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(station) => Ok(station),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn search_by_name(
    name: &str,
    executor: impl sqlx::Executor<'_, Database = Sqlite>,
) -> Result<Vec<Station>, crate::Error> {
    let pattern = format!("%{}%", name);
    let stations = sqlx::query_as!(Station, "SELECT id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh from stations  WHERE uic IS NOT NULL AND (name like ? OR info_de like ? OR info_en like ? OR info_es like ? OR info_fr like ? OR info_it like ? OR info_nb like ? OR info_nl like ? OR info_cs like ? OR info_da like ? OR info_hu like ? OR info_ja like ? OR info_ko like ? OR info_pl like ? OR info_pt like ? OR info_ru like ? OR info_sv like ? OR info_tr like ? OR info_zh like ?)", pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern, pattern)
          .fetch_all(executor)
          .await?;
    Ok(stations)
}

pub async fn search_by_position(
    latitude: f64,
    longitude: f64,
    executor: impl sqlx::Executor<'_, Database = Sqlite>,
) -> Result<Vec<Station>, crate::Error> {
    // First, get a larger set of candidates using a bounding box
    // This is more efficient for the initial filtering
    //
    // TODO: extract into a constant that is visible
    // and perhaps configurable as an env var
    let approx_distance_deg = 1.0; // Roughly 100km at equator

    let mut stations = sqlx::query_as!(
        Station,
        r#"
        SELECT id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh
        FROM stations
        WHERE
            latitude IS NOT NULL
            AND longitude IS NOT NULL
            AND latitude BETWEEN ? - ? AND ? + ?
            AND longitude BETWEEN ? - ? AND ? + ?
        "#,
        latitude,
        approx_distance_deg,
        latitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg
    )
    .fetch_all(executor)
    .await?;

    stations.sort_by_cached_key(|place| {
        let place_lat = place.latitude.expect("Latitude is not present");
        let place_lon = place.longitude.expect("Longitude is not present");
        let distance = haversine_distance(latitude, longitude, place_lat, place_lon);
        (distance * 10000f64) as i64
    });

    // Return the closest 20
    Ok(stations.into_iter().take(20).collect())
}

pub async fn search_by_name_and_position(
    name: &str,
    latitude: f64,
    longitude: f64,
    executor: impl sqlx::Executor<'_, Database = Sqlite>,
) -> Result<Vec<Station>, crate::Error> {
    // First, get a larger set of candidates using a bounding box
    // This is more efficient for the initial filtering
    //
    // TODO: extract into a constant that is visible
    // and perhaps configurable as an env var
    let approx_distance_deg = 1.0; // Roughly 100km at equator

    let name_pattern = format!("%{}%", name.to_lowercase());
    let mut stations = sqlx::query_as!(
        Station,
        r#"
        SELECT id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh
        FROM stations
        WHERE
            lower(name) LIKE ?
            AND latitude IS NOT NULL
            AND longitude IS NOT NULL
            AND latitude BETWEEN ? - ? AND ? + ?
            AND longitude BETWEEN ? - ? AND ? + ?
        "#,
        name_pattern,
        latitude,
        approx_distance_deg,
        latitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg
    )
    .fetch_all(executor)
    .await?;

    stations.sort_by_cached_key(|place| {
        let place_lat = place.latitude.expect("Latitude is not present");
        let place_lon = place.longitude.expect("Longitude is not present");
        let distance = haversine_distance(latitude, longitude, place_lat, place_lon);
        let name_score = if place.name.to_lowercase() == name.to_lowercase() {
            0.0
        } else {
            1.0
        };

        ((name_score + (distance / 100.0)) * 10000f64) as i64
    });

    // Return the closest 20
    Ok(stations.into_iter().take(20).collect())
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius_km = 6371.0;

    let lat1_rad = lat1 * PI / 180.0;
    let lon1_rad = lon1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let lon2_rad = lon2 * PI / 180.0;

    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin()
        + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_km * c
}

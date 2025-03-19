use crate::db::error::DbError;
use crate::db::pool::DbPool;
use crate::db::station_record::StationRecord;
use std::f64::consts::PI;

pub struct Search;

impl Search {
    pub async fn all(db: &DbPool, limit: i32) -> Result<Vec<StationRecord>, DbError> {
        let stations = sqlx::query_as!(
            StationRecord,
            "SELECT id, name, uic, latitude, longitude, country, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic IS NOT NULL LIMIT ?",
            limit
       )
       .fetch_all(db)
       .await?;

        Ok(stations)
    }

    pub async fn by_name(
        db: &DbPool,
        name: &str,
        limit: i32,
    ) -> Result<Vec<StationRecord>, DbError> {
        let pattern = format!("%{}%", name);
        let stations = sqlx::query_as!(
            StationRecord,
            "SELECT id, name, uic, latitude, longitude, country, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic IS NOT NULL AND (name like $1 OR info_de like $1 OR info_en like $1 OR info_es like $1 OR info_fr like $1 OR info_it like $1 OR info_nb like $1 OR info_nl like $1 OR info_cs like $1 OR info_da like $1 OR info_hu like $1 OR info_ja like $1 OR info_ko like $1 OR info_pl like $1 OR info_pt like $1 OR info_ru like $1 OR info_sv like $1 OR info_tr like $1 OR info_zh like $1) LIMIT $2",
            pattern,
            limit
        )
        .fetch_all(db)
        .await?;

        Ok(stations)
    }

    pub async fn by_place_id(db: &DbPool, place_id: &String) -> Result<StationRecord, DbError> {
        sqlx::query_as!(
            StationRecord,
            "SELECT id, name, uic, latitude, longitude, country, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic = $1",
            place_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(DbError::RecordNotFound(format!(
            "Could not find station with uic #{}",
            &place_id
        )))
    }

    /// Search for stations near a specific geographic point
    pub async fn by_position(
        pool: &DbPool,
        latitude: f64,
        longitude: f64,
        limit: i32,
    ) -> Result<Vec<StationRecord>, DbError> {
        // First, get a larger set of candidates using a bounding box
        // This is more efficient for the initial filtering
        //
        // TODO: extract into a constant that is visible
        // and perhaps configurable as an env var
        let approx_distance_deg = 1.0; // Roughly 100km at equator

        let query = sqlx::query_as!(
            StationRecord,
            r#"
            SELECT *
            FROM stations
            WHERE
                latitude IS NOT NULL
                AND longitude IS NOT NULL
                AND latitude BETWEEN ? - ? AND ? + ?
                AND longitude BETWEEN ? - ? AND ? + ?
            LIMIT ?
            "#,
            latitude,
            approx_distance_deg,
            latitude,
            approx_distance_deg,
            longitude,
            approx_distance_deg,
            longitude,
            approx_distance_deg,
            limit
        );

        // Fetch candidates
        let mut stations = query.fetch_all(pool).await?;
        stations.sort_by_cached_key(|place| {
            let place_lat = place.latitude.expect("Latitude is not present");
            let place_lon = place.longitude.expect("Longitude is not present");
            let distance = haversine_distance(latitude, longitude, place_lat, place_lon);
            (distance * 10000f64) as i64
        });

        Ok(stations)
    }

    /// Search for stations by name and proximity to geographic coordinates
    pub async fn by_name_and_position(
        pool: &DbPool,
        name: &str,
        latitude: f64,
        longitude: f64,
        limit: i32,
    ) -> Result<Vec<StationRecord>, DbError> {
        // First, get stations matching name within a bounding box
        let approx_distance_deg = 1.0; // Roughly 100km at equator

        let name_pattern = format!("%{}%", name.to_lowercase());
        let query = sqlx::query_as!(
            StationRecord,
            r#"
            SELECT *
            FROM stations
            WHERE
                lower(name) LIKE ?
                AND latitude IS NOT NULL
                AND longitude IS NOT NULL
                AND latitude BETWEEN ? - ? AND ? + ?
                AND longitude BETWEEN ? - ? AND ? + ?
            LIMIT ?
            "#,
            name_pattern,
            latitude,
            approx_distance_deg,
            latitude,
            approx_distance_deg,
            longitude,
            approx_distance_deg,
            longitude,
            approx_distance_deg,
            limit
        );

        // Fetch candidates
        let mut stations = query.fetch_all(pool).await?;
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

        Ok(stations)
    }
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

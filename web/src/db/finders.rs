use crate::db::error::DbError;
use crate::db::pool::DbPool;
use crate::db::station_record::StationRecord;
use std::f64::consts::PI;

pub async fn find_station(db: &DbPool, place_id: &String) -> Result<StationRecord, DbError> {
    sqlx::query_as!(
        StationRecord,
        "SELECT id, name, uic, latitude, longitude, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic = $1",
        place_id
    )
    .fetch_optional(db)
    .await?
    .ok_or(DbError::RecordNotFound(format!(
        "Could not find station with uic #{}",
        &place_id
    )))
}

pub async fn find_all_stations(db: &DbPool) -> Result<Vec<StationRecord>, DbError> {
    let stations = sqlx::query_as!(
        StationRecord,
        "SELECT id, name, uic, latitude, longitude, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic IS NOT NULL"
    )
    .fetch_all(db)
    .await?;
    Ok(stations)
}

pub async fn search_all_stations(db: &DbPool, name: &str) -> Result<Vec<StationRecord>, DbError> {
    let pattern = format!("%{}%", name);
    let stations = sqlx::query_as!(StationRecord, "SELECT id, name, uic, latitude, longitude, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh from stations  WHERE uic IS NOT NULL AND (name like $1 OR info_de like $1 OR info_en like $1 OR info_es like $1 OR info_fr like $1 OR info_it like $1 OR info_nb like $1 OR info_nl like $1 OR info_cs like $1 OR info_da like $1 OR info_hu like $1 OR info_ja like $1 OR info_ko like $1 OR info_pl like $1 OR info_pt like $1 OR info_ru like $1 OR info_sv like $1 OR info_tr like $1 OR info_zh like $1)", pattern)
            .fetch_all(db)
            .await?;
    Ok(stations)
}

/// Search for stations near a specific geographic point
pub async fn search_stations_by_position(
    pool: &DbPool,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<StationRecord>, DbError> {
    // First, get a larger set of candidates using a bounding box
    // This is more efficient for the initial filtering
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
        "#,
        latitude,
        approx_distance_deg,
        latitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg,
        longitude,
        approx_distance_deg
    );

    // Fetch candidates
    let db_stations = query.fetch_all(pool).await?;
    // Convert to GeoPositionSearchResult, which includes custom fields for calculating distance
    let mut geo_search_results: Vec<GeoPositionSearchResult> =
        db_stations.into_iter().map(Into::into).collect();

    // Calculate actugeo_search_resultsal distances and sort
    for result in &mut geo_search_results {
        if let (Some(lat), Some(lon)) = (result.station.latitude, result.station.longitude) {
            // Add a custom field for distance (we'll use this for sorting)
            result.distance = Some(haversine_distance(latitude, longitude, lat, lon));
        }
    }

    // Sort by distance
    geo_search_results.sort_by(|a, b| {
        a.distance
            .unwrap_or(f64::MAX)
            .partial_cmp(&b.distance.unwrap_or(f64::MAX))
            .unwrap()
    });

    let db_stations: Vec<StationRecord> = geo_search_results.into_iter().map(Into::into).collect();

    // Return the closest 20
    Ok(db_stations.into_iter().take(20).collect())
}

/// Search for stations by name and proximity to geographic coordinates
pub async fn search_stations_by_name_and_position(
    pool: &DbPool,
    name: &str,
    latitude: f64,
    longitude: f64,
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
    );

    // Fetch candidates
    let db_stations = query.fetch_all(pool).await?;
    // Convert to GeoPositionSearchResult, which includes custom fields for calculating distance
    let mut geo_search_results: Vec<GeoPositionSearchResult> =
        db_stations.into_iter().map(Into::into).collect();

    // Calculate scores based on name match and distance
    for result in &mut geo_search_results {
        if let (Some(lat), Some(lon)) = (result.station.latitude, result.station.longitude) {
            let distance = haversine_distance(latitude, longitude, lat, lon);
            result.distance = Some(distance);

            // Calculate a relevance score (lower is better)
            // We weight exact name matches higher than partial matches
            let name_score = if result.station.name.to_lowercase() == name.to_lowercase() {
                0.0
            } else {
                1.0
            };

            // Combined score (we'll store this temporarily in the distance field)
            // Lower score = better match
            result.relevance_score = Some(name_score + (distance / 100.0));
        }
    }

    // Sort by combined relevance score
    geo_search_results.sort_by(|a, b| {
        a.relevance_score
            .unwrap_or(f64::MAX)
            .partial_cmp(&b.relevance_score.unwrap_or(f64::MAX))
            .unwrap()
    });

    let db_stations: Vec<StationRecord> = geo_search_results.into_iter().map(Into::into).collect();

    // Return the closest 20
    Ok(db_stations.into_iter().take(20).collect())
}

/// Calculate haversine distance in kilometers between two points
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

// Needed for geo_position searches, where two computed
// fields are required for sorting by distance:
// - distance: The distance from the given position.
// - relevance_score: The relevance score of the station.
struct GeoPositionSearchResult {
    /// The base station record
    pub station: StationRecord,

    /// Distance from the search point in kilometers
    pub distance: Option<f64>,

    /// Computed relevance score (lower is better)
    pub relevance_score: Option<f64>,
}

impl GeoPositionSearchResult {
    /// Create a new GeoPositionSearchResult from a StationRecord
    pub fn new(station: StationRecord) -> Self {
        Self {
            station,
            distance: None,
            relevance_score: None,
        }
    }
}

impl From<StationRecord> for GeoPositionSearchResult {
    fn from(station: StationRecord) -> Self {
        Self::new(station)
    }
}

impl From<GeoPositionSearchResult> for StationRecord {
    fn from(result: GeoPositionSearchResult) -> Self {
        result.station
    }
}

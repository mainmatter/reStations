use crate::db::error::DbError;
use crate::db::pool::DbPool;
use crate::db::station_record::StationRecord;

// Approximate distance measured in degrees of latitude/longitude.
// This value is used to create a geographic bounding box around a specified point.
pub const APPROXIMATE_DISTANCE: f32 = 1.0; // Roughly 100km at equator

pub struct Search;

impl Search {
    pub async fn all(db: &DbPool, limit: i32) -> Result<Vec<StationRecord>, DbError> {
        let stations = sqlx::query_as!(
            StationRecord,
            r#"SELECT id, name, uic, latitude, longitude, country, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh
            FROM stations
            WHERE uic IS NOT NULL
            LIMIT $1"#,
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
            r#"SELECT id, name, uic, latitude, longitude, country, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh
            FROM stations
            WHERE uic IS NOT NULL
            AND (name like $1 OR info_de like $1 OR info_en like $1 OR info_es like $1 OR info_fr like $1 OR info_it like $1 OR info_nb like $1 OR info_nl like $1 OR info_cs like $1 OR info_da like $1 OR info_hu like $1 OR info_ja like $1 OR info_ko like $1 OR info_pl like $1 OR info_pt like $1 OR info_ru like $1 OR info_sv like $1 OR info_tr like $1 OR info_zh like $1)
            LIMIT $2"#,
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
        let approx_distance_deg = APPROXIMATE_DISTANCE;

        // Ordering by distance using Pythagorean calculation
        // 6371 is approximate radius of earth in km
        let query = sqlx::query_as!(
            StationRecord,
            r#"
            SELECT *
            FROM stations
            WHERE
                latitude IS NOT NULL
                AND longitude IS NOT NULL
                AND latitude BETWEEN $1 - $3 AND $1 + $3
                AND longitude BETWEEN $2 - $3 AND $2 + $3
            ORDER BY
                ((latitude - $1) * (latitude - $1)) +
                ((longitude - $2) * (longitude - $2))
            ASC
            LIMIT $4
            "#,
            latitude,
            longitude,
            approx_distance_deg,
            limit
        );

        // Fetch candidates
        let stations = query.fetch_all(pool).await?;

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
        let approx_distance_deg = APPROXIMATE_DISTANCE;
        let name_pattern = format!("%{}%", name.to_lowercase());

        // Use the same efficient distance calculation in SQL as by_position()
        let query = sqlx::query_as!(
            StationRecord,
            r#"
            SELECT *
            FROM stations
            WHERE
                (lower(name) LIKE $1 OR
                 lower(info_de) LIKE $1 OR
                 lower(info_en) LIKE $1 OR
                 lower(info_es) LIKE $1 OR
                 lower(info_fr) LIKE $1 OR
                 lower(info_it) LIKE $1 OR
                 lower(info_nb) LIKE $1 OR
                 lower(info_nl) LIKE $1 OR
                 lower(info_cs) LIKE $1 OR
                 lower(info_da) LIKE $1 OR
                 lower(info_hu) LIKE $1 OR
                 lower(info_ja) LIKE $1 OR
                 lower(info_ko) LIKE $1 OR
                 lower(info_pl) LIKE $1 OR
                 lower(info_pt) LIKE $1 OR
                 lower(info_ru) LIKE $1 OR
                 lower(info_sv) LIKE $1 OR
                 lower(info_tr) LIKE $1 OR
                 lower(info_zh) LIKE $1)
                AND latitude IS NOT NULL
                AND longitude IS NOT NULL
                AND latitude BETWEEN $2 - $4 AND $2 + $4
                AND longitude BETWEEN $3 - $4 AND $3 + $4
            ORDER BY
                ((latitude - $2) * (latitude - $2)) +
                ((longitude - $3) * (longitude - $3))
            ASC
            LIMIT $5
            "#,
            name_pattern,
            latitude,
            longitude,
            approx_distance_deg,
            limit
        );

        // Fetch results already sorted by distance
        let stations = query.fetch_all(pool).await?;

        Ok(stations)
    }
}

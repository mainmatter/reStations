#[cfg(feature = "test-helpers")]
use fake::{
    faker::{address::en::*, boolean::en::Boolean},
    Dummy,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Sqlite;
use validator::Validate;

// Approximate distance measured in degrees of latitude/longitude.
// This value is used to create a geographic bounding box around a specified point.
pub const APPROXIMATE_DISTANCE: f32 = 1.0; // Roughly 100km at equator

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
    let stations = sqlx::query_as!(
        Station,
        "SELECT
            id,
            name,
            uic,
            latitude,
            longitude,
            country,
            country_hint,
            info_de,
            info_en,
            info_es,
            info_fr,
            info_it,
            info_nb,
            info_nl,
            info_cs,
            info_da,
            info_hu,
            info_ja,
            info_ko,
            info_pl,
            info_pt,
            info_ru,
            info_sv,
            info_tr,
            info_zh
        FROM
            stations"
    )
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
        "SELECT
            id,
            name,
            uic,
            latitude,
            longitude,
            country,
            country_hint,
            info_de,
            info_en,
            info_es,
            info_fr,
            info_it,
            info_nb,
            info_nl,
            info_cs,
            info_da,
            info_hu,
            info_ja,
            info_ko,
            info_pl,
            info_pt,
            info_ru,
            info_sv,
            info_tr,
            info_zh
        FROM
            stations
        WHERE
            uic = ?",
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
    let stations = sqlx::query_as!(
        Station,
        "SELECT
            id,
            name,
            uic,
            latitude,
            longitude,
            country,
            country_hint,
            info_de,
            info_en,
            info_es,
            info_fr,
            info_it,
            info_nb,
            info_nl,
            info_cs,
            info_da,
            info_hu,
            info_ja,
            info_ko,
            info_pl,
            info_pt,
            info_ru,
            info_sv,
            info_tr,
            info_zh
        FROM
            stations
        WHERE
            uic IS NOT NULL
        AND (
            name like ?
            OR info_de like ?
            OR info_en like ?
            OR info_es like ?
            OR info_fr like ?
            OR info_it like ?
            OR info_nb like ?
            OR info_nl like ?
            OR info_cs like ?
            OR info_da like ?
            OR info_hu like ?
            OR info_ja like ?
            OR info_ko like ?
            OR info_pl like ?
            OR info_pt like ?
            OR info_ru like ?
            OR info_sv like ?
            OR info_tr like ?
            OR info_zh like ?
        )",
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern,
        pattern
    )
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

    let stations = sqlx::query_as!(
        Station,
        r#"
        SELECT
            id,
            name,
            uic,
            latitude,
            longitude,
            country,
            country_hint,
            info_de,
            info_en,
            info_es,
            info_fr,
            info_it,
            info_nb,
            info_nl,
            info_cs,
            info_da,
            info_hu,
            info_ja,
            info_ko,
            info_pl,
            info_pt,
            info_ru,
            info_sv,
            info_tr,
            info_zh
        FROM
            stations
        WHERE
            latitude IS NOT NULL
            AND longitude IS NOT NULL
            AND latitude BETWEEN $1 - $3 AND $1 + $3
            AND longitude BETWEEN $2 - $3 AND $2 + $3
        ORDER BY
            ((latitude - $1) * (latitude - $1)) +
            ((longitude - $2) * (longitude - $2))
        ASC
        "#,
        latitude,
        longitude,
        approx_distance_deg,
    )
    .fetch_all(executor)
    .await?;

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
    let stations = sqlx::query_as!(
        Station,
        r#"
        SELECT
            id,
            name,
            uic,
            latitude,
            longitude,
            country,
            country_hint,
            info_de,
            info_en,
            info_es,
            info_fr,
            info_it,
            info_nb,
            info_nl,
            info_cs,
            info_da,
            info_hu,
            info_ja,
            info_ko,
            info_pl,
            info_pt,
            info_ru,
            info_sv,
            info_tr,
            info_zh
        FROM
            stations
        WHERE
            lower(name) LIKE ?
            AND latitude IS NOT NULL
            AND longitude IS NOT NULL
            AND latitude BETWEEN $2 - $4 AND $2 + $4
            AND longitude BETWEEN $3 - $4 AND $3 + $4
        ORDER BY
            ((latitude - $2) * (latitude - $2)) +
            ((longitude - $3) * (longitude - $3))
        ASC
        "#,
        name_pattern,
        latitude,
        longitude,
        approx_distance_deg,
    )
    .fetch_all(executor)
    .await?;

    // Return the closest 20
    Ok(stations.into_iter().take(20).collect())
}

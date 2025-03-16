#[cfg(feature = "test-helpers")]
use fake::{faker::name::en::*, Dummy};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Sqlite;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Station {
    pub id: i64,
    pub name: String,
    pub uic: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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
    // these are examples only
    #[cfg_attr(feature = "test-helpers", dummy(faker = "Name()"))]
    #[validate(length(min = 1))]
    pub name: String,
    pub uic: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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
    let stations = sqlx::query_as!(Station, "SELECT id, name, uic, latitude, longitude, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations")
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
        "SELECT id, name, uic, latitude, longitude, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh FROM stations WHERE uic = $1",
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

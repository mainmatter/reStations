use crate::{
    entities::stations::{Station, StationChangeset},
    DbPool,
};
use validator::Validate;

pub async fn create(station: StationChangeset, db: &DbPool) -> Result<Station, anyhow::Error> {
    station.validate()?;

    let record = sqlx::query!(
        "INSERT INTO stations (id, name, uic, latitude, longitude, country, country_hint, info_de, info_en, info_es, info_fr, info_it, info_nb, info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv, info_tr, info_zh) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
        station.id,
        station.name,
        station.uic,
        station.latitude,
        station.longitude,
        station.country,
        station.country_hint,
        station.info_de,
        station.info_en,
        station.info_es,
        station.info_fr,
        station.info_it,
        station.info_nb,
        station.info_nl,
        station.info_cs,
        station.info_da,
        station.info_hu,
        station.info_ja,
        station.info_ko,
        station.info_pl,
        station.info_pt,
        station.info_ru,
        station.info_sv,
        station.info_tr,
        station.info_zh
    )
    .fetch_one(db)
    .await?;

    Ok(Station {
        id: record.id,
        name: station.name,
        uic: station.uic,
        latitude: station.latitude,
        longitude: station.longitude,
        country: station.country,
        country_hint: station.country_hint,
        info_de: station.info_de,
        info_en: station.info_en,
        info_es: station.info_es,
        info_fr: station.info_fr,
        info_it: station.info_it,
        info_nb: station.info_nb,
        info_nl: station.info_nl,
        info_cs: station.info_cs,
        info_da: station.info_da,
        info_hu: station.info_hu,
        info_ja: station.info_ja,
        info_ko: station.info_ko,
        info_pl: station.info_pl,
        info_pt: station.info_pt,
        info_ru: station.info_ru,
        info_sv: station.info_sv,
        info_tr: station.info_tr,
        info_zh: station.info_zh,
    })
}

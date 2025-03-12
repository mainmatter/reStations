use serde::{Deserialize, Serialize};

/// Single record in the [stations.csv](https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv) database.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct StationRecord {
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

// Needed for geo_position searches, where two computed
// fields are required for sorting by distance:
// - distance: The distance from the given position.
// - relevance_score: The relevance score of the station.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchStationRecord {
    pub id: i64,
    pub name: String,
    pub uic: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    // computed field
    pub distance: Option<f64>,
    // computed field
    pub relevance_score: Option<f64>,
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

impl From<StationRecord> for SearchStationRecord {
    fn from(db_record: StationRecord) -> Self {
        Self {
            id: db_record.id,
            name: db_record.name,
            uic: db_record.uic,
            latitude: db_record.latitude,
            longitude: db_record.longitude,
            distance: None,
            relevance_score: None,
            info_de: db_record.info_de,
            info_en: db_record.info_en,
            info_es: db_record.info_es,
            info_fr: db_record.info_fr,
            info_it: db_record.info_it,
            info_nb: db_record.info_nb,
            info_nl: db_record.info_nl,
            info_cs: db_record.info_cs,
            info_da: db_record.info_da,
            info_hu: db_record.info_hu,
            info_ja: db_record.info_ja,
            info_ko: db_record.info_ko,
            info_pl: db_record.info_pl,
            info_pt: db_record.info_pt,
            info_ru: db_record.info_ru,
            info_sv: db_record.info_sv,
            info_tr: db_record.info_tr,
            info_zh: db_record.info_zh,
        }
    }
}

impl From<SearchStationRecord> for StationRecord {
    fn from(search_record: SearchStationRecord) -> Self {
        Self {
            id: search_record.id,
            name: search_record.name,
            uic: search_record.uic,
            latitude: search_record.latitude,
            longitude: search_record.longitude,
            info_de: search_record.info_de,
            info_en: search_record.info_en,
            info_es: search_record.info_es,
            info_fr: search_record.info_fr,
            info_it: search_record.info_it,
            info_nb: search_record.info_nb,
            info_nl: search_record.info_nl,
            info_cs: search_record.info_cs,
            info_da: search_record.info_da,
            info_hu: search_record.info_hu,
            info_ja: search_record.info_ja,
            info_ko: search_record.info_ko,
            info_pl: search_record.info_pl,
            info_pt: search_record.info_pt,
            info_ru: search_record.info_ru,
            info_sv: search_record.info_sv,
            info_tr: search_record.info_tr,
            info_zh: search_record.info_zh,
        }
    }
}

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

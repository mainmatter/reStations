use serde::{Deserialize, Serialize};

/// Single record in the [stations.csv](https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv) database.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StationRecord {
    // TODO figure out exact types for each fields
    // Where necessary, create newtypes that parse and validate the input
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub uic: String,
    pub uic8_sncf: String,
    pub latitude: String,
    pub longitude: String,
    pub parent_station_id: String,
    pub country: String,
    pub time_zone: String,
    pub is_city: String,
    pub is_main_station: String,
    pub is_airport: String,
    pub is_suggestable: String,
    pub country_hint: String,
    pub main_station_hint: String,
    pub sncf_id: String,
    pub sncf_tvs_id: String,
    pub sncf_is_enabled: String,
    pub entur_id: String,
    pub entur_is_enabled: String,
    pub db_id: String,
    pub db_is_enabled: String,
    pub busbud_id: String,
    pub busbud_is_enabled: String,
    pub distribusion_id: String,
    pub distribusion_is_enabled: String,
    pub flixbus_id: String,
    pub flixbus_is_enabled: String,
    pub cff_id: String,
    pub cff_is_enabled: String,
    pub leoexpress_id: String,
    pub leoexpress_is_enabled: String,
    pub obb_id: String,
    pub obb_is_enabled: String,
    pub ouigo_id: String,
    pub ouigo_is_enabled: String,
    pub trenitalia_id: String,
    pub trenitalia_is_enabled: String,
    pub trenitalia_rtvt_id: String,
    pub trenord_id: String,
    pub ntv_rtiv_id: String,
    pub ntv_id: String,
    pub ntv_is_enabled: String,
    pub hkx_id: String,
    pub hkx_is_enabled: String,
    pub renfe_id: String,
    pub renfe_is_enabled: String,
    pub atoc_id: String,
    pub atoc_is_enabled: String,
    pub benerail_id: String,
    pub benerail_is_enabled: String,
    pub westbahn_id: String,
    pub westbahn_is_enabled: String,
    pub sncf_self_service_machine: String,
    pub same_as: String,
    pub info_de: String,
    pub info_en: String,
    pub info_es: String,
    pub info_fr: String,
    pub info_it: String,
    pub info_nb: String,
    pub info_nl: String,
    pub info_cs: String,
    pub info_da: String,
    pub info_hu: String,
    pub info_ja: String,
    pub info_ko: String,
    pub info_pl: String,
    pub info_pt: String,
    pub info_ru: String,
    pub info_sv: String,
    pub info_tr: String,
    pub info_zh: String,
    pub normalised_code: String,
    pub iata_airport_code: String,
}

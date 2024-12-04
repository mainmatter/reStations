/// Single record in the [stations.csv](https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv) database.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StationRecord {
    // TODO figure out exact types for each fields
    // Where necessary, create newtypes that parse and validate the input
    pub id: String,
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
    #[serde(rename = "info:de")]
    pub info_de: String,
    #[serde(rename = "info:en")]
    pub info_en: String,
    #[serde(rename = "info:es")]
    pub info_es: String,
    #[serde(rename = "info:fr")]
    pub info_fr: String,
    #[serde(rename = "info:it")]
    pub info_it: String,
    #[serde(rename = "info:nb")]
    pub info_nb: String,
    #[serde(rename = "info:nl")]
    pub info_nl: String,
    #[serde(rename = "info:cs")]
    pub info_cs: String,
    #[serde(rename = "info:da")]
    pub info_da: String,
    #[serde(rename = "info:hu")]
    pub info_hu: String,
    #[serde(rename = "info:ja")]
    pub info_ja: String,
    #[serde(rename = "info:ko")]
    pub info_ko: String,
    #[serde(rename = "info:pl")]
    pub info_pl: String,
    #[serde(rename = "info:pt")]
    pub info_pt: String,
    #[serde(rename = "info:ru")]
    pub info_ru: String,
    #[serde(rename = "info:sv")]
    pub info_sv: String,
    #[serde(rename = "info:tr")]
    pub info_tr: String,
    #[serde(rename = "info:zh")]
    pub info_zh: String,
    pub normalised_code: String,
    pub iata_airport_code: String,
}

impl Default for StationRecord {
    fn default() -> StationRecord {
        StationRecord {
            id: "".to_string(),
            name: "".to_string(),
            slug: "".to_string(),
            uic: "".to_string(),
            uic8_sncf: "".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            parent_station_id: "".to_string(),
            country: "".to_string(),
            time_zone: "".to_string(),
            is_city: "".to_string(),
            is_main_station: "".to_string(),
            is_airport: "".to_string(),
            is_suggestable: "".to_string(),
            country_hint: "".to_string(),
            main_station_hint: "".to_string(),
            sncf_id: "".to_string(),
            sncf_tvs_id: "".to_string(),
            sncf_is_enabled: "".to_string(),
            entur_id: "".to_string(),
            entur_is_enabled: "".to_string(),
            db_id: "".to_string(),
            db_is_enabled: "".to_string(),
            busbud_id: "".to_string(),
            busbud_is_enabled: "".to_string(),
            distribusion_id: "".to_string(),
            distribusion_is_enabled: "".to_string(),
            flixbus_id: "".to_string(),
            flixbus_is_enabled: "".to_string(),
            cff_id: "".to_string(),
            cff_is_enabled: "".to_string(),
            leoexpress_id: "".to_string(),
            leoexpress_is_enabled: "".to_string(),
            obb_id: "".to_string(),
            obb_is_enabled: "".to_string(),
            ouigo_id: "".to_string(),
            ouigo_is_enabled: "".to_string(),
            trenitalia_id: "".to_string(),
            trenitalia_is_enabled: "".to_string(),
            trenitalia_rtvt_id: "".to_string(),
            trenord_id: "".to_string(),
            ntv_rtiv_id: "".to_string(),
            ntv_id: "".to_string(),
            ntv_is_enabled: "".to_string(),
            hkx_id: "".to_string(),
            hkx_is_enabled: "".to_string(),
            renfe_id: "".to_string(),
            renfe_is_enabled: "".to_string(),
            atoc_id: "".to_string(),
            atoc_is_enabled: "".to_string(),
            benerail_id: "".to_string(),
            benerail_is_enabled: "".to_string(),
            westbahn_id: "".to_string(),
        }
    }
}
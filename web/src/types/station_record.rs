use serde::{Deserialize, Serialize};

/// Single record in the [stations.csv](https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv) database.
#[derive(Debug, Serialize, Default, PartialEq)]
#[serde(default)]
pub struct StationRecord {
    // TODO figure out exact types for each fields
    // Where necessary, create newtypes that parse and validate the input
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub uic: String,
    pub uic8_sncf: String,
    pub latitude: f32,
    pub longitude: f32,
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
    pub distribusion_is_enabled: bool,
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

impl<'de> serde::Deserialize<'de> for StationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StationRecordVisitor;

        impl<'de> serde::de::Visitor<'de> for StationRecordVisitor {
            type Value = StationRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // TODO provide info for error messages
                todo!()
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let record = StationRecord::builder();
                
                while let Some(field) = map.next_key::<&'de str>()? {
                    match field {
                        "id" => todo!(),
                        "name" => todo!(),
                        "slug" => todo!(),
                        "uic" => todo!(),
                        "uic8_sncf" => todo!(),
                        "latitude" => todo!(),
                        "longitude" => todo!(),
                        "parent_station_id" => todo!(),
                        "country" => todo!(),
                        "time_zone" => todo!(),
                        "is_city" => todo!(),
                        "is_main_station" => todo!(),
                        "is_airport" => todo!(),
                        "is_suggestable" => todo!(),
                        "country_hint" => todo!(),
                        "main_station_hint" => todo!(),
                        "sncf_id" => todo!(),
                        "sncf_tvs_id" => todo!(),
                        "sncf_is_enabled" => todo!(),
                        "entur_id" => todo!(),
                        "entur_is_enabled" => todo!(),
                        "db_id" => todo!(),
                        "db_is_enabled" => todo!(),
                        "busbud_id" => todo!(),
                        "busbud_is_enabled" => todo!(),
                        "distribusion_id" => todo!(),
                        "distribusion_is_enabled" => todo!(),
                        "flixbus_id" => todo!(),
                        "flixbus_is_enabled" => todo!(),
                        "cff_id" => todo!(),
                        "cff_is_enabled" => todo!(),
                        "leoexpress_id" => todo!(),
                        "leoexpress_is_enabled" => todo!(),
                        "obb_id" => todo!(),
                        "obb_is_enabled" => todo!(),
                        "ouigo_id" => todo!(),
                        "ouigo_is_enabled" => todo!(),
                        "trenitalia_id" => todo!(),
                        "trenitalia_is_enabled" => todo!(),
                        "trenitalia_rtvt_id" => todo!(),
                        "trenord_id" => todo!(),
                        "ntv_rtiv_id" => todo!(),
                        "ntv_id" => todo!(),
                        "ntv_is_enabled" => todo!(),
                        "hkx_id" => todo!(),
                        "hkx_is_enabled" => todo!(),
                        "renfe_id" => todo!(),
                        "renfe_is_enabled" => todo!(),
                        "atoc_id" => todo!(),
                        "atoc_is_enabled" => todo!(),
                        "benerail_id" => todo!(),
                        "benerail_is_enabled" => todo!(),
                        "westbahn_id" => todo!(),
                        "westbahn_is_enabled" => todo!(),
                        "sncf_self_service_machine" => todo!(),
                        "same_as" => todo!(),
                        "info_de" => todo!(),
                        "info_en" => todo!(),
                        "info_es" => todo!(),
                        "info_fr" => todo!(),
                        "info_it" => todo!(),
                        "info_nb" => todo!(),
                        "info_nl" => todo!(),
                        "info_cs" => todo!(),
                        "info_da" => todo!(),
                        "info_hu" => todo!(),
                        "info_ja" => todo!(),
                        "info_ko" => todo!(),
                        "info_pl" => todo!(),
                        "info_pt" => todo!(),
                        "info_ru" => todo!(),
                        "info_sv" => todo!(),
                        "info_tr" => todo!(),
                        "info_zh" => todo!(),
                        "normalised_code" => todo!(),
                        "iata_airport_code" => todo!(),
                        _ => panic!("Not good. Maybe return an error?"),
                    }
                }

                Ok(record.build())
            }
        }

        deserializer.deserialize_map(StationRecordVisitor)
    }
}

/*
{
    "things": 123,
    ...
    "busbud_id": "blabla",
    "busbud_enabled": false

}


{
    "things": 123,
    ...
    "busbud": {
        "id": "blabla",
        "enabled": false
    }

}


{
    "things": 123,
    ...

    "id": "blabla",
    "enabled": false

}


*/

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Id {
    id: String,
    is_enabled: bool,
}

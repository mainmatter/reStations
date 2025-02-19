use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// Single record in the [stations.csv](https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv) database.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct StationRecord {
    // TODO figure out exact types for each fields
    // Where necessary, create newtypes that parse and validate the input
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub uic: String,
    pub uic8_sncf: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
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
    #[serde(flatten, with = "distribusion")]
    pub distribusion: Id,
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
    #[serde(flatten, with = "info")]
    pub info: Info,
    pub normalised_code: String,
    pub iata_airport_code: String,
}

serde_with::with_prefix!(distribusion "distribusion_");
serde_with::with_prefix!(info "info:");

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Info {
    pub de: String,
    pub en: String,
    pub es: String,
    pub fr: String,
    pub it: String,
    pub nb: String,
    pub nl: String,
    pub cs: String,
    pub da: String,
    pub hu: String,
    pub ja: String,
    pub ko: String,
    pub pl: String,
    pub pt: String,
    pub ru: String,
    pub sv: String,
    pub tr: String,
    pub zh: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Id {
    id: Option<String>,
    #[serde(deserialize_with = "Id::parse_is_enabled")]
    is_enabled: bool,
}

impl Id {
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn parse_is_enabled<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IsEnabledVisitor;

        impl Visitor<'_> for IsEnabledVisitor {
            type Value = bool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, r#"either "t" or "f""#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "t" => Ok(true),
                    "f" => Ok(false),
                    s => Err(E::invalid_value(
                        serde::de::Unexpected::Str(s),
                        &r#""t" or "f""#,
                    )),
                }
            }
        }
        deserializer.deserialize_str(IsEnabledVisitor)
    }
}

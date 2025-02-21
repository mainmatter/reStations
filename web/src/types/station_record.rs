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
    pub is_city: bool,
    pub is_main_station: bool,
    pub is_airport: bool,
    pub is_suggestable: bool,
    pub country_hint: bool,
    pub main_station_hint: bool,
    #[serde(flatten, with = "sncf")]
    pub sncf: Id,
    pub sncf_tvs_id: String,
    #[serde(flatten, with = "entur")]
    pub entur: Id,
    #[serde(flatten, with = "db")]
    pub db: Id,
    #[serde(flatten, with = "busbud")]
    pub busbud: Id,
    #[serde(flatten, with = "distribusion")]
    pub distribusion: Id,
    #[serde(flatten, with = "flixbus")]
    pub flixbus: Id,
    #[serde(flatten, with = "cff")]
    pub cff: Id,
    #[serde(flatten, with = "leoexpress")]
    pub leoexpress: Id,
    #[serde(flatten, with = "obb")]
    pub obb: Id,
    #[serde(flatten, with = "ouigo")]
    pub ouigo: Id,
    #[serde(flatten, with = "trenitalia")]
    pub trenitalia: Id,
    pub trenitalia_rtvt_id: String,
    pub trenord_id: String,
    pub ntv_rtiv_id: String,
    #[serde(flatten, with = "ntv")]
    pub ntv: Id,
    #[serde(flatten, with = "hkx")]
    pub hkx: Id,
    #[serde(flatten, with = "renfe")]
    pub renfe: Id,
    #[serde(flatten, with = "atoc")]
    pub atoc: Id,
    #[serde(flatten, with = "benerail")]
    pub benerail: Id,
    #[serde(flatten, with = "westbahn")]
    pub westbahn: Id,
    pub sncf_self_service_machine: String,
    pub same_as: String,
    #[serde(flatten, with = "info")]
    pub info: Info,
    pub normalised_code: String,
    pub iata_airport_code: String,
}

serde_with::with_prefix!(sncf "sncf_");
serde_with::with_prefix!(entur "entur_");
serde_with::with_prefix!(db "db_");
serde_with::with_prefix!(busbud "busbud_");
serde_with::with_prefix!(distribusion "distribusion_");
serde_with::with_prefix!(flixbus "flixbus_");
serde_with::with_prefix!(cff "cff_");
serde_with::with_prefix!(leoexpress "leoexpress_");
serde_with::with_prefix!(obb "obb_");
serde_with::with_prefix!(ouigo "ouigo_");
serde_with::with_prefix!(trenitalia "trenitalia_");
serde_with::with_prefix!(ntv "ntv_");
serde_with::with_prefix!(hkx "hkx_");
serde_with::with_prefix!(renfe "renfe_");
serde_with::with_prefix!(atoc "atoc_");
serde_with::with_prefix!(benerail "benerail_");
serde_with::with_prefix!(westbahn "westbahn_");

serde_with::with_prefix!(info "info:");

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Info {
    pub de: Option<String>,
    pub en: Option<String>,
    pub es: Option<String>,
    pub fr: Option<String>,
    pub it: Option<String>,
    pub nb: Option<String>,
    pub nl: Option<String>,
    pub cs: Option<String>,
    pub da: Option<String>,
    pub hu: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub pl: Option<String>,
    pub pt: Option<String>,
    pub ru: Option<String>,
    pub sv: Option<String>,
    pub tr: Option<String>,
    pub zh: Option<String>,
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

            // Values incoming from boolean fields in the CSV
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

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    1 => Ok(true),
                    0 => Ok(false),
                    _ => Err(E::invalid_value(
                        serde::de::Unexpected::Signed(v),
                        &"0 or 1",
                    )),
                }
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    1 => Ok(true),
                    0 => Ok(false),
                    _ => Err(E::invalid_value(
                        serde::de::Unexpected::Unsigned(v),
                        &"0 or 1",
                    )),
                }
            }
        }

        deserializer.deserialize_any(IsEnabledVisitor)
    }
}

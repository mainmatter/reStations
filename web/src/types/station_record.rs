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
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub is_city: bool,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub is_main_station: bool,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub is_airport: bool,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub is_suggestable: bool,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub country_hint: bool,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    pub main_station_hint: bool,
    #[serde(flatten, with = "sncf")]
    pub sncf: ProviderId,
    pub sncf_tvs_id: String,
    #[serde(flatten, with = "entur")]
    pub entur: ProviderId,
    #[serde(flatten, with = "db")]
    pub db: ProviderId,
    #[serde(flatten, with = "busbud")]
    pub busbud: ProviderId,
    #[serde(flatten, with = "distribusion")]
    pub distribusion: ProviderId,
    #[serde(flatten, with = "flixbus")]
    pub flixbus: ProviderId,
    #[serde(flatten, with = "cff")]
    pub cff: ProviderId,
    #[serde(flatten, with = "leoexpress")]
    pub leoexpress: ProviderId,
    #[serde(flatten, with = "obb")]
    pub obb: ProviderId,
    #[serde(flatten, with = "ouigo")]
    pub ouigo: ProviderId,
    #[serde(flatten, with = "trenitalia")]
    pub trenitalia: ProviderId,
    pub trenitalia_rtvt_id: String,
    pub trenord_id: String,
    pub ntv_rtiv_id: String,
    #[serde(flatten, with = "ntv")]
    pub ntv: ProviderId,
    #[serde(flatten, with = "hkx")]
    pub hkx: ProviderId,
    #[serde(flatten, with = "renfe")]
    pub renfe: ProviderId,
    #[serde(flatten, with = "atoc")]
    pub atoc: ProviderId,
    #[serde(flatten, with = "benerail")]
    pub benerail: ProviderId,
    #[serde(flatten, with = "westbahn")]
    pub westbahn: ProviderId,
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
pub struct ProviderId {
    #[serde(deserialize_with = "ProviderId::deserialize")]
    id: Option<String>,
    #[serde(deserialize_with = "BoolDeserializer::deserialize")]
    is_enabled: bool,
}

impl ProviderId {
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProviderIdVisitor;

        impl Visitor<'_> for ProviderIdVisitor {
            type Value = Option<String>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, r#"a string, integer, or null"#)
            }

            // Values incoming from provider id fields in the CSV
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(v.to_string()))
            }

            // Values incoming from provider id fields in the CSV
            // Believe it or not, the deserializer interprets "NAN" strings as floats (NaN) :P
            // There's actually a station with that provider ID.
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(v.to_string()))
            }

            // Values incoming from provider id fields in the CSV
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(v.to_string()))
                }
            }

            // Values incoming from provider id fields in the CSV
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_any(ProviderIdVisitor)
    }
}

pub struct BoolDeserializer;

impl BoolDeserializer {
    fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

            // Values incoming from boolean fields in the SQLite db
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
        }

        deserializer.deserialize_any(IsEnabledVisitor)
    }
}

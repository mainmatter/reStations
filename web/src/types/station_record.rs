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
    pub sncf_id: String,
    #[serde(flatten, with = "sncf_tvs")]
    pub sncf_tvs: Id,
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
    #[serde(flatten, with = "trenitalia_rtvt")]
    pub trenitalia_rtvt: Id,
    #[serde(flatten, with = "trenord")]
    pub trenord: Id,
    #[serde(flatten, with = "ntv_rtiv")]
    pub ntv_rtiv: Id,
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

serde_with::with_prefix!(sncf_tvs "sncf_tvs_");
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
serde_with::with_prefix!(trenitalia_rtvt "trenitalia_rtvt_");
serde_with::with_prefix!(trenord "trenord_");
serde_with::with_prefix!(ntv_rtiv "ntv_rtiv_");
serde_with::with_prefix!(ntv "ntv_");
serde_with::with_prefix!(hkx "hkx_");
serde_with::with_prefix!(renfe "renfe_");
serde_with::with_prefix!(atoc "atoc_");
serde_with::with_prefix!(benerail "benerail_");
serde_with::with_prefix!(westbahn "westbahn_");

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
                    "f" => Ok(true),
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

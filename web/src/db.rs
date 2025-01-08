use super::types::station_record::StationRecord;

pub type Error = rusqlite::Error;
pub type Connection = rusqlite::Connection;
pub type Result<T, E = Error> = rusqlite::Result<T, E>;

pub fn create_connection() -> Result<Connection, Error> {
    Connection::open_in_memory()
}

pub fn create_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch(include_str!("./db.sql"))
}

pub fn insert_station(db: &Connection, record: &StationRecord) -> Result<usize, rusqlite::Error> {
    db.execute(
        "INSERT into stations (name,slug,uic,uic8_sncf,latitude,longitude,parent_station_id,country,time_zone,is_city,is_main_station,is_airport,is_suggestable,country_hint,main_station_hint,sncf_id,sncf_tvs_id,sncf_is_enabled,entur_id,entur_is_enabled,db_id,db_is_enabled,busbud_id,busbud_is_enabled,distribusion_id,distribusion_is_enabled,flixbus_id,flixbus_is_enabled,cff_id,cff_is_enabled,leoexpress_id,leoexpress_is_enabled,obb_id,obb_is_enabled,ouigo_id,ouigo_is_enabled,trenitalia_id,trenitalia_is_enabled,trenitalia_rtvt_id,trenord_id,ntv_rtiv_id,ntv_id,ntv_is_enabled,hkx_id,hkx_is_enabled,renfe_id,renfe_is_enabled,atoc_id,atoc_is_enabled,benerail_id,benerail_is_enabled,westbahn_id,westbahn_is_enabled,sncf_self_service_machine,same_as,info_de,info_en,info_es,info_fr,info_it,info_nb,info_nl,info_cs,info_da,info_hu,info_ja,info_ko,info_pl,info_pt,info_ru,info_sv,info_tr,info_zh,normalised_code,iata_airport_code) \
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39,?40,?41,?42,?43,?44,?45,?46,?47,?48,?49,?50,?51,?52,?53,?54,?55,?56,?57,?58,?59,?60,?61,?62,?63,?64,?65,?66,?67,?68,?69,?70,?71,?72,?73,?74,?75)",
        rusqlite::params![record.name.as_str(),record.slug.as_str(),record.uic.as_str(),record.uic8_sncf.as_str(),record.latitude.as_str(),record.longitude.as_str(),record.parent_station_id.as_str(),record.country.as_str(),record.time_zone.as_str(),record.is_city.as_str(),record.is_main_station.as_str(),record.is_airport.as_str(),record.is_suggestable.as_str(),record.country_hint.as_str(),record.main_station_hint.as_str(),record.sncf_id.as_str(),record.sncf_tvs_id.as_str(),record.sncf_is_enabled.as_str(),record.entur_id.as_str(),record.entur_is_enabled.as_str(),record.db_id.as_str(),record.db_is_enabled.as_str(),record.busbud_id.as_str(),record.busbud_is_enabled.as_str(),record.distribusion_id.as_str(),record.distribusion_is_enabled.as_str(),record.flixbus_id.as_str(),record.flixbus_is_enabled.as_str(),record.cff_id.as_str(),record.cff_is_enabled.as_str(),record.leoexpress_id.as_str(),record.leoexpress_is_enabled.as_str(),record.obb_id.as_str(),record.obb_is_enabled.as_str(),record.ouigo_id.as_str(),record.ouigo_is_enabled.as_str(),record.trenitalia_id.as_str(),record.trenitalia_is_enabled.as_str(),record.trenitalia_rtvt_id.as_str(),record.trenord_id.as_str(),record.ntv_rtiv_id.as_str(),record.ntv_id.as_str(),record.ntv_is_enabled.as_str(),record.hkx_id.as_str(),record.hkx_is_enabled.as_str(),record.renfe_id.as_str(),record.renfe_is_enabled.as_str(),record.atoc_id.as_str(),record.atoc_is_enabled.as_str(),record.benerail_id.as_str(),record.benerail_is_enabled.as_str(),record.westbahn_id.as_str(),record.westbahn_is_enabled.as_str(),record.sncf_self_service_machine.as_str(),record.same_as.as_str(),record.info_de.as_str(),record.info_en.as_str(),record.info_es.as_str(),record.info_fr.as_str(),record.info_it.as_str(),record.info_nb.as_str(),record.info_nl.as_str(),record.info_cs.as_str(),record.info_da.as_str(),record.info_hu.as_str(),record.info_ja.as_str(),record.info_ko.as_str(),record.info_pl.as_str(),record.info_pt.as_str(),record.info_ru.as_str(),record.info_sv.as_str(),record.info_tr.as_str(),record.info_zh.as_str(),record.normalised_code.as_str(),record.iata_airport_code.as_str()],
    )
}

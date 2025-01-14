use super::types::station_record::StationRecord;

pub type Error = sqlx::Error;
pub type Connection = sqlx::sqlite::SqlitePool;
pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub async fn create_connection() -> Result<Connection, Error> {
    let connection = Connection::connect("sqlite::memory:").await?;
    Ok(connection)
}

pub async fn create_tables(db: &Connection) -> Result<(), Error> {
    let statements = include_str!("./db.sql")
        .split(';')
        .map(|stmt| stmt.trim())
        .filter(|stmt| !stmt.is_empty());

    for stmt in statements {
        sqlx::query(stmt).execute(db).await?;
    }

    Ok(())
}

pub async fn insert_station(db: &Connection, record: &StationRecord) -> Result<(), Error> {
    sqlx::query(
        r#"INSERT into stations (name,slug,uic,uic8_sncf,latitude,longitude,parent_station_id,country,time_zone,is_city,is_main_station,is_airport,is_suggestable,country_hint,main_station_hint,sncf_id,sncf_tvs_id,sncf_is_enabled,entur_id,entur_is_enabled,db_id,db_is_enabled,busbud_id,busbud_is_enabled,distribusion_id,distribusion_is_enabled,flixbus_id,flixbus_is_enabled,cff_id,cff_is_enabled,leoexpress_id,leoexpress_is_enabled,obb_id,obb_is_enabled,ouigo_id,ouigo_is_enabled,trenitalia_id,trenitalia_is_enabled,trenitalia_rtvt_id,trenord_id,ntv_rtiv_id,ntv_id,ntv_is_enabled,hkx_id,hkx_is_enabled,renfe_id,renfe_is_enabled,atoc_id,atoc_is_enabled,benerail_id,benerail_is_enabled,westbahn_id,westbahn_is_enabled,sncf_self_service_machine,same_as,info_de,info_en,info_es,info_fr,info_it,info_nb,info_nl,info_cs,info_da,info_hu,info_ja,info_ko,info_pl,info_pt,info_ru,info_sv,info_tr,info_zh,normalised_code,iata_airport_code) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37,$38,$39,$40,$41,$42,$43,$44,$45,$46,$47,$48,$49,$50,$51,$52,$53,$54,$55,$56,$57,$58,$59,$60,$61,$62,$63,$64,$65,$66,$67,$68,$69,$70,$71,$72,$73,$74,$75)"#
    )
    .bind(record.name.as_str()).bind(record.slug.as_str()).bind(record.uic.as_str()).bind(record.uic8_sncf.as_str()).bind(record.latitude.as_str()).bind(record.longitude.as_str()).bind(record.parent_station_id.as_str()).bind(record.country.as_str()).bind(record.time_zone.as_str()).bind(record.is_city.as_str()).bind(record.is_main_station.as_str()).bind(record.is_airport.as_str()).bind(record.is_suggestable.as_str()).bind(record.country_hint.as_str()).bind(record.main_station_hint.as_str()).bind(record.sncf_id.as_str()).bind(record.sncf_tvs_id.as_str()).bind(record.sncf_is_enabled.as_str()).bind(record.entur_id.as_str()).bind(record.entur_is_enabled.as_str()).bind(record.db_id.as_str()).bind(record.db_is_enabled.as_str()).bind(record.busbud_id.as_str()).bind(record.busbud_is_enabled.as_str()).bind(record.distribusion_id.as_str()).bind(record.distribusion_is_enabled.as_str()).bind(record.flixbus_id.as_str()).bind(record.flixbus_is_enabled.as_str()).bind(record.cff_id.as_str()).bind(record.cff_is_enabled.as_str()).bind(record.leoexpress_id.as_str()).bind(record.leoexpress_is_enabled.as_str()).bind(record.obb_id.as_str()).bind(record.obb_is_enabled.as_str()).bind(record.ouigo_id.as_str()).bind(record.ouigo_is_enabled.as_str()).bind(record.trenitalia_id.as_str()).bind(record.trenitalia_is_enabled.as_str()).bind(record.trenitalia_rtvt_id.as_str()).bind(record.trenord_id.as_str()).bind(record.ntv_rtiv_id.as_str()).bind(record.ntv_id.as_str()).bind(record.ntv_is_enabled.as_str()).bind(record.hkx_id.as_str()).bind(record.hkx_is_enabled.as_str()).bind(record.renfe_id.as_str()).bind(record.renfe_is_enabled.as_str()).bind(record.atoc_id.as_str()).bind(record.atoc_is_enabled.as_str()).bind(record.benerail_id.as_str()).bind(record.benerail_is_enabled.as_str()).bind(record.westbahn_id.as_str()).bind(record.westbahn_is_enabled.as_str()).bind(record.sncf_self_service_machine.as_str()).bind(record.same_as.as_str()).bind(record.info_de.as_str()).bind(record.info_en.as_str()).bind(record.info_es.as_str()).bind(record.info_fr.as_str()).bind(record.info_it.as_str()).bind(record.info_nb.as_str()).bind(record.info_nl.as_str()).bind(record.info_cs.as_str()).bind(record.info_da.as_str()).bind(record.info_hu.as_str()).bind(record.info_ja.as_str()).bind(record.info_ko.as_str()).bind(record.info_pl.as_str()).bind(record.info_pt.as_str()).bind(record.info_ru.as_str()).bind(record.info_sv.as_str()).bind(record.info_tr.as_str()).bind(record.info_zh.as_str()).bind(record.normalised_code.as_str()).bind(record.iata_airport_code.as_str())
    .execute(db)
    .await?;

    Ok(())
}

// TODO return a stream of json chunks
pub async fn find_all_stations(db: &Connection) -> Result<Vec<StationRecord>, sqlx::Error> {
    let stations: Vec<StationRecord> = sqlx::query_as("SELECT * from stations").fetch_all(db).await?;
    Ok(stations)
}

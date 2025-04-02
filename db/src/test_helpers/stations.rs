use crate::{
    entities::stations::{Station, StationChangeset},
    DbPool,
};
use validator::Validate;

pub async fn create(station: StationChangeset, db: &DbPool) -> Result<Station, anyhow::Error> {
    station.validate()?;

    let record = sqlx::query!(
        "INSERT INTO stations (id, name, uic, latitude, longitude, country) VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        station.id,
        station.name,
        station.uic,
        station.latitude,
        station.longitude,
        station.country,
    )
    .fetch_one(db)
    .await?;

    Ok(Station {
        id: record.id,
        name: station.name,
        uic: station.uic,
        latitude: station.latitude,
        longitude: station.longitude,
        country: station.country,
    })
}

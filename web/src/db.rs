use rusqlite::{Connection, Result, Error};

pub fn create_tables(db: &Connection) -> Result<(), Error> {
    db.execute_batch(include_str!("./db.sql"))
}

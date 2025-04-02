CREATE TABLE stations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    uic TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    country TEXT
);

CREATE UNIQUE INDEX stations_id_idx ON stations (id);

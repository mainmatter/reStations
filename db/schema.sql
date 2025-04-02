CREATE TABLE stations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    uic TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    country TEXT,
    info_de TEXT,
    info_en TEXT,
    info_es TEXT,
    info_fr TEXT,
    info_it TEXT,
    info_nb TEXT,
    info_nl TEXT,
    info_cs TEXT,
    info_da TEXT,
    info_hu TEXT,
    info_ja TEXT,
    info_ko TEXT,
    info_pl TEXT,
    info_pt TEXT,
    info_ru TEXT,
    info_sv TEXT,
    info_tr TEXT,
    info_zh TEXT
);

CREATE UNIQUE INDEX stations_id_idx ON stations (id);

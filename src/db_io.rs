use rusqlite::Connection;

use crate::solar_data::SolarData;

const CREATE_SOLAR_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS solar (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER,
    tracker_id INTEGER,
    timestamp INTEGER,
    energy_generation INTEGER,
    power_generation INTEGER,
    temperature REAL,
    voltage REAL,
    uploaded INTEGER
)";

const CREATE_SOLAR_TABLE_INDEX_SQL: &str =
    "CREATE INDEX IF NOT EXISTS solar_idx ON solar(uploaded)";

const CREATE_EVENT_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS solar_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER,
    tracker_id INTEGER,
    timestamp INTEGER,
    event INTEGER,
    uploaded INTEGER
)";

const CREATE_SOLAR_EVENT_TABLE_INDEX_SQL: &str =
    "CREATE INDEX IF NOT EXISTS solar_idx ON solar(uploaded)";

const INSERT_SOLAR_DATA_SQL: &str = "
    INSERT INTO solar(
        device_id,
        tracker_id,
        timestamp,
        energy_generation,
        power_generation,
        temperature,
        voltage,
        uploaded
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?);
";

const INSERT_SOLAR_EVENT_SQL: &str = "
    INSERT INTO solar_event(
        device_id,
        tracker_id,
        timestamp,
        event,
        uploaded
    ) VALUES (?, ?, ?, ?, ?);
";

pub struct DbIO {
    db: Connection,
}

impl DbIO {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let db = Connection::open(db_path)?;
        db.execute(CREATE_SOLAR_TABLE_SQL, ())?;
        db.execute(CREATE_EVENT_TABLE_SQL, ())?;
        db.execute(CREATE_SOLAR_TABLE_INDEX_SQL, ())?;
        db.execute(CREATE_SOLAR_EVENT_TABLE_INDEX_SQL, ())?;
        Ok(Self { db })
    }

    pub fn insert_solar_data(&self, solar_data: SolarData) -> Result<(), rusqlite::Error> {
        self.db
            .execute(
                INSERT_SOLAR_DATA_SQL,
                (
                    solar_data.device_id,
                    solar_data.tracker_id,
                    solar_data.timestamp,
                    solar_data.energy_generation,
                    solar_data.power_generation,
                    solar_data.temperature,
                    solar_data.voltage,
                    false,
                ),
            )
            .map(|_| {})
    }

    pub fn insert_event(
        &self,
        device_id: u8,
        tracker_id: u8,
        timestamp: usize,
        event: u8,
    ) -> Result<(), rusqlite::Error> {
        self.db
            .execute(
                INSERT_SOLAR_EVENT_SQL,
                (device_id, tracker_id, timestamp, event, false),
            )
            .map(|_| {})
    }
}

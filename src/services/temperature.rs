use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InternalTemperatureInfo {
    pub temperature: f64,
    pub humidity: f64,
}



#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExternalTemperatureInfo {
    /// Celsius
    pub temperature: f64,
    /// % 
    pub humidity: f64,
    /// hPa
    pub pressure: f64,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawTempuratureInfoJson(String);

#[derive(Debug)]
pub enum TempuratureInfoError {
    InvalidField,
}

#[post("/update", data = "<data>")]
pub fn update(db: &State<crate::DbConn>, data: Json<InternalTemperatureInfo>) -> String {
    save_int_tempurature_record(&data, db);
    format!(
        "Saved tempurature record  {}",
        db.lock()
            .expect("unable to get lock on db")
            .last_insert_rowid()
    )
}

#[post("/internal", data = "<data>")]
pub fn internal_post(db: &State<crate::DbConn>, data: Json<InternalTemperatureInfo>) {
    save_int_tempurature_record(&data.into_inner(), db);
}

#[post("/external", data = "<data>")]
pub fn external_post(db: &State<crate::DbConn>, data: Json<ExternalTemperatureInfo>) {
    save_ext_tempurature_record(&data.into_inner(), db);
}

#[get("/internal")]
pub fn internal_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    
    let latest = get_int_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

#[get("/external")]
pub fn external_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    let latest = get_ext_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}


#[get("/external/latest/<cnt>")]
pub fn external_get_many(db: &State<crate::DbConn>, cnt: usize) -> RawTempuratureInfoJson {
    let latest = get_ext_temperature_records(db, cnt);
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

#[get("/internal/latest/<cnt>")]
pub fn internal_get_many(db: &State<crate::DbConn>, cnt: usize) -> RawTempuratureInfoJson {
    let latest = get_int_temperature_records(db, cnt);
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

#[get("/both")]
pub fn both_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    let latest = get_int_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

fn get_most_recent_temperature(db: &State<crate::DbConn>) -> f64 {
    db.lock()
        .unwrap()
        .query_row(
            "SELECT temp from InternalTemperatureRecords order by id desc limit 1;",
            [],
            |row| row.get(0),
        )
        .expect("unable to find tempurature record")
}

fn save_int_tempurature_record(temp_info: &InternalTemperatureInfo, db: &State<crate::DbConn>) {
    db.lock()
        .expect("unable to get db lock")
        .execute(
            "INSERT INTO InternalTemperatureRecords(temp, humidity) VALUES (?1, ?2)",
            [temp_info.temperature, temp_info.humidity],
        )
        .expect("unable to insert temperature record");
}

fn save_ext_tempurature_record(temp_info: &ExternalTemperatureInfo, db: &State<crate::DbConn>) {
    db.lock()
        .expect("unable to get db lock")
        .execute(
            "INSERT INTO ExternalTemperatureRecords(temp, humidity, pressure) VALUES (?1, ?2, ?3)",
            [temp_info.temperature, temp_info.humidity, temp_info.pressure],
        )
        .expect("unable to insert temperature record");
}


fn get_int_temperature_records(db: &State<crate::DbConn>, rows_to_get: usize) -> Vec<InternalTemperatureInfo> {
    let binding = db.lock().expect("unable to get db lock");
    let mut stmt = binding
        .prepare("SELECT temp, humidity from InternalTemperatureRecords ORDER BY id DESC LIMIT ?1;")
        .expect("unable to prepare statement");

    let rows = stmt
        .query_map([rows_to_get], |row| {
            Ok(InternalTemperatureInfo {
                temperature: row.get(0)?,
                humidity: row.get(1)?,
            })
        })
        .expect("unable to query map");

    let mut results = vec![];
    for row in rows {
        results.push(row.expect("unable to get row"));
    }

    results
}


fn get_ext_temperature_records(db: &State<crate::DbConn>, rows_to_get: usize) -> Vec<ExternalTemperatureInfo> {
    let binding = db.lock().expect("unable to get db lock");
    let mut stmt = binding
        .prepare("SELECT temp, humidity, pressure from ExternalTemperatureRecords ORDER BY id DESC LIMIT ?1;")
        .expect("unable to prepare statement");

    let rows = stmt
        .query_map([rows_to_get], |row| {
            Ok(ExternalTemperatureInfo {
                temperature: row.get(0)?,
                humidity: row.get(1)?,
                pressure: row.get(2)?,
            })
        })
        .expect("unable to query map");

    let mut results = vec![];
    for row in rows {
        results.push(row.expect("unable to get row"));
    }

    results
}

pub enum TemperatureRecordType {
    Internal,
    External,
}

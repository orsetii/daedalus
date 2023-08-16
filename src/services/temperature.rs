use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TemperatureInfo {
    pub temperature: f64,
    pub humidity: f64,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawTempuratureInfoJson(String);

#[derive(Debug)]
pub enum TempuratureInfoError {
    InvalidField,
}

#[post("/update", data = "<data>")]
pub fn update(db: &State<crate::DbConn>, data: Json<TemperatureInfo>) -> String {
    save_tempurature_record(&data, db);
    format!(
        "Saved tempurature record  {}",
        db.lock()
            .expect("unable to get lock on db")
            .last_insert_rowid()
    )
}

#[post("/internal", data = "<data>")]
pub fn internal_post(db: &State<crate::DbConn>, data: Json<TemperatureInfo>) {
    save_tempurature_record(&data.into_inner(), db);
}

#[post("/external", data = "<data>")]
pub fn external_post(db: &State<crate::DbConn>, data: Json<TemperatureInfo>) {
    save_tempurature_record(&data.into_inner(), db);
}

#[get("/internal")]
pub fn internal_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    
    let latest = get_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

#[get("/external")]
pub fn external_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    let latest = get_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

#[get("/both")]
pub fn both_get(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    let latest = get_temperature_records(db, 1)[0];
    RawTempuratureInfoJson(serde_json::to_string(&latest).unwrap())
}

fn get_most_recent_temperature(db: &State<crate::DbConn>) -> f64 {
    db.lock()
        .unwrap()
        .query_row(
            "SELECT temp from TempuratureRecords order by id desc limit 1;",
            [],
            |row| row.get(0),
        )
        .expect("unable to find tempurature record")
}

fn save_tempurature_record(temp_info: &TemperatureInfo, db: &State<crate::DbConn>) {
    db.lock()
        .expect("unable to get db lock")
        .execute(
            "INSERT INTO TempuratureRecords(temp, humidity) VALUES (?1, ?2)",
            [temp_info.temperature, temp_info.humidity],
        )
        .expect("unable to insert temperature record");
}


fn get_temperature_records(db: &State<crate::DbConn>, rows_to_get: usize) -> Vec<TemperatureInfo> {
    let binding = db.lock().expect("unable to get db lock");
    let mut stmt = binding
        .prepare("SELECT temp, humidity from TempuratureRecords ORDER BY id DESC LIMIT ?1;")
        .expect("unable to prepare statement");

    let rows = stmt
        .query_map([rows_to_get], |row| {
            Ok(TemperatureInfo {
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

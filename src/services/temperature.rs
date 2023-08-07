

use rocket::serde::{json::Json, Deserialize};
use rocket::State;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TempuratureInfo {
    pub tempurature: f64,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawTempuratureInfoJson(String);

#[derive(Debug)]
pub enum TempuratureInfoError {
    InvalidField,
}

#[post("/update", data = "<data>")]
pub fn update(db: &State<crate::DbConn>, data: Json<TempuratureInfo>) -> String {
    save_tempurature_record(&data, db);
    format!(
        "Saved tempurature record  {}",
        db.lock()
            .expect("unable to get lock on db")
            .last_insert_rowid()
    )
}

#[get("/")]
pub fn index(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    let result = get_most_recent_tempurature(db);

    debug!("Retreieved latest temp value from db: {:?}", result);
    let json = format!("{{ \"temp\": \"{}\" }}", result);
    RawTempuratureInfoJson(json)
}

fn get_most_recent_tempurature(db: &State<crate::DbConn>) -> f64 {
    db.lock()
        .unwrap()
        .query_row(
            "SELECT temp from TempuratureRecords order by id desc limit 1;",
            [],
            |row| row.get(0),
        )
        .expect("unable to find tempurature record")
}

fn save_tempurature_record(temp_info: &TempuratureInfo, db: &State<crate::DbConn>) {
    db.lock()
        .expect("unable to get db lock")
        .execute(
            "INSERT INTO TempuratureRecords(temp) VALUES (?1)",
            [temp_info.tempurature],
        )
        .expect("unable to insert tempurature record");
}

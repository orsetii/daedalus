//! This module defines the endpoints for
//! receiving and processing the CAN traffic from the Van.
//!
//! The key part of this functionality is filtering. Obviously the CAN program
//! receiving the traffic from the bus performs filtering/processing to determine
//! what packets are what, what is happening. Events deemed major enough are then
//! sent to the endpoints below.

use chrono::Utc;
use rocket::response::content;
use rocket::response::status;
use rocket::serde::*;
use rocket::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct Command {
    pub id: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: String,
    pub expanded: bool,
}

impl Command {
    pub fn new() -> Self {
        Self {
            id: 0,
            timestamp: Utc::now(),
            data: format!("{:x?}", [0x21, 0x89, 0x12, 0xFF, 0x2F]),
            expanded: false,
        }
    }
}

#[post("/")]
pub fn post() -> &'static str {
    return "CAN message receieved";
}

#[get("/latest/<cnt>")]
pub fn get_latest(
    db: &State<crate::DbConn>,
    cnt: usize,
) -> status::Custom<content::RawJson<String>> {
    // TODO retreive page via id from database
    let mut res = Vec::new();
    for _i in 0..cnt {
        res.push(Command::new());
    }

    status::Custom(
        rocket::http::Status::Ok,
        content::RawJson(serde_json::to_string(&res).unwrap()),
    )
}

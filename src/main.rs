use std::sync::Mutex;

use rusqlite::{params, Connection, Result};
use services::{tempurature, CAN};

#[macro_use]
extern crate rocket;
extern crate tracing;

pub mod db;
pub mod services;
pub mod utils;

type DbConn = Mutex<Connection>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::fmt().init();

    let conn = Connection::open_in_memory().expect("in memory db");
    db::init(&conn);

    let _rocket = rocket::build()
        .manage(Mutex::new(conn))
        .mount("/hello", routes![index])
        .mount("/CAN", routes![CAN::post])
        .mount(
            "/tempurature",
            routes![tempurature::update, tempurature::index],
        )
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "ok"
}

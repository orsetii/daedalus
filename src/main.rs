use std::sync::Mutex;

use config::Config;
use rusqlite::{Connection, Result};
use services::{docs, temperature, CAN};

#[macro_use]
extern crate rocket;
extern crate tracing;

mod config;
mod cors;
pub mod db;
pub mod services;
pub mod utils;

type DbConn = Mutex<Connection>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::fmt().init();

    let config = Config {
        doc_root_path: String::from("/"),
    };

    let conn = Connection::open_in_memory().expect("in memory db");
    db::init(&conn);

    let _rocket = rocket::build()
        .manage(Mutex::new(conn))
        .manage(config)
        .mount("/hello", routes![index])
        .mount("/CAN", routes![CAN::post, CAN::get_latest])
        .mount(
            "/temperature",
            routes![temperature::update, temperature::index],
        )
        .mount(
            "/docs",
            routes![
                docs::get_by_id,
                docs::delete,
                docs::archive,
                docs::update_by_id,
                docs::create,
                docs::get_index
            ],
        )
        .attach(cors::CORS)
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "ok"
}

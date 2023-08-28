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

    let conn = Connection::open_in_memory().expect("in memory db");
    db::init(&conn);

    let config = Config {
        doc_root_path: String::from("/"),
    };

    let swarm = services::hive::Swarm::new();


    println!();


    let _rocket = rocket::build()
        .manage(Mutex::new(conn))
        .manage(config)
        .manage(Mutex::new(swarm))
        .mount("/hello", routes![index])
        .mount("/CAN", routes![CAN::post, CAN::get_latest])
        .mount(
            "/temperature",
            routes![temperature::update, temperature::internal_get, temperature::external_get, temperature::both_get, temperature::internal_post, temperature::external_post, temperature::internal_get_many, temperature::external_get_many],
        )
        .mount(
            "/docs",
            routes![
                docs::get_by_id,
                docs::delete,
                docs::archive,
                docs::update_by_id,
                docs::create,
                docs::get_index,
                docs::search_by_title,
                docs::get_recent
            ],
        )
        .mount("/hive", routes![services::hive::register, services::hive::index])
        .mount("/time", routes![services::time::time, services::time::time_no_seconds, services::time::current_date, services::time::timezone])
        .attach(cors::CORS)
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "ok"
}

use services::{tempurature, CAN};

#[macro_use]
extern crate rocket;
extern crate tracing;

pub mod services;
pub mod utils;

#[get("/")]
fn index() -> &'static str {
    "ok"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::fmt().init();

    let _rocket = rocket::build()
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

use services::CAN;

#[macro_use]
extern crate rocket;

mod services;

#[get("/")]
fn index() -> &'static str {
    "ok"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .mount("/CAN", routes![CAN::post])
        .launch()
        .await?;

    Ok(())
}

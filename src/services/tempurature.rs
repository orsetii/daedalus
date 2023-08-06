use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use tracing::{span, Level};

#[post("/update")]
pub fn update() -> &'static str {
    return "CAN message receieved";
}

#[get("/")]
pub fn index(db: &State<crate::DbConn>) -> RawTempuratureInfoJson {
    // TODO Retrieve from database, and insert.
    let a: f64 = db
        .lock()
        .unwrap()
        .query_row(
            "SELECT temp from TempuratureRecords order by id desc limit 1;",
            [],
            |row| row.get(0),
        )
        .unwrap();
    debug!("Retreieved latest temp value from db: {:?}", a);
    let json = format!("{{ \"temp\": \"{}\" }}", a);
    RawTempuratureInfoJson(json)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TempuratureInfo<'r> {
    type Error = TempuratureInfoError;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Failure((Status::BadRequest, TempuratureInfoError::InvalidField))
    }
}

#[derive(Debug, Clone, Copy)]
struct TempuratureInfo<'r> {
    pub _id: &'r uuid::Uuid,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawTempuratureInfoJson(String);

#[derive(Debug)]
enum TempuratureInfoError {
    InvalidField,
}

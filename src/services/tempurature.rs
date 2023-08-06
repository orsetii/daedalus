use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use tracing::{span, Level};

#[post("/update")]
pub fn update() -> &'static str {
    return "CAN message receieved";
}

#[get("/")]
pub fn index() -> RawTempuratureInfoJson {
    // TODO Retrieve from database, and insert.
    RawTempuratureInfoJson("{ \"hi\": \"world\" }")
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
pub struct RawTempuratureInfoJson(&'static str);

#[derive(Debug)]
enum TempuratureInfoError {
    InvalidField,
}

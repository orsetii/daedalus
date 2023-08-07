//! This is the module for storing and managing documentation and various documents of the
//! Icarus project.
//!
//! Note that documentation is stored both on the file system, and in the database.
use rocket::{
    response::{self, content, status, Result},
    State,
};
use uuid::Uuid;

mod fs;

#[get("/<id>")]
pub fn get_by_id(
    db: &State<crate::DbConn>,
    id: &str,
) -> status::Custom<response::content::RawJson<String>> {
    // TODO retreive page via id from database
    let doc = fs::get_by_id(Uuid::parse_str(id).unwrap());
    status::Custom(
        rocket::http::Status::Ok,
        content::RawJson(serde_json::to_string(&doc).unwrap()),
    )
}

#[post("/<id>")]
pub fn update_by_id(db: &State<crate::DbConn>, id: &str) {
    // TODO update the page here
}

#[get("/")]
pub fn get_index(db: &State<crate::DbConn>) {
    // TODO retreive all page titles, and their ids
}

/// Creates a new page with the received data
#[post("/new")]
pub fn create(db: &State<crate::DbConn>) {
    // TODO require input from Json with various metadata and the page content itself.
}

/// Archives a page
#[post("/archive/<id>")]
pub fn archive(db: &State<crate::DbConn>, id: &str) {
    // TODO require input from Json with various metadata and the page content itself.
}

/// Deletes a page
#[post("/delete/<id>")]
pub fn delete(db: &State<crate::DbConn>, id: &str) {
    // TODO require input from Json with various metadata and the page content itself.
}

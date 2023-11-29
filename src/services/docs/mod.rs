//! This is the module for storing and managing documentation and various documents of the
//! Icarus project.
//!
//! Note that documentation is stored both on the file system, and in the database.
#![allow(dead_code)]
#![allow(unused_variables)]

use rocket::{
    response::{self, content, status},
    State,
};

mod fs;
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

#[get("/title/<title>")]
pub fn get_by_title(title: &str) -> status::Custom<content::RawJson<String>> {
    let doc = fs::get_docs_by_title(title);
    status::Custom(
        rocket::http::Status::Ok,
        content::RawJson(serde_json::to_string(&doc.first()).unwrap()),
    )
}

// function that is a rocket endpoint, that takes a title, uses get_doc_by_title to find a matching document
#[get("/search/title/<title>")]
pub fn search_by_title(title: &str) -> status::Custom<content::RawJson<String>> {
    let docs = fs::get_docs_by_title(title);
    status::Custom(
        rocket::http::Status::Ok,
        content::RawJson(serde_json::to_string(&docs).unwrap()),
    )
}

// function that is a rocket endpoint, returns the most recently changed documents, from the filesystem, use other functions defined in this file
#[get("/recent/<n>")]
pub fn get_recent(n: &str) -> status::Custom<content::RawJson<String>> {
    let docs = fs::get_recent_docs(n.parse::<usize>().unwrap());
    status::Custom(
        rocket::http::Status::Ok,
        content::RawJson(serde_json::to_string(&docs).unwrap()),
    )
}




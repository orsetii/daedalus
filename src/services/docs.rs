use rocket::State;

#[get("/<id>")]
pub fn get_by_id(db: &State<crate::DbConn>, id: &str) {
    // TODO retreive page via id from database
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

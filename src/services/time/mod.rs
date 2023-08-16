use lazy_static::lazy_static;
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}



pub fn get_tz(lng: f64, lat: f64) -> &'static str {
    FINDER.get_tz_name(lng, lat)
}


#[get("/timezone")]
pub fn timezone() -> &'static str {
    // TODO get lat and long from somewhere
    //get_tz_offset()
    "TODO"
}

#[get("/")]
pub fn time() -> String {
    let now = chrono::Utc::now();
    format!("{}", now.format("%H:%M:%S"))
}

#[get("/no_seconds")]
pub fn time_no_seconds() -> String {
    let now = chrono::Utc::now();
    format!("{}", now.format("%H:%M"))
}

#[get("/date")]
pub fn current_date() -> String {
    let now = chrono::Utc::now();
    format!("{}", now.format("%Y/%m/%d"))
}
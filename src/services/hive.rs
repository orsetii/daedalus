
use std::sync::Mutex;

use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Swarm {
    pub devices: Vec<Device>,
    pub healthy: bool,
}

impl Swarm {
    pub fn new() -> Self {
        Swarm { devices: vec![], healthy: true }
    }
    // add a device to the swarm
    pub fn add_device(&mut self, device: NewDevice) {
        self.devices.push(Device {
            id: uuid::Uuid::new_v4().to_string(),
            name: device.name,
            ip: String::from(""),
            last_seen: chrono::Utc::now(),
            registered_at: chrono::Utc::now(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDevice {
    pub name: String,
}


#[post("/register", data = "<data>")]
pub fn register(swarm: &State<Mutex<Swarm>>, data: Json<NewDevice>) -> String {
    // get the ip from the request
    warn!("registering device: {:?}", data);
    swarm.lock().unwrap().add_device(data.into_inner());


    format!("Registered device")
}

#[get("/")]
pub fn index(swarm: &State<Mutex<Swarm>>) -> String {
    let s = swarm.lock().unwrap().clone();
   serde_json::to_string(&s).unwrap()
}
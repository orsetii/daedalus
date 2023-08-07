//! The filesystem interface for the documentation module.
//!
//!

use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Doc {
    pub name: String,
    pub content: Content,
}

impl Doc {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: String::from(name),
            content: Content(String::from(content)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Content(String);

// TODO impl interface for db and fs operations
pub fn get_by_name(name: &str) -> Doc {
    Doc::new("test", "test content")
}

pub fn get_by_id(id: Uuid) -> Doc {
    Doc::new("test", "test content")
}

//! The filesystem interface for the documentation module.
//!
//!

use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOCS_FOLDER: String = String::from(env!("CARGO_MANIFEST_DIR").to_string()  + "/docs");
}



#[derive(Deserialize, Serialize)]
pub struct Content(String);

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


pub fn get_doc(doc_path: &str) -> std::io::Result<Doc> {

    let path = std::path::Path::new(&*DOCS_FOLDER).join(doc_path);
    let content = &std::fs::read_to_string(path)?;

    Ok(Doc::new(&get_file_name_from_path(doc_path), content))
}

fn get_file_name_from_path(path: &str) -> String {
    let mut parts = path.split("/");
    if path.contains("\\") {
        parts = path.split("\\");
    }
    let final_name = parts.last().unwrap();
    String::from(final_name)
}


// write a test for get_doc to ensure it returns the correct doc
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc() {
        let doc = get_doc("test.md").unwrap();
        assert_eq!(doc.name, "test.md");
        assert_eq!(doc.content.0, "test content");
    }

    #[test]
    fn test_get_file_name_from_path() {
        let name = get_file_name_from_path("test.md");
        assert_eq!(name, "test.md");
    }
}   
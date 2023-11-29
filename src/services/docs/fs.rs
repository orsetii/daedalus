//! The filesystem interface for the documentation module.
//!
//!

use std::time::SystemTime;

use lazy_static::lazy_static;
use rocket::{
    form::validate::Contains,
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

lazy_static! {
    static ref DOCS_FOLDER: String = String::from(env!("CARGO_MANIFEST_DIR").to_string() + "/docs");
}

#[derive(Deserialize, Serialize)]
pub struct Content(String);

#[derive(Deserialize, Serialize)]
pub struct Doc {
    pub title: String,
    pub content: Content,
    pub modified_at: SystemTime,
}

impl Doc {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            title: String::from(name),
            content: Content(String::from(content)),
            modified_at: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn from_file(path_raw: &str) -> std::io::Result<Self> {
        let path = std::path::Path::new(&*DOCS_FOLDER).join(path_raw);
        warn!("path: {:?}", path);

        let modified_at = std::fs::metadata(path.clone())?.modified().unwrap();
        let content = &std::fs::read_to_string(path.clone())?;

        Ok(Self {
            title: String::from(&get_file_name_from_path(path_raw)),
            content: if content.len() != 0 {
                let mut lines = content.lines();
                let first_line = lines.next().unwrap();
                let content = lines.collect::<Vec<&str>>().join("\n");

                Content(content)
            } else {
                Content(String::new())
            },
            modified_at,
        })
    }
}

pub fn get_doc(doc_path: &str) -> std::io::Result<Doc> {
    let path = std::path::Path::new(&*DOCS_FOLDER).join(doc_path);
    warn!("path: {:?}", path);
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

// a function that searches a vector of docs for any that contain a string in their title
pub fn get_docs_by_title(title: &str) -> Vec<Doc> {
    let mut docs = Vec::new();
    let all_docs = get_all_docs();
    for doc in all_docs {
        if doc.title.contains(title) {
            docs.push(doc);
        }
    }
    docs
}

// a function that searches a vector of docs for a doc with some text in content
pub fn get_doc_by_content(content: &str) -> Option<Doc> {
    let docs = get_all_docs();
    for doc in docs {
        if doc.content.0.contains(content) {
            return Some(doc);
        }
    }
    None
}

pub fn get_all_docs() -> Vec<Doc> {
    let mut docs = Vec::new();
    let paths = std::fs::read_dir(&*DOCS_FOLDER).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && path.ends_with(".md") {
            let sub_paths = std::fs::read_dir(path).unwrap();
            for sub_path in sub_paths {
                let sub_path = sub_path.unwrap().path();
                if sub_path.is_file() {
                    let doc = Doc::from_file(sub_path.to_str().unwrap()).unwrap();
                    docs.push(doc);
                }
            }
        } else if path.is_file() {
            let doc = Doc::from_file(path.to_str().unwrap()).unwrap();
            docs.push(doc);
        }
    }
    docs
}

pub fn get_recent_docs(n: usize) -> Vec<Doc> {
    let mut docs = get_all_docs();
    docs.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    docs.truncate(n);
    docs
}

// write a test for get_doc to ensure it returns the correct doc
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc() {
        let doc = get_doc("test.md").unwrap();
        assert_eq!(doc.title, "test.md");
        assert_eq!(doc.content.0, "test content");
    }

    #[test]
    fn test_get_file_name_from_path() {
        let name = get_file_name_from_path("test.md");
        assert_eq!(name, "test.md");
    }
}

// std
use std::fs::{read_to_string, write};
use std::path::Path;

// crates
use rocket::{request::Form, *}; // change glob import
use serde::{Serialize, Deserialize}; 
use serde_json::{from_str};

// local

#[derive(Serialize, Debug, Copy, Clone)]
/// I have no idea why I can't use None variants for the TemplateContext struct, so I have to use this hack.
pub struct NoData();

#[derive(Serialize, Debug, Clone)]
/// Represents an HTTP 404 error
pub struct NotFoundError {
    /// Route is the current route; this can be changed to a static page later
    pub route: String,
}

/// Form data for the /page/new route, for creating a new page
#[derive(FromForm, Debug)]
pub struct NewPage {
    pub data: String,
    pub new_url: String,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct PageIndex {
    pub files: Vec<String>
}

pub fn get_page_index() -> PageIndex {
    let index_from_file = read_to_string(Path::new("content/pages/index.json")).unwrap_or(String::from("Somethign went wrong."));
    let index: PageIndex = from_str(&index_from_file.as_str()).unwrap_or(PageIndex { files: vec![String::from("Something went wrong.")]});
    println!("INDEX: {:#?}", index);
    index
}

pub fn update_index(item: &String) -> Result<(), std::io::Error> {
    let mut index = get_page_index().to_owned();
    index.files.push(item.to_owned());
    write(Path::new("content/pages/index.json"), serde_json::to_string(&index).unwrap_or(String::from("Something went wrong.")).as_str())
}

/// Represents the content of a single custom page
#[derive(Serialize, Debug, Clone)]
pub struct Page {
    pub content: String,
}

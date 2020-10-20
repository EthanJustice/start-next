// std

// external
use rocket::{request::Form, *};
use serde::Serialize; // change glob import

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

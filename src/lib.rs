// std

// external
use rocket::{request::Form, *};
use serde::Serialize; // change glob import

// local

#[derive(Serialize)]
/// Represents an HTTP 404 error
pub struct NotFoundError {
    /// Route is the current route; this can be changed to a static page later
    pub route: String,
}

/// Form data for the /page/new route, for creating a new page
#[derive(FromForm)]
pub struct NewPage {
    pub data: String,
    pub new_url: String,
}

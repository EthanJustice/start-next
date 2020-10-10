// std

// external
use rocket::*; // glob for now
use rocket_contrib::{serve::StaticFiles, templates::Template};

// local

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}

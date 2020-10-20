#![feature(proc_macro_hygiene, decl_macro)]
// std

// external
use serde::Serialize;

use rocket::{request::Form, *}; // change glob import
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

// local
use start_next::{NewPage, NotFoundError};

#[derive(Serialize)]
struct TemplateContext<T: Serialize> {
    title: &'static str,
    parent: &'static str,
    data: T,
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "not-found",
        &TemplateContext {
            title: "Not Found",
            parent: "layout",
            data: &NotFoundError {
                route: req.uri().to_string(),
            },
        },
    )
}

#[post("/new/page", format = "application/json", data = "<items>")]
fn add_new(items: Option<Form<NewPage>>) -> String {
    String::new()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![add_new])
        .mount(
            "/",
            StaticFiles::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/static"),
                Options::Index,
            ),
        )
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]
// std

// external
use serde::Serialize;

use rocket::{request::Form, *}; // change glob import
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

// local
use start_next::{NewPage, NoData, NotFoundError};

#[derive(Serialize)]
struct TemplateContext<T: Serialize> {
    title: &'static str,
    parent: &'static str,
    data: std::option::Option<T>,
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "not-found",
        &TemplateContext {
            title: "Not Found",
            parent: "layout",
            data: Some(&NotFoundError {
                route: req.uri().to_string(),
            }),
        },
    )
}

#[get("/")]
fn get_index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "home :: start",
            parent: "layout",
            data: Some(NoData()),
        },
    )
}

#[get("/pages")]
fn get_pages() -> Template {
    Template::render(
        "pages/index",
        &TemplateContext {
            title: "pages :: start",
            parent: "layout",
            data: Some(NoData()),
        },
    )
}

#[get("/paste")]
fn get_paste() -> Template {
    Template::render(
        "paste/index",
        &TemplateContext {
            title: "paste :: start",
            parent: "layout",
            data: Some(NoData()),
        },
    )
}

#[post("/new/page", format = "application/json", data = "<_items>")]
fn add_new_page(_items: Option<Form<NewPage>>) -> String {
    String::new()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![add_new_page, get_index, get_pages, get_paste])
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

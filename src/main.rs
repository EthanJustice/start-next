#![feature(proc_macro_hygiene, decl_macro)]
// std
use std::fs::{create_dir_all, write, File};

// external
use serde::Serialize;

use rocket::{request::Form, response::Redirect, *}; // change glob import
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

use pulldown_cmark::{html, Options as PulldownOptions, Parser};

// local
use start_next::{NewPage, NoData, NotFoundError};

#[derive(Serialize)]
struct TemplateContext<T: Serialize, S: Into<String>> {
    title: S,
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

#[get("/pages", rank = 5)]
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

#[get("/page/<page>")]
fn get_page_item(page: String) -> Template {
    Template::render(
        "pages/index",
        &TemplateContext {
            title: format!("pages :: {}", page),
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

#[post(
    "/new/page",
    format = "application/x-www-form-urlencoded",
    data = "<items>"
)]
fn add_new_page(items: Option<Form<NewPage>>) -> Result<Redirect, &'static str> {
    println!("ITEMS: {:#?}", items);
    match items {
        Some(form) => {
            create_dir_all(format!(
                "{}/content/pages/{}",
                env!("CARGO_MANIFEST_DIR"),
                form.new_url.replace(" ", "-")
            ))
            .unwrap();

            let options = PulldownOptions::all();
            let parser = Parser::new_ext(form.data.as_str(), options);
            let mut html = String::new();
            html::push_html(&mut html, parser);

            write(
                format!(
                    "{}/content/pages/{}.html",
                    env!("CARGO_MANIFEST_DIR"),
                    form.new_url
                ),
                html,
            )
            .unwrap();

            Ok(Redirect::to(format!(
                "/page/{}",
                form.new_url.replace(" ", "-")
            )))
        }
        None => Err("Invalid input"),
    }
}

fn main() {
    //    if std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/content")).is_dir() == false {
    //        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/content/pages"))
    //            .expect("Failed to generate setup directory structure, aborting...");
    //    }
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

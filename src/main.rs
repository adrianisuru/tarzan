#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;

// Needed because 'ref' is a reserved word
// https://stackoverflow.com/questions/54055375/how-to-create-an-endpoint-with-a-rust-keyword-as-a-query-dynamic-parameter
#[derive(FromForm)]
struct Reff {
    #[form(field = "ref")]
    reff: String,
}

#[get("/?<path>&<format>&<reff..>")]
fn get_archive(path: String, reff: Form<Reff>, format: Option<String>) -> String {
    format!(
        "{}\n{}\n{}",
        path,
        reff.reff,
        format.unwrap_or("tar.gz".to_string())
    )
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_archive, index])
        .launch();
}

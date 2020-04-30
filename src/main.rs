#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate qrcode_generator;

use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::process::Command;

fn genkey() -> String {
    let output = Command::new("sh")
        .args(&["-c", "wg", "genkey"])
        .output()
        .expect("failed to execute process");
    let stdout = output.stdout;
    let privkey = String::from_utf8(stdout)
        .unwrap();
    return privkey;
}

#[get("/")]
fn index() -> String {
    "sup dog".to_string()
}

#[get("/new")]
fn new_key() -> String {
    let new_key = genkey();
    new_key.to_string()
}

#[catch(404)]
fn not_found() -> String {
    "not found".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index, new_key
        ])
        .mount("/static", StaticFiles::from("./static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate qrcode_generator;

use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use qrcode_generator::QrCodeEcc;
use std::process::Command;

fn genkey() -> String {
    let output = Command::new("wg")
        .arg("genkey")
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
fn new_key() -> Template {
    let new_key = genkey();
    let qr_code: String = qrcode_generator::to_svg_to_string(new_key, QrCodeEcc::Low, 256, None)
        .unwrap();
    let qr_code: String = base64::encode(qr_code);
    let context: JsonValue = json!({
        "qr_code": qr_code,
    });
    Template::render("new", context)
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

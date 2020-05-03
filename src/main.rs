#![feature(proc_macro_hygiene, decl_macro, plugin)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate qrcode_generator;
mod routes;
mod data;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;


#[catch(404)]
fn not_found() -> String {
    "not found".to_string()
}

fn main() {
    let wg_opts = data::WireGuardOptions {
        ..Default::default()
    };
    println!("{:#?}", wg_opts);
    rocket::ignite()
        .mount("/", routes![
            routes::index, routes::generate
        ])
        .mount("/static", StaticFiles::from("./static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

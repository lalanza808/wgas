#![feature(proc_macro_hygiene, decl_macro, plugin)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
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
            routes::home, routes::add_peer, routes::save_peer_config,
        ])
        .mount("/static", StaticFiles::from("./static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

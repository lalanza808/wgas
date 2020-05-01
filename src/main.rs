#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate qrcode_generator;

use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use qrcode_generator::QrCodeEcc;
use structopt::StructOpt;
use std::process::Command;


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "k", long = "pubkey")]
    pubkey: String,
    #[structopt(short = "r", long = "route")]
    route: String,
    #[structopt(short = "e", long = "endpoint")]
    endpoint: String,
    #[structopt(short = "p", long = "port")]
    port: u32,
    #[structopt(short = "d", long = "dns")]
    dns: String,
}

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

#[get("/generate")]
fn generate(cli_args: State<Cli>) -> Template {
    let new_key = genkey();
    let full_config = format!("[Interface]
PrivateKey = {}
Address = 10.66.66.2/32
DNS = {}

[Peer]
PublicKey = {}
AllowedIPs = {}
Endpoint = {}:{}
PersistentKeepalive = 21",
        new_key.trim_end(),
        cli_args.dns,
        cli_args.pubkey,
        cli_args.route,
        cli_args.endpoint,
        cli_args.port
    );
    let qr_code: String = qrcode_generator::to_svg_to_string(
        &full_config, QrCodeEcc::Low, 256, None
    ).unwrap();
    let qr_code: String = base64::encode(qr_code);
    let context: JsonValue = json!({
        "qr_code": qr_code,
        "full_config": full_config
    });
    Template::render("new", context)
}

#[catch(404)]
fn not_found() -> String {
    "not found".to_string()
}

fn main() {
    let cli_args = Cli::from_args();
    println!("{:#?}", cli_args);
    rocket::ignite()
        .manage(cli_args)
        .mount("/", routes![
            index, generate
        ])
        .mount("/static", StaticFiles::from("./static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

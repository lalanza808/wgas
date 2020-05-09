#[path = "data.rs"]
mod data;
#[path = "helpers.rs"]
mod helpers;

use data::{WireGuardOptions, PeerConfig};
use helpers::{wg_cmd, sh_cmd};
use rocket_contrib::templates::Template;
use rocket_contrib::json::{JsonValue, Json};
use std::fs::File;
use std::io::prelude::*;
use std::fs;


#[get("/")]
pub fn home() -> Template {
    let show_config = wg_cmd(&["show"]);
    let whoami = sh_cmd("whoami");
    let uptime = sh_cmd("uptime");
    let hostname = sh_cmd("hostname");
    let netstat_info = sh_cmd("netstat -tan | grep \"ESTABLISHED\\|CLOSE_WAIT\"");
    let shell_ps1 = format!(
        "{}@{} $",
        whoami.trim_end(),
        hostname.trim_end()
    );
    let context: JsonValue = json!({
        "show_config": show_config.trim_end(),
        "uptime": uptime.trim_end(),
        "netstat_info": netstat_info,
        "shell_ps1": shell_ps1,
    });

    Template::render("home", context)
}

#[get("/add-peer")]
pub fn add_peer() -> Template {
    let new_key = wg_cmd(&["genkey"]);
    let state = WireGuardOptions { ..Default::default() };
    let context: JsonValue = json!({
        "privkey": new_key.trim_end(),
        "state": state,
    });

    Template::render("add_peer", context)
}

#[post("/save-peer", data = "<peer_config>")]
pub fn save_peer_config(peer_config: Json<PeerConfig>) -> JsonValue {
    println!("{:#?}", peer_config);
//     let peer_config = format!("[Peer]
// # name = {}
// PublicKey = {}
// AllowedIPs = {}/32")
//     let mut file = File::create("/tmp/wgas.conf").unwrap();
//     let conf_str = serde_json::to_string(&input.into_inner()).unwrap();
//     file.write_all().unwrap();
//     let wg_set = wg_cmda(&["set", "wg0", "private-key", "/tmp/wgas.conf"]); // todo - randomize
    // let file_removed = match fs::remove_file("/tmp/wgas.conf"){
    //     Ok(_) => true,
    //     Err(_) => false
    // };
    json!({
        "eyo": "hello",
        "config_data": "asd",
        // "response": wg_set.trim_end(),
        // "key_cleared": file_removed
    })
}

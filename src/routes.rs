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
    let show_config_cmd = wg_cmd(vec!["show".to_string()]);
    let whoami = sh_cmd("whoami".to_string());
    let w = sh_cmd("w".to_string());
    let hostname = sh_cmd("hostname".to_string());
    let netstat_info = sh_cmd("netstat -tan | grep \"ESTABLISHED\\|CLOSE_WAIT\"".to_string());
    let shell_ps1 = format!(
        "{}@{} $",
        whoami.trim_end(),
        hostname.trim_end()
    );
    let context: JsonValue = json!({
        "show_config": show_config_cmd.0.trim_end(),
        "w": w.trim_end(),
        "netstat_info": netstat_info,
        "shell_ps1": shell_ps1,
    });

    Template::render("home", context)
}

#[get("/add-peer")]
pub fn add_peer() -> Template {
    let privkey_cmd = wg_cmd(vec!["genkey".to_string()]);
    let generate_pubkey = format!("echo '{}' | wg pubkey", privkey_cmd.0.trim_end());
    let pubkey_cmd = sh_cmd(generate_pubkey);
    let state = WireGuardOptions { ..Default::default() };
    let context: JsonValue = json!({
        "privkey": privkey_cmd.0.trim_end(),
        "pubkey": pubkey_cmd.trim_end(),
        "state": state,
    });

    Template::render("add_peer", context)
}

#[post("/save-peer", data = "<peer_config>")]
pub fn save_peer(peer_config: Json<PeerConfig>) -> JsonValue {
    let state = WireGuardOptions { ..Default::default() };
    let new_peer_config = format!(
        "[Peer]\n# name = {}\nPublicKey = {}\nAllowedIPs = {}/32",
        peer_config.name,
        peer_config.pubkey,
        peer_config.ipaddr,
    );
    let file_path = format!(
        "./etc/{}-{}-wgas.conf",
        peer_config.name,
        peer_config.ipaddr,
    );
    let mut file = File::create(&file_path).unwrap();
    file.write_all(&new_peer_config.as_bytes()).unwrap();
    let wg_set = wg_cmd(vec!["addconf".to_string(), state.interface, file_path.clone()]);
    let file_removed = match fs::remove_file(file_path){
         Ok(_) => true,
         Err(_) => false
     };
    json!({
        "command_stdout": wg_set.0.trim_end(),
        "command_exit": wg_set.1,
        "file_removed": file_removed
    })
}

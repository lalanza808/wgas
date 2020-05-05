#[path = "data.rs"]
mod data;
#[path = "helpers.rs"]
mod helpers;

use data::WireGuardOptions;
use helpers::{wg_cmd, sh_cmd};
use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use qrcode_generator::QrCodeEcc;


#[get("/")]
pub fn home() -> Template {
    let show_config = wg_cmd("show".to_string());
    let whoami = sh_cmd("whoami".to_string());
    let uptime = sh_cmd("uptime".to_string());
    let hostname = sh_cmd("hostname".to_string());
    let netstat_info = sh_cmd("netstat -tan | grep \"ESTABLISHED\\|CLOSE_WAIT\"".to_string());
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
    let new_key = wg_cmd("genkey".to_string());
    let state = WireGuardOptions { ..Default::default() };
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
        state.dns,
        state.pubkey,
        state.route,
        state.endpoint,
        state.port
    );
    let qr_code: String = qrcode_generator::to_svg_to_string(
        &full_config, QrCodeEcc::Low, 256, None
    ).unwrap();
    let qr_code: String = base64::encode(qr_code);
    let context: JsonValue = json!({
        "qr_code": qr_code,
        "full_config": full_config
    });
    Template::render("add_peer", context)
}

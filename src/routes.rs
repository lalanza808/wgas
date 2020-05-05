#[path = "data.rs"]
mod data;
#[path = "helpers.rs"]
mod helpers;

use data::WireGuardOptions;
use helpers::wg_cmd;
use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use qrcode_generator::QrCodeEcc;


#[get("/")]
pub fn index() -> Template {
    let current_config = wg_cmd("show".to_string());
    let context: JsonValue = json!({
        "show_config": current_config
    });
    Template::render("index", context)
}

#[get("/generate")]
pub fn generate() -> Template {
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
    Template::render("generate", context)
}

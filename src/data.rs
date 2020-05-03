use std::env;

#[derive(Debug)]
pub struct WireGuardOptions {
    pub pubkey: String,
    pub route: String,
    pub endpoint: String,
    pub sudo: String,
    pub port: String,
    pub dns: String,
}


impl Default for WireGuardOptions {
    fn default() -> WireGuardOptions {
        WireGuardOptions {
            pubkey: env!("WGAS_PUBKEY").to_string(),
            endpoint: env!("WGAS_ENDPOINT").to_string(),
            sudo: env::var("WGAS_SUDO").unwrap_or("false".to_string()),
            dns: env::var("WGAS_DNS").unwrap_or("1.1.1.1".to_string()),
            route: env::var("WGAS_ROUTE").unwrap_or("0.0.0.0/0".to_string()),
            port: env::var("WGAS_PORT").unwrap_or(51820.to_string()),
        }
    }
}

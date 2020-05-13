use std::env;
use std::net::Ipv4Addr;


#[derive(Debug, Deserialize, Serialize)]
pub struct WireGuardOptions {
    pub pubkey: String,
    pub route: String,
    pub endpoint: String,
    pub sudo: String,
    pub port: String,
    pub dns: String,
    pub network: String,
    pub interface: String
}


impl Default for WireGuardOptions {
    fn default() -> WireGuardOptions {
        WireGuardOptions {
            pubkey: env::var("WGAS_PUBKEY").unwrap(),
            endpoint: env::var("WGAS_ENDPOINT").unwrap(),
            sudo: env::var("WGAS_SUDO").unwrap_or("false".to_string()),
            dns: env::var("WGAS_DNS").unwrap_or("1.1.1.1".to_string()),
            route: env::var("WGAS_ROUTE").unwrap_or("0.0.0.0/0".to_string()),
            port: env::var("WGAS_PORT").unwrap_or(51820.to_string()),
            network: env::var("WGAS_NETWORK").unwrap_or("10.66.66.1/24".to_string()),
            interface: env::var("WGAS_INTERFACE").unwrap_or("wg0".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PeerConfig {
    pub name: String,
    pub pubkey: String,
    pub ipaddr: Ipv4Addr,
}

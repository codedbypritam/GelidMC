use serde::Deserialize;
use std::fs;
use crate::server::GelidServer;

mod server;

fn main() {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap()).unwrap();

    println!("Starting server at {}:{}", config.server_ip, config.port);

    let network_profile = server::NetworkProfile::new(config.server_ip, config.port);
    let server: GelidServer = GelidServer::new(network_profile);

    
}

#[derive(Deserialize)]
struct Config {
    server_ip: String,
    port: u16
}
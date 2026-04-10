pub struct GelidServer {
    pub network_profile: NetworkProfile
}

impl GelidServer {
    pub fn new(network_profile: NetworkProfile) -> Self {
        Self { network_profile }
    }
}

pub struct NetworkProfile {
    address: String,
    port: u16
}

impl NetworkProfile {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub fn get_socket_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
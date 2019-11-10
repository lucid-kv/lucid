use std::net::Ipv4Addr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub default: Base,
    pub authentication: Authentication,
    pub persistence: Persistence,
    pub webui: WebUI,
    pub store: Store
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            default: Base {
                bind_address: Ipv4Addr::LOCALHOST.to_string(),
                port: 7021, // TODO: change after implementing SSL
                port_ssl: 7021,
                use_ssl: false,
            },
            authentication: Authentication {
                enabled: true,
                root_token: String::new(),
                secret_key: String::new(),
            },
            persistence: Persistence {
                enabled: false,
                location: String::new(),
            },
            webui: WebUI {
                enabled: false
            },
            store: Store {
                max_limit: 7340032
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
    pub bind_address: String,
    pub port: i32,
    pub port_ssl: i32,
    pub use_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authentication {
    pub enabled: bool,
    pub root_token: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persistence {
    pub enabled: bool,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebUI {
    pub enabled: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub max_limit: i32
}
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use app_dirs::{AppDataType, AppDirsError};
use log::LevelFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub general: General,
    pub authentication: Authentication,
    pub persistence: Persistence,
    pub encryption: Encryption,
    pub webui: WebUI,
    pub store: Store,
    pub http: Http,
    pub logging: Logging,
}

impl Configuration {
    pub fn get_path() -> Result<PathBuf, AppDirsError> {
        let mut path = app_dirs::get_app_root(AppDataType::UserConfig, &crate::APP_INFO)?;
        path.push("lucid.yml");
        Ok(path)
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            general: General {
                bind_address: IpAddr::from(Ipv4Addr::LOCALHOST),
                port: 7020,
                port_ssl: 7021,
                use_ssl: true,
                ssl_certificate: String::new(),
                ssl_certificate_key: String::new(),
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
            encryption: Encryption {
                enabled: false,
                private_key: String::new(),
                iv: String::new(),
            },
            webui: WebUI { enabled: false },
            store: Store { max_limit: 7340032 },
            http: Http {
                request_size_limit: 8388608,
            },
            logging: Logging {
                level: LevelFilter::Info,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct General {
    pub bind_address: IpAddr,
    pub port: u16,
    pub port_ssl: u16,
    pub use_ssl: bool,
    pub ssl_certificate: String,
    pub ssl_certificate_key: String,
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
pub struct Encryption {
    pub enabled: bool,
    pub private_key: String,
    pub iv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebUI {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub max_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Http {
    pub request_size_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    pub level: LevelFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

use std::sync::{Arc, RwLock};

use crate::configuration::Configuration;
use crate::server::Server;

pub struct Lucid {
    configuration: Arc<RwLock<Configuration>>,
}

impl Lucid {
    pub fn new(configuration: Configuration) -> Self {
        Lucid {
            configuration: Arc::new(RwLock::new(configuration)),
        }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let server = Server::new(self.configuration.clone());
        server.run().await;
        Ok(())
    }
}

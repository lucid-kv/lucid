use std::sync::{Arc, RwLock};

use snafu::Snafu;

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

    pub fn run(&self) -> Result<(), LucidError> {
        let server = Server::new(self.configuration.clone());
        server.run();
        return Ok(());
    }
}

#[derive(Debug, Snafu)]
pub enum LucidError {}

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use lucid::Lucid;

mod lucid;
mod logger;
mod kvstore;
mod server;
mod configuration;

fn main() -> Result<(), std::io::Error> {
    let mut lucid = Lucid::default();
    return lucid.initialize();
}

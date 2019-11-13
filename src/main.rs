#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

//extern crate hyper;
//extern crate hyper_openssl;

use lucid::Lucid;

mod lucid;
mod kvstore;
mod server;
mod configuration;

fn main() -> Result<(), std::io::Error> {
    let mut lucid = Lucid::default();
    lucid.initialize()
}
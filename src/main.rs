#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

mod lucid;
mod logger;
mod server;

use lucid::Lucid;

fn main() -> Result<(), String> {
    let mut lucid = Lucid::default();
    lucid.initialize_cli();
    Ok(())
}

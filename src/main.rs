#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

mod logger;
mod lucid;
mod server;

use lucid::Lucid;

fn main() -> Result<(), String> {
    let mut lucid = Lucid::default();
    match lucid.init() {
        Ok(_) => {}
        Err(_) => {}
    }
    Ok(())
}

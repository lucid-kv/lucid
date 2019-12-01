#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use chrono::Utc;
use fern::Dispatch;
use log::LevelFilter;

use lucid::Lucid;

mod configuration;
mod kvstore;
mod lucid;
mod server;

fn main() -> Result<(), std::io::Error> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .apply()
        .expect("Couldn't start logger");
    log::set_max_level(LevelFilter::Debug);
    let mut lucid = Lucid::default();
    lucid.initialize()
}

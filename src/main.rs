#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use chrono::Utc;
use fern::Dispatch;
use log::LevelFilter;
use fern::colors::{Color, ColoredLevelConfig};

use lucid::Lucid;

mod configuration;
mod kvstore;
mod lucid;
mod server;

fn main() -> Result<(), std::io::Error> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Magenta);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                colors.color(record.level()),
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

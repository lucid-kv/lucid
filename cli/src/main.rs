#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

use std::fmt;

use clap::{App, AppSettings};
use snafu::Snafu;

const CREDITS: &'static str = "\
                               +-----------------+-----------------------+--------------------+\n\
                               |               Lucid KV Development Credits                   |\n\
                               +-----------------+-----------------------+--------------------+\n\
                               | Clint Mourlevat | me@clint.network      | Lucid Founder      |\n\
                               | Jonathan Serra  | jonathan@blocs.fr     | Core Development   |\n\
                               | CephalonRho     | CephalonRho@gmail.com | Core Development   |\n\
                               | Rigwild         | me@rigwild.dev        | Web UI Development |\n\
                               +-----------------+-----------------------+--------------------+";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let long_version = format!("{}\n{}", crate_version!(), CREDITS);
    let cli_yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(&cli_yaml)
        .version(crate_version!())
        .long_version(long_version.as_str());

    let matches = match app.get_matches_safe() {
        Ok(x) => x,
        Err(clap::Error {
            kind: clap::ErrorKind::HelpDisplayed,
            message,
            ..
        })
        | Err(clap::Error {
            kind: clap::ErrorKind::VersionDisplayed,
            message,
            ..
        }) => {
            println!("{}", message);
            return Ok(());
        }
        Err(e) => return Err(Error::ParseCli { source: e }),
    };

    if let Some(store_matches) = matches.subcommand_matches("store") {
        if let Some(get_matches) = matches.subcommand_matches("store") {
            if let Some(key) = matches.value_of("key") {
                
            }
        }
    }
    Ok(())
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ParseCli { source: clap::Error },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

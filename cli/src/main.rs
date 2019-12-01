#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

mod client;

use client::{KvPutResponse, LucidClient, LucidClientError};

use std::fmt;

use clap::App;
use snafu::{ResultExt, Snafu};

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

    let client = LucidClient::new(
        matches.value_of("uri").unwrap(),
        matches.value_of("secret").map(|x| x.to_owned()),
    )
    .context(CreateClient)?;

    if let Some(store_matches) = matches.subcommand_matches("store") {
        if let Some(get_matches) = store_matches.subcommand_matches("get") {
            let key = get_matches.value_of("key").unwrap();
            let response = client.get(key).await.context(RequestFailed)?;
            if let Some(element) = response {
                if element.mime_type == "text/plain" {
                    println!("{}", String::from_utf8_lossy(&element.data))
                } else {
                    println!("{:?}", &element.data)
                }
            } else {
                println!("Key  \"{}\" not found", key);
            }
        } else if let Some(put_matches) = store_matches.subcommand_matches("put") {
            let key = put_matches.value_of("key").unwrap();
            let value = put_matches.value_of("value").unwrap();
            let response = client
                .put(key, value.as_bytes().to_owned())
                .await
                .context(RequestFailed)?;
            match response {
                KvPutResponse::Created => {
                    println!("Successfully created key {} with value {}", key, value)
                }
                KvPutResponse::Updated => {
                    println!("Successfully updated key {} with value {}", key, value)
                }
            }
        } else if let Some(delete_matches) = store_matches.subcommand_matches("delete") {
            let key = delete_matches.value_of("key").unwrap();
            if client.delete(key).await.context(RequestFailed)? {
                println!("Successfully deleted key \"{}\"", key)
            } else {
                println!("Key \"{}\" not found", key)
            }
        }
    }
    Ok(())
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ParseCli { source: clap::Error },
    #[snafu(display("Failed to create the client: {}", source))]
    CreateClient { source: LucidClientError },
    #[snafu(display("{}", source))]
    RequestFailed { source: LucidClientError },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

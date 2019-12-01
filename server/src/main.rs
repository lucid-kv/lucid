#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

mod configuration;
mod kvstore;
mod lucid;
mod server;

use lucid::{Lucid, LucidError};

use std::{
    fmt,
    fs::{self, File},
    path::Path,
};

use app_dirs::{AppDirsError, AppInfo};
use chrono::{DateTime, Duration, Utc};
use clap::{App, AppSettings};
use configuration::{Claims, Configuration};
use fern::Dispatch;
use jsonwebtoken::Header;
use log::LevelFilter;
use rand::Rng;
use ring::digest;
use snafu::{ResultExt, Snafu};

const APP_INFO: AppInfo = AppInfo {
    name: "lucid",
    author: "Clint.Network",
};

const BANNER: &'static str = r###"
██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝
"###;

const CREDITS: &'static str = "\
                               +-----------------+-----------------------+--------------------+\n\
                               |               Lucid KV Development Credits                   |\n\
                               +-----------------+-----------------------+--------------------+\n\
                               | Clint Mourlevat | me@clint.network      | Lucid Founder      |\n\
                               | Jonathan Serra  | jonathan@blocs.fr     | Core Development   |\n\
                               | CephalonRho     | CephalonRho@gmail.com | Core Development   |\n\
                               | Rigwild         | me@rigwild.dev        | Web UI Development |\n\
                               +-----------------+-----------------------+--------------------+";

fn main() -> Result<(), Error> {
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

    println!("{}", BANNER);
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

    let config_path = {
        if let Some(config) = matches.value_of("config") {
            Path::new(config).to_path_buf()
        } else {
            Configuration::get_path().context(GetConfigDir)?
        }
    };
    if let Some(init_matches) = matches.subcommand_matches("init") {
        if config_path.exists() && !init_matches.is_present("force") {
            return Err(Error::AlreadyInitialized);
        } else {
            let mut config = Configuration::default();
            let secret_key = generate_secret_key();
            config.authentication.root_token = issue_jwt(&secret_key, None)?;
            config.authentication.secret_key = secret_key;
            fs::create_dir_all(config_path.parent().unwrap()).context(CreateConfigDir)?;
            serde_yaml::to_writer(
                File::create(&config_path).context(CreateConfigFile)?,
                &config,
            )
            .context(WriteConfigFile)?;
            info!(
                "Lucid successfully initialized in {}",
                config_path.to_string_lossy()
            );
        }
    } else {
        if config_path.exists() {
            let config: Configuration =
                serde_yaml::from_reader(File::open(&config_path).context(OpenConfigFile)?)
                    .context(ReadConfigFile)?;
            log::set_max_level(config.logging.level); // this has to be executed every time the logging configuration changes
            Lucid::new(config).run().context(RunServer)?;
        } else {
            return Err(Error::ConfigurationNotFound);
        }
    }
    Ok(())
}

fn generate_secret_key() -> String {
    let secret_key_bytes = digest::digest(&digest::SHA256, &rand::thread_rng().gen::<[u8; 32]>());
    secret_key_bytes.as_ref().iter().fold(
        String::with_capacity(secret_key_bytes.as_ref().len() * 2),
        |mut acc, x| {
            acc.push_str(&format!("{:0>2x}", x));
            acc
        },
    )
}

fn issue_jwt(secret_key: &str, expiration: Option<DateTime<Utc>>) -> Result<String, Error> {
    let lucid_root_claims = Claims {
        sub: String::from("Lucid Root Token"),
        iss: String::from("http://127.0.0.1:7021/"), // TODO: check issuer, maybe set the proper uri
        iat: Utc::now().timestamp(),
        exp: match expiration {
            Some(exp) => exp.timestamp(),
            None => (Utc::now() + Duration::weeks(52 * 3)).timestamp(),
        },
    };
    jsonwebtoken::encode(&Header::default(), &lucid_root_claims, secret_key.as_ref())
        .context(EncodeJwt)
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ParseCli { source: clap::Error },
    #[snafu(display("{}", source))]
    RunServer { source: LucidError },
    #[snafu(display("Configuration file not found"))]
    ConfigurationNotFound,
    #[snafu(display("The Lucid node has already been initialized"))]
    AlreadyInitialized,
    #[snafu(display("Unable to get the Lucid configuration directory: {}", source))]
    GetConfigDir { source: AppDirsError },
    #[snafu(display("Unable to create the Lucid configuration directory: {}", source))]
    CreateConfigDir { source: std::io::Error },
    #[snafu(display("Unable to create the Lucid configuration file: {}", source))]
    CreateConfigFile { source: std::io::Error },
    #[snafu(display("Unable to write the Lucid configuration file: {}", source))]
    WriteConfigFile { source: serde_yaml::Error },
    #[snafu(display("Unable to open the Lucid configuration file: {}", source))]
    OpenConfigFile { source: std::io::Error },
    #[snafu(display("Unable to read the Lucid configuration file: {}", source))]
    ReadConfigFile { source: serde_yaml::Error },
    #[snafu(display("Error while encoding the JWT root token: {}", source))]
    EncodeJwt { source: jsonwebtoken::errors::Error },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

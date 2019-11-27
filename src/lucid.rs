use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind, Write},
    path::Path,
    str::FromStr,
};

use app_dirs::*;
use chrono::{DateTime, Duration, Utc};
use clap::App;
use jsonwebtoken::*;
use rand::Rng;
use ring::digest::SHA256;
use log::LevelFilter;

use crate::configuration::{Claims, Configuration};
use crate::server::Server;

pub struct Lucid {
    configuration_location: String,
    configuration: Option<Configuration>
}

impl Lucid {
    pub fn default() -> Lucid {
        Lucid {
            configuration_location: String::new(),
            configuration: None
        }
    }

    pub fn show_banner(&self) {
        println!(r###"
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝
 "###);
    }

    fn show_version(&self) {
        println!("Lucid Version {}\n", crate_version!());
    }

    fn show_help(&self, commands: &mut App) {
        commands.print_help().unwrap();
        println!("\n");
    }

    pub fn initialize(&mut self) -> Result<(), std::io::Error> {
        let cli_yaml = load_yaml!("cli.yml");
        let mut commands = App::from_yaml(cli_yaml)
            .name(crate_description!());
        self.show_banner();
        match self.handle_cli(&mut commands) {
            Some(usage) => println!("{}{}", usage, match usage.to_owned().contains("USAGE") {
                true => "\n",
                false => ""
            }),
            None => self.show_help(&mut commands)
        }
        return Ok(());
    }

    fn handle_cli(&mut self, commands: &mut App) -> Option<&str> {
        match commands.get_matches_from_safe_borrow(::std::env::args_os()) {
            Ok(cli) => {
                if cli.is_present("help") {
                    return None;
                }
                if cli.is_present("version") {
                    self.show_version();
                    return Some("")
                }

                if let Some(matches) = cli.subcommand_matches("cli") {
                    fn display_cli_help() {
                        info!(
                            "This is all the available commands:\n\
                             - set       [key] - Set an object\n\
                             - get       [key] - Get an object\n\
                             - lock      [key] - Lock an object\n\
                             - unlock    [key] - Unlock an object\n\
                             - expire    [key] - Set an object expiration date\n\
                             - increment [key] - Increment the value of an object\n\
                             - decrement [key] - Decrement the value of an object\n\
                             - drop      [key] - Drop an object"
                        );
                    }

                    if matches.is_present("help") {
                        info!("Welcome to the Lucid Command Line Interface (CLI)\n");
                        display_cli_help();
                        return Some("");
                    } else {
                        info!("Welcome to the Lucid Command Line Interface (CLI)\nType 'help' to display all commands.\n");

                        // TODO: Try to connect to the remote endpoint
                        // TODO: Use env var to set remote endpoint

                        let mut input = String::new();
                        loop {
                            // TODO: Display the good endpoint
                            print!("{}> ", "127.0.0.1:7021");
                            io::stdout().flush().unwrap();
                            match io::stdin().read_line(&mut input) {
                                Ok(_) => {
                                    match input.trim().as_ref() {
                                        "exit" | "quit" => {
                                            info!("Exiting Lucid CLI");
                                            break;
                                        }
                                        "help" | "?" | "-h" | "/?" => {
                                            display_cli_help();
                                        }
                                        _ => {
                                            warn!("Unknown command '{}'", input.trim());
                                        }
                                    }
                                    println!();
                                    input.clear();
                                }
                                _ => ()
                            }
                        }
                        std::process::exit(0);
                    }
                }

                if let Some(matches) = cli.subcommand_matches("init") {
                    let secret_key = match matches.value_of("secret") {
                        Some(secret) => {
                            secret.to_owned()
                        }
                        None => {
                            let secret_key_bytes = ring::digest::digest(&SHA256, &rand::thread_rng().gen::<[u8; 32]>());
                            secret_key_bytes.as_ref().iter().fold(
                                String::with_capacity(secret_key_bytes.as_ref().len() * 2),
                                |mut acc, x| {
                                    acc.push_str(&format!("{:0>2x}", x));
                                    acc
                                },
                            )
                        }
                    };

                    let mut has_configuration_file: Option<&str> = None;

                    if let Some(configuration_file) = matches.value_of("config") {
                        has_configuration_file = Some(configuration_file);
                    }

                    match self.initialize_node(has_configuration_file, secret_key, matches.is_present("force")) {
                        Ok(lucid_yml_location) => {
                            info!("Lucid successfully initialized in {}", lucid_yml_location);
                        },
                        Err(e) => {
                            error!("Unable to initialize Lucid node: {}", e.get_ref().unwrap().description());
                        }
                    }
                    return Some("");
                }

                if let Some(matches) = cli.subcommand_matches("server") {
                    &mut self.configure(matches.value_of("config"));
                    match &self.configuration {
                        Some(config) => {
                            let mut lucid_server = Server::new();
                            lucid_server.configure(config.clone());
                            lucid_server.run();
                        },
                        None => {
                            warn!("The Lucid node is not successfully configured.");
                        }
                    };
                    return Some("");
                }

                if let Some(matches) = cli.subcommand_matches("settings") {
                    &mut self.configure(matches.value_of("config"));
                    if let Some(_) = &self.configuration {
                        match fs::read_to_string(&self.configuration_location) {
                            Ok(content) => info!("Configuration location: {}\n\n{}", &self.configuration_location, content),
                            Err(err) => error!("{}", err)
                        }
                    }
                    return Some("");
                }

                if let Some(matches) = cli.subcommand_matches("store") {
                    &mut self.configure(matches.value_of("config"));
                    unimplemented!("Not implemented");
                }

                if let Some(matches) = cli.subcommand_matches("tokens") {
                    &mut self.configure(matches.value_of("config"));
                    if matches.is_present("issue") {
                        if let Some(config) = &self.configuration {
                            match &self.issue_jwt(&config.clone().authentication.secret_key, None) {
                                Some(token) => {
                                    info!("JWT token successfully generated: {}", token);
                                },
                                None => {
                                    info!("Unable to generate JWT token.");
                                }
                            }
                        }
                    }
                    return Some("");
                }
            }
            Err(e) => {
                return Some(Box::leak(e.message.into_boxed_str()));
            }
        }
        None
    }

    fn issue_jwt(&self, secret_key: &String, expiration: Option<DateTime<Utc>>) -> Option<String> {
        let lucid_root_claims = Claims {
            sub: String::from("Lucid Root Token"),
            iss: String::from("http://127.0.0.1:7021/"), // TODO: check issuer, maybe set the proper uri
            iat: Utc::now().timestamp(),
            exp: match expiration {
                Some(exp) => exp.timestamp(),
                None => (Utc::now() + Duration::weeks(52 * 3)).timestamp()
            },
        };
        match encode(&Header::default(), &lucid_root_claims, secret_key.as_ref()) {
            Ok(token) => Some(token),
            _ => None,
        }
    }

    // Initialize the node by creating a lucid.yml configuration file
    fn initialize_node(&mut self, configuration_file: Option<&str>, secret_key: String, force: bool) -> Result<&str, std::io::Error> {
        let lucid_yml = match configuration_file {
            Some(custom_configuration_file) => String::from(custom_configuration_file),
            None => match get_data_root(AppDataType::UserConfig) {
                Ok(mut appdata_root) => {
                    &appdata_root.push("lucid");
                    fs::create_dir_all(&appdata_root.clone().into_os_string().into_string().unwrap())?;
                    &appdata_root.push("lucid.yml");
                    appdata_root.clone().into_os_string().into_string().unwrap()
                },
                Err(_) => {
                    return Err(Error::new(ErrorKind::Interrupted, "Unable to get the Lucid configuration folder."));
                }
            }
        };

        if Path::new(&lucid_yml).exists() && !force {
            return Err(Error::new(ErrorKind::Interrupted, "The Lucid node is already initialized."));
        } else {
            match File::create(lucid_yml.clone()) {
                Ok(mut file_handle) => {
                    match &self.issue_jwt(&secret_key, None) {
                        Some(root_token) => {
                            let mut default_configuration = Configuration::default();
                            default_configuration.authentication.root_token = root_token.clone();
                            default_configuration.authentication.secret_key = secret_key;
                            if file_handle.write_all(serde_yaml::to_string(&default_configuration).unwrap().as_bytes()).is_ok() {
                                return Ok(Box::leak(lucid_yml.into_boxed_str()));
                            }
                            return Err(Error::new(ErrorKind::Interrupted, "Unable to create default configuration."));
                        },
                        None => {
                            return Err(Error::new(ErrorKind::Interrupted, "Unable to create the JWT root token."));
                        }
                    }
//                    let r = &self.issue_jwt(&secret_key, None);
//                    return Err(Error::new(ErrorKind::Interrupted, "Unable to create the JWT root token."));
                },
                Err(_) => {
                    return Err(Error::new(ErrorKind::Interrupted, "Unable to create the Lucid configuration file."));
                }
            }
        }
    }

    // Configure the current instance with the default or a specific configuration file
    fn configure(&mut self, configuration_file: Option<&str>) {
        let mut lucid_yml = String::new();
        match configuration_file {
            Some(conf) => lucid_yml = String::from(conf),
            None => match get_data_root(AppDataType::UserConfig) {
                Ok(mut appdata_root) => {
                    &appdata_root.push("lucid");
                    fs::create_dir_all(&appdata_root.clone().into_os_string().into_string().unwrap()).unwrap();
                    &appdata_root.push("lucid.yml");
                    lucid_yml = appdata_root.into_os_string().into_string().unwrap();
                },
                Err(_) => warn!("Unable to get the Lucid configuration folder.")
            }
        };

        if Path::new(&lucid_yml).exists() {
            match fs::read_to_string(&lucid_yml) {
                Ok(content) => {
                    let configuration: Configuration = serde_yaml::from_str(&content).unwrap();
                    log::set_max_level(LevelFilter::from_str(&configuration.logging.level).expect("Invalid logging level in configuration"));
                    self.configuration_location = lucid_yml;
                    self.configuration = Some(configuration);
                },
                Err(_) => warn!("Unable to read the Lucid configuration file.")
            }
        } else {
            warn!("This configuration file does not exists.");
        }
    }
}
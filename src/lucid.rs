use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

use app_dirs::*;
use chrono::*;
use clap::App;
use jsonwebtoken::*;

use crate::configuration::Configuration;
use crate::logger::{Logger, LogLevel};
use crate::server::Server;

include!("crossplatform.rs");

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigurationFile {
    endpoint: &'static str,
    root_token: String,
    secret_key: String,
    use_tls: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

pub struct Lucid {
    configuration: Option<Configuration>
}

impl Lucid {
    pub fn default() -> Lucid {
        Lucid {
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
            .name(crate_description!())
            .bin_name(get_binary());
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
                        crate::logger::print(&LogLevel::Information, "This is all the available commands:");
                        println!(" - set       [key] - Set an object");
                        println!(" - get       [key] - Get an object");
                        println!(" - lock      [key] - Lock an object");
                        println!(" - unlock    [key] - Unlock an object");
                        println!(" - expire    [key] - Set an object expiration date");
                        println!(" - increment [key] - Increment the value of an object");
                        println!(" - decrement [key] - Decrement the value of an object");
                        println!(" - drop      [key] - Drop an object");
                    }

                    if matches.is_present("help") {
                        println!("Welcome to the Lucid Command Line Interface (CLI)\n");
                        display_cli_help();
                        return Some("");
                    } else {
                        println!("Welcome to the Lucid Command Line Interface (CLI)\nType 'help' to display all commands.\n");

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
                                            crate::logger::print(&LogLevel::Information, "Exiting Lucid CLI");
                                            break;
                                        }
                                        "help" | "?" | "-h" | "/?" => {
                                            display_cli_help();
                                        }
                                        _ => {
                                            crate::logger::print(&LogLevel::Warning, format!("Unknown command '{}'", input.trim()).as_ref());
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
                    use ring::digest::SHA256;
                    use rand::Rng;
                    let secret_key_bytes = ring::digest::digest(&SHA256, &rand::thread_rng().gen::<[u8; 32]>());
                    let mut secret_key = secret_key_bytes.as_ref().iter().fold(
                        String::with_capacity(secret_key_bytes.as_ref().len() * 2),
                        |mut acc, x| {
                            acc.push_str(&format!("{:0>2x}", x));
                            acc
                        },
                    );

                    match matches.value_of("secret") {
                        Some(secret) => {
                            secret_key = secret.to_owned();
                        }
                        None => ()
                    }

                    let mut has_configuration_file: Option<&str> = None;

                    if let Some(configuration_file) = matches.value_of("config") {
                        has_configuration_file = Some(configuration_file);
                    }

                    match &mut self.initialize_node(has_configuration_file, secret_key, matches.is_present("force")) {
                        Ok(_lucid_yml_location) => {
                            // TODO: display location in logs
                            &self.log(LogLevel::Success, "Lucid successfully initialized.", None);
                        },
                        Err(e) => {
                            &self.log(LogLevel::Error, "Unable to initialize Lucid node.", Some(e.get_ref().unwrap().description()));
                        }
                    }
                    return Some("");
                }

                if let Some(matches) = cli.subcommand_matches("server") {
                    // Configure instance if --config args is passed
                    &mut self.configure(matches.value_of("config"));

                    // Run server if the instance is successfully configured
                    match &self.configuration {
                        Some(config) => {
                            let mut lucid_server = Server::new();
                            lucid_server.configure(&config);
                            lucid_server.run();
                        },
                        None => {
                            &self.log(LogLevel::Warning, "The Lucid node is not successfully configured.", None);
                        }
                    };
                    return Some("");
                }

                if let Some(_matches) = cli.subcommand_matches("settings") {
                    unimplemented!("Not implemented");
                }

                if let Some(_matches) = cli.subcommand_matches("store") {
                    unimplemented!("Not implemented");
                }

                if let Some(_matches) = cli.subcommand_matches("tokens") {
                    unimplemented!("Not implemented");
                }
            }
            Err(e) => {
                return Some(Box::leak(e.message.into_boxed_str()));
            }
        }
        None
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
                    let lucid_root_claims = Claims {
                        sub: String::from("Lucid Root Token"),
                        iss: String::from("https://127.0.0.1:7021/"), // TODO: check issuer, maybe set the proper uri
                        iat: Utc::now().timestamp(),
                        exp: (Utc::now() + Duration::weeks(52 * 3)).timestamp(),    // TODO: look the expiration delay
                    };
                    match encode(&Header::default(), &lucid_root_claims, secret_key.as_ref()) {
                        Ok(root_token) => {
                            // TODO: migrate to toml
                            let default_configuration = ConfigurationFile {
                                endpoint: "127.0.0.1:7021",
                                root_token,
                                secret_key,
                                use_tls: false,
                            };
                            if file_handle.write_all(serde_yaml::to_string(&default_configuration).unwrap().as_bytes()).is_ok() {
                                return Ok(Box::leak(lucid_yml.into_boxed_str()));
                            }
                            return Err(Error::new(ErrorKind::Interrupted, "Holly shit."));
                        },
                        Err(_e) => {
                            return Err(Error::new(ErrorKind::Interrupted, "Unable to create the JWT root token."));
                        }
                    }
                },
                Err(_) => {
                    return Err(Error::new(ErrorKind::Interrupted, "Unable to create the Lucid configuration file."));
                }
            }
        }
    }

    // Configure the current instance with the default or a specific configuration file
    fn configure(&mut self, configuration_file: Option<&str>) -> Result<(), std::io::Error> {
        let lucid_yml = match configuration_file {
            None => match get_data_root(AppDataType::UserConfig) { // TODO: check app data location
                Ok(mut appdata_root) => {
                    &appdata_root.push("lucid");
                    fs::create_dir_all(&appdata_root.clone().into_os_string().into_string().unwrap())?;
                    &appdata_root.push("lucid.yml");
                    appdata_root.into_os_string().into_string().unwrap()
                },
                Err(_) => {
                    return Err(Error::new(ErrorKind::Interrupted, "Unable to get the Lucid configuration folder."));
                }
            },
            Some(conf) => String::from(conf)
        };

        if Path::new(&lucid_yml).exists() {
            match fs::read_to_string(&lucid_yml) {
                Ok(content) => {
                    let configuration_file_yaml: serde_json::Value = serde_yaml::from_str(&content).unwrap();
                    self.configuration = Some(Configuration {
                        endpoint: configuration_file_yaml["endpoint"].to_string(),
                        location: (&lucid_yml).to_string(),
                        use_tls: false,
                    });
                    return Ok(());
                },
                Err(_) => {
                    return Err(Error::new(ErrorKind::Interrupted, "Unable to read the Lucid configuration file."));
                }
            }
        } else {
            return Err(Error::new(ErrorKind::Interrupted, "Not initialized Lucid node."));
        }
    }
}
//use lucid::Lucid;

use crate::logger::Logger;
use crate::server::Server;

pub struct Lucid {
    server_instance: i32,
    configuration_file: String,
}

impl Lucid {
    pub fn default() -> Lucid {
        Lucid {
            server_instance: 25,
            configuration_file: String::from("nothing"),
        }
    }

    pub fn init(&self) -> Result<(), String> {
        println!("mdr");
        // self.mdr();
        Ok(())
    }
}

//use app_dirs::*;
//use chrono::*;
//use clap::App;
//use crypto::digest::Digest;
//use jsonwebtoken::*;
//
//mod server;
//
//#[derive(Debug, Serialize, Deserialize)]
//struct JwtClaims {
//    sub: String,
//    // company: String,
//    iat: i64,
//    exp: i64,
//}
//
//include!("crossplatform.rs");
//include!("error.rs");
//include!("utils.rs");
//
//struct InitConfiguration {
//    configuration_file: String,
//    root_token: String,
//}
//
//pub struct Lucid {
//    sever_instance: server::Server,
//}
//
//impl Lucid {
//    pub fn new() -> Lucid {
//        Lucid {
//            sever_instance: server::Server::new(),
//        }
//    }
//
//    pub fn show_banner(&self) -> () {
//        println!(
//            r###"
// ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
// ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
// ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
// ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
// ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝
// "###
//        );
//    }
//
//    pub fn show_description(&self) -> () {
//        println!("A Fast, Secure and Distributed KV store with a HTTP API.");
//        println!("Written in Rust by Clint.Network (twitter.com/clint_network)\n");
//    }
//
//    pub fn default(&mut self) -> Result<(), String> {
//        let yaml = load_yaml!("cli.yml");
//        let mut commands = App::from_yaml(yaml)
//            .name(crate_description!())
//            .bin_name(get_binary());
//
//        self.show_banner();
//
//        if self.parse_cli(&mut commands).is_none() {
//            commands.print_help().unwrap();
//            print!("\n\n");
//        } else {
//            println!();
//        }
//
//        Ok(())
//    }
//
//    pub fn parse_cli(&mut self, commands: &mut App) -> Option<()> {
//        match commands.get_matches_from_safe_borrow(::std::env::args_os()) {
//            Ok(cli) => {
//                if cli.is_present("version") {
//                    self.show_description();
//                    display_error(
//                        LucidError::Information,
//                        format!("Lucid Version {}", crate_version!()),
//                    );
//                    return Some(());
//                }
//
//                if let Some(matches) = cli.subcommand_matches("cli") {
//                    self.show_description();
//                    fn display_cli_help() {
//                        display_error(
//                            LucidError::Information,
//                            String::from("This is all the available commands:\n"),
//                        );
//                        println!(" - set       [key] - Set an object");
//                        println!(" - get       [key] - Get an object");
//                        println!(" - lock      [key] - Lock an object");
//                        println!(" - unlock    [key] - Unlock an object");
//                        println!(" - expire    [key] - Set an object expiration date");
//                        println!(" - increment [key] - Increment the value of an object");
//                        println!(" - decrement [key] - Decrement the value of an object");
//                        println!(" - drop      [key] - Drop an object");
//                    }
//
//                    if matches.is_present("help") {
//                        println!("Welcome to the Lucid Command Line Interface (CLI)\n");
//                        display_cli_help();
//                        return Some(());
//                    } else {
//                        use std::io;
//                        use std::io::Write;
//
//                        println!("Welcome to the Lucid Command Line Interface (CLI)\nType 'help' to display all commands.\n");
//
//                        // TODO: Try to connect to the remote endpoint
//                        // TODO: Use env var to set remote endpoint
//
//                        let mut input = String::new();
//                        loop {
//                            // TODO: Display the good endpoint
//                            print!("{}> ", "127.0.0.1:7021");
//                            io::stdout().flush().unwrap();
//                            match io::stdin().read_line(&mut input) {
//                                Ok(_) => {
//                                    match input.trim().as_ref() {
//                                        "exit" | "quit" => {
//                                            display_error(
//                                                LucidError::Information,
//                                                String::from("Exiting Lucid CLI\n"),
//                                            );
//                                            break;
//                                        }
//                                        "help" | "?" | "-h" | "/?" => {
//                                            display_cli_help();
//                                        }
//                                        _ => {
//                                            display_error(
//                                                LucidError::Error,
//                                                format!("Unknown command '{}'", input.trim()),
//                                            );
//                                        }
//                                    }
//                                    println!();
//                                    input.clear();
//                                }
//                                Err(_) => {}
//                            }
//                        }
//                        std::process::exit(0);
//                    }
//                }
//
//                if let Some(matches) = cli.subcommand_matches("init") {
//                    self.show_description();
//
//                    use crypto::sha2::Sha256;
//                    use rand::Rng;
//                    let mut hasher = Sha256::new();
//                    hasher.input(&rand::thread_rng().gen::<[u8; 32]>());
//                    let mut secret_key = hasher.result_str();
//
//                    match matches.value_of("secret") {
//                        Some(secret) => {
//                            secret_key = secret.to_owned();
//                        }
//                        None => ()
//                    }
//                    match self.init(secret_key, matches.is_present("force")) {
//                        Ok(init_configuration) => {
//                            display_error(
//                                LucidError::Information,
//                                String::from("Lucid successfully initialized."),
//                            );
//                            display_error(
//                                LucidError::Information,
//                                format!(
//                                    "Configuration file: {}",
//                                    init_configuration.configuration_file
//                                ),
//                            );
//                            display_error(
//                                LucidError::Information,
//                                format!("Root Token: {}", init_configuration.root_token),
//                            );
//                        }
//                        Err(e) => {
//                            display_error(
//                                LucidError::Error,
//                                String::from("Unable to initialize Lucid."),
//                            );
//                            display_error(LucidError::Error, e);
//                        }
//                    }
//                    return Some(());
//                }
//
//                if let Some(matches) = cli.subcommand_matches("server") {
//                    self.show_description();
//                    if let Some(configuration_file) = matches.value_of("config") {
//                        use std::path::Path;
//                        if !Path::new(&configuration_file).exists() {
//                            display_error(
//                                LucidError::Error,
//                                format!(
//                                    "Unable to found the configuration file '{}'.",
//                                    &configuration_file
//                                ),
//                            );
//                            return Some(());
//                        }
//                        self.sever_instance
//                            .set_configuration_file(configuration_file.to_owned());
//                    }
//
//                    if !self.sever_instance.has_configuration_file() {
//                        match self.get_default_configuratin_file() {
//                            Ok(configuration_file) => {
//                                self.sever_instance
//                                    .set_configuration_file(String::from(&configuration_file));
//                                self.sever_instance.load_configuration();
//                                display_error(
//                                    LucidError::Warning,
//                                    format!(
//                                        "No configuration file specified, default file used: {}",
//                                        &configuration_file
//                                    ),
//                                );
//                            }
//                            Err(_) => (),
//                        }
//                    }
//                    match self.sever_instance.run() {
//                        Ok(server) => {
//                            display_error(
//                                LucidError::Information,
//                                format!(
//                                    "Running Lucid server on {endpoint} | PID: {pid}",
//                                    endpoint = server.socket(),
//                                    pid = std::process::id()
//                                ),
//                            );
//                            display_error(
//                                LucidError::Information,
//                                format!(
//                                    "Lucid API Endpoint: {scheme}://{endpoint}/api/",
//                                    scheme = "https",
//                                    endpoint = server.socket()
//                                ),
//                            );
//                            display_error(
//                                LucidError::Information,
//                                String::from("Use Ctrl+C to stop the server."),
//                            );
//                        }
//                        Err(e) => {
//                            display_error(
//                                LucidError::Error,
//                                String::from("Unable to launch Lucid server."),
//                            );
//                            display_error(LucidError::Error, String::from(e.to_string()));
//                        }
//                    }
//                    return Some(());
//                }
//
//                if let Some(matches) = cli.subcommand_matches("tokens") {
//                    self.show_description();
//                    match matches.subcommand_name() {
//                        Some("issue") => {
//                            return Some(());
//                        }
//                        Some("revoke") => {
//                            if let Some(token) = matches.value_of("token") {
//                                println!("lol{}", token);
//                            }
//                            return Some(());
//                        }
//                        None => {}
//                        _ => {}
//                    }
//                    return None;
//                }
//            }
//            Err(e) => {
//                print!("{}", e);
//                return Some(());
//            }
//        };
//        return None;
//    }
//
//    fn get_default_configuratin_file(&self) -> Result<String, ()> {
//        const APP_INFO: AppInfo = AppInfo {
//            name: "Lucid",
//            author: "Lucid",
//        };
//        match get_app_root(AppDataType::UserConfig, &APP_INFO) {
//            Ok(app_root) => {
//                let mut configuration_file = app_root;
//                configuration_file.push("lucid.yml");
//                return Ok(format!(
//                    "{}",
//                    &configuration_file.into_os_string().into_string().unwrap()
//                ));
//            }
//            Err(_) => {}
//        }
//        return Err(());
//    }
//
//    fn init(&self, secret_key: String, force: bool) -> Result<InitConfiguration, String> {
//        const APP_INFO: AppInfo = AppInfo {
//            name: "Lucid",
//            author: "Lucid",
//        };
//        match app_root(AppDataType::UserConfig, &APP_INFO) {
//            Ok(app_root) => {
//                let mut configuration_file = app_root;
//                configuration_file.push("lucid.yml");
//
//                let lucid_root_claims = JwtClaims {
//                    sub: String::from("Lucid Root Token"),
//                    iat: Utc::now().timestamp(),
//                    exp: (Utc::now() + Duration::weeks(52 * 3)).timestamp(),
//                };
//
//                match encode(&Header::default(), &lucid_root_claims, secret_key.as_ref()) {
//                    Ok(root_token) => {
//                        use std::fs::*;
//                        use std::io::prelude::*;
//
//                        use std::path::Path;
//                        if Path::new(&configuration_file).exists() && !force {
//                            return Err(format!(
//                                "The Lucid configuration file already exist: {}",
//                                &configuration_file.into_os_string().into_string().unwrap()
//                            ));
//                        }
//
//                        let lucid_yml = File::create(&configuration_file);
//                        if lucid_yml.is_ok() {
//                            use yaml_rust::{YamlEmitter, YamlLoader};
//                            let docs = YamlLoader::load_from_str(
//                                &format!(
//                                    "
//endpoint: 127.0.0.1:7021
//root_token: {token}
//secret_key: {secret}
//",
//                                    token = root_token,
//                                    secret = secret_key
//                                )[..],
//                            )
//                                .unwrap();
//                            let doc = &docs[0];
//
//                            let mut lucid_conf = String::new();
//                            let mut emitter = YamlEmitter::new(&mut lucid_conf);
//                            emitter.dump(doc).unwrap();
//
//                            if lucid_yml.unwrap().write_all(lucid_conf.as_bytes()).is_ok() {
//                                return Ok(InitConfiguration {
//                                    configuration_file: configuration_file
//                                        .into_os_string()
//                                        .into_string()
//                                        .unwrap(),
//                                    root_token: root_token.to_owned(),
//                                });
//                            }
//                        }
//                        return Err(format!(
//                            "Cannot create the configuration file. {}",
//                            &configuration_file.into_os_string().into_string().unwrap()
//                        ));
//                    }
//                    Err(_) => {
//                        return Err(String::from("Cannot generate the root JWT token."));
//                    }
//                };
//            }
//            Err(_) => {
//                // TODO: manage errors
//                return Err("Cannot get the application root folder.".to_owned());
//            }
//        }
//    }
//}

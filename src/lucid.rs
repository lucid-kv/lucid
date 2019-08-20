mod server;

use chrono::*;
use clap::{App};
use app_dirs::*;
use jsonwebtoken::*;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    // company: String,
    iat: i64,
    exp: i64
}

include!("crossplatform.rs");
include!("error.rs");
include!("utils.rs");

struct InitConfiguration
{
    configuration_file: String,
    root_token: String
}

pub struct Lucid {
    sever_instance: server::Server,
}

impl Lucid {
    pub fn new() -> Lucid {
        Lucid {
            sever_instance: server::Server::new(),
        }
    }

    pub fn show_banner(&self) -> () {
        println!("{}", r###"
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝ 
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝  
 "###);
    }

    pub fn show_description(&self) -> () {
        println!("A Fast, Secure and Distributed KV store with a HTTP API.");
        println!("Written in Rust by Clint.Network (twitter.com/clint_network)\n");
    }


    pub fn default(&self) -> Result<(), String> {
        let yaml = load_yaml!("cli.yml");
        let mut commands = App::from_yaml(yaml)
            .name(crate_description!())
            .bin_name(get_binary());
        
        self.show_banner();

        if self.parse_cli(&mut commands).is_none()
        {
            commands.print_help().unwrap();
            print!("\n\n");
        }
        else {
            print!("\n");
        }
        
        Ok(())
    }

    pub fn parse_cli(&self, commands: &mut App) -> Option<()>
    {
        match commands.get_matches_from_safe_borrow(::std::env::args_os())
        {
            Ok(cli) => {
                if cli.is_present("version") {
                    self.show_description();
                    println!("Lucid Version {}", crate_version!());
                    return Some(());
                }

                // else if cli.is_present("help") {
                //     return Some(());
                // }

                if let Some(matches) = cli.subcommand_matches("cli") {
                    self.show_description();
                    fn display_cli_help()
                    {
                        display_error(LucidError::Information, String::from("This is all the available commands:"));
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
                        return Some(())
                    }
                    else {
                        use std::io;
                        use std::io::Write;

                        println!("Welcome to the Lucid Command Line Interface (CLI)\nType 'help' to display all commands.\n");

                        // TODO: Try to connect to the remote endpoint
                        // TODO: Use env var to set remote endpoint

                        let mut input = String::new();
                        loop {
                            // TODO: Display the good endpoint
                            print!("{}> ", "127.0.0.1:7221");
                            io::stdout().flush().unwrap();
                            match io::stdin().read_line(&mut input) {
                                Ok(_) => {
                                    match input.trim().as_ref()
                                    {
                                        "exit" | "quit" => {
                                            display_error(LucidError::Information, String::from("Exiting Lucid CLI\n"));
                                            break;
                                        },
                                        "help" | "?" | "-h" | "/?" => {
                                            display_cli_help();
                                        }
                                        _ => {
                                            display_error(LucidError::Error, format!("Unknown command '{}'", input.trim()));
                                        }
                                    }
                                    println!("");
                                    input.clear();
                                }
                                Err(_) => {}
                            }
                        }
                        std::process::exit(0);
                    }
                }

                if let Some(matches) = cli.subcommand_matches("init") {
                    self.show_description();
                    // TODO: Generate Key
                    let mut secret_key = "mdr";
                    match matches.value_of("secret")
                    {
                        Some(secret) => {
                            secret_key = secret;
                        },
                        None => { }
                    }
                    match self.init(secret_key)
                    {
                        Ok(init_configuration) => {
                            display_error(LucidError::Information, String::from("Lucid successfully initialized."));
                            display_error(LucidError::Information, format!("Configuration file: {}", init_configuration.configuration_file));
                            display_error(LucidError::Information, format!("Root Token: {}", init_configuration.root_token));
                        },
                        Err(e) => {
                            display_error(LucidError::Error, String::from("Unable to initialize Lucid"));
                            display_error(LucidError::Error, e);
                        }
                    }
                    return Some(());
                }

                if let Some(matches) = cli.subcommand_matches("server") {
                    self.show_description();
                    if let Some(configuration_file) = matches.value_of("config")
                    {
                        self.sever_instance.set_configuration_file(configuration_file.to_owned());
                    }
                    match self.sever_instance.run()
                    {
                        Ok(server) => {
                            display_error(LucidError::Information, format!("Running Lucid server on {}", server.socket()));
                            display_error(LucidError::Information, String::from("Use Ctrl+C to stop the server."));
                        },
                        Err(_) => {
                            display_error(LucidError::Error, String::from("Unable to launch Lucid server."));
                        }
                    }
                    return Some(());
                }

                if let Some(matches) = cli.subcommand_matches("tokens") {
                    self.show_description();
                    match matches.subcommand_name()
                    {
                        Some("issue") => {

                        },
                        None => {}
                    }
                    if let Some(mdr) = matches.subcommand_matches("issue")
                    {
                        println!("mdr");
                    }
                    // if matches.is_present("secret") {
                    //     match matches.value_of("secret")
                    //     {
                    //         Some(secret) => {
                    //             secret_key = secret;
                    //         },
                    //         None => { }
                    //     }
                    // }
                    // do
                    return Some(());
                }
            },
            Err(e) => {
                print!("{}", e);
                return Some(());
            }
        };
        return None;
    }

    fn init(&self, secret_key: &str) -> Result<InitConfiguration, String>
    {
        const APP_INFO: AppInfo = AppInfo{name: "Lucid", author: "Lucid"};
        match app_root(AppDataType::UserConfig, &APP_INFO)
        {
            Ok(app_root) => {
                let mut configuration_file = app_root;
                configuration_file.push("lucid.yaml");
                
                let lucid_root_claims = JwtClaims
                {
                    sub: String::from("Lucid Root Token"),
                    iat: Utc::now().timestamp(),
                    exp: (Utc::now() + Duration::weeks(52 * 3)).timestamp()
                };

                match encode(&Header::default(), &lucid_root_claims, secret_key.as_ref())
                {
                    Ok(root_token) => {
                        use std::fs::*;
                        use std::io::prelude::*;
                        
                        let file = File::create(&configuration_file);
                        if file.is_ok()
                        {
                            use yaml_rust::{YamlLoader, YamlEmitter};
                            let docs = YamlLoader::load_from_str(&format!("
bind: 127.0.0.1:{port}
root_token: {token}
secret_key: {secret}
", port=7021, token=root_token, secret=secret_key)[..]).unwrap();
                            let doc = &docs[0];

                            let mut lucid_conf = String::new();
                            let mut emitter = YamlEmitter::new(&mut lucid_conf);
                            emitter.dump(doc).unwrap();

                            if file.unwrap().write_all(lucid_conf.as_bytes()).is_ok() {
                                return Ok(InitConfiguration
                                {
                                    configuration_file: configuration_file.into_os_string().into_string().unwrap(),
                                    root_token: root_token.to_owned()
                                });
                            }
                        }
                        return Err(format!("Cannot create the configuration file. {}", &configuration_file.into_os_string().into_string().unwrap()));
                    },
                    Err(_) => {
                        return Err(String::from("Cannot generate the root JWT token."));
                    }
                };
            },
            Err(_) => {
                // TODO: manage errors
                return Err("Cannot get the application root folder.".to_owned());
            }
        }
    }
}
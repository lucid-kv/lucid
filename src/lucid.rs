use clap::App;

use crate::logger::{Logger, LogLevel};
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

    pub fn show_banner(&self) {
        println!(r###"
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝
 "###);
    }

    fn show_description(&self) {
        println!("A Fast, Secure and Distributed KV store with a HTTP API.");
        println!("Written in Rust by Clint.Network (twitter.com/clint_network)\n");
    }

    fn show_version(&self) {
        println!("Lucid Version {}\n", crate_version!());
    }

    fn show_help(&self, commands: &mut App) -> () {
        commands.print_help().unwrap();
        println!("\n");
    }

    #[cfg(target_os = "windows")]
    fn get_binary(&self) -> &'static str {
        "lucid.exe"
    }

    #[cfg(not(target_os = "windows"))]
    fn get_binary(&self) -> &'static str {
        "./lucid"
    }

    pub fn initialize_cli(&self) {
        let cli_yaml = load_yaml!("cli.yml");
        let mut commands = App::from_yaml(cli_yaml)
            .name(crate_description!())
            .bin_name(self.get_binary());

        self.show_banner();

        match self.handle_cli(&mut commands) {
            None => {
                self.show_help(&mut commands);
            }
            Some(usage) => {
                println!("{}\n", usage);
            }
        }
    }

    fn handle_cli(&self, commands: &mut App) -> Option<&str> {
        match commands.get_matches_from_safe_borrow(::std::env::args_os()) {
            Ok(cli) => {
                if cli.is_present("help") {
                    return None;
                }

                self.show_description();

                if cli.is_present("version") {
                    self.show_version();
                    return Some("")
                }

                if let Some(matches) = cli.subcommand_matches("cli") {}
                if let Some(matches) = cli.subcommand_matches("init") {

                }
                if let Some(matches) = cli.subcommand_matches("server") {

                }
                if let Some(matches) = cli.subcommand_matches("settings") {}
                if let Some(matches) = cli.subcommand_matches("store") {}
                if let Some(matches) = cli.subcommand_matches("tokens") {}
            }
            Err(e) => {
                return Some(Box::leak(e.message.into_boxed_str()));
            }
        }
        Some("")
    }
}
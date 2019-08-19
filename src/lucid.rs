mod server;

use clap::{App};

include!("crossplatform.rs");
include!("utils.rs");

pub struct Lucid {
    sever_instance: server::Server,
}

impl Lucid {
    pub fn new() -> Lucid {
        Lucid {
            sever_instance: server::Server::new(),
        }
    }

    pub fn banner(&self) -> () {
        println!("{}", r###"
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝ 
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝  
 "###);
    }

    pub fn init(&self) -> Result<(), String> {
        let yaml = load_yaml!("cli.yml");
        let mut commands = App::from_yaml(yaml)
            .name(crate_description!())
            .bin_name(get_binary());

        match self.parse_cli(&mut commands)
        {
            Some(_) => { },
            None => {
                self.banner();
                commands.print_help().unwrap();
                println!("\n");
            }
        };

        Ok(())
    }

    pub fn parse_cli(&self, commands: &mut App) -> Option<()>
    {
        if let Ok(cli) = commands.get_matches_from_safe_borrow(::std::env::args_os()) {
            if cli.is_present("version") {
                println!("Lucid Version {}", crate_version!());
                return Some(());
            }
            else if cli.is_present("help") {
                return None;
            }

            match cli.subcommand_name() {
                Some("server") => {
                    match self.sever_instance.run()
                    {
                        Ok(_) => { },
                        Err(e) => {
                            println!("Unable to launch Lucid server.\n{}", e);
                        }
                    }
                    return Some(());
                },
                None => {
                    return None;
                },
                _ => {
                    return None;
                }
            }
        }
        return None;
    }
}
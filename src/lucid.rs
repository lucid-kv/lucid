extern crate clap;

use std::io::Write;
use nickel::status::StatusCode::NotFound;
use nickel::*;
use clap::{App, SubCommand};
    
include!("crossplatform.rs");

pub struct Lucid;

impl Lucid
{
    pub fn init() -> Lucid
    {
        Lucid { }
    }

    pub fn commandline(&self) -> Lucid
    {
        let mut commands = App::new("High performance and distributed KV ledger.")
            .bin_name(get_binary())
            // .version("0.1.0")
            .author("Written in Rust by Clint.Network")
            .subcommand(SubCommand::with_name("monitor")
                .about("controls testing features"))
            .subcommand(SubCommand::with_name("server")
                .about("Run an instance as server"))
            .subcommand(SubCommand::with_name("auth")
                .about("Manage credentials of the instance"))
            .subcommand(SubCommand::with_name("members")
                .about("Manage members of the cluster"))
            .subcommand(SubCommand::with_name("cli")
                .about("Spawn to the command line interface"))
            .subcommand(SubCommand::with_name("settings")
                .about("Configure the instance"));

        match commands.get_matches_from_safe_borrow(::std::env::args_os())
        {
            Ok(content) => {
                if content.subcommand_matches("server").is_some()
                {
                    let mut server = Nickel::new();
                    // server.options()

                    // server.post("/mdr", handler: H)
                    // server.get("/mdr", middleware!("This is the /bar handler"));
                    server.get("/mdr", middleware! { |request|
                        format!("OK OK OK")
                    });

                    fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
                        if let Some(ref mut res) = err.stream {
                            if res.status() == NotFound {
                                res.write_all(b"404 Not Found").expect("Unable to write in the stream");
                                // res.headers().append_raw("salut", b"dsds".to_vec());
                                // let _ = res.write_all(format!("{:#?}", res.headers()).to_string().as_bytes());
                                return Halt(())
                            }
                        }
                        Continue(())
                    }

                    let custom_handler: fn(&mut NickelError, &mut Request) -> Action = custom_404;
                    server.handle_error(custom_handler);
                    server.listen("127.0.0.1:7210").unwrap();
                }
                else
                {
                    commands.print_help().unwrap();
                }
            },
            Err(_) => {
                commands.print_help().unwrap();
            }
        }
        Lucid { }
    }

    pub fn banner(&self) -> Lucid
    {
        let lucid_banner = r###"  _   _   _  ___ ___ ___  _ 
 | | | | | |/ __|_ _|   \| |
 | |_| |_| | (__ | || |) |_|
 |____\___/ \___|___|___/(_)
"###;
        println!("{}", lucid_banner);
        Lucid { }
    }

    // pub fn listen(&self) -> Lucid
    // {
    //     Lucid { }
    // }
}

// pub fn banner() -> String
// {
//     "lol".to_string()
// }

// pub trait Lucid
// {
//     fn summarize() -> &'static str;
// }

// impl Lucid
// {
//     fn summarize() -> &'static str
//     {
//         println!("ok");
//         "alut"
//     }
// }

// extern crate clap; 
// use clap::{App, SubCommand};

// #[cfg(target_os = "linux")]
// fn get_binary() -> &'static str {
//     "lucid"
// }

// // And this function only gets compiled if the target OS is *not* linux
// #[cfg(not(target_os = "linux"))]
// fn get_binary() -> &'static str {
//     "lucid.exe"
// }

// fn main()
// {
//     let lucid_banner = r###"  _   _   _  ___ ___ ___  _ 
//  | | | | | |/ __|_ _|   \| |
//  | |_| |_| | (__ | || |) |_|
//  |____\___/ \___|___|___/(_)
// "###;
//     println!("{}", lucid_banner);

//     let mut commands = App::new("High performance and distributed KV ledger.")
//         .bin_name(get_binary())
//         // .version("0.1.0")
//         .author("Written in Rust by Clint.Network")
//         .subcommand(SubCommand::with_name("monitor")
//             .about("controls testing features"))
//         .subcommand(SubCommand::with_name("server")
//             .about("Run an instance as server"))
//         .subcommand(SubCommand::with_name("auth")
//             .about("Manage credentials of the instance"))
//         .subcommand(SubCommand::with_name("members")
//             .about("Manage members of the cluster"))
//         .subcommand(SubCommand::with_name("settings")
//             .about("Configure the instance"))
//         ;
//         //     .about("controls testing features")
//         //     .version("1.3")
//         //     .author("Someone E. <someone_else@other.com>")
//         //     .arg_from_usage("-d, --debug 'Print debug information'"));

//     match commands.get_matches_from_safe_borrow(::std::env::args_os())
//     {
//         Ok(content) => {
//             if content.subcommand_matches("test").is_some()
//             {
//                 println!("yeah");
//             }
//             else
//             {
//                 commands.print_help().unwrap();
//             }
//         },
//         Err(_) => {
//             commands.print_help().unwrap();
//         }
//     }
// }
use clap::{Arg, Command};
use std::process::exit;

fn main() {
    let cmd = Command::new("kvs")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help("Please provide one of the following: --key")
        .subcommand(
            Command::new("set")
                .about("Set a key-value pair")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("Get a value by key")
                .arg(Arg::new("key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a key-value pair")
                .arg(Arg::new("key").required(true)),
        )
        .get_matches();

    match cmd.subcommand_name() {
        Some("set") => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some("get") => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some("rm") => {
            eprintln!("unimplemented");
            exit(1);
        }
        None => panic!("No subcommand provided"),
        _ => panic!("Invalid subcommand provided"),
    }
}

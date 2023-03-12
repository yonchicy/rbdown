use log;
mod login;

use clap::{arg, command, Command};

fn main() {
    env_logger ::init();
    log::debug!("running");
    let matches = command!() // requires `cargo` feature
        .arg(arg!([url] "要下载的视频的 av|bv|BV|ep|ss|地址"))
        .subcommand(Command::new("login").about("login by QR code"))
        .get_matches();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("url") {
        println!("Value for name: {}", name);
    }
    match matches.subcommand() {
        Some(("login",_)) => {
            login::loggin();
        },
        _ => {}
    }
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd

    // Continued program logic goes here...
}

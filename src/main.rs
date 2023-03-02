use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([url] "要下载的视频的 av|bv|BV|ep|ss|地址"))
        // .arg(
        //     arg!(
        //         -c --config <FILE> "Sets a custom config file"
        //     )
        //     // We don't have syntax yet for optional options, so manually calling `required`
        //     .required(false)
        //     .value_parser(value_parser!(PathBuf)),
        // )
        // .arg(arg!(
        //     -d --debug ... "Turn debugging information on"
        // ))
        .subcommand(Command::new("login").about("login by QR code"))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("url") {
        println!("Value for name: {}", name);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd

    // Continued program logic goes here...
}

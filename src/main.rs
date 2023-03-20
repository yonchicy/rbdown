use std::fmt::Debug;

use log;
mod login;
mod downloader;
mod client;

use clap::{arg, command, Command};

use crate::{downloader::download_vedio, client::Client, login::login_by_qrcode};

fn main() {
    env_logger ::init();
    log::debug!("running");
    let client :Client= Default::default();
    let matches = command!() // requires `cargo` feature
        .arg(arg!([url] "要下载的视频的 bv 地址"))
        .subcommand(Command::new("login").about("login by QR code"))
        .get_matches();
    // You can check the value provided by positional arguments, or option arguments
    match matches.subcommand() {
        Some(("login",_)) => {
            login_by_qrcode(client);
        },
        _ => {}
    }
    if let Some(bv) = matches.get_one::<String>("url") {
        log::info!("要下载的bv号是:{}",bv);
        download_vedio(bv);
    }
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd

    // Continued program logic goes here...
}

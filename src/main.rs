use core::panic;
use std::{collections::HashMap, fmt::Debug, io::Write};

use log;
mod client;
mod downloader;
mod login;

use clap::{arg, command, Command};
use reqwest::Url;
use serde_json::json;

use crate::{client::Client, downloader::download_vedio, login::login_by_qrcode};

fn main() {
    env_logger::init();
    log::debug!("running");
    let client: Client = Default::default();
    let matches = command!() // requires `cargo` feature
        .arg(arg!([url] "要下载的视频的 bv 地址"))
        .subcommand(Command::new("login").about("login by QR code"))
        .get_matches();
    // You can check the value provided by positional arguments, or option arguments
    let info = match matches.subcommand() {
        Some(("login", _)) => login_by_qrcode(client).unwrap(),
        _ => {
            panic!("")
        }
    };
    let pairs: HashMap<String, String> = Url::parse(info.url.as_str())
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect();
    let url_str = serde_json::to_string(&pairs).unwrap();
    // let pairs:HashMap<String,String> = back_url.query_pairs().into_owned().collect();

    let mut file = std::fs::File::create("./target/sessdata.json").unwrap();
    file.write_all(url_str.as_bytes()).expect("写入cookie失败");
    let file = std::fs::File::create("./target/cookie.json").unwrap();
    serde_json::to_writer_pretty(&file, &info).unwrap();
    println!("登陆成功");
    if let Some(bv) = matches.get_one::<String>("url") {
        log::info!("要下载的bv号是:{}", bv);
        download_vedio(bv);
    }
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd

    // Continued program logic goes here...
}

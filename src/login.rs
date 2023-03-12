use log;
use qrcode::{render::unicode, QrCode};
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::{thread, time};
use url::{ParseError, Url, UrlQuery};

pub fn loggin() -> Result<(), ParseError> {
    // let loginUrl = "https://passport.bilibili.com/qrcode/getLoginUrl";
    let loginUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
    let test = String::from("valuhelle");
    println!("{}",test);
    log::debug!("in loggin function");
    let ret_json = reqwest::blocking::get(loginUrl).unwrap().json::<serde_json::Value>().unwrap();
    log::debug!("ret_txt {:#?}", ret_json);
let url = &ret_json["data"]["url"].to_string();
    log::debug!("url {}", url);
    let qrcode = &ret_json["data"]["qrcode_key"].to_string();
    log::debug!("qrcode_key {}", qrcode);
    // let code = QrCode::new(qrcode).unwrap();
    // let image = code
    //     .render::<unicode::Dense1x2>()
    //     .dark_color(unicode::Dense1x2::Light)
    //     .light_color(unicode::Dense1x2::Dark)
    //     .build();

    // log::info!("请扫描二维码登录");
    // println!("{}", image);

    // loop {
    //     thread::sleep(time::Duration::from_millis(1000));

    //     let ret_url = getLoginStatus(qrcode);
    //     log::debug!("获得扫码结果回复:{:?}",ret_url);
    //     let back: serde_json::Value = serde_json::from_str(ret_url.as_str()).unwrap();
    //     let data = &back["data"]["code"].to_string();
    //     log::debug!("扫码服务器应答为:{:?}",data);
    //     println!("{:?}",data);
    // }
    // token, url = 申请二维码()
    // 生成二维码(url) # 等待客户端扫码
    // while True:
    //     status, cookie = 扫码登录(token)
    //     match status:
    //         case 未扫描:
    //             continue
    //         case 二维码超时 | 二维码失效:
    //             提示('二维码失效或超时') # 需要用户重新操作
    //             break
    //         case 已扫描未确认:
    //             提示('扫描成功')
    //         case 登录成功:
    //             提示('扫描成功')
    //             存储cookie(cookie)
    //             SSO登录页面跳转()
    //             break
    Ok(())
}

fn getLoginStatus(qrcode: &str) -> String {
    let queryUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
    let send_url = Url::parse_with_params(queryUrl, &[("qrcode_key", qrcode)])
        .unwrap()
        .to_string();
    log::debug!("向扫码服务器发送的url为{}",send_url);
    let ret_txt = reqwest::blocking::get(send_url).unwrap().text().unwrap();

    ret_txt
}

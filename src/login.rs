use crate::client::{Client, LoginInfo};
use anyhow::Result;
use log;
use qrcode::{render::unicode, QrCode};
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::io::Write;
use std::{thread, time};
use url::{ParseError, Url, UrlQuery};

pub fn login_by_qrcode(client: Client) -> Result<LoginInfo> {
    // let loginUrl = "https://passport.bilibili.com/qrcode/getLoginUrl";
    let value = client.get_qrcode()?;
    let code = QrCode::new(value["data"]["url"].as_str().unwrap()).unwrap();
    let img = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}",img);
    client.check_qrcode_status(value)
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
    // Ok(())
}


// fn getLoginStatus(qrcode: &str) -> String {
//     let queryUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
//     let send_url =  Url::parse_with_params(queryUrl, &[("qrcode_key", qrcode)])
//             .unwrap().to_string();
            
//     log::debug!("向扫码服务器发送的url为{}",send_url);
//     let ret_txt = reqwest::blocking::get(send_url).unwrap().text().unwrap();
//     ret_txt
// }

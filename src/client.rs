use anyhow::{bail, Result};
use reqwest::cookie::CookieStore;
use reqwest::dns::Resolve;
use reqwest::header;
use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

#[derive(Debug)]

pub struct Client {
    pub client: reqwest::blocking::Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Referer",
            header::HeaderValue::from_static("https://www.bilibili.com/"),
        );
        headers.insert("Connection", header::HeaderValue::from_static("keep-alive"));
        let cookie_store = cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        Client {
            client: reqwest::blocking::Client::builder()
                .cookie_provider(std::sync::Arc::clone(&cookie_store))
                .user_agent(
                    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/63.0.3239.108",
                )
                .default_headers(headers)
                .timeout(Duration::new(60, 0))
                .build()
                .unwrap(),
            cookie_store,
        }
    }
    pub fn check_qrcode_status(&self, value: Value) -> Result<LoginInfo> {
        let queryUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
        log::debug!("value : {:#?}",value);
        // log::debug!("qrcode_key : {:#?}",value["data"]["qrcode_key"]);
        let form = json!({
            "qrcode_key":value["data"]["qrcode_key"]
        });
        loop {
            // log::debug!("向扫码服务器发送的url为{}", send_url);
            // let ret_txt = reqwest::blocking::get(send_url).unwrap().text().unwrap();
            std::thread::sleep(core::time::Duration::from_millis(1000));
            log::debug!("尝试获取二维码扫描结果");
            let res: ResponseData = self.client.get(queryUrl).query(&form).send()?.json()?;
            log::debug!("{:#?}", res);
            assert_eq!(res.code,0);
            let data = res.data;
            log::debug!("{:#?}", data);
            match data {
                LoginInfo { code: 0, .. } => {
                    log::debug!("扫码成功");
                    return Ok(data);
                }
                LoginInfo { code: 86038, .. } => {
                    bail!("二维码已失效")
                }
                _ => {
                    log::info!("等待扫码")
                }
            }

            // for cookie in res.cookies(){
            //     println!("{}",cookie.)
            // }
        }
    }
    pub fn get_qrcode(&self) -> Result<Value> {
        let loginUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
        Ok(self.client.get(loginUrl).send()?.json()?)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginInfo {
    url: String,
    refresh_token: String,
    timestamp: u32,
    pub code: i32,
    message: String, // pub token_info: TokenInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenInfo {
    pub access_token: String,
    expires_in: u32,
    mid: u32,
    refresh_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseData {
    pub code: i32,
    pub data: LoginInfo,
    message: String,
}

impl Display for ResponseData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(untagged)]
// pub enum ResponseValue {
//     Login(LoginInfo),
//     Value(serde_json::Value),
// }
impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

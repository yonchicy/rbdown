
use serde_json;

use reqwest;

pub fn loggin() {
    let loginUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
    let ret_txt = reqwest::blocking::get(loginUrl).unwrap().text().unwrap();
    let json:serde_json::Value = serde_json::from_str(ret_txt.as_str()).unwrap();
    let url =  & json["data"]["url"];

}
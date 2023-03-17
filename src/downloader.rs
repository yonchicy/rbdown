use url::{ParseError, Url, UrlQuery};
pub fn download_vedio(bv: &str) {
    let url = "https://api.bilibili.com/x/player/playurl";
    let cid = get_cid(bv);
}
fn get_cid(bv: &str) -> i64 {
    let url = "https://api.bilibili.com/x/web-interface/view";

    let send_url = Url::parse_with_params(url, &[("bvid", bv)])
        .unwrap()
        .to_string();

    log::debug!("向扫码服务器发送的url为{}", send_url);
    let ret_txt = reqwest::blocking::get(send_url).unwrap().text().unwrap();
    let back: serde_json::Value = serde_json::from_str(ret_txt.as_str()).unwrap();

    let data = back["code"].as_i64().unwrap();
    let cid = back["data"]["cid"].as_i64().unwrap();
    if data != 0 {
        log::error!("获取bv号视频cid失败");
    }
    log::debug!("back code {} ", data);
    log::debug!("back cid {} ", cid);
    cid
}

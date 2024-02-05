// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64;
use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::net::TcpListener;
use url::form_urlencoded;
use url::Url;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Deserialize)]
struct ApiResponseData {
    cid: i32,
}
#[derive(Deserialize)]
struct ApiResponse {
    data: ApiResponseData,
}
#[derive(Serialize)]
struct ResponseData {
    result: String,
    cookie: Option<String>,
}

lazy_static! {
    static ref COOKIE_STR: Mutex<String> = Mutex::new("".to_string());
    static ref PROXY_FILE_BASE_URL: Mutex<String> = Mutex::new("127.0.0.1:3000".to_string());
}
// static mut COOKIE_STR: String = "buvid3=E675259E-CE29-29E9-577D-A96EB47E26EC32951infoc; b_nut=1698312432; i-wanna-go-back=-1; b_ut=7; _uuid=B10D419106-BC65-65EC-58C2-77DE8213ED3532275infoc; buvid4=05D4C7C1-955B-C658-3F45-4503C6B7478933854-023102617-n9JVkHlXJFhcM8fOkrAmAw%3D%3D; DedeUserID=253197450; DedeUserID__ckMd5=cab81eaeee7343ee; CURRENT_FNVAL=4048; rpdid=|(J~kmuJum~J0J'uYm)JukYlu; LIVE_BUVID=AUTO5216983127142774; enable_web_push=ENABLE; iflogin_when_web_push=1; header_theme_version=CLOSE; fingerprint=56abd5c3d642b00343d5a5af249fa8fa; buvid_fp_plain=undefined; buvid_fp=56abd5c3d642b00343d5a5af249fa8fa; CURRENT_QUALITY=64; PVID=1; hit-dyn-v2=1; home_feed_column=5; browser_resolution=1920-1049; b_lsid=4E9485EE_18D530D3023; SESSDATA=b4a2009d%2C1722047398%2Caa2a5%2A12CjA5rJa7siTU4zj37WpjWLCCFSf5wCfPNIFypS-IvZM-Bdmfyua3x_3Ytd_8pauGKTcSVll4c2FOczl4SmhPd2NuMC1DR0cwS01rQkNWbTVHckRTS1RUVml3TklxNjhWWHhyR0JVdFhMREFKeEJ0SFRfN2NTSzQzVmxjRmdOVXd2UExrNGtYMUdBIIEC; bili_jct=f985ddb1aec1212e929d1fe1fc7bb901; sid=ewuaisaj; bp_video_offset_253197450=891872318641406097".to_string();
const USER_AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
#[tauri::command]
async fn set_cookie_val(cookieval: String) -> Result<String, String> {
    println!("setCookieStr:{}", cookieval);
    let mut data = COOKIE_STR.lock().unwrap();
    *data = cookieval;
    Ok("Cookie value set successfully".to_string())
}
#[tauri::command]
async fn get_user_info() -> Result<String, String> {
    let cookie_value = {
        let data = COOKIE_STR.lock().unwrap();
        data.to_owned()
    };
    println!("cookie_value:{}", cookie_value);
    let url: &str = "https://api.bilibili.com/x/web-interface/nav";
    let response = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(
            header::COOKIE,
            HeaderValue::from_str(&cookie_value).unwrap(),
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.text().await.map_err(|e| e.to_string())?)
}
#[tauri::command]
async fn get_file_proxy_baseurl() -> Result<String, String> {
    println!("GET_PROXY_FILE_BASE_URL");
    let url = {
        let data = PROXY_FILE_BASE_URL.lock().unwrap();
        data.to_owned()
    };
    println!("PROXY_FILE_BASE_URL:{}", url);
    Ok(url)
}

#[tauri::command]
async fn get_search_result(keyword: String, page: String) -> Result<String, String> {
    let cookie_value = {
        let data = COOKIE_STR.lock().unwrap();
        data.to_owned()
    };
    let url: &str = "https://api.bilibili.com/x/web-interface/wbi/search/type";
    let response = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(
            header::COOKIE,
            HeaderValue::from_str(&cookie_value).unwrap(),
        )
        .query(&[
            ("keyword", keyword),
            ("search_type", "video".to_string()),
            ("order", "totalrank".to_string()),
            ("duration", "0".to_string()),
            ("tids", "0".to_string()),
            ("page", page),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.text().await.map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn get_loop_login_query(qrcodekey: String) -> Result<String, String> {
    println!("qrcodekey:{}", qrcodekey);
    let url: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
    let response = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, USER_AGENT)
        .query(&[("qrcode_key", qrcodekey)])
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let headers = response.headers().clone(); // 克隆响应头
    let result: String = response.text().await.map_err(|e| e.to_string())?;
    let mut response_data = ResponseData {
        result,
        cookie: None,
    };

    if let Some(cookie) = headers.get("set-cookie") {
        response_data.cookie = Some(cookie.to_str().unwrap().to_string());
    }
    let json_response = serde_json::to_string(&response_data).map_err(|e| e.to_string())?;
    println!("json_response:{}", json_response);
    Ok(json_response)
}
#[tauri::command]
async fn get_login_png_url() -> Result<String, String> {
    let url: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
    let response = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.text().await.map_err(|e| e.to_string())?)
}
#[tauri::command]
async fn get_video_url(id: String) -> Result<String, String> {
    let cookie_value = {
        let data = COOKIE_STR.lock().unwrap();
        data.to_owned()
    };
    let url: &str = "https://api.bilibili.com/x/web-interface/view";
    let cid_response = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(
            header::COOKIE,
            HeaderValue::from_str(&cookie_value).unwrap(),
        )
        .query(&[("aid", id.clone())])
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json::<ApiResponse>()
        .await
        .map_err(|err| err.to_string())?;
    let cid = cid_response.data.cid.to_string();

    let url2: &str = "https://api.bilibili.com/x/player/wbi/playurl";
    let response = reqwest::Client::new()
        .get(url2)
        .header(header::USER_AGENT, USER_AGENT)
        .header(
            header::COOKIE,
            HeaderValue::from_str(&cookie_value).unwrap(),
        )
        .query(&[("avid", id), ("qn", "80".to_string()), ("cid", cid)])
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.text().await.map_err(|e| e.to_string())?)
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let cookie_value = {
        let data = COOKIE_STR.lock().unwrap();
        data.to_owned()
    };
    println!("handle_request_cookie:{}",cookie_value);
    let uri = req.uri();
    let query_params = match uri.query() {
        Some(query) => query,
        None => "",
    };
    // 解析查询参数，获取名为 "url" 的值
    let mut url_value = String::new();
    for (key, value) in form_urlencoded::parse(query_params.as_bytes()) {
        if key == "url" {
            url_value = value.into_owned();
            break;
        }
    }

    // 对 URL 进行 Base64 解码
    let decoded_bytes = base64::decode(url_value).unwrap();
    let decoded_url = String::from_utf8(decoded_bytes).unwrap();

    // 从url中获取Host
    let re = Regex::new(r#"(?:https?://)?([^/?#]+)"#).unwrap();
    let mut host = "".to_string();
    if let Some(caps) = re.captures(&decoded_url) {
        host = caps
            .get(0)
            .map_or("", |m| m.as_str())
            .to_string()
            .replace("https://", "");
        println!("host: {}", host);
        // println!("{}", &caps[0]);
    }
    // 提供基础 URL 信息，并将相对 URL 解析为绝对 URL

    //这段代码我看不懂完全看不懂  我也不知道这个域名是哪来的
    let base_url = "https://example.com"; // 替换为实际的基础 URL
    let target_url = Url::options()
        .base_url(Some(&base_url.parse().unwrap()))
        .parse(&decoded_url);

    let target_url = match target_url {
        Ok(url) => url,
        Err(err) => {
            // 处理解析失败的情况
            return Ok(Response::builder()
                .status(400)
                .body(Body::from(format!("Failed to parse URL: {}", err)))
                .unwrap());
        }
    };

    // 创建一个支持 HTTPS 的 HttpsConnector
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    println!("url:{}", target_url.as_str());
    // 创建代理请求，并设置相应的请求头部
    let mut dest_req = Request::builder()
        .method(req.method())
        .uri(target_url.as_str())
        .body(Body::empty())
        .unwrap();

    *dest_req.headers_mut() = req.headers().clone();
    dest_req
        .headers_mut()
        .insert(header::REFERER, "https://www.bilibili.com".parse().unwrap()); // 替换为实际的 referer  //下载视频必校验
    dest_req
        .headers_mut()
        .insert(header::COOKIE, cookie_value.parse().unwrap()); // 替换为实际的 cookie
    dest_req
        .headers_mut()
        .insert(header::USER_AGENT, USER_AGENT.parse().unwrap()); // 替换为实际的 user agent
    dest_req
        .headers_mut()
        .insert(header::HOST, host.parse().unwrap()); // 替换为实际的主机名  视频封面HOST 必校验

    let res = client.request(dest_req).await.unwrap();

    Ok(res)
}
async fn find_available_port() -> SocketAddr {
    let initial_addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let mut addr = initial_addr;
    let mut listener = TcpListener::bind(&addr).await;
    println!("port:{}", addr.port().to_string());
    while listener.is_err() {
        // 端口被占用，尝试自动选择下一个端口
        let new_port = match addr.port().checked_add(1) {
            Some(port) if port <= 65535 => port,
            _ => {
                // 选择其他处理方式，这里选择重新开始从初始端口
                eprintln!(
                    "Port number overflow or maximum reached, restarting from initial port..."
                );
                initial_addr.port()
            }
        };

        addr.set_port(new_port);
        addr = format!("127.0.0.1:{}", new_port).parse().unwrap();
        println!("New port: {}", new_port);
        listener = TcpListener::bind(&addr).await; // 重新尝试绑定新的端口
    }
    addr
}

#[tokio::main]
async fn main() {
    // 使用 Arc 包装地址，确保在 Hyper 服务器启动之前被正确设置
    let addr = Arc::new(Mutex::new(find_available_port().await));

    // 启动 Hyper 服务器的异步任务
    let server_handle = tokio::spawn({
        let addr = Arc::clone(&addr);
        async move {
            let locked_addr = addr.lock().unwrap().clone(); // 使用 clone 方法创建 SocketAddr 的副本
            let make_svc =
                make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
            let server = Server::bind(&locked_addr).serve(make_svc);
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        }
    });

    // 设置 Tauri 应用程序的全局变量
    {
        let locked_addr = addr.lock().unwrap();
        let mut data = PROXY_FILE_BASE_URL.lock().unwrap();
        *data = locked_addr.to_string();
        println!("Server is running on: {}", locked_addr);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            // app.emit_all("single-instance", Payload { args: argv, cwd }).expect("Failed to emit");
            let window = app.get_window("main").expect("Failed to get window");
            window.set_focus().expect("Failed to set window focus");
            window.show().expect("Failed to show window");
            // window.request_user_attention(Some(UserAttentionType::Informational)).expect("Failed to request user attention");
        }))
        .invoke_handler(tauri::generate_handler![
            get_search_result,
            get_login_png_url,
            get_video_url,
            set_cookie_val,
            get_loop_login_query,
            get_user_info,
            get_file_proxy_baseurl
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // 等待 Hyper 服务器的线程结束
    if let Err(e) = server_handle.await {
        eprintln!("hyper server thread panicked: {:?}", e);
    }
}

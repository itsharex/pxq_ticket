use super::error::PXQError;
use anyhow::Result;
use chrono::Local;
use std::sync::Arc;

use rand::Rng;
use reqwest::Client;
use serde_json::Value;
use tauri::{Manager, Window, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

const UA: &'static str = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1";
const HOST: &'static str = "m.piaoxingqiu.com";
pub const API_VER: &'static str = "4.1.2-20240305183007";
pub const API_SRC: &'static str = "WEB";
const TERMINAL_SRC: &'static str = "WEB";

pub async fn get_http_client() -> Result<Client> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .https_only(true)
        .user_agent(UA)
        .build()
        .map_err(|_| PXQError::ReqwestError)?;
    Ok(client)
}

pub async fn get_access_token(app: Arc<Window>) -> Result<String> {
    let path = app
        .app_handle()
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(".settings.dat");
    let stores = app.state::<StoreCollection<Wry>>();
    let access_token = with_store(app.app_handle(), stores, path, |store| {
        Ok(store.get("access_token").cloned())
    })?;
    Ok(match access_token {
        Some(token) => {
            if token.is_null() {
                "".to_string()
            } else {
                token.as_str().unwrap().to_string()
            }
        }
        None => "".to_string(),
    })
}

pub async fn get_user_location(app: Arc<Window>) -> Result<String> {
    let path = app
        .app_handle()
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(".settings.dat");
    let stores = app.state::<StoreCollection<Wry>>();
    let access_token = with_store(app.app_handle(), stores, path, |store| {
        Ok(store.get("user_location").cloned())
    })?;
    Ok(match access_token {
        Some(token) => {
            if token.is_null() {
                "".to_string()
            } else {
                token.as_str().unwrap().to_string()
            }
        }
        None => "".to_string(),
    })
}

pub async fn get(
    app: Arc<Window>,
    url: &str,
    form: serde_json::Value,
) -> Result<serde_json::Value> {
    let url = format!("https://{}/{}", HOST, url);

    let client = get_http_client()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let access_token = get_access_token(app)
        .await
        .map_err(|_| PXQError::FileAccessError)?;

    let data = client
        .get(url)
        .form(&form)
        .header("access-token", access_token)
        .header("host", "m.piaoxingqiu.com")
        .header("terminal-src", TERMINAL_SRC)
        .header("src", API_SRC)
        .header("ver", API_VER)
        .header("origin", format!("https://{}", HOST))
        .header("referer", format!("https://{}", HOST))
        .send()
        .await
        .map_err(|_| PXQError::ReqwestError)?
        .json::<serde_json::Value>()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    Ok(data)
}

pub async fn post(app: Arc<Window>, url: &str, json_data: Value) -> Result<serde_json::Value> {
    let url = format!("https://{}/{}", HOST, url);
    let client = get_http_client().await?;
    let access_token = get_access_token(app).await?;

    let mut request = client
        .post(&url)
        .json(&json_data)
        .header("access-token", access_token)
        .header("host", HOST)
        .header("terminal-src", TERMINAL_SRC)
        .header("ver", API_VER)
        .header("content-type", "application/json")
        .header("origin", format!("https://{}", HOST))
        .header("referer", format!("https://{}", HOST));

    if url.contains("create_order") {
        let black_box = get_black_box();
        println!("black_box:{}", black_box);
        request = request.header("Blackbox", black_box);
    }

    let data = request
        .send()
        .await
        .map_err(|_| PXQError::ReqwestError)?
        .json::<serde_json::Value>()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    Ok(data)
}

pub async fn app_post(app: Arc<Window>, url: &str, json_data: Value) -> Result<serde_json::Value> {
    let client = get_http_client().await?;
    let access_token = get_access_token(app).await?;
    let host = "appapi.caiyicloud.com";

    let mut request = client
        .post(url)
        .json(&json_data)
        .header("access-token", access_token)
        .header("host", host)
        .header("terminal-src", "ANDROID")
        .header("ver", API_VER)
        .header("content-type", "application/json")
        .header("origin", format!("https://{}", host))
        .header("referer", format!("https://{}", host));

    if url.contains("create_order") {
        let black_box = get_black_box();
        request = request.header("Blackbox", black_box);
    }

    let data = request
        .send()
        .await
        .map_err(|_| PXQError::ReqwestError)?
        .json::<serde_json::Value>()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    Ok(data)
}

fn random_str(len: usize) -> String {
    let char_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| rng.gen_range(0..char_set.len()))
        .map(|i| char_set.chars().nth(i).unwrap())
        .collect()
}

fn box_str(td: &str) -> String {
    let mut td_chars: Vec<char> = td.chars().collect();
    td_chars[0] = random_str(1).chars().nth(0).unwrap();
    td_chars.insert(4, random_str(1).chars().nth(0).unwrap());
    td_chars.insert(15, random_str(1).chars().nth(0).unwrap());
    td_chars.insert(td_chars.len() - 1, random_str(1).chars().nth(0).unwrap());
    td_chars.into_iter().collect()
}

fn get_black_box() -> String {
    let timestamp = Local::now().timestamp();
    box_str(&format!("{}{}{}", random_str(4), timestamp, random_str(9)))
}

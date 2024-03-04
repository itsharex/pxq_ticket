use std::sync::Arc;

use super::error::PXQError;
use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use tauri::{Manager, Window, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

const UA: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36";

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

pub async fn get(
    app: Arc<Window>,
    url: &str,
    form: serde_json::Value,
) -> Result<serde_json::Value> {
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
        .header("terminal-src", "WEB")
        .header("ver", "4.0.10-20240221091017")
        .header("origin", "https://m.piaoxingqiu.com")
        .header("referer", "https://m.piaoxingqiu.com")
        .send()
        .await
        .map_err(|_| PXQError::ReqwestError)?
        .json::<serde_json::Value>()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    Ok(data)
}

pub async fn post(app: Arc<Window>, url: &str, json_data: Value) -> Result<serde_json::Value> {
    let client = get_http_client().await?;

    let access_token = get_access_token(app).await?;
    println!("access_token:{}", access_token);
    let data = client
        .post(url)
        .json(&json_data)
        .header("access-token", access_token)
        .header("host", "m.piaoxingqiu.com")
        .header("terminal-src", "WEB")
        .header("ver", "4.0.10-20240221091017")
        .header("content-type", "application/json")
        .header("origin", "https://m.piaoxingqiu.com")
        .header("referer", "https://m.piaoxingqiu.com")
        .send()
        .await
        .map_err(|_| PXQError::ReqwestError)?
        .json::<serde_json::Value>()
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    Ok(data)
}

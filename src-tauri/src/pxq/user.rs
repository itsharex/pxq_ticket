use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{Manager, Window, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

use super::{
    client::{get, post},
    error::PXQError,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendVerificationCodeResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoCodeData {
    #[serde(rename = "baseCode")]
    base_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratePhoneCodeResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: PhotoCodeData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginData {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<LoginData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub nickname: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<UserProfile>,
}

#[tauri::command(async)]
pub async fn send_verification_code(
    app: Window,
    mobile: String,
    token: String,
) -> Result<SendVerificationCodeResult, PXQError> {
    let app = Arc::new(app);

    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/send_verify_code";

    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "verifyCodeUseType": "USER_LOGIN",
        "cellphone": mobile,
        "messageType": "MOBILE",
        "token": token
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value::<SendVerificationCodeResult>(data)
        .map_err(|_| PXQError::ParseDataError)?;
    Ok(result)
}

#[tauri::command(async)]
pub async fn generate_photo_code(
    app: Window,
    mobile: String,
) -> Result<GeneratePhoneCodeResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/generate_photo_code";
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "cellphone": mobile,
        "verifyCodeUseType": "USER_LOGIN",
        "messageType": "MOBILE"
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value::<GeneratePhoneCodeResult>(data)
        .map_err(|_| PXQError::ParseDataError)?;
    Ok(result)
}

#[tauri::command(async)]
pub async fn login_by_mobile(
    app: Window,
    mobile: String,
    sms_code: String,
) -> Result<LoginResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/login_or_register";
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "cellphone": mobile,
        "verifyCode": sms_code
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    println!("{:?}", data);
    let login_result =
        serde_json::from_value::<LoginResult>(data).map_err(|_| PXQError::ReqwestError)?;
    Ok(login_result)
}

#[tauri::command(async)]
pub async fn get_user_profile(app: Window) -> Result<UserProfileResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/buyer/v3/profile";
    let form = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920"
    });
    let data = get(app, url, form)
        .await
        .map_err(|_| PXQError::GetUserProfileError)?;
    let user_profile_result =
        serde_json::from_value(data).map_err(|_| PXQError::GetUserProfileError)?;
    Ok(user_profile_result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenData {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<RefreshTokenData>,
}

#[tauri::command(async)]
pub async fn refresh_token(app: Window) -> Result<RefreshTokenResult, PXQError> {
    let app = Arc::new(app);
    let path = app
        .app_handle()
        .path_resolver()
        .resolve_resource(".settings.dat")
        .unwrap();
    let stores = app.state::<StoreCollection<Wry>>();
    let token = with_store(app.app_handle(), stores, path, |store| {
        Ok(store.get("access_token").cloned())
    })
    .map_err(|_| PXQError::RefreshTokenError)?;

    if token.is_none() {
        return Err(PXQError::RefreshTokenError);
    }
    let url = format!(
        "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/refresh_token?refreshToken={}",
        token.clone().unwrap()
    );
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "refreshToken": token,
    });
    let data = post(app, url.as_str(), json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let result = serde_json::from_value(data).map_err(|_| PXQError::RefreshTokenError)?;

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAudienceData {
    pub id: String,

    #[serde(rename = "idNo")]
    pub id_no: String,

    #[serde(rename = "idType")]
    pub id_type: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserAudiencesResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<UserAudienceData>>,
}

#[tauri::command(async)]
pub async fn get_user_audiences(app: Window) -> Result<GetUserAudiencesResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/buyer/v3/user_audiences?length=500&offset=0&src=WEB&ver=4.0.13-20240223084920";
    let form = json!({});
    let data = get(app, url, form)
        .await
        .map_err(|_| PXQError::GetUserAudienceError)?;
    let result = serde_json::from_value::<GetUserAudiencesResult>(data)
        .map_err(|_| PXQError::GetUserAudienceError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLocation {
    #[serde(rename = "cityId")]
    pub city_id: String,

    #[serde(rename = "cityName")]
    pub city_name: String,

    #[serde(rename = "provinceId")]
    pub province_id: String,

    #[serde(rename = "provinceName")]
    pub province_name: String,

    #[serde(rename = "siteId")]
    pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserLocationResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<UserLocation>,
}

#[tauri::command(async)]
pub async fn get_user_location(app: Window) -> Result<GetUserLocationResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/home/pub/v5/citys/current_location?src=WEB&ver=4.0.13-20240223084920";
    let form = json!({});
    let data = get(app, url, form)
        .await
        .map_err(|_| PXQError::GetUserLocationError)?;
    let result = serde_json::from_value::<GetUserLocationResult>(data)
        .map_err(|_| PXQError::GetUserLocationError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    #[serde(rename = "addressId")]
    pub address_id: String,

    pub username: String,

    pub cellphone: String,

    #[serde(rename = "locationId")]
    pub location_id: String,

    #[serde(rename = "detailAddress")]
    pub detail_address: String,

    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserAddressResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<Address>>,
}

#[tauri::command(async)]
pub async fn get_user_address(app: Window) -> Result<GetUserAddressResult, PXQError> {
    let app = Arc::new(app);
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/buyer/v3/user/addresses?src=WEB&ver=4.0.13-20240223084920";
    let form = json!({});
    let data = get(app, url, form)
        .await
        .map_err(|_| PXQError::GetUserAddressError)?;
    let result = serde_json::from_value::<GetUserAddressResult>(data)
        .map_err(|_| PXQError::GetUserAddressError)?;
    Ok(result)
}

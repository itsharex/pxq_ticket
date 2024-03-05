use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tauri::Window;
use url::form_urlencoded;

use super::{
    client::{get, post, API_SRC, API_VER},
    error::PXQError,
    user::get_user_location_from_cache,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendCategory {
    pub code: i32,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Show {
    #[serde(rename = "searchType")]
    pub search_type: String,

    #[serde(rename = "showId")]
    pub show_id: String,

    #[serde(rename = "stdShowId")]
    pub std_show_id: String,

    #[serde(rename = "showName")]
    pub show_name: String,

    #[serde(rename = "showDate")]
    pub show_date: String,

    #[serde(rename = "cityName")]
    pub city_name: String,

    #[serde(rename = "showStatus")]
    pub show_status: String,

    #[serde(rename = "minOriginalPrice")]
    pub min_original_price: f64,

    #[serde(rename = "posterUrl")]
    pub poster_url: String,

    #[serde(rename = "venueId")]
    pub venue_id: String,

    #[serde(rename = "venueName")]
    pub venue_name: String,

    #[serde(rename = "firstShowTime")]
    pub first_show_time: i128,

    #[serde(rename = "lastShowTime")]
    pub last_show_time: i128,

    #[serde(rename = "latestSaleTime")]
    pub latest_sale_time: Option<String>,

    #[serde(rename = "backendCategory")]
    pub backend_category: BackendCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowData {
    #[serde(rename = "isLastPage")]
    pub is_last_page: bool,

    #[serde(rename = "searchData")]
    pub show_list: Vec<Show>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchShowResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<ShowData>,
}

#[tauri::command(async)]
pub async fn search_show_list(
    app: Window,
    keyword: String,
    sort_type: String,
    page: u8,
) -> Result<SearchShowResult, PXQError> {
    let app = Arc::new(app);
    let keyword: String = form_urlencoded::byte_serialize(keyword.as_bytes()).collect();
    let offset = (page - 1) * 10;
    let url = format!("cyy_gatewayapi/home/pub/v3/show_list/search?backendCategoryCode=ALL&cityId=4455&keyword={}&length=10&offset={}&pageType=SEARCH_PAGE&sortType={}&src={}&ver={}", 
    keyword, offset, sort_type, API_SRC, API_VER);

    println!("url:{}", url);
    let form = json!({});
    let data = get(app, &url, form)
        .await
        .map_err(|_| PXQError::SearchShowError)?;

    let result =
        serde_json::from_value::<SearchShowResult>(data).map_err(|_| PXQError::SearchShowError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeatPlan {
    #[serde(rename = "seatPlanId")]
    pub seat_plan_id: String,

    #[serde(rename = "stdSeatPlanId")]
    pub std_seat_plan_id: String,

    #[serde(rename = "originalPrice")]
    pub original_price: f64,

    #[serde(rename = "seatPlanName")]
    pub seat_plan_name: String,

    #[serde(rename = "hasActivity")]
    pub has_activity: bool,

    #[serde(rename = "canBuyCount")]
    pub can_buy_count: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    #[serde(rename = "showLimit")]
    pub show_limit: i32,

    #[serde(rename = "showId")]
    pub show_id: String,

    #[serde(rename = "stdShowId")]
    pub std_show_id: String,

    #[serde(rename = "supportSeatPicking")]
    pub support_seat_picking: bool,

    #[serde(rename = "originalSeatPickType")]
    pub original_seat_pick_type: String,

    #[serde(rename = "showName")]
    pub show_name: String,

    #[serde(rename = "bizShowSessionId")]
    pub session_id: String,

    #[serde(rename = "stdShowSessionId")]
    pub std_show_session_id: String,

    #[serde(rename = "sessionName")]
    pub session_name: String,

    #[serde(rename = "hasActivity")]
    pub has_activity: bool,

    #[serde(rename = "hasSessionSoldOut")]
    pub has_session_sold_out: bool,

    #[serde(rename = "seatPlans")]
    pub seat_plans: Vec<SeatPlan>,

    #[serde(rename = "sessionStatus")]
    pub session_status: String,

    #[serde(rename = "sessionSaleTime")]
    pub session_sale_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryShowSessionsResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<Session>>,
}

#[tauri::command(async)]
pub async fn query_show_sessions(
    app: Window,
    show_id: String,
) -> Result<QueryShowSessionsResult, PXQError> {
    let app = Arc::new(app);

    let url = format!("cyy_gatewayapi/show/pub/v5/show/{}/sessions", show_id);
    let form = json!({
        "src": API_SRC,
        "ver": API_VER,
        "source": "FROM_QUICK_ORDER",
        "isQueryShowBasicInfo": true,
    });
    let data = get(app, url.as_str(), form)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let result = serde_json::from_value::<QueryShowSessionsResult>(data)
        .map_err(|_| PXQError::QueryShowSessionsError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddReminderData {
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddReminderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: AddReminderData,
}

#[tauri::command(async)]
pub async fn add_reminder(
    app: Window,
    show_id: String,
    session_id: String,
) -> Result<AddReminderResult, PXQError> {
    let app = Arc::new(app);

    let url = format!(
        "cyy_gatewayapi/show/buyer/v3/shows/{}/subscribe?showSessionId={}",
        show_id, session_id
    );
    let json_data = json!({
        "src": API_SRC,
        "ver": API_VER,
        "openId": "",
        "appId": "",
        "showId": show_id,
        "subscribeTargetType": "SHOW_SESSION",
        "showSessionId": session_id,
        "remindType": "SALE_REMIND"
    });
    let data = post(app, url.as_str(), json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let result = serde_json::from_value::<AddReminderResult>(data)
        .map_err(|_| PXQError::AddReminderError)?;

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketWaitlistData {
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketWaitlistResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: TicketWaitlistData,
}

#[tauri::command(async)]
pub async fn ticket_waitlist(
    app: Window,
    show_id: String,
    session_id: String,
    seat_plan_id: String,
) -> Result<TicketWaitlistResult, PXQError> {
    let app = Arc::new(app);

    let url = format!(
        "cyy_gatewayapi/show/buyer/v3/shows/{}/subscribe?showSessionId={}",
        show_id, session_id
    );
    let json_data = json!({
        "src": API_SRC,
        "ver": API_VER,
        "openId": "",
        "appId": "",
        "showId": show_id,
        "subscribeTargetType": "SEAT_PLAN",
        "showSessionId": session_id,
        "remindType": "OOS",
        "seatPlanId": seat_plan_id
    });
    let data = post(app, url.as_str(), json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let result = serde_json::from_value::<TicketWaitlistResult>(data)
        .map_err(|_| PXQError::TicketWaitlistError)?;

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeatPlanData {
    #[serde(rename = "seatPlans")]
    pub seat_plans: Vec<SeatPlan>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetSeatPlansResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: SeatPlanData,
}

#[tauri::command(async)]
pub async fn get_seat_plans(
    app: Window,
    show_id: String,
    session_id: String,
) -> Result<GetSeatPlansResult, PXQError> {
    let app = Arc::new(app);

    let url = format!(
        "cyy_gatewayapi/show/pub/v5/show/{}/session/{}/seat_plans",
        show_id, session_id
    );
    let form = json!({
        "source": "FROM_QUICK_ORDER",
        "src": API_SRC,
        "ver": API_VER,
    });
    let data = get(app, url.as_str(), form)
        .await
        .map_err(|_| PXQError::GetSeatPlansError)?;
    let result = serde_json::from_value(data).map_err(|_| PXQError::GetSeatPlansError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteInfo {
    name: String,

    value: String,

    r#type: bool,

    code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowDetail {
    #[serde(rename = "rsCode")]
    rs_code: i32,

    // #[serde(rename="basicInfo")]
    // pub basic_info: Show,
    #[serde(rename = "noteInfos")]
    pub note_infos: Vec<NoteInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetShowDetailResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: ShowDetail,
}

#[tauri::command(async)]
pub async fn get_show_detail(
    app: Window,
    show_id: String,
) -> Result<GetShowDetailResult, PXQError> {
    let app = Arc::new(app);
    let location = get_user_location_from_cache(app.clone())
        .await
        .map_err(|_| PXQError::GetUserLocationError)?;
    let url = format!("cyy_gatewayapi/show/pub/v5/show/{}/static", show_id);
    let form = json!({
        "src": API_SRC,
        "ver": API_VER,
        "cityId": location.city_id,
        "source": "FROM_QUICK_ORDER",
        "siteId": location.site_id,
    });
    let data = get(app, &url, form)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value::<GetShowDetailResult>(data)
        .map_err(|_| PXQError::GetShowDetailError)?;
    Ok(result)
}


// 旧版库存接口
#[tauri::command(async)]
pub async fn get_seat_plans_stock(app: Window, show_id: String, session_id: String) -> Result<(), PXQError> {
    let app = Arc::new(app);
    let url = format!("cyy_gatewayapi/show/pub/v3/show/{}/show_session/{}/seat_plans_dynamic_data", show_id, session_id);
    let form = json!({});
    let data = get(app, url.as_str(), form)
        .await
        .map_err(|_| PXQError::GetSeatPlansError)?;
    println!("{:?}", data);
    Ok(())
}
use std::fmt::Debug;
use std::{sync::Arc, time::Duration};

use super::client::{app_post, API_SRC, API_VER};
use anyhow::Result;
use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::Window;

use super::user::{get_user_location_from_cache, Address};
use super::{
    client::post,
    error::PXQError,
    show::{SeatPlan, Session, Show},
    user::UserAudienceData,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupportDeliverie {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceItem {
    #[serde(rename = "priceItemName")]
    pub price_item_name: String,

    #[serde(rename = "priceItemVal")]
    pub price_item_val: f64,

    #[serde(rename = "priceItemType")]
    pub price_item_type: String,

    #[serde(rename = "direction")]
    pub direction: String,

    // #[serde(rename="tagColor")]
    // pub tag_color: String,

    // pub tag: String,
    #[serde(rename = "priceItemSpecies")]
    pub price_item_species: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreOrderData {
    #[serde(rename = "priceItems")]
    price_items: Vec<PriceItem>,

    #[serde(rename = "supportDeliveries")]
    pub support_deliveries: Vec<SupportDeliverie>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreOrderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: PreOrderData,
}

// 预下单,获取下单页面的数据
pub async fn pre_order(
    app: Arc<Window>,
    params: PreOrderParam,
) -> Result<PreOrderResult, PXQError> {
    let seat_plan_id = params.seat_plan.seat_plan_id.clone();
    let seat_plan_price = params.seat_plan.original_price as u32;
    let ticket_num = params.ticket_num;
    let show_id = params.show.show_id.clone();
    let session_id = params.session.session_id.clone();
    let ticket_items = params.ticket_items;

    let url = "https://appapi.caiyicloud.com/cyy_gatewayapi/trade/buyer/order/v5/pre_order?bizCode=FHL_M&src=android";
    let json_data = json!({
        "priorityParam": {
            "priorityId": "",
            "objectId": "",
            "objectType": ""
        },
        "items": [{
            "sku": {
                "skuId": seat_plan_id,
                "skuType": "SINGLE",
                "ticketPrice": seat_plan_price,
                "qty": *ticket_num,
                "ticketItems": *ticket_items
            },
            "spu": {
                "showId": show_id,
                "sessionId": session_id
            }
        }],
        "preFiled": {
            "addressId": "",
            "audienceIds": []
        }
    });
    let data = app_post(app.clone(), url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let _ = show_log(app.clone(), data.to_string().as_str()).await;
    let result =
        serde_json::from_value::<PreOrderResult>(data).map_err(|_| PXQError::PreOrderError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderData {
    #[serde(rename = "createTime")]
    pub create_time: u128,

    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(rename = "orderNumber")]
    pub order_number: String,

    #[serde(rename = "unPaidTransactionIds")]
    pub un_paid_transaction_ids: Vec<String>,

    #[serde(rename = "paidDeadLineTime")]
    pub paid_dead_line_time: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<CreateOrderData>,
}

#[derive(Debug, Clone)]
pub struct GetExpressPriceItemParam {
    pub seat_plan: Arc<SeatPlan>,
    pub ticket_num: Arc<u16>,
    pub show: Arc<Show>,
    pub session: Arc<Session>,
    pub ticket_items: Arc<Vec<Value>>,
    pub deliver_method: String,
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetExpressPriceItemResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<PriceItem>>,
}

// 获取快递费用数据
pub async fn get_express_price_items(
    app: Arc<Window>,
    params: GetExpressPriceItemParam,
) -> Result<GetExpressPriceItemResult, PXQError> {
    let url = "https://appapi.caiyicloud.com/cyy_gatewayapi/trade/buyer/order/v5/price_items";
    let seat_plan_id = params.seat_plan.seat_plan_id.clone();
    let seat_plan_price = params.seat_plan.original_price as u32;
    let ticket_num = params.ticket_num;
    let show_id = params.show.show_id.clone();
    let session_id = params.session.session_id.clone();
    let ticket_items = params.ticket_items;

    let json_data = json!({
        "priorityParam": {
            "priorityId": "",
            "objectId": "",
            "objectType": ""
        },
        "items": [{
            "sku": {
                "skuId": seat_plan_id,
                "skuType": "SINGLE",
                "ticketPrice": seat_plan_price,
                "qty": *ticket_num,
                "ticketItems": *ticket_items
            },
            "spu": {
                "showId": show_id,
                "sessionId": session_id
            },
            "deliverMethod": params.deliver_method
        }],
        "locationCityId": params.address.location.location_id,
        "addressId": params.address.address_id
    });
    let data = app_post(app.clone(), url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let _ = show_log(app.clone(), &format!("获取快递价格信息:{}", data)).await;
    let result = serde_json::from_value::<GetExpressPriceItemResult>(data)
        .map_err(|_| PXQError::GetExpressPriceItemError)?;

    Ok(result)
}

// 创建订单
pub async fn create_order(
    app: Arc<Window>,
    params: CreateOrderParam,
) -> Result<CreateOrderResult, PXQError> {
    let ticket_num = params.ticket_num;
    let seat_plan = params.seat_plan;
    let price_items = params.price_items;
    let ticket_items = params.ticket_items;
    let audiences = params.audiences;
    let deliver_method = params.deliver_method;
    let session_id = params.session.session_id.clone();
    let express_price_items = params.express_price_items;
    let mut price_item_param = Vec::new();
    let show_id = params.show.show_id.clone();

    let mut payment_amount = seat_plan.original_price * *ticket_num as f64;

    for item in price_items {
        price_item_param.push(json!({

            "priceItemVal": format!("{}", item.price_item_val),
            "priceDisplay": format!("￥{}", item.price_item_val),
            "priceItemName": item.price_item_name,
            "direction": item.direction,
            "priceItemType": item.price_item_type,
            "subPriceItemType": "",
            "priceItemId": show_id,
            "priceItemIdType": "",
            "priceItemDesc": "",
            "discountId": "",
            "priceItemSpecies": item.price_item_species,
            "skuId": "",
            "subPriceItemId": null,
            "applyTickets": [

            ],
            "applyProducts": [

            ],
            "itemId": "",
            "deliverVal": "0"
        }))
    }

    if params.is_express {
        for item in express_price_items {
            price_item_param.push(json!({

                "priceItemVal": format!("{}", item.price_item_val),
                "priceDisplay": format!("￥{}", item.price_item_val),
                "priceItemName": item.price_item_name,
                "direction": item.direction,
                "priceItemType": item.price_item_type,
                "subPriceItemType": "",
                "priceItemId": show_id.clone(),
                "priceItemIdType": "",
                "priceItemDesc": "",
                "discountId": "",
                "priceItemSpecies": item.price_item_species,
                "skuId": "",
                "subPriceItemId": null,
                "applyTickets": [],
                "applyProducts": [

                ],
                "itemId": "",
                "deliverVal": "0"

            }));
            payment_amount += item.price_item_val;
        }
    }

    let address_params = match params.is_express {
        true => {
            json!({
                "addressId": params.address.clone().unwrap().address_id
            })
        }
        false => json!({}),
    };

    let contact_param = match params.is_express {
        true => {
            json!({
                "receiver": params.address.clone().unwrap().username,
                "cellphone": params.address.unwrap().cellphone,
            })
        }
        false => json!({}),
    };

    let mut sku_ticket_items = Vec::new();

    for i in 0..ticket_items.len() {
        sku_ticket_items.push(json!({
            "id": ticket_items[i]["id"],
            "audienceId": audiences[i].id,
        }))
    }

    let location = get_user_location_from_cache(app.clone()).await?;

    let url = "https://appapi.caiyicloud.com/cyy_gatewayapi/trade/buyer/order/v5/create_order?bizCode=FHL_M&src=android";

    let mut json_data = json!({
        "locationParam": {
            "locationCityId": location.city_id
        },
        "paymentParam": {
            "totalAmount": format!("{}", payment_amount),
            "payAmount": format!("{}", payment_amount),
        },
        "priceItemParam": price_item_param,
        "items": [{
            "sku": {
                "skuId": seat_plan.seat_plan_id,
                "skuType": "SINGLE",
                "ticketPrice": seat_plan.original_price,
                "qty": *ticket_num,
                "ticketItems": sku_ticket_items,
            },
            "spu": {
                "showId": show_id,
                "sessionId": session_id
            },
            "deliverMethod": deliver_method
        }],
        "priorityParam": {
            "priorityId": "",
            "objectId": "",
            "objectType": ""
        },
        "showGroupId": ""
    });

    if params.is_express {
        json_data["addressParam"] = address_params;
        json_data["contactParam"] = contact_param;
    }

    let data = app_post(app.clone(), url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let _ = show_log(app.clone(), &format!("创建订单返回数据:{}", data)).await;

    let result = serde_json::from_value::<CreateOrderResult>(data)
        .map_err(|_| PXQError::CreateOrderError)?;
    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMsg {
    pub msg: String,
}

// 日志输出事件
pub async fn show_log(app: Arc<Window>, msg: &str) -> Result<(), PXQError> {
    let msg = msg.to_string();
    tokio::spawn(async move {
        let _ = app
            .emit("show-log", LogMsg { msg })
            .map_err(|_| PXQError::AddReminderError);
    });
    Ok(())
}

// 随机字符串
fn random_number_str(len: usize) -> String {
    let mut random_number = rand::thread_rng().gen_range(1..=10000).to_string();
    while random_number.len() != len {
        random_number = format!("0{}", random_number);
    }
    random_number
}

// 毫秒转时分秒
pub fn ms_to_hms(ms: i64) -> (u64, u64, f64) {
    let sec = ms as f64 / 1000.0;
    let hour = (sec / 3600.0) as u64;
    let rem = sec % 3600.0;
    let min = (rem / 60.0) as u64;
    let sec = rem % 60.0;
    (hour, min, sec)
}

// 随机数
fn rand_u64(value: i64) -> u64 {
    let min_value = value / 5 * 4;
    let max_value = value + value / 10;
    let mut rng = rand::thread_rng();
    rng.gen_range(min_value..max_value) as u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuyTicketParam {
    show: Show,

    session: Session,

    #[serde(rename = "seatPlan")]
    seat_plan: SeatPlan,

    #[serde(rename = "ticketNum")]
    ticket_num: u16,

    audiences: Vec<UserAudienceData>,

    address: Option<Address>,

    #[serde(rename = "isExpress")]
    is_express: bool, // 是否快递票

    #[serde(rename = "isRealname")]
    is_realname: bool, // 是否实名
}

#[derive(Debug, Clone)]
pub struct PreOrderParam {
    pub seat_plan: Arc<SeatPlan>,
    pub ticket_num: Arc<u16>,
    pub show: Arc<Show>,
    pub session: Arc<Session>,
    pub ticket_items: Arc<Vec<Value>>,
}

#[derive(Debug, Clone)]
pub struct CreateOrderParam {
    price_items: Vec<PriceItem>,
    seat_plan: Arc<SeatPlan>,
    ticket_num: Arc<u16>,
    show: Arc<Show>,
    session: Arc<Session>,
    ticket_items: Arc<Vec<Value>>,
    audiences: Arc<Vec<UserAudienceData>>,
    deliver_method: String,
    is_express: bool,
    express_price_items: Vec<PriceItem>,
    address: Option<Address>,
}

// 开始购买
pub async fn start(app: Arc<Window>, params: Arc<BuyTicketParam>) -> Result<bool, PXQError> {
    let show = Arc::new(params.show.clone());
    let session = Arc::new(params.session.clone());
    let seat_plan = Arc::new(params.seat_plan.clone());
    let ticket_num = Arc::new(params.ticket_num);
    let audiences = Arc::new(params.audiences.clone());

    let current_time = Local::now().timestamp_millis();

    let mut ticket_items = Vec::new();
    for _ in 0..audiences.len() {
        ticket_items.push(json!({
            "comboItemId": "",
            "seatConcreteId": null,
            "seatGroupId": null,
            "ticketSeatId": null,
            "zoneConcreteId": null,
            "id": format!("{}100{}", current_time % 1000, random_number_str(4)),
            "combinedTicketPhoto": ""
        }))
    }
    let ticket_items = Arc::new(ticket_items);

    let pre_order_param = PreOrderParam {
        seat_plan: seat_plan.clone(),
        ticket_num: ticket_num.clone(),
        show: show.clone(),
        session: session.clone(),
        ticket_items: ticket_items.clone(),
    };
    let res = pre_order(app.clone(), pre_order_param).await?;

    let _ = show_log(app.clone(), &format!("预下单返回数据:{:?}", res)).await;
    if res.status_code != 200 {
        let _ = show_log(app.clone(), &format!("预下单失败, {}.", res.comments)).await;
        return Err(PXQError::PreOrderError);
    }

    let _ = show_log(app.clone(), "预下单成功, 开始提交订单").await;
    let deliver_method = res.data.support_deliveries[0].name.clone();
    let mut express_price_items = Vec::new();
    if params.is_express {
        // 快递票
        let get_express_price_item_res = get_express_price_items(
            app.clone(),
            GetExpressPriceItemParam {
                seat_plan: seat_plan.clone(),
                ticket_num: ticket_num.clone(),
                show: show.clone(),
                session: session.clone(),
                ticket_items: ticket_items.clone(),
                deliver_method: deliver_method.clone(),
                address: params.address.clone().unwrap(),
            },
        )
        .await?;
        if get_express_price_item_res.status_code == 200 {
            express_price_items = get_express_price_item_res.data.unwrap();
        }
    }
    let create_order_params = CreateOrderParam {
        price_items: res.data.price_items,
        seat_plan: seat_plan.clone(),
        ticket_num: ticket_num.clone(),
        session: session.clone(),
        show: show.clone(),
        ticket_items: ticket_items.clone(),
        audiences: audiences.clone(),
        deliver_method: deliver_method.clone(),
        express_price_items,
        is_express: params.is_express,
        address: params.address.clone(),
    };
    let create_order_res = create_order(app.clone(), create_order_params).await?;
    if create_order_res.status_code == 200 {
        let _ = show_log(
            app.clone(),
            &format!(
                "\n**********创建订单成功, 订单号:{}, 请及时打开APP付款!************\n",
                create_order_res.data.unwrap().order_number,
            ),
        )
        .await;
        Ok(true)
    } else {
        let _ = show_log(
            app.clone(),
            &format!("创建订单失败, {}\n", create_order_res.comments),
        )
        .await;
        Ok(false)
    }
}

// 购买门票
#[tauri::command(async)]
pub async fn buy_tickets(app: tauri::Window, params: BuyTicketParam) -> Result<(), PXQError> {
    let app = Arc::new(app);
    let params = Arc::new(params);
    let show = Arc::new(params.show.clone());
    let session = Arc::new(params.session.clone());
    let seat_plan = Arc::new(params.seat_plan.clone());
    let ticket_num = Arc::new(params.ticket_num);
    let audiences = Arc::new(params.audiences.clone());

    let audiences_str: String = audiences
        .iter()
        .map(|audience| audience.name.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let msg = format!(
        "
        开始购票：
            名称: {}
            场次: {}
            票档: {}
            数量: {}
            观众:  {}
        ",
        show.show_name, session.session_name, seat_plan.seat_plan_name, ticket_num, audiences_str
    );
    show_log(app.clone(), &msg).await?;

    let session_status = session.session_status.clone();
    if session_status == "ON_SALE" || session_status == "PRE_SALE" {
        let is_success = start(app.clone(), params).await?;
        if is_success {
            let _ = show_log(app.clone(), "购票成功, 停止运行...").await;
        }
        return Ok(());
    }

    let session_start_time = session.session_sale_time;
    if session_status == "PENDING" && session_start_time.is_none() {
        let _ = show_log(
            app.clone(),
            &format!("{}, 暂未公布开抢时间, 停止运行...", show.show_name),
        )
        .await;
        return Err(PXQError::ShowTimeUnknownError);
    }

    let session_start_time = session_start_time.unwrap();
    let (s, r) = async_channel::unbounded::<bool>();
    let (exit_s, exit_r) = async_channel::unbounded::<bool>();
    let interval = rand_u64(100);
    let earliest_submit_time = 0;

    let id = app.listen("stop-buy-tickets", move |_| {
        let _ = exit_s.send_blocking(true);
    });

    let _ = show_log(app.clone(), "演唱会门票暂未开售, 等待开抢中...").await;
    // 轮询等待开抢
    loop {
        tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(interval)) => { // 等待开抢
                    let local: DateTime<Local> = Local::now();
                    let millis = local.timestamp_millis();
                    let time_left_millis = session_start_time - millis;
                    if time_left_millis <= earliest_submit_time {
                        let _ = s.send(true).await;
                    }else{
                        let (hours, minutes, seconds) = ms_to_hms(time_left_millis);
                        let _ = show_log(app.clone(), &format!("\r\t开抢倒计时:{}小时:{}分钟:{:.3}秒\t", hours, minutes, seconds)).await;
                    }

                }

                _ = exit_r.recv() => {
                    let _ = show_log(app.clone(), "手动停止运行").await;
                    app.unlisten(id);
                    return Ok(());
                }

                _ = r.recv() => {
                    for _ in 0..20 {
                      match start(app.clone(), params.clone()).await {
                          Ok(is_success) => {
                            if is_success {
                                break;
                            }
                          },
                          Err(e) => {
                            let _ = show_log(app.clone(), e.to_string().as_str()).await;
                          },
                      };
                      tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                    return Ok(());
                }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetailState {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(rename = "orderNumber")]
    pub order_number: String,

    #[serde(rename = "firstShowName")]
    pub first_show_name: String,

    #[serde(rename = "qty")]
    pub num: i32,

    #[serde(rename = "displayPosterURL")]
    pub display_poster_url: String,

    #[serde(rename = "payAmount")]
    pub pay_amount: f64,

    #[serde(rename = "orderDetailState")]
    pub order_detail_state: OrderDetailState,

    #[serde(rename = "firstSessionName")]
    pub firsts_ession_name: String,

    #[serde(rename = "cityName")]
    pub city_name: String,

    #[serde(rename = "showTimeDesc")]
    pub show_time_desc: String,

    #[serde(rename = "firstVenueName")]
    pub first_venue_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPendingOrderListResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTerminateOrderListResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<Order>>,
}

#[tauri::command(async)]
pub async fn get_pending_orders(app: Window) -> Result<GetPendingOrderListResult, PXQError> {
    let app = Arc::new(app);
    let url = "cyy_gatewayapi/trade/buyer/order/v3/order_list?length=10&offset=0&orderStatusQuery=ONGOING";
    let json_data = json!({
        "src": API_SRC,
        "ver": API_VER,
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value(data).map_err(|_| PXQError::GetPendingOrderListError)?;
    Ok(result)
}

#[tauri::command(async)]
pub async fn get_terminate_orders(app: Window) -> Result<GetTerminateOrderListResult, PXQError> {
    let app = Arc::new(app);
    let url = "cyy_gatewayapi/trade/buyer/order/v3/order_list?length=10&offset=0&orderStatusQuery=TERMINATED";
    let json_data = json!({
        "src": API_SRC,
        "ver": API_VER,
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;

    let result = serde_json::from_value(data).map_err(|_| PXQError::GetTerminateOrderListError)?;
    Ok(result)
}

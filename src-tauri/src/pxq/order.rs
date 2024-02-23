use std::{fmt::format, time::Duration};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use rand::Rng;
use chrono::{DateTime, Local, TimeZone};

use super::{
    client::{get, post},
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

    #[serde(rename="priceItemName")]
    pub price_item_name: String,
    
    #[serde(rename="priceItemVal")]
    pub price_item_val: f64,

    #[serde(rename="priceItemType")] 
    pub price_item_type: String,

    #[serde(rename="direction")]
    pub direction: String,

    // #[serde(rename="tagColor")]
    // pub tag_color: String,

    // pub tag: String,

    #[serde(rename="priceItemSpecies")]
    pub price_item_species: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreOrderData {
    #[serde(rename="priceItems")]
    price_items: Vec<PriceItem>,

    #[serde(rename="supportDeliveries")]
    pub support_deliveries: Vec<SupportDeliverie>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreOrderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: PreOrderData,
}

#[tauri::command(async)]
pub async fn pre_order(
    app: tauri::Window,
    seat_plan_id: String,
    seat_plan_price: u32,
    ticket_num: u16,
    show_id: String,
    session_id: String,
    ticket_items: Vec<Value>,
) -> Result<PreOrderResult, PXQError> {
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/trade/buyer/order/v5/pre_order";
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "priorityId": "",
        "items": [{
            "sku": {
                "skuId": seat_plan_id,
                "skuType": "SINGLE",
                "ticketPrice": seat_plan_price,
                "qty": ticket_num,
                "ticketItems": ticket_items
            },
            "spu": {
                "showId": show_id,
                "sessionId": session_id
            }
        }]
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    
    let result = serde_json::from_value::<PreOrderResult>(data).map_err(|_|PXQError::PreOrderError)?;
    Ok(result)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderData {

    #[serde(rename="createTime")]
    pub create_time: u128,

    #[serde(rename="orderId")]
    pub order_id: String,

    #[serde(rename="orderNumber")]
    pub order_number: String,

    #[serde(rename="unPaidTransactionIds")]
    pub un_paid_transaction_ids: Vec<String>,

    #[serde(rename="paidDeadLineTime")]
    pub paid_dead_line_time: u128,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<CreateOrderData>
}

pub async fn create_order(app: tauri::Window, 
    price_items: Vec<PriceItem>, 
    seat_plan: SeatPlan,
    ticket_num: u16,
    show_id: String,
    sesion_id: String,
    ticket_items: Vec<Value>,
    audiences: Vec<UserAudienceData>,
    deliver_method: String,
    ) -> Result<CreateOrderResult, PXQError> {
    let mut price_item_param = Vec::new();
    let payment_amount = seat_plan.original_price * ticket_num as f64;
    for item in price_items {
        price_item_param.push(json!({
            "priceItemName": item.price_item_name,
            "priceItemVal": format!("{}", item.price_item_val),
            "priceItemType": item.price_item_type,
            "priceItemSpecies": item.price_item_species,
            "direction": item.direction,
            "applyTickets": [],
            "priceDisplay": format!("￥{}", item.price_item_val)
        }))
    }

    let mut sku_ticket_items = Vec::new();

    for i in 0..ticket_items.len() {
        sku_ticket_items.push(json!({
            "id": ticket_items[i]["id"],
            "audienceId": audiences[i].id,
        }))
    }

    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/trade/buyer/order/v5/create_order";
    
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "addressParam": {},
        "locationParam": {
            "locationCityId": "4401"
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
                "qty": ticket_num,
                "ticketItems": sku_ticket_items,
            },
            "spu": {
                "showId": show_id,
                "sessionId": sesion_id
            },
            "deliverMethod": deliver_method
        }],
        "priorityId": "",
        "many2OneAudience": {}
    });

    let data = post(app, url, json_data).await.map_err(|_|PXQError::ReqwestError)?;

    println!("{:?}", data);
    let result = serde_json::from_value::<CreateOrderResult>(data).map_err(|_|PXQError::CreateOrderError)?;
    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMsg {
    pub msg: String,
}

pub async fn show_log(app: &tauri::Window, msg: &str) -> Result<(), PXQError> {
    app.emit(
        "show-log",
        LogMsg {
            msg: msg.to_string(),
        },
    )
    .map_err(|_| PXQError::AddReminderError)?;
    Ok(())
}

fn random_number_str() -> String {
    let mut random_number = rand::thread_rng().gen_range(1..=10000).to_string();
    while random_number.len() != 5 {
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


    fn rand_i64(value: i64) -> u64 {
        let min_value = value / 5 * 4;
        let max_value = value + value / 10;
        let mut rng = rand::thread_rng();
        rng.gen_range(min_value..max_value) as u64
    }


#[tauri::command(async)]
pub async fn buy_tickets(
    app: tauri::Window,
    show: Show,
    session: Session,
    seat_plan: SeatPlan,
    ticket_num: u16,
    audiences: Vec<UserAudienceData>,
) -> Result<(), PXQError> {
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
    show_log(&app, &msg).await?;
    
    let session_status = session.session_status.clone();
    if session_status == "ON_SALE" {
        let current_time = Local::now().timestamp_millis();

        let mut ticket_items = Vec::new();
        for _ in 0..audiences.len() {
            ticket_items.push(json!({
                "id": format!("{}1000{}", current_time, random_number_str())
            }))
        }
      
        let res = pre_order(app.clone(), seat_plan.seat_plan_id.clone(), seat_plan.original_price as u32, ticket_num, show.show_id.clone(), session.clone().session_id.clone(), ticket_items.clone()).await?;
        let _ = show_log(&app, &format!("预下单返回数据:{:?}", res)).await;
        if res.status_code == 200 {
            let _ = show_log(&app, "预下单成功, 开始提交订单").await;
            let create_order_res = create_order(app.clone(), res.data.price_items, seat_plan.clone(), ticket_num, show.show_id, session.clone().session_id, ticket_items, audiences.clone(), res.data.support_deliveries[0].name.clone()).await?;
            let msg = format!("创建订单返回数据: {:?}", create_order_res);
            if create_order_res.status_code == 200 {
                let _ = show_log(&app, &format!("创建订单成功, 订单号:{}, 请及时打开APP付款!\n{}", create_order_res.data.unwrap().order_number, msg)).await;
            }else {
                let _ = show_log(&app, &format!("创建订单失败, {}\n{}",create_order_res.comments,  msg, )).await;
                return Err(PXQError::CreateOrderError);
            }
        }else{
            let _ = show_log(&app, "预下单失败, 停止运行...").await;
            return Err(PXQError::PreOrderError);
        }
        return Ok(());
    }

    let session_start_time = session.session_sale_time;
    if session_status == "PENDING" && session_start_time.is_none() {
        let _ = show_log(&app, &format!("{}, 暂未公布开抢时间, 停止运行...", show.show_name)).await;
        return Err(PXQError::ShowTimeUnknownError);
    }
    let session_start_time = session_start_time.unwrap();
    let (s, r) = async_channel::unbounded::<bool>();
    let (exit_s, exit_r) = async_channel::unbounded::<bool>();
    let interval = rand_i64(100);
    let earliest_submit_time = 0;

    let id = app.listen("stop-buy-tickets",  move |event| {
        let _ = exit_s.send_blocking(true);
    });
 

    let _ = show_log(&app, "演唱会门票暂未开售, 等待开抢中...").await;
    // 轮询等待开抢
    loop {
        tokio::select! {

            _ = tokio::time::sleep(Duration::from_millis(interval)) => {
                let local: DateTime<Local> = Local::now();
                let millis = local.timestamp_millis();
                let time_left_millis = session_start_time - millis;
                if time_left_millis <= earliest_submit_time {
                    let _ = s.send(true).await;
                }else{
                    let (hours, minutes, seconds) = ms_to_hms(time_left_millis);
                    let _ = show_log(&app, &format!("\r\t开抢倒计时:{}小时:{}分钟:{:.3}秒\t", hours, minutes, seconds)).await?;
                }

            }

            _ = exit_r.recv() => {
                let _ = show_log(&app, &format!("进程已终止...")).await?;
                app.unlisten(id);
                return Ok(());
            }

            _ = r.recv() => {
                for i in 0..10 {
                    let current_time = Local::now().timestamp_millis();

                    let mut ticket_items = Vec::new();
                    for _ in 0..audiences.clone().len() {
                        ticket_items.push(json!({
                            "id": format!("{}1000{}", current_time, random_number_str())
                        }))
                    }
                    let res = pre_order(app.clone(), seat_plan.clone().seat_plan_id, seat_plan.clone().original_price as u32, ticket_num, show.clone().show_id.clone(), session.clone().session_id.clone(), ticket_items.clone()).await?;
                    let _ = show_log(&app, &format!("预下单返回数据:{:?}", res)).await;
                    if res.status_code == 200 {
                        let _ = show_log(&app, "预下单成功, 开始提交订单").await;
                        let create_order_res = create_order(app.clone(), res.data.price_items, seat_plan.clone(), ticket_num, show.clone().show_id, session.clone().session_id, ticket_items, audiences.clone(), res.data.support_deliveries[0].name.clone()).await?;
                        let msg = format!("创建订单返回数据: {:?}", create_order_res);
                        if create_order_res.status_code == 200 {
                            let _ = show_log(&app, &format!("创建订单成功, 订单号:{}, 请及时打开APP付款!\n{}", create_order_res.data.unwrap().order_number, msg)).await;
                        }else {
                            let _ = show_log(&app, &format!("创建订单失败, {}\n{}",create_order_res.comments,  msg, )).await;
                            continue;
                        }
                    }else{
                        let _ = show_log(&app, "预下单失败").await;
                        continue;
                    }
                }
            }
    }
}
}
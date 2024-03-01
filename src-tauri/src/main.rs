// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use pxq_ticket::pxq;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    let manager = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_store::Builder::default().build());
    manager
        .invoke_handler(tauri::generate_handler![
            pxq::user::send_verification_code,
            pxq::user::generate_photo_code,
            pxq::user::login_by_mobile,
            pxq::user::refresh_token,
            pxq::user::get_user_profile,
            pxq::user::get_user_audiences,
            pxq::user::get_user_location,
            pxq::show::search_show_list,
            pxq::show::query_show_sessions,
            pxq::show::add_reminder,
            pxq::show::ticket_waitlist,
            pxq::show::get_seat_plans,
            pxq::order::pre_order,
            pxq::order::buy_tickets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

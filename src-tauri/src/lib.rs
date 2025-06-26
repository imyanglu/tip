// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use chrono::Local;
use serde::Serialize;
use tauri::{Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, Window};
pub mod model;
pub struct AppState {
    pub config: Arc<model::Config>,
}

fn load_config(path: &str) -> Result<model::Config, Box<dyn std::error::Error>> {
    let fs_res = fs::read_to_string(path)?;
    let config: model::Config = serde_json::from_str(&fs_res)?;
    Ok(config)
}

#[tauri::command]
fn get_earned_day(state: tauri::State<AppState>) -> f32 {
    state.config.get_earned_day()
}

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}
fn start_periodic_push(window: WebviewWindow) {
    std::thread::spawn(move || loop {
        let message = format!("{}", Local::now());
        let _ = window.emit("infos", Payload { message });
        std::thread::sleep(Duration::from_secs(1));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let res = load_config("./config.json");
    if let Err(ref e) = res {
        print!("Error loading config: {}", e.to_string())
    }
    let config = Arc::new(res.unwrap());
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口
            let window = app.get_webview_window("main").unwrap();
            start_periodic_push(window.clone());
            print!("xxx");
            // 获取当前显示器的信息
            if let Some(monitor) = window.current_monitor()? {
                let screen_size = monitor.size();
                let window_size = window.outer_size()?;

                // 计算位置：靠右 + 垂直居中
                let x = screen_size.width.saturating_sub(window_size.width); // 避免负数
                let y = (screen_size.height.saturating_sub(window_size.height)) / 2;

                // 设置位置
                window.set_position(LogicalPosition::new(x as f64, y as f64))?;
            }
            window.show()?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: config.clone(),
        })
        .invoke_handler(tauri::generate_handler![get_earned_day])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

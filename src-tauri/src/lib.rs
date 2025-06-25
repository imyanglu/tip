// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::sync::Arc;
pub mod model;
pub struct AppState {
    pub config: Arc<model::Config>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn load_config(path: &str) -> Result<model::Config, Box<dyn std::error::Error>> {
    let fs_res = fs::read_to_string(path)?;
    let config: model::Config = serde_json::from_str(&fs_res)?;
    Ok(config)
}

#[tauri::command]
// 定义一个名为my_custom_command的函数
fn my_custom_command() {
    // 打印一条消息，表示该函数被从JavaScript中调用
    println!("I was invoked from JavaScript!");
}
#[tauri::command]
fn get_earned_day(state: tauri::State<AppState>) -> f32 {
    state.config.get_earned_day()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let res = load_config("./config.json");
    if let Err(ref e) = res {
        print!("Error loading config: {}", e.to_string())
    }
    let config = Arc::new(res.unwrap());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: config.clone(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            get_earned_day
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

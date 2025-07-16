use std::sync::atomic::AtomicBool;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;

use std::{fs, thread};

use serde::Serialize;
use tauri::{Emitter, Manager, State, WindowEvent};
pub mod info;
pub mod model;
pub mod win;
pub struct AppState {
    // pub config: Arc<model::Config>,
    pub watch_process: Arc<AtomicBool>,
}

#[tauri::command]
fn get_system_info() -> info::SysInfo {
    info::get_sys_info()
}

#[tauri::command]
fn get_process_info() -> Result<Vec<win::ProcessInfo>, String> {
    let res = win::get_poc();
    if res.is_none() {
        return Err(String::from("读取错误"));
    } else {
        return Ok(res.unwrap());
    }
}
#[tauri::command]
fn kill_process(pid: u32) -> bool {
    win::kill_process(pid)
}
#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn watch_process(window: tauri::Window, state: tauri::State<AppState>) {
    state
        .watch_process
        .store(true, std::sync::atomic::Ordering::Relaxed);
    let mut process_info = get_process_info().unwrap();
    let running = state.watch_process.clone();
    let window = window.clone();
    thread::spawn(move || {
        while running.load(std::sync::atomic::Ordering::Relaxed) {
            let current_process_info = get_process_info().unwrap();

            if process_info != current_process_info {
                process_info = current_process_info;
                window.emit("process_change", process_info.clone()).unwrap();
            }

            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    // 阻止默认行为（直接关闭）
                    println!("阻止默认行为{}", window.label());
                    let win_label = window.label();
                    if win_label == "processLabel" {
                        let state = window.state::<AppState>();
                        let running = state.watch_process.clone();
                        running.store(false, std::sync::atomic::Ordering::Relaxed);

                        // 然后允许关闭窗口（也可以延迟几秒关闭）
                        window.close().unwrap(); // 或者 api.prevent_close(); 保留不关
                    }
                }
                WindowEvent::Destroyed => {
                    println!("窗口已销毁，可以做最终清理");
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            watch_process: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_process_info,
            kill_process,
            watch_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

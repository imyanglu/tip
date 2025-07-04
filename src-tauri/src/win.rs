use serde::Serialize;
use windows::{
    Win32::Foundation::*, Win32::System::Diagnostics::ToolHelp::*, Win32::System::ProcessStatus::*,
    Win32::System::Threading::*,
};

// 将 Windows UTF-16 字符串转换为 Rust 字符串
fn wide_to_string(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}
fn get_process_memory_usage(pid: u32) -> Option<(String, usize)> {
    unsafe {
        let handle_res = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if handle_res.is_err() {
            return None;
        }
        let h_process = handle_res.unwrap();
        // 获取映像路径
        let mut buffer = [0u16; 260];
        let len = K32GetProcessImageFileNameW(h_process, &mut buffer) as usize;
        let file_path = if len > 0 {
            wide_to_string(&buffer)
        } else {
            "<未知>".to_string()
        };
        let mut mem_counters = PROCESS_MEMORY_COUNTERS::default();
        let mem_ok = K32GetProcessMemoryInfo(
            h_process,
            &mut mem_counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .as_bool();
        let memory_kb = if mem_ok {
            mem_counters.WorkingSetSize / 1024
        } else {
            0
        };
        CloseHandle(h_process);
        // println!("进程路径: {}", file_path);
        return Some((file_path, memory_kb));
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    name: String,
    path: String,
    memory_kb: usize,
    pid: u32,
}
pub fn get_poc() -> Result<Vec<ProcessInfo>, Box<dyn std::error::Error>> {
    let mut process_list: Vec<ProcessInfo> = vec![];
    unsafe {
        // 获取所有进程快照
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        if snapshot == INVALID_HANDLE_VALUE {
            panic!("Failed to get snapshot");
        }
        // 初始化结构体
        let mut entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        // 获取第一个进程
        if Process32First(snapshot, &mut entry).is_ok() {
            loop {
                // 将 UTF-16 转换为 Rust 字符串
                let exe_name = String::from_utf16_lossy(
                    &entry
                        .szExeFile
                        .iter()
                        .map(|&x| x as u16)
                        .take_while(|x| *x != 0)
                        .collect::<Vec<u16>>(),
                );
                let pid = entry.th32ProcessID;
                let pmc = get_process_memory_usage(pid);
                if let Some((path, memory_kb)) = pmc {
                    process_list.push(ProcessInfo {
                        name: exe_name,
                        path,
                        memory_kb,
                        pid,
                    });
                }
                if !Process32Next(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }

        Ok(process_list)
    }
}

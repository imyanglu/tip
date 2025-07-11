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

#[derive(Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    name: String,
    path: String,
    memory_kb: usize,
    private_memory_kb: usize,
    pid: u32,
}
pub fn get_poc() -> Option<Vec<ProcessInfo>> {
    let mut process_list: Vec<ProcessInfo> = vec![];
    unsafe {
        let mut pids = [0u32; 1024];
        let size = (pids.len() * std::mem::size_of::<u32>()) as u32;
        let mut bytes_returned = 0;

        if !K32EnumProcesses(pids.as_mut_ptr(), size, &mut bytes_returned).as_bool() {
            return None;
        }
        let mem_size = std::mem::size_of::<u32>();

        let count = bytes_returned as usize / mem_size;
        for &pid in &pids[..count] {
            if pid == 0 {
                continue;
            }
            // 获取进程句柄
            let h_process_res =
                OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
            if h_process_res.is_err() {
                continue;
            }
            let h_process = h_process_res.unwrap();
            // 获取私有内存
            let mut mem_ex: PROCESS_MEMORY_COUNTERS_EX = std::mem::zeroed();
            mem_ex.cb = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
            K32GetProcessMemoryInfo(h_process, &mut mem_ex as *mut _ as *mut _, mem_ex.cb);
            let private_usage_kb = mem_ex.PrivateUsage / 1024;
            // 获取映像路径及内存大小
            let mut buffer = [0u16; 260];
            let len = K32GetProcessImageFileNameW(h_process, &mut buffer) as usize;
            let image_path = if len > 0 {
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
            let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
            GetProcessMemoryInfo(
                h_process,
                &mut counters as *mut _ as *mut _,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
            );
            process_list.push(ProcessInfo {
                path: image_path.clone(),
                name: image_path,
                private_memory_kb: counters.PrivateUsage / 1024,
                pid: pid,
                memory_kb,
            });

            CloseHandle(h_process);
        }
        return Some(process_list);
    }
}

pub fn kill_process(pid: u32) -> bool {
    unsafe {
        let handle_res = OpenProcess(PROCESS_TERMINATE, false, pid);
        if handle_res.is_err() {
            return false;
        }
        let h_process = handle_res.unwrap();
        TerminateProcess(h_process, 0);
        CloseHandle(h_process);
        return true;
    }
}

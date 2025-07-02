use windows::{
    core::*, Win32::Foundation::*, Win32::System::Diagnostics::ToolHelp::*,
    Win32::System::Threading::*,
};
fn get_process_memory_usage(pid: u32) -> Option<PROCESS_MEMORY_COUNTERS> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if handle.is_invalid() {
            return None;
        }

        let mut pmc = PROCESS_MEMORY_COUNTERS::default();
        pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        let res = GetProcessMemoryInfo(handle, &mut pmc, pmc.cb);
        CloseHandle(handle);

        if !res.as_bool() {
            return None;
        }

        Some(pmc)
    }
}
#[test]
fn test() {
    get_poc().unwrap();
}
fn get_poc() -> Result<()> {
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
        println!("{:?}", entry);
        // 获取第一个进程
        if Process32First(snapshot, &mut entry).is_ok() {
            loop {
                // 将 UTF-16 转换为 Rust 字符串
                let exe_name = String::from_utf16_lossy(
                    &entry
                        .szExeFile
                        .iter()
                        .take_while(|&&c| c != 0)
                        .map(|&x| x as u16)
                        .collect::<Vec<u16>>(),
                );
                let pid = entry.th32ProcessID;
                let pmc = get_process_memory_usage(pid);
                if let Some(mem_info) = pmc {
                    println!("进程 PID={} 的内存使用情况：", pid);
                    println!(
                        "工作集大小 (WorkingSetSize): {} bytes",
                        mem_info.WorkingSetSize
                    );
                    println!(
                        "峰值工作集大小 (PeakWorkingSetSize): {} bytes",
                        mem_info.PeakWorkingSetSize
                    );
                    println!(
                        "页面文件使用量 (PagefileUsage): {} bytes",
                        mem_info.PagefileUsage
                    );
                } else {
                    println!("无法获取进程内存信息，可能权限不足或进程不存在");
                }

                // 获取下一个进程
                if !Process32Next(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }

        Ok(())
    }
}

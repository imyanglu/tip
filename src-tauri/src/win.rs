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

                println!("PID: {:<6} Name: {}", entry.th32ProcessID, exe_name);

                // 获取下一个进程
                if !Process32Next(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }

        Ok(())
    }
}

use std::path::Path;

use serde::Serialize;
use std::fs;
use std::io;

use sys_info::{self, DiskInfo, Error, MemInfo};
#[derive(Serialize)]
#[serde(rename_all = "camelCase")] //
pub struct SysInfo {
    temp_dir_size: u64,
    hostname: String,
    cpu_num: u32,
    cpu_speed: u64,
    proc_total: u64,
    os_release: String,
    os_type: String,
    disk_total: Option<u64>,
    disk_free: Option<u64>,
    mem_info: Option<MemInfoJson>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MemInfoJson {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}
fn get_dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;

    if path.is_dir() {
        for entry_result in fs::read_dir(path)? {
            let entry = entry_result?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // 递归计算子目录大小
                total_size += get_dir_size(&entry_path)?;
            } else if entry_path.is_file() {
                // 累加文件大小
                let metadata = fs::metadata(&entry_path)?;
                total_size += metadata.len();
            }
        }
    }

    Ok(total_size)
}
pub fn get_sys_info() -> SysInfo {
    let temp_dir = std::env::temp_dir();
    let temp_size = get_dir_size(Path::new(&temp_dir)).unwrap_or(0);
    let disk_info = sys_info::disk_info();
    let mem_info = sys_info::mem_info();
    let mut disk_total: Option<u64> = None;
    let mut disk_free: Option<u64> = None;
    if disk_info.is_ok() {
        let disk = disk_info.unwrap();
        disk_total = Some(disk.total);
        disk_free = Some(disk.free);
    }
    let mut mem_info_json: Option<MemInfoJson> = None;
    if mem_info.is_ok() {
        let mem = mem_info.unwrap();
        mem_info_json = Some(MemInfoJson {
            total: mem.total,
            free: mem.free,
            avail: mem.avail,
            buffers: mem.buffers,
            cached: mem.cached,
            swap_total: mem.swap_total,
            swap_free: mem.swap_free,
        });
    }
    SysInfo {
        temp_dir_size: temp_size,
        hostname: sys_info::hostname().unwrap_or(String::from("读取错误")),
        cpu_num: sys_info::cpu_num().unwrap_or(0),
        cpu_speed: sys_info::cpu_speed().unwrap_or(0),
        os_release: sys_info::os_release().unwrap_or(String::from("获取错误")),
        proc_total: sys_info::proc_total().unwrap_or(0),
        os_type: sys_info::os_type().unwrap_or(String::from("获取错误")),
        disk_free,
        disk_total,
        mem_info: mem_info_json, // mem_info: sys_info::mem_info(),
    }
}

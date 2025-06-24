use std::fs;

use chrono::prelude::*;
use serde_json;
mod model;
// fn calc_time_gap(end: DateTime<Utc>) -> i64 {
//     let now = Utc::now();
//     let duration = end - now;
//     duration.num_seconds()
// }

// fn calc_date_gap(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
//     let duration = end - start;
//     duration.num_hours()
// }

pub fn load_config(path: &str) -> Result<model::Config, Box<dyn std::error::Error>> {
    let fs_res = fs::read_to_string(path)?;
    let config: model::Config = serde_json::from_str(&fs_res)?;
    Ok(config)
}
fn main() {
    let res = load_config("./config.json");

    if let Err(ref e) = res {
        print!("Error loading config: {}", e.to_string())
    }
    let config = res.unwrap();
    config.get_work_hours_day();

    // let is_work_day = config.is_work_day(Local::now());
}

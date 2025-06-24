use chrono::{Duration, prelude::*};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    salary: i64,
    start_work_time: String,
    end_work_time: String,
    holidays: Vec<Vec<u32>>,
}
impl Config {
    pub fn get_time_work_ends(&self) -> (i64, i64, i64) {
        let work_end_time = NaiveTime::parse_from_str(&self.end_work_time, "%H:%M").unwrap();
        let today = Local::now().date_naive();
        let now = Local::now();
        let work_end_date = today.and_time(work_end_time);
        let local_dt = Local.from_local_datetime(&work_end_date).unwrap();
        let diff = local_dt - now;
        if diff > Duration::zero() {
            let seconds = diff.num_seconds();
            return (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
        }
        return (-1, -1, -1);
    }
    pub fn is_work_day(&self, date: DateTime<Local>) -> bool {
        let m = date.month() - 1;
        let date = date.day();
        let month_holidays = self.holidays.get(m as usize).unwrap();
        !month_holidays.contains(&date)
    }
    pub fn get_work_hours_day(&self) -> f32 {
        let work_start_time = NaiveTime::parse_from_str(&self.start_work_time, "%H:%M").unwrap();
        let work_end_time = NaiveTime::parse_from_str(&self.end_work_time, "%H:%M").unwrap();
        let time_gap = work_end_time
            .signed_duration_since(work_start_time)
            .num_seconds();
        return (time_gap as f32 / 3600.0);
    }

    pub fn get_days_in_current_month(&self, today: NaiveDateTime) -> u32 {
        let year = today.year();
        let month = today.month();
        let next_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
        };
        // 当月最后一天 = 下月第一天 - 1天
        let last_day_of_month = next_month.pred_opt().unwrap();
        last_day_of_month.day()
    }
    fn get_earned_day(&self) -> f32 {
        let now = Local::now().naive_local();
        let days = self.get_days_in_current_month(now);
        let holiday_count = self.holidays.get(now.month() as usize - 1).unwrap().len();
        let work_day_count = days - holiday_count as u32;
        // let earned_day = work_day_count * (self.salary / days) as f32;
        // let hours = self.get_work_hours_day();

        0.0
    }
}

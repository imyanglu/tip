use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WorkInfo {
    worked_money: f32,
    worked_time: i64,
    work_end_seconds: i64,
    next_vacation_date: Option<VacationDate>,
    next_vacation_duration: i64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct VacationDate {
    date: String,
    name: Vec<String>,
    days: u8,
    adjustment: u8,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    salary: i64,
    start_work_time: String,
    end_work_time: String,
    vacation: Vec<VacationDate>,
    restdays: Vec<Vec<u32>>,
}

impl Config {
    pub fn get_time_work_ends(&self) -> i64 {
        let work_end_time = NaiveTime::parse_from_str(&self.end_work_time, "%H:%M").unwrap();
        let today = Local::now().date_naive();
        let now = Local::now();
        let work_end_date = today.and_time(work_end_time);
        let local_dt = Local.from_local_datetime(&work_end_date).unwrap();
        let diff = local_dt - now;
        if diff > Duration::zero() {
            diff.num_seconds()
        } else {
            -1
        }
    }
    pub fn get_worked_seconds_day(&self) -> i64 {
        let work_start_time = NaiveTime::parse_from_str(&self.start_work_time, "%H:%M").unwrap();
        let now = Local::now().naive_local();
        let today = Local::now().date_naive();
        let work_start_time = today.and_time(work_start_time);
        let diff = now.signed_duration_since(work_start_time);
        if diff > Duration::zero() {
            diff.num_seconds()
        } else {
            -1
        }
    }
    pub fn is_work_day(&self, date: DateTime<Local>) -> bool {
        let m = date.month() - 1;
        let date = date.day();
        let month_restdays = self.restdays.get(m as usize).unwrap();
        !month_restdays.contains(&date)
    }
    pub fn get_work_seconds_day(&self) -> f32 {
        let work_start_time = NaiveTime::parse_from_str(&self.start_work_time, "%H:%M").unwrap();
        let work_end_time = NaiveTime::parse_from_str(&self.end_work_time, "%H:%M").unwrap();
        let time_gap = work_end_time
            .signed_duration_since(work_start_time)
            .num_seconds();
        time_gap as f32
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
    pub fn get_earned_day(&self) -> f32 {
        let now = Local::now().naive_local();
        let days = self.get_days_in_current_month(now);
        let holiday_count = self.restdays.get(now.month() as usize - 1).unwrap().len();
        let work_day = days - holiday_count as u32;
        let avg_salary = self.salary as f32 / work_day as f32;
        let all_seconds_on_day = self.get_work_seconds_day();
        let worked_seconds = self.get_worked_seconds_day();
        let earned_day = avg_salary * worked_seconds as f32 / all_seconds_on_day as f32;
        earned_day
    }
    fn parse_date(date: &str) -> NaiveDate {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        return date;
    }

    pub fn get_worked_time_day(&self) -> (i64, i64, i64) {
        let worked_seconds = self.get_worked_seconds_day();
        let hours = worked_seconds / 3600;
        let minutes = (worked_seconds % 3600) / 60;
        let seconds = worked_seconds % 60;
        (hours, minutes, seconds)
    }
    pub fn get_next_vacation(&self) -> Option<(i64, VacationDate)> {
        let today = Local::now().naive_local();

        let closest_vacation = self
            .vacation
            .iter()
            .filter_map(|item| {
                let i = Self::parse_date(&item.date).and_hms_opt(0, 0, 0).unwrap();
                if i > today {
                    Some((item, i - today))
                } else {
                    None
                }
            })
            .min_by_key(|(_, duration)| *duration);
        if closest_vacation.is_none() {
            return None;
        } else {
            let next_vacation_res = closest_vacation.unwrap();
            let next_vacation = next_vacation_res.0;
            let duration = next_vacation_res.1;

            return Some((duration.num_seconds(), next_vacation.clone()));
        }
    }
    pub fn get_worked_info(&self) -> WorkInfo {
        let next_vacation_res = self.get_next_vacation();
        if next_vacation_res.is_none() {
            return WorkInfo {
                worked_money: self.get_earned_day(),
                worked_time: self.get_worked_seconds_day(),
                work_end_seconds: self.get_time_work_ends(),
                next_vacation_date: None,
                next_vacation_duration: -1,
            };
        }
        let next_vacation = next_vacation_res.unwrap();
        WorkInfo {
            worked_money: self.get_earned_day(),
            worked_time: self.get_worked_seconds_day(),
            work_end_seconds: self.get_time_work_ends(),
            next_vacation_date: Some(next_vacation.1),
            next_vacation_duration: next_vacation.0,
        }
    }
}

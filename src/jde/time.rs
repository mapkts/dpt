use chrono;
use chrono::Local;
use std::time::Duration;
use tokio::time::sleep;

pub fn today() -> String {
    Local::today().format("%Y/%m/%d").to_string()
}

pub fn today_pred(days: i64) -> String {
    let duration = chrono::Duration::days(days);
    Local::today()
        .checked_sub_signed(duration)
        .unwrap()
        .format("%Y/%m/%d")
        .to_string()
}

pub fn yestoday() -> String {
    Local::today().pred().format("%Y/%m/%d").to_string()
}

pub fn nextday() -> String {
    Local::today().succ().format("%Y/%m/%d").to_string()
}

pub async fn delay_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

pub fn century() -> String {
    Local::today().format("%C").to_string()
}

pub fn short_year() -> String {
    Local::today().format("%y").to_string()
}

pub fn month() -> String {
    Local::today().format("%m").to_string()
}

pub fn yyyymm() -> String {
    Local::today().format("%Y%m").to_string()
}

pub fn today_is_monday() -> bool {
    Local::today().format("%A").to_string().as_str() == "Monday"
}

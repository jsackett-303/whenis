use chrono::prelude::*;
use chrono::NaiveDateTime;
use std::process;

fn handle_timestamp(timestamp: String) -> () {
    let code = match timestamp.trim().parse::<i64>() {
        Err(_) => {
            println!("Please provide a valid unix timestamp");
            1
        }
        Ok(t) => {
            let d = NaiveDateTime::from_timestamp(t, 0);
            println! {"{}", d.format("%Y-%m-%d %H:%M:%SZ")};
            0
        }
    };

    process::exit(code);
}

fn main() {
    let unix_time = match std::env::args().nth(1) {
        None => Utc::now().timestamp().to_string(),
        Some(u) => u,
    };

    handle_timestamp(unix_time);
}

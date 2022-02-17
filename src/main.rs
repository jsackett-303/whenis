use chrono::prelude::*;
use chrono::NaiveDateTime;
use std::process;

fn invalid_input() -> () {
    println!("Please provide a valid unix timestamp");
    process::exit(1);
}

fn handle_timestamp(timestamp: String) -> () {
    match timestamp.trim().parse::<i64>() {
        Err(_) => invalid_input(),
        Ok(t) => {
            let d = NaiveDateTime::from_timestamp(t, 0);
            println! {"{}", d.format("%Y-%m-%d %H:%M:%SZ")};
            process::exit(0);
        }
    }
}

fn main() {
    match std::env::args().nth(1) {
        None => {
            let unix_time = Utc::now().timestamp().to_string();
            handle_timestamp(unix_time)
        }
        Some(unix_time) => handle_timestamp(unix_time),
    }
}

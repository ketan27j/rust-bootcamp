use chrono::{Local, Utc};
use dotenv::dotenv;
use std::env;
fn main() {
    dotenv().ok();
    let utc_time = Utc::now();
    let local_time = Local::now();
    println!("local time is {}",local_time);
    println!("UTC time is {}",utc_time);
    let var = env::var("URL").unwrap();
    println!("URL {}",var);
}
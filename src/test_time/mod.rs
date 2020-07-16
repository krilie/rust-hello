use chrono::prelude::*;

extern crate chrono;

#[test]
fn main() {
    let dt = Local::now();
    println!("{}", dt.to_rfc3339());
    println!("dt: {}", dt);
    println!("dt: {}", dt.timestamp_millis());
}
use chrono::{Local, Timelike};
use std::io;

fn main() {
    println!("Enter your name: ");
    let mut name: String = String::new();

    let stdin = io::stdin();
    let now = Local::now();
    let formatted_time = now.hour();

    stdin
        .read_line(&mut name)
        .expect("Failed to read your name");

    let name: &str = name.trim();

    if formatted_time <= 12 {
        println!(
            "Good morning, {}. \n The time is {} am",
            name, formatted_time
        );
    } else if formatted_time <= 17 {
        println!(
            "Good afternoon, {}. \n The time is {} pm",
            name, formatted_time
        );
    } else {
        println!(
            "Good evening, {}. \n The time is {} pm",
            name, formatted_time
        );
    }
}

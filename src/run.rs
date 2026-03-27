use crate::models::*;
use colored::Colorize;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn run(file: String) {
    let fcs: FlashCardSet = from_str(&read_to_string(file).expect("Failed to read file"))
        .expect("Failed to parse JSON");

    match fcs.name {
        Some(n) => println!("{} {}", "Welcome to".bright_black(), n.green()),
        None => (),
    }

    match fcs.author {
        Some(a) => println!("{} {}", "created by".bright_black(), a.green()),
        None => (),
    }

    match fcs.total {
        0 => (),
        _ => println!(
            "{} {}",
            "with total marks".bright_black(),
            fcs.total.to_string().green()
        ),
    }
}

use crate::models::*;
use crate::new::prompt;
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
        Some(a) => println!("{} {}", "Created by".bright_black(), a.green()),
        None => (),
    }

    match fcs.total {
        0 => (),
        _ => println!(
            "{} {}",
            "With total marks".bright_black(),
            fcs.total.to_string().green()
        ),
    }

    let mut score = 0u32;

    for flashcard in fcs.flashcards {
        println!("");
        let answer = prompt(&flashcard.question).expect("Expected answer via input");
        if answer == flashcard.answer {
            score += flashcard.weight;
            println!("{}", "You answered correctly!".green());
        } else {
            println!(
                "{} {}",
                "You answered incorrectly! Expected".red(),
                flashcard.answer.green()
            );
        }
    }

    match fcs.total {
        0 => (),
        _ => println!(
            "\n{} {} {} {}",
            "You scored".blue(),
            score.to_string().green(),
            "out of".blue(),
            fcs.total.to_string().green()
        ),
    }
}

use crate::models::*;
use colored::Colorize;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io::{BufWriter, Write, stdin, stdout};

fn prompt(text: &str) -> Option<String> {
    print!("{}", text.blue());
    stdout().flush().expect("Failed to flush stdout");

    let mut buf = String::new();
    let n = stdin().read_line(&mut buf).unwrap_or(0);

    if n == 0 {
        None
    } else {
        Some(buf.trim_end().trim_start().to_string())
    }
}

pub fn new(path: String) {
    println!(
        "{}",
        "Please enter some extra information about the flashcard set.".green()
    );
    println!(
        "{}",
        "You may skip the following inputs by sending EOF.".bright_black()
    );

    let name = prompt("What is the name of the new flashcard set? ");
    let author = prompt("Who is the author of the new flashcard set? ");

    println!("\n{}", "Please create your flashcards.".green());
    println!(
        "{}\n",
        "You may stop creating flashcards by sending EOF".bright_black()
    );

    let mut flashcards: Vec<FlashCard> = Vec::new();
    let mut total = 0u32;

    while let Some(question) = prompt("Flashcard question: ") {
        let answer = match prompt("Flashcard answer: ") {
            Some(a) => a,
            None => break,
        };

        let weight = match prompt("Flashcard weight: ") {
            Some(w_str) => {
                let w = w_str.parse::<u32>().unwrap_or(0);
                total += w;
                w
            }
            None => 0,
        };

        flashcards.push(FlashCard {
            question,
            answer,
            weight,
        });
    }

    let file = File::create(path).expect("Failed to create file");
    let writer = BufWriter::new(file);

    to_writer_pretty(
        writer,
        &FlashCardSet {
            name,
            author,
            total,
            flashcards: Some(flashcards),
        },
    )
    .expect("Failed to write to file");
}

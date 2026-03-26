use colored::Colorize;
use std::io::{Write, stdin, stdout};

fn prompt(text: &str) -> Option<String> {
    print!("{}", text.blue());
    stdout()
        .flush()
        .unwrap_or_else(|e| println!("{}", e.to_string().red()));

    let mut buf = String::new();
    let n = stdin().read_line(&mut buf).unwrap_or(0);

    if n == 0 {
        None
    } else {
        Some(buf.trim_end().trim_start().to_string())
    }
}

pub fn new() {
    println!(
        "{}",
        "At any optional input, you may send EOF to signal nothing.".bright_black()
    );

    let name = prompt("(Optional) What is the name of the new flashcard set? ");
    let author = prompt("(Optional) Who is the author of the new flashcard set? ");
}

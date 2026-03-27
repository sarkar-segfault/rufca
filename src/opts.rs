use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rufca",
    author,
    version,
    about,
    help_template = "{name} {version}\n{about}\n{author}\nhttps://github.com/sarkar-segfault/rufca\n\n{all-args}"
)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Creates a new flashcard set interactively
    New {
        /// The file to store the flashcards in
        file: String,
    },

    /// Runs an existing flashcard set interactively
    Run {
        // The flashcard file to run
        file: String,
    },
}

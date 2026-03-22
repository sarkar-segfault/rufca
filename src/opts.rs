use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rufca", version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    New { file: String },
    Run { file: String },
}

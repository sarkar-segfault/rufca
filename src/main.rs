mod models;
mod new;
mod opts;

use clap::Parser;
use colored::Colorize;

fn main() {
    let opts = opts::Options::parse();

    match opts.cmd {
        opts::Command::New { file } => new::new(file),
        opts::Command::Run { file } => println!("run {file:?}"),
    }
}

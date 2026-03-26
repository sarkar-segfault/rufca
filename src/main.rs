mod models;
mod new;
mod opts;

use clap::Parser;

fn main() {
    let opts = opts::Options::parse();

    match opts.cmd {
        opts::Command::New { file } => println!("new {file:?}"),
        opts::Command::Run { file } => println!("run {file:?}"),
    }
}

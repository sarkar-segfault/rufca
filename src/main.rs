mod models;
mod new;
mod opts;
mod run;

use clap::Parser;

fn main() {
    let opts = opts::Options::parse();

    match opts.cmd {
        opts::Command::New { file } => new::new(file),
        opts::Command::Run { file } => run::run(file),
    }
}

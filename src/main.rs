use std::process;

use clap::Parser;

fn main() {
    let args = minigrep::args::Args::parse();

    if let Err(e) = minigrep::run(&args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

use clap::{Parser, ValueEnum};

mod commands;
mod utils;

use commands::{build, test};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    command: Mode,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Build,
    Bump,
    Format,
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Mode::Build => build::run(),
        Mode::Bump => println!("b"),
        Mode::Format => println!("c"),
        Mode::Test => test::run(),
    }
}

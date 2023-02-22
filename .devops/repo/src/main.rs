use clap::{Parser, ValueEnum};

mod commands;
mod utils;

use commands::{build, fmt, test};

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
    Fmt,
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Mode::Build => build::run(),
        Mode::Bump => println!("b"),
        Mode::Fmt => fmt::run(),
        Mode::Test => test::run(),
    }
}

use clap::{Parser, ValueEnum};

mod commands;
mod utils;

use commands::{build, fmt, test};
use utils::{read_applications, repo_root};

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

    let root = repo_root();
    let apps = read_applications();

    match cli.command {
        Mode::Build => build::run(apps, root),
        Mode::Bump => println!("b"),
        Mode::Fmt => fmt::run(apps, root),
        Mode::Test => test::run(apps, root),
    }
}

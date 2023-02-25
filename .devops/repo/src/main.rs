mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Mode};
use commands::{build, bump, fmt, test};
use utils::{read_applications, repo_root};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let root = repo_root();

    match cli.command {
        Mode::Build(opt) => match opt.apps {
            Some(apps) => build::run(apps, root),
            None => build::run(read_applications(), root),
        },
        Mode::Bump(opt) => match opt.apps {
            Some(apps) => bump::run(apps, root),
            None => bump::run(read_applications(), root),
        },
        Mode::Fmt(opt) => match opt.apps {
            Some(apps) => fmt::run(apps, root),
            None => fmt::run(read_applications(), root),
        },
        Mode::Test(opt) => match opt.apps {
            Some(apps) => test::run(apps, root),
            None => test::run(read_applications(), root),
        },
    }
}

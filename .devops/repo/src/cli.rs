use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(about = "Utility crate for maintaining the repository")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Mode,
}

#[derive(Clone, Subcommand)]
pub(crate) enum Mode {
    /// Compile the Sway and Rust programs in each project
    Build(Opt),
    /// Bump each project from its current `fuel-toolchain.toml` to the one in this repository.
    ///
    /// Any errors when bumping will result in returning to the original toolchain
    Bump {
        /// Provide a list of applications or omit to operate on all applications
        apps: Option<Vec<String>>,
    },
    /// Format the Sway and Rust files in each project
    Fmt(Opt),
    /// Run the Rust tests for each project
    Test {
        /// Provide a list of applications or omit to operate on all applications
        apps: Option<Vec<String>>,
    },
}

#[derive(Args, Clone)]
pub(crate) struct Opt {
    /// List of applications to run the command on e.g AMM escrow fundraiser ...
    #[clap(index = 2)]
    pub(crate) apps: Option<Vec<String>>,

    /// Program type to operate on: `rust`, `sway` or `all` to operate on both
    #[clap(index = 1)]
    pub(crate) program: Program,
}

#[derive(Clone, ValueEnum)]
pub(crate) enum Program {
    /// Rust and Sway files
    All,
    /// Rust files
    Rust,
    /// Sway files
    Sway,
}

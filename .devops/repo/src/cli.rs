use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Utility crate for maintaining the repository")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Mode,
}

#[derive(Clone, Subcommand)]
pub(crate) enum Mode {
    /// Build the Sway contracts for each project
    Build(Opt),
    /// Bump each project from its current `fuel-toolchain.toml` to the one in this repository
    Bump(Opt),
    /// Format the Sway and Rust files in each project
    Fmt(Opt),
    /// Run the Rust tests for each project
    Test(Opt),
}

#[derive(Args, Clone)]
pub(crate) struct Opt {
    /// Provide a list of applications or omit to operate on all applications
    pub(crate) apps: Option<Vec<String>>,
}

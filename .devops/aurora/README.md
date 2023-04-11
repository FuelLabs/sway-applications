## Instructions

To use the tool there are two other tools that must be installed.

- [`Rust`](https://www.rust-lang.org/tools/install) to enable the usage of `cargo`
- `Forc` with the recommendation of doing it through [`Fuelup`](https://github.com/FuelLabs/fuelup)

Once `cargo` and `forc` are installed you may proceed to the subsequent sections.

### Build the tool

Change into the following directory `/path/to/sway-applications/.devops/aurora/<here>` and run

```sh
cargo build
```

After building the executable you may run it directly from `/aurora/target/debug/aurora <command>` or with the usage below.

### Usage

```sh
Utility crate for maintaining the repository

Usage: aurora <COMMAND>

Commands:
  build  Compile the Sway and Rust programs in each project
  bump   Bump each project from its current `fuel-toolchain.toml` to the one in this repository
  fmt    Format the Sway and Rust files in each project
  test   Run the Rust tests for each project
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

For more information about each command run

```sh
cargo run <command> --help
```

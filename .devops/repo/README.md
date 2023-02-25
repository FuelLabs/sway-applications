## Instructions

To use the tool there are two other tools that must be installed.

- [`Rust`](https://www.rust-lang.org/tools/install) to enable the usage of `cargo`
- `Forc` with the recommendation of doing it through [`Fuelup`](https://github.com/FuelLabs/fuelup)

Once `cargo` and `forc` are installed you may proceed to the subsequent sections.

### Build the tool

Change into the following directory `/path/to/sway-applications/.devops/repo/<here>` and run

```sh
cargo build
```

After building the executable you may run it directly from `/repo/target/debug/repo <command>` or with the usage below.

### Usage

The tool has 4 modes. 

Each command may be used on its own in which case it will be performed on all applications within the repository. Alternatively, a list of applications may be specified.

The commands all follow the same format.

```sh
cargo run <command> app1 app2 ...
```

#### `build`

Build the Sway contracts for each project

#### `test`

Run the Rust tests for each project

#### `fmt`

Format the Sway and Rust files in each project

#### `bump`

Bump each project from its current `fuel-toolchain.toml` to the one in this repository. If the bump fails then the project will be restored to its previous toolchain

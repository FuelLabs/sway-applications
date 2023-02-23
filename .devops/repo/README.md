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

#### `build`

Build the Sway contracts for each project

```sh
cargo run build
```

#### `test`

Run the Rust tests for each project

```sh
cargo run test
```

#### `fmt`

Format the Sway and Rust files in each project

```sh
cargo run fmt
```

#### `bump`

Bump each project from its current `fuel-toolchain.toml` to the one in this repository. If the bump fails then the project will be restored to its previous toolchain

```sh
cargo run bump
```

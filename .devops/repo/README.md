## Instructions

### Build the tool

Change into the following directory `/path/to/sway-applications/.devops/repo/<here>` and run

```sh
cargo build
```

### Usage

The tool has 4 modes.

#### `build`

Building the Sway contracts in each project

```sh
cargo run build
```

#### `test`

Running the Rust tests for each project

```sh
cargo run test
```

#### `fmt`

Formatting the Sway and Rust files in each project

```sh
cargo run fmt
```

#### `bump`

Bumping each project from its current `fuel-toolchain.toml` to the one in this repository. If the bump fails then the project will be restored to its previous toolchain

```sh
cargo run bump
```

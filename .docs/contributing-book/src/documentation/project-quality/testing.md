# Testing

The [Sway Applications repository](https://github.com/FuelLabs/sway-applications) generally follows the current file structure when testing using the [Rust SDK](https://github.com/FuelLabs/fuels-rs).

```
tests/
├── functions/
|     └── 1 file per ABI function
├── utils/
|     ├── interface.rs
|     ├── setup.rs
|     └── mod.rs
└── harness.rs
```

## `functions`

The `functions` directory contains 1 file per function declared in the `abi` and all **test cases** (not utility/helper functions) for that function are contained within that module. 

There are two possibilities with any function call and they are that either the call succeeds or it reverts. For this reason each file contains two modules:

- `success`
- `revert`

All of the tests where the function does not revert should be contained inside the `success` module while the reverting calls (panics) should be contained inside the `revert` module.

There are two additional alternatives to consider:

1) The `abi` may be split into multiple `abi` blocks
   1) In this case each `abi` may be its own directory containing its functions
   2) This changes the structure from having all functions in 1 directory to having categorized functions in their relevant directories
2) In some cases it may be reasonable to further separate the function into its own directory which contains two files
   1) `success`
   2) `revert`

## `utils`

The `utils` directory contains utility functions and abstractions which allow the tests in the `functions` directory to remain small, clean and "DRY" (do not repeat yourself).

This can be achieved by separating content into files, internally creating modules in `mod.rs` or a mixture of both.

The `interface.rs` file contains `abi` wrappers which are functions that are generally limited to calling the contract with the relevant arguments. In the case where there are multiple `abi` blocks (as mentioned in [`functions`](#functions)) the `interface.rs` file would be changed to follow a similar structure.

The `setup.rs` file contains code that generates the contracts/scripts/predicates and sets up the environment for the tests in the [`functions`](#functions) directory.

## `harness.rs`

The `harness` file is the entry point for the tests, and thus it contains the `functions` and `utils` modules. This is what is executed when `cargo test` is run.



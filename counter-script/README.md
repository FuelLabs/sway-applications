<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/counter-script-logo-dark-theme.png">
        <img alt="SwayApps Counter Script Logo" width="400px" src=".docs/counter-script-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.42.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.42.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.18.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.18.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.43.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.43.0-blue" />
    </a>
</p>

## Overview

The script is an example which demonstrates how to call contracts in an arbitrary order and test them using the Rust SDK.

The script manipulates contract state through ABI calls.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
counter-script
├── project
│   ├── contracts
│   │   └── counter
│   │       └── src/main.sw
│   ├── libraries
│   │   └── src/interface.sw
│   ├── scripts
│   │   └── interaction_script
│   │       └── src/main.sw
│   ├── tests/src/harness.rs
│   ├── README.md
│   └── SPECIFICATION.md
├── ui
│   ├── README.md
│   └── SPECIFICATION.md
└── README.md
```

## Running the project

In order to run the subsequent commands change into the following directory `/path/to/counter-script/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```

## Walkthrough/Additional Information

### Execution flow

The script calls all 3 methods of the contract in a specific order, along with asserting the retrieved values from the contract in order to demonstrate that the contract is behaving as expected.

Note: "asserted" refers to the use of the `require` function, which asserts, or requires, that the input condition be true. If the input condition is false, the script stops executing and reverts.

Following is the order of execution:
- the `count` method is called and the returned value is stored in a variable `count`
- `count` is asserted to be equal to 0, as that is the default value of the `count` variable
- the `increment` method is called and the returned value is stored in the variable `count`
- `count` is now asserted to be equal to 1, as after incrementing 0 by 1, the result will be 1.
- the `increment` method is called again and the returned value is stored in the variable `count`
- `count` is now asserted to be equal to 2, as after incrementing 1 by 1, the result will be 2.
- If the value of the parameter `clear`, which was passed into the script main function, is true, the `clear` method is called.
- the `count` method is called and the returned value is stored in a variable `count`
- the `count` variable is returned, script execution is finished.

### Overview of the Rust integration test

The rust integration test deploys the counter contract, and then executes the script. One of the tests is with the `clear` set to true, another test is with the `clear` set to false.

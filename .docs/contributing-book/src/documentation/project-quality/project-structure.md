# Project Structure

In order to navigate through a project easily there needs to be a structure that compartmentalizes concepts. This means that code is grouped together based on some concept. 

For example, here is an example structure that we follow for `Sway` files in the `src` directory.

```
src/
├── data_structures.sw
├── errors.sw
├── events.sw
├── interface.sw
├── main.sw
└── utils.sw
```

In the example above there are no directories however it may make sense for a project to categorize concepts differently such as splitting the `data_structures.sw` into a directory containing individual Sway modules.

## data_structures.sw

Contains data structures written for your project.

- structs
- enums
- trait implementations

## errors.sw

Contains enums that are used in `require(..., MyError::SomeError)` statements.
The enums are split into individual errors e.g. `DepositError`, `OwnerError` etc.

```rust
{{#include ../../code/connect-four/src/errors.sw:3:}}
```

## events.sw

Contains structs definitions which are used inside `log()` statements.

```rust
{{#include ../../code/connect-four/src/events.sw:3:}}
```

## interface.sw

Anything that may be exposed to the user e.g. the Application Binary Interface (`abi`) for your contract(s).

This means that the `events.sw` may not be necessary and that information can be held with the `abi`. Similarly, certain data structures may be more suited to be in the interface.

## main.sw

The entry point to your contract that contains the implementation of your `abi`.

## utils.sw

Any private functions (helper functions) that your contracts use inside their functions.
Since a contract cannot call its own `ABI` functions, we need to abstract away some of the code into private functions so that they can be reused in your contract
# Internal Project

These are projects that do not provide an externally available interface.

Here is an example structure that we follow for `Sway` files in the `src` directory.

```
src/
├── data_structures.sw
├── errors.sw
├── events.sw
├── interface.sw
├── main.sw
└── utils.sw
```

In the example above there are no directories, however, it may make sense for a project to categorize concepts differently such as splitting the `data_structures.sw` into a directory containing individual `Sway` modules.

## data_structures.sw

Contains data structures written for your project.

- structs
- enums
- trait implementations

## errors.sw

Contains enums that are used in `require(..., MyError::SomeError)` statements.
The enums are split into individual errors e.g. `DepositError`, `OwnerError` etc.

```sway
{{#include ../../../code/connect_four/src/errors.sw:error}}
```

## events.sw

Contains structs definitions which are used inside `log()` statements.

```sway
{{#include ../../../code/connect_four/src/events.sw:event}}
```

## interface.sw

The Application Binary Interface (`ABI`) for your contract(s).

This means that the `events.sw` may not be necessary and that information can be held with the `ABI`. Similarly, certain data structures may be more suited to be in the interface.

## main.sw

The entry point to your contract that contains the implementation of your `ABI`.

## utils.sw

Any private functions (helper functions) that your contracts use inside their functions.
Since a contract cannot call its own `ABI` functions, we need to abstract away some of the code into private functions so that they can be reused in your contract.
# Project Structure

In order to navigate through a project easily there needs to be a structure that compartmentalizes concepts. This means that code is grouped together based on some concept. 

For example, here is an example structure that we follow for `Sway` files in the `src` directory.

- `data_structures.sw`
  - Contains your custom data structures
    - structs
    - enums
    - Any trait implementations of your custom structures
- `errors.sw`
  - Contains enums that are used in `require(..., MyError::SomeError)` statements
  - The enums are split into individual errors e.g. `DepositError`, `OwnerError` etc.

    ```rust
    pub enum DepositError {
        IncorrectAsset: (),
        InsufficientAmount: (),
    }
    ```

- `events.sw`
  - Contains structs definitions which are used inside `log()` statements

    ```rust 
    log(Deposit { user, amount });
    ```

- `interface.sw`
  - Anything that may be exposed to the user e.g. the Application Binary Interface (`abi`) for your contract(s)
  - This means that the `events.sw` may not be necessary and that information can be held with the `abi`
- `main.sw`
  - The entry point to your contract that contains the implementation of your `abi`
- `utils.sw`
  - Any private functions (helper functions) that your contracts use inside their functions
  - Since a contract cannot call its own function we need to abstract away the code into some functions so that they can be used inside many functions of your contract

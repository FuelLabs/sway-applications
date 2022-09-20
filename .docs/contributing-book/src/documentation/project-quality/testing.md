# Testing

Testing is a large topic to cover therefore this section will only cover some points that are followed in the repository.

## File Separation

There are three components to the tests and they have the following structure.

```
tests/
├── functions/
|     └── 1 file per ABI function
├── utils/
|     └── mod.rs
└── harness.rs
```

### `functions`

The `functions` directory contains 1 file per function declared in the `ABI` and all **test cases** (not utility / helper functions) for that function are contained within that module. 

There are two possibilities with any function call and either the call succeeds or it reverts. For this reason each file is split into two sections:

- `success`
- `revert`

All of the tests where the function does not revert should be contained inside the `success` case while the reverting calls (panics) should be contained inside the `revert` module.

### `utils`

The `utils` directory contains utility functions and abstractions which allow the tests in the `functions` directory to remain small, clean and "DRY" (do not repeat yourself).

This can be achieved by separating content into files, internally creating modules in `mod.rs` or a mixture of both.

The repository follows the pattern of putting utility functions in `mod.rs` and separating them internally into `ABI` wrappers and test helpers. The `ABI` wrappers are functions which directly call the contract function with the arguments passed in while the test helpers are general utility functions such as creating a new contract instance for a new test etc.

### `harness.rs`

This file is the one that is called by `forc test` and thus it only contains the modules `functions` and `utils` in order to bring them into scope so that the tests can run.

## Testing Suggestions

<!-- TODO: this is a mess and it should be cleaned up and categorized at some point-->

Here are some tips on how to approach testing:

- Similar to [code structure](code-structure.md) content in each file should be ordered alphabetically, with one exception, so that it's easy to navigate
  - Test conditions in the order in which they may occur
    - If a test has multiple assertions then the first assertion should be tested first, second assertion second etc.
- Check the code coverage
  - All assertions & requirements should be tested
  - Check boundary conditions to see if the values passed in work throughout the entire range
- There should be positive and negative test cases meaning that a test should pass with correct data passed in but it should also revert when incorrect data is used
- When writing a test that changes state the test should first assert the initial condition before performing some operation and then testing the outcome of that operation
  - If the initial condition is not proven to be what is expected then there is no guarantee that the operation has performed the correct behavior
  - This also means that the initial condition should be compared to the post condition
- Comments should only be added to explain sections of each test if they provide insight into some complex behavior
  - If a function sets up the initial environment then there is no point in adding a comment "set up the environment" because the function name should be clear enough e.g. `fn setup()`
- Any tests that are ignored should be documented in the test so that the reader knows why something is currently unimplemented
  - Do not leave in commented out tests. `#[ignore]` them
- Unit tests should remain as unit tests
  - Do not bundle multiple different checks into one test unless it becomes semantically meaningless when separating them
- Checking that a behavior continues to work more than once may be necessary at times
  - If a user can deposit then there should be a test to see that they can deposit more than once

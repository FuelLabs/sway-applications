# Testing

There's a lot that can be said about testing so it's difficult to sum it all up in a few paragraphs.

Here are some pointers:

- Order your modules alphabetically
- Split your code up into two modules
  - The tests themselves
    - It's easier to navigate a file for tests when it only consists of the tests
  - All of the utility (helper) functions used for testing and keeping your tests DRY (do not repeat yourself)
    - Split the utilities into two modules
      - 1 module for your custom code such as creating wallets, generating setup parameters etc.
      - 1 module which wraps your abi calls
        - If you have an abi function `create_game(player_two: Player, player_one: Player) -> Game` then you should have a function which takes appropriate parameters and makes a call to `contract.create_game(player_two, player_one).call().await.unwrap()`
        - This allows you to call the same abi wrapper function by passing in the arguments instead of repeating any additional setup that you have in order to make the call
        - Note that this should only call your contract and return data rather than have complex code such as a `setup()` might have
- You should have a test module inside your test file for each abi call that can be made
  - Each test module should be split into 2
    - 1 `success` case where all of your tests work as expected
    - 1 `revert` case where you intentionally test your functions in order to make them break
- Ensure that your code coverage covers all possible cases
  - Check boundary conditions
  - Assertions
  - Calling things out of order
- When writing a test make sure to perform a "before" and "after" check to ensure that your code works as expected
  - Most people only check the "after" assertions however this may lead to your test passing falsely because
    - There is a bug in your test
    - There is a bug in your implementation
    - Your test and implementation may be correct however there is a bug in the code you are using
- The tests do not need to be in alphabetical order and instead they should be sequentially ordered in the way the code operates from start to finish
  - If you have 3 assertions to test then the first assertion should be at the top, the next test should be the next assertion and the last test should be the last assertion
- As long as you structure your code appropriately and name your tests well (reference the earlier sections) then you do not need to document your tests at all but comments are always nice when they are useful

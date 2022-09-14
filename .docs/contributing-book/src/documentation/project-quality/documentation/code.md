# Code

For guidance refer to how [Rust](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) documents code.

There are two ways to document your code

- Using `///` for official documentation
  - This allows tools to be used and parse out your documentation automatically
- Using `//` for comments
  - These are helper comments within the code to guide the developer who is reading your code
  - They do not get parsed out via automatic doc generation

Each function that is written should be documented (the `///`). The only exception applies to the functions inside the `impl` of your contract (notice that the example above does not have any documentation, that would be on the `abi` that is imported). The documentation for those functions should be on the `abi` because a developer will have access to your `abi` and not necessarily your implementation.

When writing documentation make sure that your arguments are in alphabetical order and any assertions that you have in your function are also documented in the same order. It makes it easier to read the code when you see everything in sequential order rather than searching for where an assertion is placed in the function.

Example

```rust
{{#include ../../../code/connect-four/src/interface.sw::36}}
```

In addition to documenting your functions make sure to document your data structures, events, errors etc. if needed. It is your job to explain to the reader what your code is an what it does so do not make the reader guess. It might be obvious to you but not to the reader.

That being said, there is good documentation and bad documentation.

Example

```rust
{{#include ../../../code/bad_documentation/src/lib.sw:3:19}}
```

Documenting your code does not stop at writing documentation. It is also important to name your variables, functions and data structures appropriately. This is a difficult task and it can be argued to be a skill and an art. If your name is too verbose, then it will be difficult to read (because of how much screenspace it takes up) and annoying to use. On the other hand, an abbreviated name requires insider knowledge to be able to infer what the variable is (you could document it extensively however that still means the reader must remember what the documentation says).

There are two general rules to follow:

- Do not abbreviate your names (or use acronyms) unless the abbreviation is used extensively in the area and it is easy to perform an online search and find out more information
  - Remember, if someone starts to abbreviate variables / uses notation that you are unfamiliar with then that forces you to start searching for the information and remembering what something represents
  - Wouldn't it be easier to just read a well named, unabbreviated name instead?
    - `temp` -> `temperature`
      - Do you mean "temporary"? In the context it might be easier to figure out but that requires context for understanding and thus the developer has failed to clearly and accurately convey their intent
    - `x` -> ???
      - What is this temporary, single character variable? Never do this, ever, unless you're dealing with something like an "x-coordinate" where it makes sense to use "x"
- Everything should be a statement instead of a question because statements can be true / false and thus the intent from the developer to the reader is clearer - something either is or is not and the developer is not conveying any uncertainty (this makes it easier to debug)
  - Some examples
    - `can_change` -> `authorized`
      - The "can" can be a question or a statement. Are you asking the reader or are you telling the reader?
    - `is_on` -> `enabled`
      - Similarly, "is" can be a question here rather than a simple declaration

There are many guides online indicating how to create meaningful names. If you are unsure how to name something then how is the reader meant to know what the variable is meant to represent?

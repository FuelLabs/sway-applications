# Code Structure

In order to structure your code there is only 1 rule that you must remember and that is that everything must always be ordered alphabetically - as long as there isn't a dependency error or a better name for a default field of an enum. The reason is that it provides a structure that almost everyone will be familiar with. This means that nobody has to learn how you have decided to structure your code and thus where to look for a variable, function, import etc. This makes it really easy to navigate your entire project and in turn navigate all projects without having to memorize the intricacies of each project.

To be explicit

- All dependencies must be declared in alphabetical order just below the declaration of the program type e.g. below `contract;`
- All imports must be in alphabetical order and any modules within those imports must be in alphabetical order
- All functions, structs, enums, fields inside structs and enums, parameters of a function, parameters in documentation etc. must be in alphabetical order in all files (this does not apply to variables inside a function)

Example

```rust
{{#include ../../code/connect-four/src/main.sw}}
```

The only other aspect to remember is to use the formatter before commiting your work.

- `cargo fmt` to format your `Rust` files (SDK tests)
- `forc fmt` to format your `Sway` files

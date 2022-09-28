# Code Structure

Structuring code in a way that is easy to navigate allows for a greater developer experience. In order to achieve this, there are some guidelines to consider.

1. Fields in all structures should be alphabetical
2. Functions should be declared by the weight of their purity e.g.
   1. `read & write` first
   2. `read` second
   3. `pure` last
3. Structures should be grouped into modules and the content inside should be alphabetically ordered
4. Dependencies and imports should be alphabetical
5. Document the parameters of the interface in alphabetical order and preferably declare them in the same order in the function signature

An important aspect to remember is to use the formatter(s) to format the code prior to a commit.

- `cargo fmt` to format `Rust` files
- `forc fmt` to format `Sway` files

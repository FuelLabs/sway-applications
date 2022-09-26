# Code Structure

Structuring code in a way that is easy to navigate allows for a greater developer experience. In order to achieve this, there are some guidelines to consider.

1. Fields in data structures should be alphabetical
2. Avoid ordering by "admin functions", "getters", etc.
   1. Group data structures by module and alphabetically order the content there
   2. Functions should be declared by the weight of their purity e.g.
      1. `read & write` first
      2. `read` second
      3. `pure` last
4. Dependencies and imports should be alphabetical
5. Document the parameters of the interface in alphabetical order and preferably declare them in the same order

An important aspect to remember is to use the formatter(s) to format the code prior to a commit.

- `cargo fmt` to format `Rust` files
- `forc fmt` to format `Sway` files

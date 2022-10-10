# Introduction

Repositories typically have a [CONTRIBUTING.md](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/setting-guidelines-for-repository-contributors) file which outlines the expectations when contributing to the project.

This project has moved from the markdown document to a webpage view through the use of [mdbook](https://rust-lang.github.io/mdBook/).

## Installation

If you wish to alter the documentation presented in this book then follow the following instructions.

1. Install [Rust](https://www.rust-lang.org/tools/install) if it's not installed.
2. Install [mdbook](https://rust-lang.github.io/mdBook/).

   ```bash
   cargo install mdbook
   ```

3. To [build](https://rust-lang.github.io/mdBook/cli/build.html) the book make sure that you are in `/.docs/contributing-book` and run

   ```bash
   mdbook build
   ```

4. To develop the book in real time, in the browser, run

   ```bash
   mdbook serve --open
   ```

## How to edit the book

Each page is written in markdown so there's not much to learn specifically for `mdbook` but you're free to read their documentation for additional information.

If you wish to add a new page then it must be listed in the [SUMMARY.md](src/SUMMARY.md).

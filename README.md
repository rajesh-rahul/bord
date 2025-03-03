# Bord - SQLite Language Server (WIP)

A work-in-progress language server for SQLite.

Some preliminary info:
- Contains an incremental, error-resilient, recursive descent parser for SQLite
  - Contains three different CST implementations (to find the most performant one)
- Uses Ungrammar to generate AST and provide autocomplete suggestions
  - The `sqlite.ungram` file is ensured to be in sync with the handwritten parser via tests
  - Enables context sensitive keyword completions (like in JetBrains DataGrip)
- Can run in the browser with WASM

## Credits

This project is made possible by studying other projects and resources, especially:

- matklad's:
  - [Resilient LL Parsing Tutorial](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)
    article
  - [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    article
  - [Explaining rust-analyzer](https://www.youtube.com/watch?v=I3RXottNwk0&list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) video series
- SQLite's:
  - [Documentation](https://www.sqlite.org/docs.html)
- Rust lemon-rs:
  - [Github Link](https://github.com/gwenn/lemon-rs)
- Projects that contain SQL code extracted from the official test suite:
  - https://github.com/bkiers/sqlite-parser/tree/master/src/test/resources
  - https://github.com/codeschool/sqlite-parser/tree/master/test/sql/official-suite
    - This project has a script that extracts SQL from the official test suite
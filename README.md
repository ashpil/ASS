# ASS - Actually Semantic Styles

ASS is a WIP markup language, designed to fill a similar role as HTML/CSS do today, but as one entity that concerns itself with both content and styling, while reducing boilerplate.

Head to [`/documents`](/documents) for more information.

## Install

[Install Rust](https://www.rust-lang.org/tools/install), and Cargo will handle everything else for you. Just do `cargo run` from the directory.

When the program is run, it will prompt you with a choice of either parsing the styling portion of the language or the markup portion of the language. Once you make your choice (either works), put in test.asml to parse an example of the markup langauge or test.ass for an example of the styling language. Note you could also try with your own program. The path to the input file is relative to the directory. The output will be an AST in the case of the markup language and a hashmap in the case of the styling language.

**Note that just in case Rust is difficult to install, we also included a binary(ass) which can be run: `./ass`**

## Running Tests

To run unit tests use `cargo test`.

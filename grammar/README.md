# leo-grammar

[![Crates.io](https://img.shields.io/crates/v/leo-grammar.svg?color=neon)](https://crates.io/crates/leo-grammar)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

## Command-line instructions

To generate an AST of the Leo program and save it as a JSON file , run:
```
leo_grammar {PATH/TO/INPUT_FILENAME}.leo {PATH/TO/OUTPUT_DIRECTORY (optional)}
```
If no output directory is provided, then the program will store the JSON file in the local working directory.

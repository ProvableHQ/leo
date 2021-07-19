# leo-package

[![Crates.io](https://img.shields.io/crates/v/leo-package.svg?color=neon)](https://crates.io/crates/leo-package)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

## Description

This module defines the structure of a Leo project package. And describes behavior of package internals, such
as Leo Manifest (Leo.toml), Lock File (Leo.lock), source files and imports. 

Mainly used by Leo binary.

## Structure

Each directory in this crate mirrors a corresponding file generated in a new Leo project package:

```
package/src
├── errors  # crate level error definitions
├── imports # program imports management
├── inputs  # program inputs directory
├── outputs # program outputs directory
├── root    # program root: Leo.toml, Leo.lock 
└── source  # source files directory
```

## Testing

Package features functional tests in the `tests/` directory.

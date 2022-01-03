// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.
#![allow(clippy::upper_case_acronyms)]

//! The `README.md` file.

use crate::PackageFile;

use serde::Deserialize;

pub static README_FILENAME: &str = "README.md";

#[derive(Deserialize)]
pub struct README {
    pub package_name: String,
}

impl README {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }
}

impl PackageFile for README {
    type ParentDirectory = super::RootDirectory;

    fn template(&self) -> String {
        format!(
            r"# {}

## Build Guide

To compile this Leo program, run:
```bash
leo build
```

To test this Leo program, run:
```bash
leo test
```

## Development

To output the number of constraints, run:
```bash
leo build -d
```
",
            self.package_name
        )
    }
}

impl std::fmt::Display for README {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "README.md")
    }
}

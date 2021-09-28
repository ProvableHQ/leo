// Copyright (C) 2019-2021 Aleo Systems Inc.
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

#![doc = include_str!("../README.md")]

/* pub mod common;
pub use common::*;

pub mod prelude;
pub use prelude::*;
 */
use leo_ast::Program;
use leo_errors::{ImportError, Result};

#[macro_use]
extern crate include_dir;

use include_dir::Dir;
use indexmap::IndexMap;

static STDLIB: Dir = include_dir!(".");

fn resolve_file(file: &str, mapping: Option<&str>) -> Result<Program> {
    let resolved = STDLIB
        .get_file(&file)
        .ok_or_else(|| ImportError::no_such_stdlib_file(file))?
        .contents_utf8()
        .ok_or_else(|| ImportError::failed_to_read_stdlib_file(file))?;

    let ast = leo_parser::parse_ast(&file, resolved)?.into_repr();
    ast.set_core_mapping(mapping);

    Ok(ast)
}

pub fn resolve_prelude_modules() -> Result<IndexMap<Vec<String>, Program>> {
    let mut preludes: IndexMap<Vec<String>, Program> = IndexMap::new();

    for module in STDLIB.find("prelude/*.leo").unwrap() {
        // If on windows repalce \\ with / as all paths are stored in unix style.
        let path = module.path().to_str().unwrap_or("").replace("\\", "/");
        let program = resolve_file(&path, None)?;

        let removed_extension = path.replace(".leo", "");
        let mut parts: Vec<String> = vec![String::from("std")];
        parts.append(
            &mut removed_extension
                .split('/')
                .map(str::to_string)
                .collect::<Vec<String>>(),
        );
        preludes.insert(parts, program);
    }

    Ok(preludes)
}

pub fn resolve_stdlib_module(module: &str) -> Result<Program> {
    let mut file_path = module.replace(".", "/");
    file_path.push_str(".leo");

    let mapping = if module == "unstable.blake2s" {
        Some("blake2s")
    } else {
        None
    };

    resolve_file(&file_path, mapping)
}

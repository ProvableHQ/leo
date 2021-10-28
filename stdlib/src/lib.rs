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

use leo_ast::Program;
use leo_errors::{ImportError, Result};

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
use indexmap::IndexMap;

// TODO replace with a macro like include_dir crate but keep deterministic order
lazy_static! {
    static ref STDLIB: Mutex<IndexMap<String, String>> = Mutex::new(IndexMap::new());
}

static PRELUDE: &[&str] = &[
    "prelude/address.leo",
    "prelude/bool.leo",
    "prelude/char.leo",
    "prelude/field.leo",
    "prelude/group.leo",
    "prelude/i8.leo",
    "prelude/i16.leo",
    "prelude/i32.leo",
    "prelude/i64.leo",
    "prelude/i128.leo",
    "prelude/u8.leo",
    "prelude/u16.leo",
    "prelude/u32.leo",
    "prelude/u64.leo",
    "prelude/u128.leo",
];

pub fn static_include_stdlib() {
    // not iterating through prelude above since include_macro require explicit context
    let mut stdlib = STDLIB.lock().unwrap();
    stdlib.insert(
        "prelude/address.leo".to_string(),
        include_str!("./../prelude/address.leo").to_string(),
    );
    stdlib.insert(
        "prelude/bool.leo".to_string(),
        include_str!("./../prelude/bool.leo").to_string(),
    );
    stdlib.insert(
        "prelude/char.leo".to_string(),
        include_str!("./../prelude/char.leo").to_string(),
    );
    stdlib.insert(
        "prelude/field.leo".to_string(),
        include_str!("./../prelude/field.leo").to_string(),
    );
    stdlib.insert(
        "prelude/group.leo".to_string(),
        include_str!("./../prelude/group.leo").to_string(),
    );
    stdlib.insert(
        "prelude/i8.leo".to_string(),
        include_str!("./../prelude/i8.leo").to_string(),
    );
    stdlib.insert(
        "prelude/i16.leo".to_string(),
        include_str!("./../prelude/i16.leo").to_string(),
    );
    stdlib.insert(
        "prelude/i32.leo".to_string(),
        include_str!("./../prelude/i32.leo").to_string(),
    );
    stdlib.insert(
        "prelude/i64.leo".to_string(),
        include_str!("./../prelude/i64.leo").to_string(),
    );
    stdlib.insert(
        "prelude/i128.leo".to_string(),
        include_str!("./../prelude/i128.leo").to_string(),
    );
    stdlib.insert(
        "prelude/u8.leo".to_string(),
        include_str!("./../prelude/u8.leo").to_string(),
    );
    stdlib.insert(
        "prelude/u16.leo".to_string(),
        include_str!("./../prelude/u16.leo").to_string(),
    );
    stdlib.insert(
        "prelude/u32.leo".to_string(),
        include_str!("./../prelude/u32.leo").to_string(),
    );
    stdlib.insert(
        "prelude/u64.leo".to_string(),
        include_str!("./../prelude/u64.leo").to_string(),
    );
    stdlib.insert(
        "prelude/u128.leo".to_string(),
        include_str!("./../prelude/u128.leo").to_string(),
    );

    stdlib.insert(
        "unstable/blake2s.leo".to_string(),
        include_str!("./../unstable/blake2s.leo").to_string(),
    );
}

fn resolve_file(file: &str, mapping: bool) -> Result<Program> {
    let stdlib = STDLIB.lock().unwrap();

    let resolved = stdlib
        .get(&file.to_string())
        .ok_or_else(|| ImportError::no_such_stdlib_file(file))?;

    let ast = leo_parser::parse_ast(file, resolved)?.into_repr();
    if mapping {
        ast.set_core_mapping();
    }

    Ok(ast)
}

pub fn resolve_prelude_modules() -> Result<IndexMap<Vec<String>, Program>> {
    let mut preludes: IndexMap<Vec<String>, Program> = IndexMap::new();

    for path in PRELUDE.iter() {
        // If on windows repalce \\ with / as all paths are stored in unix style.
        let program = resolve_file(path, true)?;

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

    resolve_file(&file_path, true)
}

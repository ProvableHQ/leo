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

use crate::{get_output, EdwardsTestCompiler};

use std::{env::current_dir, path::PathBuf};

static MAIN_FILE_NAME: &str = "tests/compiler/main.leo";

// Compiler tests rely on knowledge of local directories. They should be run locally only.

#[test]
#[ignore]
fn test_parse_program_from_string() {
    // Parse program from string with compiler.
    let program_string = include_str!("main.leo");
    let mut compiler_no_path = EdwardsTestCompiler::new("".to_string(), PathBuf::new(), PathBuf::new());

    compiler_no_path.parse_program_from_string(program_string).unwrap();

    // Parse program from file path with compiler.
    let mut local = current_dir().unwrap();
    local.push(MAIN_FILE_NAME);

    let compiler_with_path =
        EdwardsTestCompiler::parse_program_without_input("".to_string(), local, PathBuf::new()).unwrap();

    // Compare output bytes.
    let expected_output = get_output(compiler_no_path);
    let actual_output = get_output(compiler_with_path);

    assert_eq!(expected_output, actual_output);
}

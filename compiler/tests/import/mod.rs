// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{assert_satisfied, parse_program};

use std::env::{current_dir, set_current_dir};

static TEST_SOURCE_DIRECTORY: &str = "tests/import";

// Import tests rely on knowledge of local directories. They should be run locally only.

pub fn set_local_dir() {
    let mut local = current_dir().unwrap();
    local.push(TEST_SOURCE_DIRECTORY);

    set_current_dir(local).unwrap();
}

#[test]
#[ignore]
fn test_basic() {
    set_local_dir();

    let bytes = include_str!("basic.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_multiple() {
    set_local_dir();

    let bytes = include_str!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_star() {
    set_local_dir();

    let bytes = include_str!("star.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_star_fail() {
    set_local_dir();

    let bytes = include_str!("star_fail.leo");
    assert!(parse_program(bytes).is_err());
}

#[test]
#[ignore]
fn test_alias() {
    set_local_dir();

    let bytes = include_str!("alias.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// naming tests
#[test]
#[ignore]
fn test_names_pass() {
    set_local_dir();

    let bytes = include_str!("names.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_names_fail_1() {
    set_local_dir();

    let bytes = include_str!("names_dash_a.leo");
    assert!(parse_program(bytes).is_err());
}

#[test]
#[ignore]
fn test_names_fail_2() {
    set_local_dir();

    let bytes = include_str!("names_a_dash.leo");
    assert!(parse_program(bytes).is_err());
}

#[test]
#[ignore]
fn test_names_fail_3() {
    set_local_dir();

    let bytes = include_str!("names_underscore.leo");
    assert!(parse_program(bytes).is_err());
}

#[test]
#[ignore]
fn test_names_fail_4() {
    set_local_dir();

    let bytes = include_str!("names_dollar.leo");
    assert!(parse_program(bytes).is_err());
}

// more complex tests
#[test]
#[ignore]
fn test_many_import() {
    set_local_dir();

    let bytes = include_str!("many_import.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_many_import_star() {
    set_local_dir();

    let bytes = include_str!("many_import_star.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

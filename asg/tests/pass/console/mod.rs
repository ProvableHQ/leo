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

use crate::{load_asg};

#[test]
fn test_log() {
    let program_string = include_str!("log.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_log_parameter() {
    let program_string = include_str!("log_parameter.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_log_parameter_many() {
    let program_string = include_str!("log_parameter_many.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_log_input() {
    let program_string = include_str!("log_input.leo");
    load_asg(program_string).unwrap();
}

// Debug

#[test]
fn test_debug() {
    let program_string = include_str!("debug.leo");
    load_asg(program_string).unwrap();
}

// Error

#[test]
fn test_error() {
    let program_string = include_str!("error.leo");
    load_asg(program_string).unwrap();
}

// Assertion

#[test]
fn test_assert() {
    let program_string = include_str!("assert.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_conditional_assert() {
    let program_string = include_str!("conditional_assert.leo");
    load_asg(program_string).unwrap();
}

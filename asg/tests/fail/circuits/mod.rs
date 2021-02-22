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

use leo_asg::new_context;

use crate::load_asg;

// Expressions

#[test]
fn test_inline_fail() {
    let program_string = include_str!("inline_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_inline_undefined() {
    let program_string = include_str!("inline_undefined.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

// Members

#[test]
fn test_member_variable_fail() {
    let program_string = include_str!("member_variable_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_member_function_fail() {
    let program_string = include_str!("member_function_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_member_function_invalid() {
    let program_string = include_str!("member_function_invalid.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mut_member_function_fail() {
    let program_string = r#"
        circuit Foo {
            function echo(mut self, x: u32) -> u32 {
                return x
            }
        }
        
        function main() {
            let a = Foo { };
        
            console.assert(a.echo(1u32) == 1u32);
        }"#;
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_member_static_function_invalid() {
    let program_string = include_str!("member_static_function_invalid.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_member_static_function_undefined() {
    let program_string = include_str!("member_static_function_undefined.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

// Mutability

#[test]
fn test_mutate_function_fail() {
    let program_string = include_str!("mut_function_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mutate_self_variable_fail() {
    let program_string = include_str!("mut_self_variable_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mutate_self_function_fail() {
    let program_string = include_str!("mut_self_function_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mutate_self_static_function_fail() {
    let program_string = include_str!("mut_self_static_function_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mutate_static_function_fail() {
    let program_string = include_str!("mut_static_function_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_mutate_variable_fail() {
    let program_string = include_str!("mut_variable_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

// Self

#[test]
fn test_self_fail() {
    let program_string = include_str!("self_fail.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_self_member_invalid() {
    let program_string = include_str!("self_member_invalid.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

#[test]
fn test_self_member_undefined() {
    let program_string = include_str!("self_member_undefined.leo");
    load_asg(&new_context(), program_string).err().unwrap();
}

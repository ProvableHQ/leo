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

use crate::load_asg;

#[test]
fn test_empty() {
    let program_string = include_str!("empty.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_iteration() {
    let program_string = include_str!("iteration.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_const_args() {
    let program_string = r#"
    function one(const value: u32) -> u32 {
        return value + 1;
    }
    
    function main() {
        let a = 0u32;
    
        for i in 0..10 {
            a += one(i);
        }
    
        console.assert(a == 20u32);
    }
    "#;
    load_asg(program_string).unwrap();
}

#[test]
fn test_const_args_used() {
    let program_string = r#"
    function index(arr: [u8; 3], const value: u32) -> u8 {
        return arr[value];
    }
    
    function main() {
        let a = 0u8;
        const arr = [1u8, 2, 3];
    
        for i in 0..3 {
            a += index(arr, i);
        }
    
        console.assert(a == 6u8);
    }
    "#;
    load_asg(program_string).unwrap();
}

#[test]
fn test_const_args_fail() {
    let program_string = r#"
    function index(arr: [u8; 3], const value: u32) -> u8 {
        return arr[value];
    }
    
    function main(x_value: u32) {
        let a = 0u8;
        const arr = [1u8, 2, 3];
    
        a += index(arr, x_value);
    
        console.assert(a == 1u8);
    }
    "#;
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_iteration_repeated() {
    let program_string = include_str!("iteration_repeated.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_newlines() {
    let program_string = include_str!("newlines.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_multiple_returns() {
    let program_string = include_str!("multiple_returns.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_multiple_returns_main() {
    let program_string = include_str!("multiple_returns_main.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_repeated_function_call() {
    let program_string = include_str!("repeated.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_return() {
    let program_string = include_str!("return.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_undefined() {
    let program_string = include_str!("undefined.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_value_unchanged() {
    let program_string = include_str!("value_unchanged.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_array_input() {
    let program_string = include_str!("array_input.leo");
    load_asg(program_string).err().unwrap();
}

// Test return multidimensional arrays

#[test]
fn test_return_array_nested_pass() {
    let program_string = include_str!("return_array_nested_pass.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_return_array_tuple_pass() {
    let program_string = include_str!("return_array_tuple_pass.leo");
    load_asg(program_string).unwrap();
}

// Test return tuples

#[test]
fn test_return_tuple() {
    let program_string = include_str!("return_tuple.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_return_tuple_conditional() {
    let program_string = include_str!("return_tuple_conditional.leo");
    load_asg(program_string).unwrap();
}

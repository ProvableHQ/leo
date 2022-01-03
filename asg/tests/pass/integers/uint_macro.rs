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

macro_rules! test_uint {
    ($name: ident) => {
        pub struct $name {}

        impl super::IntegerTester for $name {
            fn test_min() {
                let program_string = include_str!("min.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_max() {
                let program_string = include_str!("max.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_add() {
                let program_string = include_str!("add.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_sub() {
                let program_string = include_str!("sub.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_mul() {
                let program_string = include_str!("mul.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_div() {
                let program_string = include_str!("div.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_pow() {
                let program_string = include_str!("pow.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_eq() {
                let program_string = include_str!("eq.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_ne() {
                let program_string = include_str!("ne.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_ge() {
                let program_string = include_str!("ge.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_gt() {
                let program_string = include_str!("gt.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_le() {
                let program_string = include_str!("le.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_lt() {
                let program_string = include_str!("lt.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_console_assert() {
                let program_string = include_str!("console_assert.leo");
                crate::load_asg(program_string).unwrap();
            }

            fn test_ternary() {
                let program_string = include_str!("ternary.leo");
                crate::load_asg(program_string).unwrap();
            }
        }
    };
}

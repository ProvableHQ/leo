// Copyright (C) 2019-2023 Aleo Systems Inc.
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

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod code_generation;
pub use code_generation::*;

pub mod common;
pub use common::*;

pub mod dead_code_elimination;
pub use dead_code_elimination::*;

pub mod destructuring;
pub use destructuring::*;

pub mod flattening;
pub use flattening::*;

pub mod function_inlining;
pub use function_inlining::*;

pub mod loop_unrolling;
pub use self::loop_unrolling::*;

pub mod pass;
pub use self::pass::*;

pub mod static_single_assignment;
pub use static_single_assignment::*;

pub mod symbol_table_creation;
pub use symbol_table_creation::*;

pub mod type_checking;
pub use type_checking::*;

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

pub mod flattening;
pub use flattening::*;

pub mod loop_unrolling;
pub use self::loop_unrolling::*;

pub mod pass;
pub use self::pass::*;

pub mod static_single_assignment;
pub use static_single_assignment::*;

pub mod symbol_table_creator;
pub use symbol_table_creator::*;

pub mod type_checking;
pub use type_checking::*;

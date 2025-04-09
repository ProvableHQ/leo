// Copyright (C) 2019-2025 Provable Inc.
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

mod static_analysis;
pub use static_analysis::*;

mod code_generation;
pub use code_generation::*;

mod common;
pub use common::*;

mod const_propagation;
pub use const_propagation::*;

mod const_propagation_and_unrolling;
pub use const_propagation_and_unrolling::*;

mod dead_code_elimination;
pub use dead_code_elimination::*;

mod destructuring;
pub use destructuring::*;

mod flattening;
pub use flattening::*;

mod function_inlining;
pub use function_inlining::*;

mod loop_unrolling;
pub use loop_unrolling::*;

mod pass;
pub use pass::*;

mod static_single_assignment;
pub use static_single_assignment::*;

mod symbol_table_creation;
pub use symbol_table_creation::*;

mod type_checking;
pub use type_checking::*;

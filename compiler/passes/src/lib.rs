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

mod common_subexpression_elimination;
pub use common_subexpression_elimination::*;

mod const_propagation;
pub use const_propagation::*;

mod const_prop_unroll_and_morphing;
pub use const_prop_unroll_and_morphing::*;

mod dead_code_elimination;
pub use dead_code_elimination::*;

mod destructuring;
pub use destructuring::*;

mod disambiguate;
pub use disambiguate::*;

mod flattening;
pub use flattening::*;

mod function_inlining;
pub use function_inlining::*;

mod global_items_collection;
pub use global_items_collection::*;

mod global_vars_collection;
pub use global_vars_collection::*;

mod loop_unrolling;
pub use loop_unrolling::*;

mod monomorphization;
pub use monomorphization::*;

mod option_lowering;
pub use option_lowering::*;

mod path_resolution;
pub use path_resolution::*;

mod pass;
pub use pass::*;

mod processing_async;
pub use processing_async::*;

mod processing_script;
pub use processing_script::*;

mod remove_unreachable;
pub use remove_unreachable::*;

mod static_single_assignment;
pub use static_single_assignment::*;

mod ssa_const_propagation;
pub use ssa_const_propagation::*;

mod storage_lowering;
pub use storage_lowering::*;

mod type_checking;
pub use type_checking::*;

mod name_validation;
pub use name_validation::*;

mod write_transforming;
pub use write_transforming::*;

#[cfg(test)]
mod test_passes;

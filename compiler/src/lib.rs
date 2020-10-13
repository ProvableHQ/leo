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

//! Module containing structs and types that make up a Leo program.

#![allow(clippy::module_inception)]

#[macro_use]
extern crate thiserror;

pub mod compiler;

pub mod console;
pub use self::console::*;

pub mod constraints;
pub use self::constraints::*;

pub mod definition;

pub mod errors;

pub mod expression;
pub use self::expression::*;

pub mod function;
pub use self::function::*;

pub mod import;
pub use self::import::*;

pub mod output;
pub use self::output::*;

pub mod program;
pub use self::program::*;

pub mod statement;
pub use self::statement::*;

pub mod value;
pub use self::value::*;

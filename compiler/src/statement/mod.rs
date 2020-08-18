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

//! Methods to enforce constraints on statements in a Leo program.

pub mod assert;
pub use self::assert::*;

pub mod assign;
pub use self::assign::*;

pub mod branch;
pub use self::branch::*;

pub mod conditional;
pub use self::conditional::*;

pub mod definition;
pub use self::definition::*;

pub mod iteration;
pub use self::iteration::*;

pub mod return_;
pub use self::return_::*;

pub mod statement;
pub use self::statement::*;

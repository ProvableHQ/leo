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

//! This module contains both a Director to reconstruct the AST
//! which maps over ever node of the AST and calls a reducer.
//! The Trait for a reducer are methods that can be overridden
//! to make changes to how AST nodes are rebuilt.

pub mod reconstructing_reducer;
pub use reconstructing_reducer::*;

pub mod reconstructing_director;
pub use reconstructing_director::*;

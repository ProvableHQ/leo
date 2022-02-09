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

//! Methods to enforce function input variables in a compiled Leo program.

pub mod array;
pub use self::array::*;

pub mod main_function_input;
pub use self::main_function_input::*;

pub mod input_keyword;
pub use self::input_keyword::*;

pub mod input_section;
pub use self::input_section::*;

pub mod tuple;
pub use self::tuple::*;

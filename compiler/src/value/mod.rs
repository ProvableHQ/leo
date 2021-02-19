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

//! Methods to enforce constraints on values in a Leo program.

pub mod address;
pub use self::address::*;

pub mod boolean;

pub mod field;
pub use self::field::*;

pub mod group;
pub use self::group::*;

pub mod integer;
pub use self::integer::*;

pub mod value;
pub use self::value::*;

pub(crate) fn number_string_typing(number: &str) -> (String, bool) {
    let first_char = number.chars().next().unwrap();

    // Check if first symbol is a negative.
    // If so strip it, parse rest of string and then negate it.
    if first_char == '-' {
        let uint = number.chars().next().map(|c| &number[c.len_utf8()..]).unwrap_or("");
        (uint.to_string(), true)
    } else {
        (number.to_string(), false)
    }
}

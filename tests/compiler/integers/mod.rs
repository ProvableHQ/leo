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

#[macro_use]
pub mod int_macro;

#[macro_use]
pub mod uint_macro;

pub mod integer_tester;
pub use self::integer_tester::*;

// must be below macro definitions!
pub mod u128;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

pub mod i128;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod i8;

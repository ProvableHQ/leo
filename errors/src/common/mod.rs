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

pub mod backtraced;
pub use self::backtraced::*;

pub mod formatted;
pub use self::formatted::*;

#[macro_use]
pub mod macros;
pub use self::macros::*;

pub mod span;
pub use self::span::*;

pub mod tendril_json;
pub use self::tendril_json::*;

pub mod traits;
pub use self::traits::*;

// Can make the args cleaner once https://github.com/rust-lang/rust/issues/41517 or https://github.com/rust-lang/rust/issues/63063 hits stable.
// pub(crate) type DisplayArg = impl std::fmt::Display;
// pub(crate) type DebugArg = impl std::fmt::Debug;
// pub(crate) type ErrorArg = impl std::error::Error;

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

/// This module contains a backtraced error and its methods.
pub mod backtraced;
pub use self::backtraced::*;

/// This module contains a formatted error and its methods.
pub mod formatted;
pub use self::formatted::*;

/// This module contains the macros for making errors easily.
#[macro_use]
pub mod macros;
pub use self::macros::*;

/// This module contains traits for making errors easily.
pub mod traits;
pub use self::traits::*;

// Right now for cleanliness of calling error functions we say each argument implments one of the follow types rather than giving a specific type.
// This allows us to just pass many types rather doing conversions cleaning up the code.
// The args can be made cleaneronce https://github.com/rust-lang/rust/issues/41517 or https://github.com/rust-lang/rust/issues/63063 hits stable.
// Either of why would allows to generate a type alias for these trait implmenting types.
// pub(crate) type DisplayArg = impl std::fmt::Display;
// pub(crate) type DebugArg = impl std::fmt::Debug;
// pub(crate) type ErrorArg = impl std::error::Error;

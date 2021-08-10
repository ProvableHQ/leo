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

/// ErrorCode trait that all Errors should implement.
pub trait ErrorCode: Sized {
    /// Returns the error's exit code for the program.
    fn exit_code(&self) -> i32;

    /// Returns the error's exit code mask, as to avoid conflicts.
    fn exit_code_mask() -> i32;

    /// Returns the error's code type for the program.
    fn error_type() -> String;
}

/// The LeoErrorCode which has a code identifier of 037(Leo upsidedown and backwards).
/// This is to make the exit codes unique to Leo itself.
pub trait LeoErrorCode: ErrorCode {
    /// Inlined function for efficiency.
    #[inline(always)]
    fn code_identifier() -> i8 {
        37
    }
}

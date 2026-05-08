// Copyright (C) 2019-2026 Provable Inc.
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

/// Contains the Constant Evaluation error definitions.
mod const_eval;
pub use self::const_eval::*;

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Debug, Error)]
pub enum LeoError {
    #[error(transparent)]
    Formatted(#[from] crate::Formatted),
    #[error(transparent)]
    Backtraced(#[from] crate::Backtraced),
    #[error(transparent)]
    ConstEvalError(#[from] ConstEvalError),
    #[error("")]
    LastErrorCode(i32),
    #[error(transparent)]
    SnarkVM(#[from] anyhow::Error),
}

impl LeoError {
    pub fn error_code(&self) -> String {
        use LeoError::*;
        match self {
            Formatted(e) => e.error_code(),
            Backtraced(e) => e.error_code(),
            ConstEvalError(_) => "Const Eval Error".to_string(),
            LastErrorCode(_) => unreachable!(),
            SnarkVM(_) => "SnarkVM Error".to_string(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        use LeoError::*;
        match self {
            Formatted(e) => e.exit_code(),
            Backtraced(e) => e.exit_code(),
            ConstEvalError(_) => 1,
            LastErrorCode(code) => *code,
            SnarkVM(_) => 11000,
        }
    }
}

/// The LeoWarning type that contains all sub warning types.
#[derive(Debug, Error)]
pub enum LeoWarning {
    #[error(transparent)]
    Formatted(#[from] crate::Formatted),
}

impl LeoWarning {
    pub fn error_code(&self) -> String {
        use LeoWarning::*;
        match self {
            Formatted(w) => w.warning_code(),
        }
    }
}

/// A global result type for all Leo crates, that defaults the errors to be a LeoError.
pub type Result<T, E = LeoError> = core::result::Result<T, E>;

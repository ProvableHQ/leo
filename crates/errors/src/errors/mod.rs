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

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Debug, Error)]
pub enum LeoError {
    #[error(transparent)]
    Formatted(#[from] crate::Formatted),
    #[error(transparent)]
    Backtraced(#[from] crate::Backtraced),
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
            LastErrorCode(_) => unreachable!(),
            SnarkVM(_) => "SnarkVM Error".to_string(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        use LeoError::*;
        match self {
            Formatted(e) => e.exit_code(),
            Backtraced(e) => e.exit_code(),
            LastErrorCode(code) => *code,
            SnarkVM(_) => 11000,
        }
    }

    /// Borrow a structured, LSP-agnostic view of this error.
    ///
    /// Only [`LeoError::Formatted`] variants carry a span and labeled
    /// secondary information, so other variants return `None`. The caller is
    /// expected to fall back to a synthetic package-level diagnostic in that
    /// case; see the `leo-lsp` diagnostics module for an example.
    ///
    /// [`LeoError::LastErrorCode`] is a sentinel used to signal that an error
    /// has already been emitted through a handler. It deliberately returns
    /// `None` so it is never published as a separate diagnostic.
    pub fn diagnostic_view(&self) -> Option<crate::DiagnosticView<'_>> {
        match self {
            LeoError::Formatted(formatted) => Some(formatted.diagnostic_view()),
            LeoError::Backtraced(_) | LeoError::LastErrorCode(_) | LeoError::SnarkVM(_) => None,
        }
    }

    /// Return whether this error is the sentinel raised after a handler emit.
    ///
    /// `Handler::last_err` produces [`LeoError::LastErrorCode`] when the
    /// emitter has already buffered a real diagnostic. Consumers should skip
    /// the sentinel when collecting structured diagnostics so the original
    /// error is published exactly once.
    pub fn is_last_error_code(&self) -> bool {
        matches!(self, LeoError::LastErrorCode(_))
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

    /// Borrow a structured, LSP-agnostic view of this warning.
    ///
    /// Every variant currently wraps a [`Formatted`] payload, so the view is
    /// always available. The signature is kept symmetric with
    /// [`LeoError::diagnostic_view`] so future variants that lack span data
    /// can opt out without breaking call sites.
    pub fn diagnostic_view(&self) -> Option<crate::DiagnosticView<'_>> {
        match self {
            LeoWarning::Formatted(formatted) => Some(formatted.diagnostic_view()),
        }
    }
}

/// A global result type for all Leo crates, that defaults the errors to be a LeoError.
pub type Result<T, E = LeoError> = core::result::Result<T, E>;

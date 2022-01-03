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

/// A macro that given an enum, exit code mask, error code string prefix,
/// and error methods generated through a DSL creates and generates errors
/// with a unique error code.
#[macro_export]
macro_rules! create_errors {
    (@step $code:expr,) => {
        #[inline(always)]
        // Returns the number of unique exit codes that this error type can take on.
        pub fn num_exit_codes() -> i32 {
            $code
        }
    };
    ($(#[$error_type_docs:meta])* $error_type:ident, exit_code_mask: $exit_code_mask:expr, error_code_prefix: $error_code_prefix:expr, $($(#[$docs:meta])* @$formatted_or_backtraced_list:ident $names:ident { args: ($($arg_names:ident: $arg_types:ty$(,)?)*), msg: $messages:expr, help: $helps:expr, })*) => {
        #[allow(unused_imports)] // Allow unused for errors that only use formatted or backtraced errors.
        use crate::{BacktracedError, FormattedError, LeoErrorCode, Span};

        use backtrace::Backtrace;

        // Generates the enum and implements from FormattedError and BacktracedErrors.
        #[derive(Debug, Error)]
        $(#[$error_type_docs])*
        pub enum $error_type {
            #[error(transparent)]
            FormattedError(#[from] FormattedError),

	        #[error(transparent)]
            BacktracedError(#[from] BacktracedError),
        }

        /// Implements the trait for LeoError Codes.
        impl LeoErrorCode for $error_type {
            #[inline(always)]
            fn exit_code(&self) -> i32 {
                match self {
                    Self::FormattedError(formatted) => formatted.exit_code(),
                    Self::BacktracedError(backtraced) => backtraced.exit_code()
                }
            }

            #[inline(always)]
            fn error_code(&self) -> String {
                match self {
                    Self::FormattedError(formatted) => formatted.error_code(),
                    Self::BacktracedError(backtraced) => backtraced.error_code()
                }
            }

            #[inline(always)]
            fn exit_code_mask() -> i32 {
                $exit_code_mask
            }

            #[inline(always)]
            fn error_type() -> String {
                $error_code_prefix.to_string()
            }
        }


        // Steps over the list of functions with an initial error code of 0.
        impl $error_type {
            create_errors!(@step 0i32, $(($(#[$docs])* $formatted_or_backtraced_list, $names($($arg_names: $arg_types,)*), $messages, $helps),)*);
        }
    };
    // Matches the function if it is a formatted error.
    (@step $code:expr, ($(#[$error_func_docs:meta])* formatted, $error_name:ident($($arg_names:ident: $arg_types:ty,)*), $message:expr, $help:expr), $(($(#[$docs:meta])* $formatted_or_backtraced_tail:ident, $names:ident($($tail_arg_names:ident: $tail_arg_types:ty,)*), $messages:expr, $helps:expr),)*) => {
        // Formatted errors always takes a span.
        $(#[$error_func_docs])*
        // Expands additional arguments for the error defining function.
        pub fn $error_name($($arg_names: $arg_types,)* span: &Span) -> Self {
            Self::FormattedError(
                FormattedError::new_from_span(
                    $message,
                    $help,
                    $code + Self::exit_code_mask(),
                    Self::code_identifier(),
                    Self::error_type(),
                    span,
                    // Each function always generates its own backtrace for backtrace clarity to originate from the error function.
                    Backtrace::new(),
                )
            )
        }

        // Steps the error code value by one and calls on the rest of the functions.
        create_errors!(@step $code + 1i32, $(($(#[$docs])* $formatted_or_backtraced_tail, $names($($tail_arg_names: $tail_arg_types,)*), $messages, $helps),)*);
    };
    // matches the function if it is a backtraced error.
    (@step $code:expr, ($(#[$error_func_docs:meta])* backtraced, $error_name:ident($($arg_names:ident: $arg_types:ty,)*), $message:expr, $help:expr), $(($(#[$docs:meta])* $formatted_or_backtraced_tail:ident, $names:ident($($tail_arg_names:ident: $tail_arg_types:ty,)*), $messages:expr, $helps:expr),)*) => {
        $(#[$error_func_docs])*
        // Expands additional arguments for the error defining function.
        pub fn $error_name($($arg_names: $arg_types,)*) -> Self {
            Self::BacktracedError(
                BacktracedError::new_from_backtrace(
                    $message,
                    $help,
                    $code + Self::exit_code_mask(),
                    Self::code_identifier(),
                    Self::error_type(),
                    // Each function always generates its own backtrace for backtrace clarity to originate from the error function.
                    Backtrace::new(),
                )
            )
        }

        // Steps the error code value by one and calls on the rest of the functions.
        create_errors!(@step $code + 1i32, $(($(#[$docs])* $formatted_or_backtraced_tail, $names($($tail_arg_names: $tail_arg_types,)*), $messages, $helps),)*);
    };
}

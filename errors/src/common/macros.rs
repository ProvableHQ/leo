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

#[macro_export]
macro_rules! create_errors {
    (@step $_code:expr,) => {};
    ($error_type:ident, exit_code_mask: $exit_code_mask:expr, error_code_prefix: $error_code_prefix:expr, $($names:ident { args: ($($arg_names:ident: $arg_types:ty$(,)?)*), msg: $messages:expr, help: $helps:expr, })*) => {
        use crate::{ErrorCode, FormattedError, LeoErrorCode, Span};

        #[derive(Debug, Error)]
        pub enum $error_type {
            #[error(transparent)]
            FormattedError(#[from] FormattedError),
        }

        impl LeoErrorCode for $error_type {}

        impl ErrorCode for $error_type {
            #[inline(always)]
            fn exit_code_mask() -> u32 {
                $exit_code_mask
            }
        
            #[inline(always)]
            fn error_type() -> String {
                $error_code_prefix.to_string()
            }
        
            fn new_from_span<T>(message: T, help: Option<String>, exit_code: u32, span: &Span) -> Self
            where T: ToString {
                Self::FormattedError(FormattedError::new_from_span(
                        message.to_string(),
                        help,
                        exit_code + Self::exit_code_mask(),
                        Self::code_identifier(),
                        Self::error_type(),
                        span,
                ))
            } 
        }
        
        impl $error_type {
            create_errors!(@step 0u32, $(($names($($arg_names: $arg_types,)*), $messages, $helps),)*);
        }
    };
    (@step $code:expr, ($error_name:ident($($arg_names:ident: $arg_types:ty,)*), $message:expr, $help:expr), $(($names:ident($($tail_arg_names:ident: $tail_arg_types:ty,)*), $messages:expr, $helps:expr),)*) => {
        pub fn $error_name($($arg_names: $arg_types,)* span: &Span) -> Self {
            
            Self::new_from_span($message, $help, $code, span)
        }

        create_errors!(@step $code + 1u32, $(($names($($tail_arg_names: $tail_arg_types,)*), $messages, $helps),)*);
    };

}
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

use crate::{ErrorCode, FormattedError, LeoErrorCode, new_from_span, Span};

#[derive(Debug, Error)]
pub enum StateError {
    #[error(transparent)]
    FormattedError(#[from] FormattedError),
}

impl LeoErrorCode for StateError {}

impl ErrorCode for StateError {
    #[inline(always)]
    fn exit_code_mask() -> u32 {
        6000
    }

    #[inline(always)]
    fn error_type() -> String {
        "P".to_string()
    }

    new_from_span!();
}

impl StateError {
    
}

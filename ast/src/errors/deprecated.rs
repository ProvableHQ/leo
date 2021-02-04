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

use crate::{Error as FormattedError, Span};
use leo_grammar::{annotations::AnnotationName, definitions::Deprecated};

use std::{convert::TryFrom, path::Path};

#[derive(Debug, Error)]
pub enum DeprecatedError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl DeprecatedError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            DeprecatedError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        DeprecatedError::Error(FormattedError::new_from_span(message, span))
    }
}

impl<'ast> From<Deprecated<'ast>> for DeprecatedError {
    fn from(deprecated: Deprecated<'ast>) -> Self {
        match deprecated {
            Deprecated::TestFunction(test_function) => DeprecatedError::new_from_span(
                "\"test function...\" is deprecated. Did you mean @test annotation?".to_string(),
                Span::from(test_function.span.clone()),
            ),
        }
    }
}

impl<'ast> TryFrom<AnnotationName<'ast>> for DeprecatedError {
    type Error = bool;

    fn try_from(annotation_name: AnnotationName<'ast>) -> Result<Self, bool> {
        match annotation_name {
            AnnotationName::Context(context) => Ok(DeprecatedError::new_from_span(
                "\"@context(...)\" is deprecated. Did you mean @test annotation?".to_string(),
                Span::from(context.span.clone()),
            )),
            _ => Err(false),
        }
    }
}

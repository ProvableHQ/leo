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

use crate::{FunctionInputVariable, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Enumerates the possible inputs to a function.
#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionInput {
    /// A normal function parameter.
    Variable(FunctionInputVariable),
}

impl FunctionInput {
    ///
    /// Returns Option with FunctionInputVariable if the input is a variable.
    /// Returns None otherwise.
    ///
    pub fn get_variable(&self) -> &FunctionInputVariable {
        match self {
            Self::Variable(var) => var,
        }
    }

    /// Formats the parameter to `f`.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionInput::Variable(function_input) => write!(f, "{}", function_input),
        }
    }
}

impl fmt::Display for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl PartialEq for FunctionInput {
    /// Returns true if `self == other`. Does not compare spans.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FunctionInput::Variable(left), FunctionInput::Variable(right)) => left.eq(right),
        }
    }
}

impl Eq for FunctionInput {}

impl Node for FunctionInput {
    fn span(&self) -> Span {
        use FunctionInput::*;
        match self {
            Variable(variable) => variable.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        use FunctionInput::*;
        match self {
            Variable(variable) => variable.span = span,
        }
    }
}

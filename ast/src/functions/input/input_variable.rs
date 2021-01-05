// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{FunctionInputVariable, InputKeyword, MutSelfKeyword, Node, SelfKeyword, Span};
use leo_grammar::functions::input::Input as GrammarInput;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Enumerates the possible inputs to a function.
#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionInput {
    InputKeyword(InputKeyword),
    SelfKeyword(SelfKeyword),
    MutSelfKeyword(MutSelfKeyword),
    Variable(FunctionInputVariable),
}

impl<'ast> From<GrammarInput<'ast>> for FunctionInput {
    fn from(input: GrammarInput<'ast>) -> Self {
        match input {
            GrammarInput::InputKeyword(keyword) => FunctionInput::InputKeyword(InputKeyword::from(keyword)),
            GrammarInput::SelfKeyword(keyword) => FunctionInput::SelfKeyword(SelfKeyword::from(keyword)),
            GrammarInput::MutSelfKeyword(keyword) => FunctionInput::MutSelfKeyword(MutSelfKeyword::from(keyword)),
            GrammarInput::FunctionInput(function_input) => {
                FunctionInput::Variable(FunctionInputVariable::from(function_input))
            }
        }
    }
}

impl FunctionInput {
    ///
    /// Returns `true` if the function input is the `self` or `mut self` keyword.
    /// Returns `false` otherwise.
    ///
    pub fn is_self(&self) -> bool {
        match self {
            FunctionInput::InputKeyword(_) => false,
            FunctionInput::SelfKeyword(_) => true,
            FunctionInput::MutSelfKeyword(_) => true,
            FunctionInput::Variable(_) => false,
        }
    }

    ///
    /// Returns `true` if the function input is the `mut self` keyword.
    /// Returns `false` otherwise.
    ///
    pub fn is_mut_self(&self) -> bool {
        match self {
            FunctionInput::InputKeyword(_) => false,
            FunctionInput::SelfKeyword(_) => false,
            FunctionInput::MutSelfKeyword(_) => true,
            FunctionInput::Variable(_) => false,
        }
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionInput::InputKeyword(keyword) => write!(f, "{}", keyword),
            FunctionInput::SelfKeyword(keyword) => write!(f, "{}", keyword),
            FunctionInput::MutSelfKeyword(keyword) => write!(f, "{}", keyword),
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
            (FunctionInput::InputKeyword(_), FunctionInput::InputKeyword(_)) => true,
            (FunctionInput::SelfKeyword(_), FunctionInput::SelfKeyword(_)) => true,
            (FunctionInput::MutSelfKeyword(_), FunctionInput::MutSelfKeyword(_)) => true,
            (FunctionInput::Variable(left), FunctionInput::Variable(right)) => left.eq(right),
            _ => false,
        }
    }
}

impl Eq for FunctionInput {}

impl Node for FunctionInput {
    fn span(&self) -> &Span {
        use FunctionInput::*;
        match self {
            InputKeyword(keyword) => &keyword.span,
            SelfKeyword(keyword) => &keyword.span,
            MutSelfKeyword(keyword) => &keyword.span,
            Variable(variable) => &variable.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        use FunctionInput::*;
        match self {
            InputKeyword(keyword) => keyword.span = span,
            SelfKeyword(keyword) => keyword.span = span,
            MutSelfKeyword(keyword) => keyword.span = span,
            Variable(variable) => variable.span = span,
        }
    }
}

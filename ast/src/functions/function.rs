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

use crate::{Annotation, Block, FunctionInput, Identifier, Node, Type};
use leo_errors::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Function {
    pub annotations: Vec<Annotation>,
    pub identifier: Identifier,
    pub input: Vec<FunctionInput>,
    pub output: Option<Type>,
    pub core_mapping: std::cell::RefCell<Option<String>>,
    pub block: Block,
    pub span: Span,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Function {}

impl Function {
    pub fn get_name(&self) -> &str {
        &self.identifier.name
    }

    ///
    /// Returns `true` if the function has input `self` or `mut self`.
    /// Returns `false` otherwise.
    ///
    pub fn contains_self(&self) -> bool {
        self.input.iter().any(|param| param.is_self())
    }

    ///
    /// Returns `true` if the function has input `mut self`.
    /// Returns `false` otherwise.
    ///
    pub fn contains_mut_self(&self) -> bool {
        self.input.iter().any(|param| param.is_mut_self())
    }

    ///
    /// Returns an iterator of [&FunctionInput] removing `self` and `mut self` inputs.
    ///
    pub fn filter_self_inputs(&self) -> impl Iterator<Item = &FunctionInput> {
        self.input.iter().filter(|input| !input.is_self())
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.identifier)?;

        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = self.output.as_ref().map(|type_| type_.to_string());
        if returns.is_none() {
            write!(f, "({}) {}", parameters, self.block)
        } else {
            write!(f, "({}) -> {} {}", parameters, returns.unwrap(), self.block)
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl Node for Function {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

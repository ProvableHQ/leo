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

use crate::{FunctionInput, Identifier, Span, Statement, Type};
use leo_grammar::functions::Function as GrammarFunction;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Function {
    pub identifier: Identifier,
    pub input: Vec<FunctionInput>,
    pub output: Option<Type>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Function {}

impl<'ast> From<GrammarFunction<'ast>> for Function {
    fn from(function: GrammarFunction<'ast>) -> Self {
        let function_name = Identifier::from(function.identifier);

        let parameters = function.parameters.into_iter().map(FunctionInput::from).collect();
        let returns = function.returns.map(Type::from);
        let statements = function.statements.into_iter().map(Statement::from).collect();

        Function {
            identifier: function_name,
            input: parameters,
            output: returns,
            statements,
            span: Span::from(function.span),
        }
    }
}

impl Function {
    pub fn get_name(&self) -> &str {
        &self.identifier.name
    }

    ///
    /// Returns `true` if the function has input `self` or `mut self`.
    /// Returns `false` otherwise.
    ///
    pub fn contains_self(&self) -> bool {
        self.input.iter().find(|param| param.is_self()).is_some()
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.identifier)?;

        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = self.output.as_ref().map(|type_| type_.to_string());
        let statements = self
            .statements
            .iter()
            .map(|s| format!("\t{}\n", s))
            .collect::<Vec<_>>()
            .join("");
        if returns.is_none() {
            write!(f, "({}) {{\n{}}}", parameters, statements,)
        } else {
            write!(f, "({}) -> {} {{\n{}}}", parameters, returns.unwrap(), statements,)
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

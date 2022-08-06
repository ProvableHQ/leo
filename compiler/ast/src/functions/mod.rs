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

pub mod annotation;
pub use annotation::*;

pub mod function_input;
pub use function_input::*;

use crate::{Block, Identifier, Node, Type};
use leo_span::{sym, Span, Symbol};

use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fmt;

/// A function definition.
#[derive(Clone, Serialize, Deserialize)]
pub struct Function {
    /// Annotations on the function.
    pub annotations: Vec<Annotation>,
    /// The function identifier, e.g., `foo` in `function foo(...) { ... }`.
    pub identifier: Identifier,
    /// The function's parameters.
    pub input: Vec<FunctionInput>,
    /// The function's required return type.
    pub output: Type,
    /// Any mapping to the core library.
    /// Always `None` when initially parsed.
    pub core_mapping: Cell<Option<Symbol>>,
    /// The body of the function.
    pub block: Block,
    /// The entire span of the function definition.
    pub span: Span,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Function {}

impl Function {
    /// Returns function name.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    /// Returns `true` if the function name is `main`.
    pub fn is_main(&self) -> bool {
        self.name() == sym::main
    }

    ///
    /// Private formatting method used for optimizing [fmt::Debug] and [fmt::Display] implementations.
    ///
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.identifier)?;

        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = self.output.to_string();
        write!(f, "({}) -> {} {}", parameters, returns, self.block)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(Function);

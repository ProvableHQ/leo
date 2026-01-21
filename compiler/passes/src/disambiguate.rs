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

use crate::{CompilerState, Pass};

use leo_ast::*;
use leo_errors::Result;
use leo_span::{Symbol, sym};

/// Pass that turns ambiguous calls into their proper form after type checking
/// such as get and set for mappings/vectors
pub struct Disambiguate;

impl Pass for Disambiguate {
    type Input = ();
    type Output = ();

    const NAME: &str = "Disambiguate";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = DisambiguateVisitor { state };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}

pub struct DisambiguateVisitor<'state> {
    pub state: &'state mut CompilerState,
}

impl ProgramReconstructor for DisambiguateVisitor<'_> {}

impl AstReconstructor for DisambiguateVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_intrinsic(
        &mut self,
        mut input: IntrinsicExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        input.arguments = input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect();

        if input.name == Symbol::intern("__unresolved_get") {
            match self.state.type_table.get(&input.arguments[0].id()) {
                Some(Type::Vector(..)) => {
                    input.name = sym::_vector_get;
                }
                Some(Type::Mapping(..)) => {
                    input.name = sym::_mapping_get;
                }
                _ => {
                    panic!("type checking should guarantee that no other type is expected here.")
                }
            }
        } else if input.name == Symbol::intern("__unresolved_set") {
            match self.state.type_table.get(&input.arguments[0].id()) {
                Some(Type::Vector(..)) => {
                    input.name = sym::_vector_set;
                }
                Some(Type::Mapping(..)) => {
                    input.name = sym::_mapping_set;
                }
                _ => {
                    panic!("type checking should guarantee that no other type is expected here.")
                }
            }
        }

        (input.into(), ())
    }
}

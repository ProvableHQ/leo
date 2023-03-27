// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use itertools::Itertools;
use leo_ast::*;
use leo_span::{Span, Symbol};

use crate::{unroller::Unroller, VariableSymbol, VariableType};

impl StatementReconstructor for Unroller<'_> {
    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        let scope_index = self.current_scope_index();

        // Enter the block scope.
        let previous_scope_index = self.enter_scope(scope_index);

        let block = Block {
            statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
            span: input.span,
        };

        // Exit the block scope.
        self.exit_scope(previous_scope_index);

        (block, Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // If we are unrolling a loop, then we need to repopulate the symbol table.
        if self.is_unrolling {
            let declaration =
                if input.declaration_type == DeclarationType::Const { VariableType::Const } else { VariableType::Mut };

            let insert_variable = |symbol: Symbol, type_: Type, span: Span, declaration: VariableType| {
                if let Err(err) =
                    self.symbol_table.borrow_mut().insert_variable(symbol, VariableSymbol { type_, span, declaration })
                {
                    self.handler.emit_err(err);
                }
            };

            // Insert the variables in the into the symbol table.
            match &input.place {
                Expression::Identifier(identifier) => {
                    insert_variable(identifier.name, input.type_.clone(), identifier.span, declaration)
                }
                Expression::Tuple(tuple_expression) => {
                    let tuple_type = match input.type_ {
                        Type::Tuple(ref tuple_type) => tuple_type,
                        _ => unreachable!(
                            "Type checking guarantees that if the lhs is a tuple, its associated type is also a tuple."
                        ),
                    };
                    tuple_expression.elements.iter().zip_eq(tuple_type.0.iter()).for_each(|(expression, type_)| {
                        let identifier = match expression {
                            Expression::Identifier(identifier) => identifier,
                            _ => unreachable!("Type checking guarantees that if the lhs is a tuple, all of its elements are identifiers.")
                        };
                        insert_variable(identifier.name, type_.clone(), identifier.span, declaration)
                    });
                }
                _ => unreachable!(
                    "Type checking guarantees that the lhs of a `DefinitionStatement` is either an identifier or tuple."
                ),
            }
        }
        (Statement::Definition(input), Default::default())
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        // We match on start and stop cause loops require
        // bounds to be constants.
        match (input.start_value.clone().into_inner(), input.stop_value.clone().into_inner()) {
            (Some(start), Some(stop)) => match (Type::from(&start), Type::from(&stop)) {
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8))
                | (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16))
                | (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32))
                | (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64))
                | (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128)) => {
                    (self.unroll_iteration_statement::<i128>(input, start, stop), Default::default())
                }
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8))
                | (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16))
                | (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32))
                | (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64))
                | (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128)) => {
                    (self.unroll_iteration_statement::<u128>(input, start, stop), Default::default())
                }
                _ => unreachable!("Type checking ensures that `start` and `stop` have the same type."),
            },
            // If both loop bounds are not constant, then the loop is not unrolled.
            _ => (Statement::Iteration(Box::from(input)), Default::default()),
        }
    }
}

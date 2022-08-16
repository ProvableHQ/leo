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

use leo_ast::*;

use crate::unroller::Unroller;
use crate::{VariableSymbol, VariableType};

impl StatementReconstructor for Unroller<'_> {
    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        // If we are unrolling a loop, then we need to repopulate the symbol table.
        if self.is_unrolling {
            let declaration = if input.declaration_type == DeclarationType::Const {
                VariableType::Const
            } else {
                VariableType::Mut
            };

            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                input.variable_name.name,
                VariableSymbol {
                    type_: input.type_.clone(),
                    span: input.span(),
                    declaration,
                },
            ) {
                self.handler.emit_err(err);
            }
        }
        Statement::Definition(input)
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        // We match on start and stop cause loops require
        // bounds to be constants.
        match (
            input.start_value.clone().into_inner(),
            input.stop_value.clone().into_inner(),
        ) {
            (Some(start), Some(stop)) => match (Type::from(&start), Type::from(&stop)) {
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8))
                | (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16))
                | (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32))
                | (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64))
                | (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128)) => {
                    self.unroll_iteration_statement::<i128>(input, start, stop)
                }
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8))
                | (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16))
                | (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32))
                | (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64))
                | (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128)) => {
                    self.unroll_iteration_statement::<u128>(input, start, stop)
                }
                _ => unreachable!("Type checking ensures that `start` and `stop` have the same type."),
            },
            // If both loop bounds are not constant, then the loop is not unrolled.
            _ => Statement::Iteration(Box::from(input)),
        }
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let scope_index = self.current_scope_index();

        // Enter the block scope.
        self.enter_block_scope(scope_index);
        self.block_index = 0;

        let block = Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        };

        // Exit the block scope.
        self.exit_block_scope(scope_index);
        self.block_index = scope_index + 1;

        block
    }
}

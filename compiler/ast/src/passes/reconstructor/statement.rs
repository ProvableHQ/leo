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

use crate::*;

/// A Reconstructor trait for statements in the AST.
pub trait StatementReconstructor: ExpressionReconstructor + InstructionReconstructor {
    fn reconstruct_statement(&mut self, input: Statement) -> (Statement, Self::AdditionalOutput) {
        match input {
            Statement::AssemblyBlock(stmt) => self.reconstruct_assembly_block(stmt),
            Statement::Assert(stmt) => self.reconstruct_assert(stmt),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (Statement::Block(stmt), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Console(stmt) => self.reconstruct_console(stmt),
            Statement::Decrement(stmt) => self.reconstruct_decrement(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Expression(stmt) => self.reconstruct_expression_statement(stmt),
            Statement::Increment(stmt) => self.reconstruct_increment(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Return(stmt) => self.reconstruct_return(stmt),
        }
    }

    fn reconstruct_assembly_block(&mut self, input: AssemblyBlock) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::AssemblyBlock(AssemblyBlock {
                instructions: input
                    .instructions
                    .into_iter()
                    .map(|inst| self.reconstruct_instruction(inst).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Assert(AssertStatement {
                variant: match input.variant {
                    AssertVariant::Assert(expr) => AssertVariant::Assert(self.reconstruct_expression(expr).0),
                    AssertVariant::AssertEq(left, right) => AssertVariant::AssertEq(
                        self.reconstruct_expression(left).0,
                        self.reconstruct_expression(right).0,
                    ),
                    AssertVariant::AssertNeq(left, right) => AssertVariant::AssertNeq(
                        self.reconstruct_expression(left).0,
                        self.reconstruct_expression(right).0,
                    ),
                },
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Assign(Box::new(AssignStatement {
                place: input.place,
                value: self.reconstruct_expression(input.value).0,
                span: input.span,
            })),
            Default::default(),
        )
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        (
            Block {
                statements: input
                    .statements
                    .into_iter()
                    .map(|s| self.reconstruct_statement(s).0)
                    .collect(),
                span: input.span,
            },
            Default::default(),
        )
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Conditional(ConditionalStatement {
                condition: self.reconstruct_expression(input.condition).0,
                then: self.reconstruct_block(input.then).0,
                otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_console(&mut self, input: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Console(ConsoleStatement {
                function: match input.function {
                    ConsoleFunction::Assert(expr) => ConsoleFunction::Assert(self.reconstruct_expression(expr).0),
                    ConsoleFunction::AssertEq(left, right) => ConsoleFunction::AssertEq(
                        self.reconstruct_expression(left).0,
                        self.reconstruct_expression(right).0,
                    ),
                    ConsoleFunction::AssertNeq(left, right) => ConsoleFunction::AssertNeq(
                        self.reconstruct_expression(left).0,
                        self.reconstruct_expression(right).0,
                    ),
                },
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_decrement(&mut self, input: DecrementStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Decrement(DecrementStatement {
                mapping: input.mapping,
                index: input.index,
                amount: input.amount,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Definition(DefinitionStatement {
                declaration_type: input.declaration_type,
                place: input.place,
                type_: input.type_,
                value: self.reconstruct_expression(input.value).0,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Expression(ExpressionStatement {
                expression: self.reconstruct_expression(input.expression).0,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_increment(&mut self, input: IncrementStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Increment(IncrementStatement {
                mapping: input.mapping,
                index: input.index,
                amount: input.amount,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Iteration(Box::new(IterationStatement {
                variable: input.variable,
                type_: input.type_,
                start: self.reconstruct_expression(input.start).0,
                start_value: input.start_value,
                stop: self.reconstruct_expression(input.stop).0,
                stop_value: input.stop_value,
                block: self.reconstruct_block(input.block).0,
                inclusive: input.inclusive,
                span: input.span,
            })),
            Default::default(),
        )
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Return(ReturnStatement {
                expression: self.reconstruct_expression(input.expression).0,
                finalize_arguments: input.finalize_arguments.map(|arguments| {
                    arguments
                        .into_iter()
                        .map(|argument| self.reconstruct_expression(argument).0)
                        .collect()
                }),
                span: input.span,
            }),
            Default::default(),
        )
    }
}

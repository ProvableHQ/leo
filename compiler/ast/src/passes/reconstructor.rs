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

//! This module contains a Reconstructor trait for the AST.
//! It implements default methods for each node to be made
//! given the information of the old node.

use crate::*;

/// A Reconstructor trait for expressions in the AST.
pub trait ExpressionReconstructor {
    type AdditionalOutput: Default;

    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        match input {
            Expression::Access(access) => self.reconstruct_access(access),
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Circuit(circuit) => self.reconstruct_circuit_init(circuit),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
        }
    }

    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(match input {
                AccessExpression::AssociatedFunction(function) => {
                    AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: function.ty,
                        name: function.name,
                        args: function
                            .args
                            .into_iter()
                            .map(|arg| self.reconstruct_expression(arg).0)
                            .collect(),
                        span: function.span,
                    })
                }
                AccessExpression::Member(member) => AccessExpression::Member(MemberAccess {
                    inner: Box::new(self.reconstruct_expression(*member.inner).0),
                    name: member.name,
                    span: member.span,
                }),
                AccessExpression::Tuple(tuple) => AccessExpression::Tuple(TupleAccess {
                    tuple: Box::new(self.reconstruct_expression(*tuple.tuple).0),
                    index: tuple.index,
                    span: tuple.span,
                }),
                expr => expr,
            }),
            Default::default(),
        )
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Binary(BinaryExpression {
                left: Box::new(self.reconstruct_expression(*input.left).0),
                right: Box::new(self.reconstruct_expression(*input.right).0),
                op: input.op,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                function: Box::new(self.reconstruct_expression(*input.function).0),
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_circuit_init(&mut self, input: CircuitExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Circuit(input), Default::default())
    }

    fn reconstruct_err(&mut self, input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Err(input), Default::default())
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        (Expression::Identifier(input), Default::default())
    }

    fn reconstruct_literal(&mut self, input: Literal) -> (Expression, Self::AdditionalOutput) {
        (Expression::Literal(input), Default::default())
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Ternary(TernaryExpression {
                condition: Box::new(self.reconstruct_expression(*input.condition).0),
                if_true: Box::new(self.reconstruct_expression(*input.if_true).0),
                if_false: Box::new(self.reconstruct_expression(*input.if_false).0),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_tuple(&mut self, input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Tuple(TupleExpression {
                elements: input
                    .elements
                    .into_iter()
                    .map(|element| self.reconstruct_expression(element).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Unary(UnaryExpression {
                receiver: Box::new(self.reconstruct_expression(*input.receiver).0),
                op: input.op,
                span: input.span,
            }),
            Default::default(),
        )
    }
}

/// A Reconstructor trait for statements in the AST.
pub trait StatementReconstructor: ExpressionReconstructor {
    fn reconstruct_statement(&mut self, input: Statement) -> Statement {
        match input {
            Statement::Return(stmt) => self.reconstruct_return(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Console(stmt) => self.reconstruct_console(stmt),
            Statement::Block(stmt) => Statement::Block(self.reconstruct_block(stmt)),
        }
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> Statement {
        Statement::Return(ReturnStatement {
            expression: self.reconstruct_expression(input.expression).0,
            span: input.span,
        })
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_name: input.variable_name,
            type_: input.type_,
            value: self.reconstruct_expression(input.value).0,
            span: input.span,
        })
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            place: input.place,
            value: self.reconstruct_expression(input.value).0,
            span: input.span,
        }))
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> Statement {
        Statement::Conditional(ConditionalStatement {
            condition: self.reconstruct_expression(input.condition).0,
            then: self.reconstruct_block(input.then),
            otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n))),
            span: input.span,
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        Statement::Iteration(Box::new(IterationStatement {
            variable: input.variable,
            type_: input.type_,
            start: self.reconstruct_expression(input.start).0,
            start_value: input.start_value,
            stop: self.reconstruct_expression(input.stop).0,
            stop_value: input.stop_value,
            block: self.reconstruct_block(input.block),
            inclusive: input.inclusive,
            span: input.span,
        }))
    }

    fn reconstruct_console(&mut self, input: ConsoleStatement) -> Statement {
        Statement::Console(ConsoleStatement {
            function: match input.function {
                ConsoleFunction::Assert(expr) => ConsoleFunction::Assert(self.reconstruct_expression(expr).0),
                ConsoleFunction::Error(fmt) => ConsoleFunction::Error(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| self.reconstruct_expression(p).0)
                        .collect(),
                    span: fmt.span,
                }),
                ConsoleFunction::Log(fmt) => ConsoleFunction::Log(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| self.reconstruct_expression(p).0)
                        .collect(),
                    span: fmt.span,
                }),
            },
            span: input.span,
        })
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        }
    }
}

/// A Reconstructor trait for the program represented by the AST.
pub trait ProgramReconstructor: StatementReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            name: input.name,
            network: input.network,
            expected_input: input.expected_input,
            imports: input
                .imports
                .into_iter()
                .map(|(id, import)| (id, self.reconstruct_import(import)))
                .collect(),
            functions: input
                .functions
                .into_iter()
                .map(|(i, f)| (i, self.reconstruct_function(f)))
                .collect(),
            circuits: input
                .circuits
                .into_iter()
                .map(|(i, c)| (i, self.reconstruct_circuit(c)))
                .collect(),
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            identifier: input.identifier,
            input: input.input,
            output: input.output,
            core_mapping: input.core_mapping,
            block: self.reconstruct_block(input.block),
            span: input.span,
        }
    }

    fn reconstruct_circuit(&mut self, input: Circuit) -> Circuit {
        input
    }

    fn reconstruct_import(&mut self, input: Program) -> Program {
        input
    }
}

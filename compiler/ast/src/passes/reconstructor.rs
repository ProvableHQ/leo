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

use leo_span::Span;

/// A Reconstructor trait for expressions in the AST.
pub trait ExpressionReconstructor {
    type AdditionalOutput: Default;

    fn reconstruct_expression(&mut self, expr: Expression) -> (Expression, Self::AdditionalOutput) {
        match expr.kind {
            ExpressionKind::Access(access) => self.reconstruct_access(expr.span, access),
            ExpressionKind::Binary(binary) => self.reconstruct_binary(expr.span, binary),
            ExpressionKind::Call(call) => self.reconstruct_call(expr.span, call),
            ExpressionKind::Circuit(circuit) => self.reconstruct_circuit_init(expr.span, circuit),
            ExpressionKind::Err(err) => self.reconstruct_err(expr.span, err),
            ExpressionKind::Identifier(identifier) => self.reconstruct_identifier(identifier),
            ExpressionKind::Literal(value) => self.reconstruct_literal(expr.span, value),
            ExpressionKind::Ternary(ternary) => self.reconstruct_ternary(expr.span, ternary),
            ExpressionKind::Tuple(tuple) => self.reconstruct_tuple(expr.span, tuple),
            ExpressionKind::Unary(unary) => self.reconstruct_unary(expr.span, unary),
        }
    }

    fn reconstruct_access(&mut self, span: Span, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Access(match input {
            AccessExpression::AssociatedFunction(function) => {
                AccessExpression::AssociatedFunction(AssociatedFunction {
                    ty: function.ty,
                    name: function.name,
                    args: function
                        .args
                        .into_iter()
                        .map(|arg| self.reconstruct_expression(arg).0)
                        .collect(),
                })
            }
            AccessExpression::Member(member) => AccessExpression::Member(MemberAccess {
                inner: Box::new(self.reconstruct_expression(*member.inner).0),
                name: member.name,
            }),
            AccessExpression::Tuple(tuple) => AccessExpression::Tuple(TupleAccess {
                tuple: Box::new(self.reconstruct_expression(*tuple.tuple).0),
                index: tuple.index,
            }),
            ae @ AccessExpression::AssociatedConstant(_) => ae,
        });
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_binary(&mut self, span: Span, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Binary(BinaryExpression {
            left: Box::new(self.reconstruct_expression(*input.left).0),
            right: Box::new(self.reconstruct_expression(*input.right).0),
            op: input.op,
        });
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_call(&mut self, span: Span, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Call(CallExpression {
            function: Box::new(self.reconstruct_expression(*input.function).0),
            arguments: input
                .arguments
                .into_iter()
                .map(|arg| self.reconstruct_expression(arg).0)
                .collect(),
        });
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_circuit_init(
        &mut self,
        span: Span,
        input: CircuitExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Circuit(input);
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_err(&mut self, span: Span, input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Err(input);
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        let span = input.span;
        let kind = ExpressionKind::Identifier(input);
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_literal(&mut self, span: Span, input: Literal) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Literal(input);
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_ternary(&mut self, span: Span, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Ternary(TernaryExpression {
            condition: Box::new(self.reconstruct_expression(*input.condition).0),
            if_true: Box::new(self.reconstruct_expression(*input.if_true).0),
            if_false: Box::new(self.reconstruct_expression(*input.if_false).0),
        });
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_tuple(&mut self, span: Span, input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Tuple(TupleExpression {
            elements: input
                .elements
                .into_iter()
                .map(|element| self.reconstruct_expression(element).0)
                .collect(),
        });
        (Expression { kind, span }, Default::default())
    }

    fn reconstruct_unary(&mut self, span: Span, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        let kind = ExpressionKind::Unary(UnaryExpression {
            receiver: Box::new(self.reconstruct_expression(*input.receiver).0),
            op: input.op,
        });
        (Expression { kind, span }, Default::default())
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
            annotations: input.annotations,
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

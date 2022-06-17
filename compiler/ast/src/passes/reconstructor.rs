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

//! This module contains a reconstructr Trait for the AST.
//! It implements default methods for each node to be made
//! given the information of the old node.

use crate::*;

pub trait ExpressionReconstructor {
    fn reconstruct_expression(&mut self, input: Expression) -> Expression {
        match input {
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Value(value) => self.reconstruct_value(value),
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Err(err) => self.reconstruct_err(err),
        }
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> Expression {
        Expression::Identifier(input)
    }

    fn reconstruct_value(&mut self, input: ValueExpression) -> Expression {
        Expression::Value(input)
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> Expression {
        Expression::Binary(BinaryExpression {
            left: Box::new(self.reconstruct_expression(*input.left)),
            right: Box::new(self.reconstruct_expression(*input.right)),
            op: input.op,
            span: input.span,
        })
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> Expression {
        Expression::Unary(UnaryExpression {
            inner: Box::new(self.reconstruct_expression(*input.inner)),
            op: input.op,
            span: input.span,
        })
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> Expression {
        Expression::Ternary(TernaryExpression {
            condition: Box::new(self.reconstruct_expression(*input.condition)),
            if_true: Box::new(self.reconstruct_expression(*input.if_true)),
            if_false: Box::new(self.reconstruct_expression(*input.if_false)),
            span: input.span,
        })
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> Expression {
        Expression::Call(CallExpression {
            function: Box::new(self.reconstruct_expression(*input.function)),
            arguments: input
                .arguments
                .into_iter()
                .map(|arg| self.reconstruct_expression(arg))
                .collect(),
            span: input.span,
        })
    }

    fn reconstruct_err(&mut self, input: ErrExpression) -> Expression {
        Expression::Err(input)
    }
}

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
            expression: self.reconstruct_expression(input.expression),
            span: input.span,
        })
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_names: input.variable_names.clone(),
            type_: input.type_,
            value: self.reconstruct_expression(input.value),
            span: input.span,
        })
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            assignee: input.assignee,
            value: self.reconstruct_expression(input.value),
            span: input.span,
        }))
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> Statement {
        Statement::Conditional(ConditionalStatement {
            condition: self.reconstruct_expression(input.condition),
            block: self.reconstruct_block(input.block),
            next: input.next.map(|n| Box::new(self.reconstruct_statement(*n))),
            span: input.span,
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        Statement::Iteration(Box::new(IterationStatement {
            variable: input.variable,
            type_: input.type_,
            start: self.reconstruct_expression(input.start),
            stop: self.reconstruct_expression(input.stop),
            block: self.reconstruct_block(input.block),
            inclusive: input.inclusive,
            span: input.span,
        }))
    }

    fn reconstruct_console(&mut self, input: ConsoleStatement) -> Statement {
        Statement::Console(ConsoleStatement {
            function: match input.function {
                ConsoleFunction::Assert(expr) => ConsoleFunction::Assert(self.reconstruct_expression(expr)),
                ConsoleFunction::Error(fmt) => ConsoleFunction::Error(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| self.reconstruct_expression(p))
                        .collect(),
                    span: fmt.span,
                }),
                ConsoleFunction::Log(fmt) => ConsoleFunction::Log(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| self.reconstruct_expression(p))
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

pub trait ProgramReconstructor: StatementReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            name: input.name,
            expected_input: input.expected_input,
            functions: input
                .functions
                .into_iter()
                .map(|(i, f)| (i, self.reconstruct_function(f)))
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
}

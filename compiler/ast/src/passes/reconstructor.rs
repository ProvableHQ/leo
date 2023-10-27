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
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Cast(cast) => self.reconstruct_cast(cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        }
    }

    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        match input {
            AccessExpression::Array(array) => self.reconstruct_array_access(array),
            AccessExpression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            AccessExpression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            AccessExpression::Member(member) => self.reconstruct_member_access(member),
            AccessExpression::Tuple(tuple) => self.reconstruct_tuple_access(tuple),
        }
    }

    fn reconstruct_array_access(&mut self, input: ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(AccessExpression::Array(ArrayAccess {
                array: Box::new(self.reconstruct_expression(*input.array).0),
                index: Box::new(self.reconstruct_expression(*input.index).0),
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_associated_constant(&mut self, input: AssociatedConstant) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(AccessExpression::AssociatedConstant(AssociatedConstant {
                ty: input.ty,
                name: input.name,
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_associated_function(&mut self, input: AssociatedFunction) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                ty: input.ty,
                name: input.name,
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg).0).collect(),
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_member_access(&mut self, input: MemberAccess) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(AccessExpression::Member(MemberAccess {
                inner: Box::new(self.reconstruct_expression(*input.inner).0),
                name: input.name,
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_tuple_access(&mut self, input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(AccessExpression::Tuple(TupleAccess {
                tuple: Box::new(self.reconstruct_expression(*input.tuple).0),
                index: input.index,
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_array(&mut self, input: ArrayExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Array(ArrayExpression {
                elements: input.elements.into_iter().map(|element| self.reconstruct_expression(element).0).collect(),
                span: input.span,
                id: input.id,
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
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                function: Box::new(self.reconstruct_expression(*input.function).0),
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg).0).collect(),
                external: input.external,
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_cast(&mut self, input: CastExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Cast(CastExpression {
                expression: Box::new(self.reconstruct_expression(*input.expression).0),
                type_: input.type_,
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Struct(StructExpression {
                name: input.name,
                members: input
                    .members
                    .into_iter()
                    .map(|member| StructVariableInitializer {
                        identifier: member.identifier,
                        expression: match member.expression {
                            Some(expression) => Some(self.reconstruct_expression(expression).0),
                            None => Some(self.reconstruct_expression(Expression::Identifier(member.identifier)).0),
                        },
                        span: member.span,
                        id: member.id,
                    })
                    .collect(),
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_err(&mut self, _input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
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
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_tuple(&mut self, input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Tuple(TupleExpression {
                elements: input.elements.into_iter().map(|element| self.reconstruct_expression(element).0).collect(),
                span: input.span,
                id: input.id,
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
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_unit(&mut self, input: UnitExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Unit(input), Default::default())
    }
}

/// A Reconstructor trait for statements in the AST.
pub trait StatementReconstructor: ExpressionReconstructor {
    fn reconstruct_statement(&mut self, input: Statement) -> (Statement, Self::AdditionalOutput) {
        match input {
            Statement::Assert(assert) => self.reconstruct_assert(assert),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (Statement::Block(stmt), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Console(stmt) => self.reconstruct_console(stmt),
            Statement::Const(stmt) => self.reconstruct_const(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Expression(stmt) => self.reconstruct_expression_statement(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Return(stmt) => self.reconstruct_return(stmt),
        }
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
                id: input.id,
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
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        (
            Block {
                statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
                span: input.span,
                id: input.id,
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
                id: input.id,
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
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Const(ConstDeclaration {
                place: input.place,
                type_: input.type_,
                value: self.reconstruct_expression(input.value).0,
                span: input.span,
                id: input.id,
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
                id: input.id,
            }),
            Default::default(),
        )
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Expression(ExpressionStatement {
                expression: self.reconstruct_expression(input.expression).0,
                span: input.span,
                id: input.id,
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
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            Statement::Return(ReturnStatement {
                expression: self.reconstruct_expression(input.expression).0,
                finalize_arguments: input.finalize_arguments.map(|arguments| {
                    arguments.into_iter().map(|argument| self.reconstruct_expression(argument).0).collect()
                }),
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }
}

/// A Reconstructor trait for the program represented by the AST.
pub trait ProgramReconstructor: StatementReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            imports: input
                .imports
                .into_iter()
                .map(|(id, import)| (id, (self.reconstruct_import(import.0), import.1)))
                .collect(),
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs.into_iter().map(|(i, c)| (i, self.reconstruct_struct(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => unreachable!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: self.reconstruct_block(input.block).0,
            finalize: input.finalize.map(|finalize| Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block: self.reconstruct_block(finalize.block).0,
                span: finalize.span,
                id: finalize.id,
            }),
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_struct(&mut self, input: Struct) -> Struct {
        input
    }

    fn reconstruct_import(&mut self, input: Program) -> Program {
        self.reconstruct_program(input)
    }

    fn reconstruct_mapping(&mut self, input: Mapping) -> Mapping {
        input
    }
}

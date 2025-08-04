// Copyright (C) 2019-2025 Provable Inc.
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

/// A Reconstructor trait for types in the AST.
pub trait AstReconstructor {
    type AdditionalOutput: Default;

    /* Types */
    fn reconstruct_type(&mut self, input: Type) -> (Type, Self::AdditionalOutput) {
        match input {
            Type::Array(array_type) => self.reconstruct_array_type(array_type),
            Type::Composite(composite_type) => self.reconstruct_composite_type(composite_type),
            Type::Future(future_type) => self.reconstruct_future_type(future_type),
            Type::Mapping(mapping_type) => self.reconstruct_mapping_type(mapping_type),
            Type::Tuple(tuple_type) => self.reconstruct_tuple_type(tuple_type),
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Identifier(_)
            | Type::Integer(_)
            | Type::Scalar
            | Type::Signature
            | Type::String
            | Type::Numeric
            | Type::Unit
            | Type::Err => (input.clone(), Default::default()),
        }
    }

    fn reconstruct_array_type(&mut self, input: ArrayType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Array(ArrayType {
                element_type: Box::new(self.reconstruct_type(*input.element_type).0),
                length: Box::new(self.reconstruct_expression(*input.length).0),
            }),
            Default::default(),
        )
    }

    fn reconstruct_composite_type(&mut self, input: CompositeType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Composite(CompositeType {
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                ..input
            }),
            Default::default(),
        )
    }

    fn reconstruct_future_type(&mut self, input: FutureType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Future(FutureType {
                inputs: input.inputs.into_iter().map(|input| self.reconstruct_type(input).0).collect(),
                ..input
            }),
            Default::default(),
        )
    }

    fn reconstruct_mapping_type(&mut self, input: MappingType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Mapping(MappingType {
                key: Box::new(self.reconstruct_type(*input.key).0),
                value: Box::new(self.reconstruct_type(*input.value).0),
                ..input
            }),
            Default::default(),
        )
    }

    fn reconstruct_tuple_type(&mut self, input: TupleType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Tuple(TupleType {
                elements: input.elements.into_iter().map(|element| self.reconstruct_type(element).0).collect(),
            }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        match input {
            Expression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            Expression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            Expression::Async(async_) => self.reconstruct_async(async_),
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access),
            Expression::Binary(binary) => self.reconstruct_binary(*binary),
            Expression::Call(call) => self.reconstruct_call(*call),
            Expression::Cast(cast) => self.reconstruct_cast(*cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Locator(locator) => self.reconstruct_locator(locator),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access),
            Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access),
            Expression::Unary(unary) => self.reconstruct_unary(*unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        }
    }

    fn reconstruct_array_access(&mut self, input: ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        (
            ArrayAccess {
                array: self.reconstruct_expression(input.array).0,
                index: self.reconstruct_expression(input.index).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_associated_constant(
        &mut self,
        input: AssociatedConstantExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_associated_function(
        &mut self,
        input: AssociatedFunctionExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            AssociatedFunctionExpression {
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg).0).collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_async(&mut self, input: AsyncExpression) -> (Expression, Self::AdditionalOutput) {
        (AsyncExpression { block: self.reconstruct_block(input.block).0, ..input }.into(), Default::default())
    }

    fn reconstruct_member_access(&mut self, input: MemberAccess) -> (Expression, Self::AdditionalOutput) {
        (MemberAccess { inner: self.reconstruct_expression(input.inner).0, ..input }.into(), Default::default())
    }

    fn reconstruct_repeat(&mut self, input: RepeatExpression) -> (Expression, Self::AdditionalOutput) {
        (
            RepeatExpression {
                expr: self.reconstruct_expression(input.expr).0,
                count: self.reconstruct_expression(input.count).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_tuple_access(&mut self, input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        (TupleAccess { tuple: self.reconstruct_expression(input.tuple).0, ..input }.into(), Default::default())
    }

    fn reconstruct_array(&mut self, input: ArrayExpression) -> (Expression, Self::AdditionalOutput) {
        (
            ArrayExpression {
                elements: input.elements.into_iter().map(|element| self.reconstruct_expression(element).0).collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            BinaryExpression {
                left: self.reconstruct_expression(input.left).0,
                right: self.reconstruct_expression(input.right).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            CallExpression {
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg).0).collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_cast(&mut self, input: CastExpression) -> (Expression, Self::AdditionalOutput) {
        (
            CastExpression { expression: self.reconstruct_expression(input.expression).0, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        (
            StructExpression {
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                members: input
                    .members
                    .into_iter()
                    .map(|member| StructVariableInitializer {
                        identifier: member.identifier,
                        expression: member.expression.map(|expr| self.reconstruct_expression(expr).0),
                        span: member.span,
                        id: member.id,
                    })
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_err(&mut self, _input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_literal(&mut self, input: Literal) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_locator(&mut self, input: LocatorExpression) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            TernaryExpression {
                condition: self.reconstruct_expression(input.condition).0,
                if_true: self.reconstruct_expression(input.if_true).0,
                if_false: self.reconstruct_expression(input.if_false).0,
                span: input.span,
                id: input.id,
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_tuple(&mut self, input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        (
            TupleExpression {
                elements: input.elements.into_iter().map(|element| self.reconstruct_expression(element).0).collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            UnaryExpression { receiver: self.reconstruct_expression(input.receiver).0, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_unit(&mut self, input: UnitExpression) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_statement(&mut self, input: Statement) -> (Statement, Self::AdditionalOutput) {
        match input {
            Statement::Assert(assert) => self.reconstruct_assert(assert),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (stmt.into(), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Const(stmt) => self.reconstruct_const(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Expression(stmt) => self.reconstruct_expression_statement(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Return(stmt) => self.reconstruct_return(stmt),
        }
    }

    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        (
            AssertStatement {
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
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        (AssignStatement { value: self.reconstruct_expression(input.value).0, ..input }.into(), Default::default())
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
            ConditionalStatement {
                condition: self.reconstruct_expression(input.condition).0,
                then: self.reconstruct_block(input.then).0,
                otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        (
            ConstDeclaration {
                type_: self.reconstruct_type(input.type_).0,
                value: self.reconstruct_expression(input.value).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            DefinitionStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                value: self.reconstruct_expression(input.value).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ExpressionStatement { expression: self.reconstruct_expression(input.expression).0, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        (
            IterationStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                start: self.reconstruct_expression(input.start).0,
                stop: self.reconstruct_expression(input.stop).0,
                block: self.reconstruct_block(input.block).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ReturnStatement { expression: self.reconstruct_expression(input.expression).0, ..input }.into(),
            Default::default(),
        )
    }
}

/// A Reconstructor trait for the program represented by the AST.
pub trait ProgramReconstructor: AstReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            imports: input
                .imports
                .into_iter()
                .map(|(id, import)| (id, (self.reconstruct_import(import.0), import.1)))
                .collect(),
            stubs: input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect(),
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
        }
    }

    fn reconstruct_stub(&mut self, input: Stub) -> Stub {
        Stub {
            imports: input.imports,
            stub_id: input.stub_id,
            consts: input.consts,
            structs: input.structs,
            mappings: input.mappings,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_stub(f))).collect(),
            span: input.span,
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        ProgramScope {
            program_id: input.program_id,
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            structs: input.structs.into_iter().map(|(i, c)| (i, self.reconstruct_struct(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: self.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| Input { type_: self.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: self.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: self.reconstruct_type(input.output_type).0,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        Constructor {
            annotations: input.annotations,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_function_stub(&mut self, input: FunctionStub) -> FunctionStub {
        input
    }

    fn reconstruct_struct(&mut self, input: Composite) -> Composite {
        Composite {
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: self.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect(),
            members: input
                .members
                .iter()
                .map(|member| Member { type_: self.reconstruct_type(member.type_.clone()).0, ..member.clone() })
                .collect(),
            ..input
        }
    }

    fn reconstruct_import(&mut self, input: Program) -> Program {
        self.reconstruct_program(input)
    }

    fn reconstruct_mapping(&mut self, input: Mapping) -> Mapping {
        Mapping {
            key_type: self.reconstruct_type(input.key_type).0,
            value_type: self.reconstruct_type(input.value_type).0,
            ..input
        }
    }
}

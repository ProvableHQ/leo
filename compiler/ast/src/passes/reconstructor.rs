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

//! This module contains a Reconstructor trait for the AST.
//! It implements default methods for each node to be made
//! given the information of the old node.

use crate::*;

/// A Reconstructor trait for types in the AST.
pub trait AstReconstructor {
    type AdditionalOutput: Default;
    type AdditionalInput: Default;

    /* Types */
    fn reconstruct_type(&mut self, input: Type) -> (Type, Self::AdditionalOutput) {
        match input {
            Type::Array(array_type) => self.reconstruct_array_type(array_type),
            Type::Composite(composite_type) => self.reconstruct_composite_type(composite_type),
            Type::Future(future_type) => self.reconstruct_future_type(future_type),
            Type::Mapping(mapping_type) => self.reconstruct_mapping_type(mapping_type),
            Type::Optional(optional_type) => self.reconstruct_optional_type(optional_type),
            Type::Tuple(tuple_type) => self.reconstruct_tuple_type(tuple_type),
            Type::Vector(vector_type) => self.reconstruct_vector_type(vector_type),
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
                length: Box::new(self.reconstruct_expression(*input.length, &Default::default()).0),
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
                    .map(|arg| self.reconstruct_expression(arg, &Default::default()).0)
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
            }),
            Default::default(),
        )
    }

    fn reconstruct_optional_type(&mut self, input: OptionalType) -> (Type, Self::AdditionalOutput) {
        (Type::Optional(OptionalType { inner: Box::new(self.reconstruct_type(*input.inner).0) }), Default::default())
    }

    fn reconstruct_tuple_type(&mut self, input: TupleType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Tuple(TupleType {
                elements: input.elements.into_iter().map(|element| self.reconstruct_type(element).0).collect(),
            }),
            Default::default(),
        )
    }

    fn reconstruct_vector_type(&mut self, input: VectorType) -> (Type, Self::AdditionalOutput) {
        (
            Type::Vector(VectorType { element_type: Box::new(self.reconstruct_type(*input.element_type).0) }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_expression(
        &mut self,
        input: Expression,
        additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        match input {
            Expression::Async(async_) => self.reconstruct_async(async_, additional),
            Expression::Array(array) => self.reconstruct_array(array, additional),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access, additional),
            Expression::Binary(binary) => self.reconstruct_binary(*binary, additional),
            Expression::Call(call) => self.reconstruct_call(*call, additional),
            Expression::Cast(cast) => self.reconstruct_cast(*cast, additional),
            Expression::Composite(composite_) => self.reconstruct_composite_init(composite_, additional),
            Expression::Err(err) => self.reconstruct_err(err, additional),
            Expression::Path(path) => self.reconstruct_path(path, additional),
            Expression::Literal(value) => self.reconstruct_literal(value, additional),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access, additional),
            Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat, additional),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary, additional),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple, additional),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access, additional),
            Expression::Unary(unary) => self.reconstruct_unary(*unary, additional),
            Expression::Unit(unit) => self.reconstruct_unit(unit, additional),
            Expression::Intrinsic(intr) => self.reconstruct_intrinsic(*intr, additional),
        }
    }

    fn reconstruct_array_access(
        &mut self,
        input: ArrayAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            ArrayAccess {
                array: self.reconstruct_expression(input.array, &Default::default()).0,
                index: self.reconstruct_expression(input.index, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_async(
        &mut self,
        input: AsyncExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (AsyncExpression { block: self.reconstruct_block(input.block).0, ..input }.into(), Default::default())
    }

    fn reconstruct_member_access(
        &mut self,
        input: MemberAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            MemberAccess { inner: self.reconstruct_expression(input.inner, &Default::default()).0, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_repeat(
        &mut self,
        input: RepeatExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            RepeatExpression {
                expr: self.reconstruct_expression(input.expr, &Default::default()).0,
                count: self.reconstruct_expression(input.count, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_intrinsic(
        &mut self,
        input: IntrinsicExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            IntrinsicExpression {
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &Default::default()).0)
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_tuple_access(
        &mut self,
        input: TupleAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            TupleAccess { tuple: self.reconstruct_expression(input.tuple, &Default::default()).0, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_array(
        &mut self,
        input: ArrayExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            ArrayExpression {
                elements: input
                    .elements
                    .into_iter()
                    .map(|element| self.reconstruct_expression(element, &Default::default()).0)
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_binary(
        &mut self,
        input: BinaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            BinaryExpression {
                left: self.reconstruct_expression(input.left, &Default::default()).0,
                right: self.reconstruct_expression(input.right, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_call(
        &mut self,
        input: CallExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            CallExpression {
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &Default::default()).0)
                    .collect(),
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &Default::default()).0)
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_cast(
        &mut self,
        input: CastExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            CastExpression {
                expression: self.reconstruct_expression(input.expression, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_composite_init(
        &mut self,
        input: CompositeExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            CompositeExpression {
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &Default::default()).0)
                    .collect(),
                members: input
                    .members
                    .into_iter()
                    .map(|member| CompositeFieldInitializer {
                        identifier: member.identifier,
                        expression: member
                            .expression
                            .map(|expr| self.reconstruct_expression(expr, &Default::default()).0),
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

    fn reconstruct_err(
        &mut self,
        _input: ErrExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_path(
        &mut self,
        input: Path,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_literal(
        &mut self,
        input: Literal,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            TernaryExpression {
                condition: self.reconstruct_expression(input.condition, &Default::default()).0,
                if_true: self.reconstruct_expression(input.if_true, &Default::default()).0,
                if_false: self.reconstruct_expression(input.if_false, &Default::default()).0,
                span: input.span,
                id: input.id,
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_tuple(
        &mut self,
        input: TupleExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            TupleExpression {
                elements: input
                    .elements
                    .into_iter()
                    .map(|element| self.reconstruct_expression(element, &Default::default()).0)
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_unary(
        &mut self,
        input: UnaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            UnaryExpression { receiver: self.reconstruct_expression(input.receiver, &Default::default()).0, ..input }
                .into(),
            Default::default(),
        )
    }

    fn reconstruct_unit(
        &mut self,
        input: UnitExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    /* Statements */
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
                    AssertVariant::Assert(expr) => {
                        AssertVariant::Assert(self.reconstruct_expression(expr, &Default::default()).0)
                    }
                    AssertVariant::AssertEq(left, right) => AssertVariant::AssertEq(
                        self.reconstruct_expression(left, &Default::default()).0,
                        self.reconstruct_expression(right, &Default::default()).0,
                    ),
                    AssertVariant::AssertNeq(left, right) => AssertVariant::AssertNeq(
                        self.reconstruct_expression(left, &Default::default()).0,
                        self.reconstruct_expression(right, &Default::default()).0,
                    ),
                },
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        (
            AssignStatement {
                place: self.reconstruct_expression(input.place, &Default::default()).0,
                value: self.reconstruct_expression(input.value, &Default::default()).0,
                ..input
            }
            .into(),
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
            ConditionalStatement {
                condition: self.reconstruct_expression(input.condition, &Default::default()).0,
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
                value: self.reconstruct_expression(input.value, &Default::default()).0,
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
                value: self.reconstruct_expression(input.value, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ExpressionStatement {
                expression: self.reconstruct_expression(input.expression, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        (
            IterationStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                start: self.reconstruct_expression(input.start, &Default::default()).0,
                stop: self.reconstruct_expression(input.stop, &Default::default()).0,
                block: self.reconstruct_block(input.block).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ReturnStatement {
                expression: self.reconstruct_expression(input.expression, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}

/// A Reconstructor trait for the program represented by the AST.
pub trait ProgramReconstructor: AstReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        let stubs = input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();
        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.reconstruct_program_scope(scope))).collect();
        let modules = input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect();

        Program { modules, imports: input.imports, stubs, program_scopes }
    }

    fn reconstruct_aleo_program(&mut self, input: AleoProgram) -> AleoProgram {
        AleoProgram {
            imports: input.imports,
            stub_id: input.stub_id,
            consts: input.consts,
            composites: input.composites,
            mappings: input.mappings,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_stub(f))).collect(),
            span: input.span,
        }
    }

    fn reconstruct_stub(&mut self, input: Stub) -> Stub {
        match input {
            Stub::FromLeo { program, parents } => Stub::FromLeo { program: self.reconstruct_program(program), parents },
            Stub::FromAleo { program, parents } => {
                Stub::FromAleo { program: self.reconstruct_aleo_program(program), parents }
            }
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.clone(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: input.composites.into_iter().map(|(i, c)| (i, self.reconstruct_composite(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        Module {
            program_name: input.program_name,
            path: input.path,
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: input.composites.into_iter().map(|(i, c)| (i, self.reconstruct_composite(c))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
        }
    }

    fn reconstruct_interface(&mut self, input: Interface) -> Interface {
        Interface {
            identifier: input.identifier,
            parents: input.parents.clone(),
            span: input.span,
            id: input.id,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_prototype(f))).collect(),
            records: input.records.into_iter().map(|(i, f)| (i, self.reconstruct_record_prototype(f))).collect(),
        }
    }

    fn reconstruct_function_prototype(&mut self, input: FunctionPrototype) -> FunctionPrototype {
        FunctionPrototype {
            annotations: input.annotations,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| {
                    let mut param = param.clone();
                    param.type_ = self.reconstruct_type(param.type_).0;
                    param
                })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| {
                    let mut input = input.clone();
                    input.type_ = self.reconstruct_type(input.type_).0;
                    input
                })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| {
                    let mut output = output.clone();
                    output.type_ = self.reconstruct_type(output.type_).0;
                    output
                })
                .collect(),
            output_type: self.reconstruct_type(input.output_type).0,
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_record_prototype(&mut self, input: RecordPrototype) -> RecordPrototype {
        RecordPrototype { identifier: input.identifier, span: input.span, id: input.id }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| {
                    let mut param = param.clone();
                    param.type_ = self.reconstruct_type(param.type_).0;
                    param
                })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| {
                    let mut input = input.clone();
                    input.type_ = self.reconstruct_type(input.type_).0;
                    input
                })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| {
                    let mut output = output.clone();
                    output.type_ = self.reconstruct_type(output.type_).0;
                    output
                })
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

    fn reconstruct_composite(&mut self, input: Composite) -> Composite {
        Composite {
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| {
                    let mut param = param.clone();
                    param.type_ = self.reconstruct_type(param.type_).0;
                    param
                })
                .collect(),
            members: input
                .members
                .iter()
                .map(|member| {
                    let mut member = member.clone();
                    member.type_ = self.reconstruct_type(member.type_).0;
                    member
                })
                .collect(),
            ..input
        }
    }

    fn reconstruct_mapping(&mut self, input: Mapping) -> Mapping {
        Mapping {
            key_type: self.reconstruct_type(input.key_type).0,
            value_type: self.reconstruct_type(input.value_type).0,
            ..input
        }
    }

    fn reconstruct_storage_variable(&mut self, input: StorageVariable) -> StorageVariable {
        StorageVariable { type_: self.reconstruct_type(input.type_).0, ..input }
    }
}

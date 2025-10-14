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

//! This module contains Visitor trait implementations for the AST.
//! It implements default methods for each node to be made
//! given the type of node its visiting.

use crate::*;

// TODO: The Visitor and Reconstructor patterns need a redesign so that the default implementation can easily be invoked though its implemented in an overriding trait.
// Here is a pattern that seems to work
// trait ProgramVisitor {
//     // The trait method that can be overridden
//     fn visit_program_scope(&mut self);
//
//     // Private helper function containing the default implementation
//     fn default_visit_program_scope(&mut self) {
//         println!("Do default stuff");
//     }
// }
//
// struct YourStruct;
//
// impl ProgramVisitor for YourStruct {
//     fn visit_program_scope(&mut self) {
//         println!("Do custom stuff.");
//         // Call the default implementation
//         self.default_visit_program_scope();
//     }
// }

/// A Visitor trait for types in the AST.
pub trait AstVisitor {
    /* Types */
    fn visit_type(&mut self, input: &Type) {
        match input {
            Type::Array(array_type) => self.visit_array_type(array_type),
            Type::Composite(composite_type) => self.visit_composite_type(composite_type),
            Type::Future(future_type) => self.visit_future_type(future_type),
            Type::Mapping(mapping_type) => self.visit_mapping_type(mapping_type),
            Type::Optional(optional_type) => self.visit_optional_type(optional_type),
            Type::Tuple(tuple_type) => self.visit_tuple_type(tuple_type),
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
            | Type::Err => {}
        }
    }

    fn visit_array_type(&mut self, input: &ArrayType) {
        self.visit_type(&input.element_type);
        self.visit_expression(&input.length, &Default::default());
    }

    fn visit_composite_type(&mut self, input: &CompositeType) {
        input.const_arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
    }

    fn visit_future_type(&mut self, input: &FutureType) {
        input.inputs.iter().for_each(|input| self.visit_type(input));
    }

    fn visit_mapping_type(&mut self, input: &MappingType) {
        self.visit_type(&input.key);
        self.visit_type(&input.value);
    }

    fn visit_optional_type(&mut self, input: &OptionalType) {
        self.visit_type(&input.inner);
    }

    fn visit_tuple_type(&mut self, input: &TupleType) {
        input.elements().iter().for_each(|input| self.visit_type(input));
    }

    /* Expressions */
    type AdditionalInput: Default;
    type Output: Default;

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::ArrayAccess(access) => self.visit_array_access(access, additional),
            Expression::AssociatedConstant(constant) => self.visit_associated_constant(constant, additional),
            Expression::AssociatedFunction(function) => self.visit_associated_function(function, additional),
            Expression::Async(async_) => self.visit_async(async_, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Path(path) => self.visit_path(path, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Locator(locator) => self.visit_locator(locator, additional),
            Expression::MemberAccess(access) => self.visit_member_access(access, additional),
            Expression::Repeat(repeat) => self.visit_repeat(repeat, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::TupleAccess(access) => self.visit_tuple_access(access, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        }
    }

    fn visit_array_access(&mut self, input: &ArrayAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.array, &Default::default());
        self.visit_expression(&input.index, &Default::default());
        Default::default()
    }

    fn visit_member_access(&mut self, input: &MemberAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.inner, &Default::default());
        Default::default()
    }

    fn visit_tuple_access(&mut self, input: &TupleAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.tuple, &Default::default());
        Default::default()
    }

    fn visit_array(&mut self, input: &ArrayExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        Default::default()
    }

    fn visit_associated_constant(
        &mut self,
        _input: &AssociatedConstantExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        Default::default()
    }

    fn visit_associated_function(
        &mut self,
        input: &AssociatedFunctionExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        input.arguments.iter().for_each(|arg| {
            self.visit_expression(arg, &Default::default());
        });
        Default::default()
    }

    fn visit_async(&mut self, input: &AsyncExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_block(&input.block);
        Default::default()
    }

    fn visit_binary(&mut self, input: &BinaryExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.left, &Default::default());
        self.visit_expression(&input.right, &Default::default());
        Default::default()
    }

    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        input.const_arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        Default::default()
    }

    fn visit_cast(&mut self, input: &CastExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.expression, &Default::default());
        Default::default()
    }

    fn visit_struct_init(&mut self, input: &StructExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        input.const_arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        for StructVariableInitializer { expression, .. } in input.members.iter() {
            if let Some(expression) = expression {
                self.visit_expression(expression, &Default::default());
            }
        }
        Default::default()
    }

    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_path(&mut self, _input: &Path, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_literal(&mut self, _input: &Literal, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_locator(&mut self, _input: &LocatorExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_repeat(&mut self, input: &RepeatExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.expr, &Default::default());
        self.visit_expression(&input.count, &Default::default());
        Default::default()
    }

    fn visit_ternary(&mut self, input: &TernaryExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_expression(&input.if_true, &Default::default());
        self.visit_expression(&input.if_false, &Default::default());
        Default::default()
    }

    fn visit_tuple(&mut self, input: &TupleExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        Default::default()
    }

    fn visit_unary(&mut self, input: &UnaryExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.receiver, &Default::default());
        Default::default()
    }

    fn visit_unit(&mut self, _input: &UnitExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    /* Statements */
    fn visit_statement(&mut self, input: &Statement) {
        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Const(stmt) => self.visit_const(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => self.visit_expression(expr, &Default::default()),
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default())
            }
        };
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        self.visit_expression(&input.place, &Default::default());
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_block(&mut self, input: &Block) {
        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
    }

    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_block(&input.then);
        if let Some(stmt) = input.otherwise.as_ref() {
            self.visit_statement(stmt);
        }
    }

    fn visit_const(&mut self, input: &ConstDeclaration) {
        self.visit_type(&input.type_);
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        if let Some(ty) = input.type_.as_ref() {
            self.visit_type(ty)
        }
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        if let Some(ty) = input.type_.as_ref() {
            self.visit_type(ty)
        }
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
    }

    fn visit_return(&mut self, input: &ReturnStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }
}

/// A Visitor trait for the program represented by the AST.
pub trait ProgramVisitor: AstVisitor {
    fn visit_program(&mut self, input: &Program) {
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
        input.modules.values().for_each(|module| self.visit_module(module));
        input.imports.values().for_each(|import| self.visit_import(&import.0));
        input.stubs.values().for_each(|stub| self.visit_stub(stub));
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        input.consts.iter().for_each(|(_, c)| (self.visit_const(c)));
        input.structs.iter().for_each(|(_, c)| (self.visit_struct(c)));
        input.mappings.iter().for_each(|(_, c)| (self.visit_mapping(c)));
        input.functions.iter().for_each(|(_, c)| (self.visit_function(c)));
        if let Some(c) = input.constructor.as_ref() {
            self.visit_constructor(c);
        }
    }

    fn visit_module(&mut self, input: &Module) {
        input.consts.iter().for_each(|(_, c)| (self.visit_const(c)));
        input.structs.iter().for_each(|(_, c)| (self.visit_struct(c)));
        input.functions.iter().for_each(|(_, c)| (self.visit_function(c)));
    }

    fn visit_stub(&mut self, _input: &Stub) {}

    fn visit_import(&mut self, input: &Program) {
        self.visit_program(input)
    }

    fn visit_struct(&mut self, input: &Composite) {
        input.const_parameters.iter().for_each(|input| self.visit_type(&input.type_));
        input.members.iter().for_each(|member| self.visit_type(&member.type_));
    }

    fn visit_mapping(&mut self, input: &Mapping) {
        self.visit_type(&input.key_type);
        self.visit_type(&input.value_type);
    }

    fn visit_function(&mut self, input: &Function) {
        input.const_parameters.iter().for_each(|input| self.visit_type(&input.type_));
        input.input.iter().for_each(|input| self.visit_type(&input.type_));
        input.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&input.output_type);
        self.visit_block(&input.block);
    }

    fn visit_constructor(&mut self, input: &Constructor) {
        self.visit_block(&input.block);
    }

    fn visit_function_stub(&mut self, _input: &FunctionStub) {}

    fn visit_struct_stub(&mut self, _input: &Composite) {}
}

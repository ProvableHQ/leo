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

/// A Visitor trait for expressions in the AST.
pub trait ExpressionVisitor {
    type AdditionalInput: Default;
    type Output: Default;

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::ArrayAccess(access) => self.visit_array_access(access, additional),
            Expression::AssociatedConstant(constant) => self.visit_associated_constant(constant, additional),
            Expression::AssociatedFunction(function) => self.visit_associated_function(function, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Identifier(identifier) => self.visit_identifier(identifier, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Locator(locator) => self.visit_locator(locator, additional),
            Expression::MemberAccess(access) => self.visit_member_access(access, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::TupleAccess(access) => self.visit_tuple_access(access, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        }
    }

    fn visit_array_access(&mut self, input: &ArrayAccess, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.array, additional);
        self.visit_expression(&input.index, additional);
        Default::default()
    }

    fn visit_member_access(&mut self, input: &MemberAccess, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.inner, additional);
        Default::default()
    }

    fn visit_tuple_access(&mut self, input: &TupleAccess, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.tuple, additional);
        Default::default()
    }

    fn visit_array(&mut self, input: &ArrayExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
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

    fn visit_binary(&mut self, input: &BinaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.left, additional);
        self.visit_expression(&input.right, additional);
        Default::default()
    }

    fn visit_call(&mut self, input: &CallExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_cast(&mut self, input: &CastExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.expression, additional);
        Default::default()
    }

    fn visit_struct_init(&mut self, input: &StructExpression, additional: &Self::AdditionalInput) -> Self::Output {
        for StructVariableInitializer { expression, .. } in input.members.iter() {
            if let Some(expression) = expression {
                self.visit_expression(expression, additional);
            }
        }
        Default::default()
    }

    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_identifier(&mut self, _input: &Identifier, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_literal(&mut self, _input: &Literal, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_locator(&mut self, _input: &LocatorExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_ternary(&mut self, input: &TernaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, additional);
        self.visit_expression(&input.if_true, additional);
        self.visit_expression(&input.if_false, additional);
        Default::default()
    }

    fn visit_tuple(&mut self, input: &TupleExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_unary(&mut self, input: &UnaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.receiver, additional);
        Default::default()
    }

    fn visit_unit(&mut self, _input: &UnitExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }
}

/// A Visitor trait for statements in the AST.
pub trait StatementVisitor: ExpressionVisitor {
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
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
    }

    fn visit_return(&mut self, input: &ReturnStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }
}

/// A Visitor trait for the program represented by the AST.
pub trait ProgramVisitor: StatementVisitor {
    fn visit_program(&mut self, input: &Program) {
        input.imports.values().for_each(|import| self.visit_import(&import.0));
        input.stubs.values().for_each(|stub| self.visit_stub(stub));
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        input.structs.iter().for_each(|(_, c)| (self.visit_struct(c)));

        input.mappings.iter().for_each(|(_, c)| (self.visit_mapping(c)));

        input.functions.iter().for_each(|(_, c)| (self.visit_function(c)));

        input.consts.iter().for_each(|(_, c)| (self.visit_const(c)));
    }

    fn visit_stub(&mut self, _input: &Stub) {}

    fn visit_import(&mut self, input: &Program) {
        self.visit_program(input)
    }

    fn visit_struct(&mut self, _input: &Composite) {}

    fn visit_mapping(&mut self, _input: &Mapping) {}

    fn visit_function(&mut self, input: &Function) {
        self.visit_block(&input.block);
    }

    fn visit_function_stub(&mut self, _input: &FunctionStub) {}

    fn visit_struct_stub(&mut self, _input: &Composite) {}
}

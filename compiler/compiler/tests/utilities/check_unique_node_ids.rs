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

use leo_ast::*;

use std::{collections::HashSet, marker::PhantomData};

/// A utility that checks that each node in the AST has a unique `NodeID`.
pub struct CheckUniqueNodeIds<'a> {
    /// The set of `NodeID`s that have been seen.
    seen: HashSet<NodeID>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> CheckUniqueNodeIds<'a> {
    /// Creates a new `CheckUniqueNodeId`.
    pub fn new() -> Self {
        Self { seen: HashSet::new(), _phantom: PhantomData }
    }

    /// Checks that the given `NodeID` has not been seen before.
    pub fn check(&mut self, id: NodeID) {
        if !self.seen.insert(id) {
            panic!("Duplicate NodeID found in the AST: {}", id);
        }
    }

    /// Checks that the given `Type` has a unique `NodeID`.
    pub fn check_ty(&mut self, ty: &'a Type) {
        match ty {
            Type::Identifier(identifier) => self.visit_identifier(identifier, &Default::default()),
            Type::Mapping(mapping) => {
                self.check_ty(&mapping.key);
                self.check_ty(&mapping.value);
            }
            Type::Tuple(tuple) => {
                for ty in tuple.elements() {
                    self.check_ty(ty);
                }
            }
            _ => {}
        }
    }
}

impl<'a> ExpressionVisitor<'a> for CheckUniqueNodeIds<'a> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_access(&mut self, input: &'a AccessExpression, _: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::Array(ArrayAccess { array, index, id, .. }) => {
                self.visit_expression(array, &Default::default());
                self.visit_expression(index, &Default::default());
                self.check(*id);
            }
            AccessExpression::AssociatedConstant(AssociatedConstant { ty, name, id, .. }) => {
                self.check_ty(ty);
                self.visit_identifier(name, &Default::default());
                self.check(*id);
            }
            AccessExpression::AssociatedFunction(AssociatedFunction { ty, name, arguments, id, .. }) => {
                self.check_ty(ty);
                self.visit_identifier(name, &Default::default());
                for argument in arguments {
                    self.visit_expression(argument, &Default::default());
                }
                self.check(*id);
            }
            AccessExpression::Member(MemberAccess { inner, name, id, .. }) => {
                self.visit_expression(inner, &Default::default());
                self.visit_identifier(name, &Default::default());
                self.check(*id);
            }
            AccessExpression::Tuple(TupleAccess { tuple, id, .. }) => {
                self.visit_expression(tuple, &Default::default());
                self.check(*id);
            }
        }
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, _: &Self::AdditionalInput) -> Self::Output {
        let BinaryExpression { left, right, id, .. } = input;
        self.visit_expression(left, &Default::default());
        self.visit_expression(right, &Default::default());
        self.check(*id);
    }

    fn visit_call(&mut self, input: &'a CallExpression, _: &Self::AdditionalInput) -> Self::Output {
        let CallExpression { function, arguments, external, id, .. } = input;
        self.visit_expression(function, &Default::default());
        for argument in arguments {
            self.visit_expression(argument, &Default::default());
        }
        if let Some(external) = external {
            self.visit_expression(external, &Default::default());
        }
        self.check(*id);
    }

    fn visit_cast(&mut self, input: &'a CastExpression, _: &Self::AdditionalInput) -> Self::Output {
        let CastExpression { expression, type_, id, .. } = input;
        self.visit_expression(expression, &Default::default());
        self.check_ty(type_);
        self.check(*id);
    }

    fn visit_struct_init(&mut self, input: &'a StructExpression, _: &Self::AdditionalInput) -> Self::Output {
        let StructExpression { name, members, id, .. } = input;
        self.visit_identifier(name, &Default::default());
        for StructVariableInitializer { identifier, expression, id, .. } in members {
            self.visit_identifier(identifier, &Default::default());
            if let Some(expression) = expression {
                self.visit_expression(expression, &Default::default());
            }
            self.check(*id);
        }
        self.check(*id);
    }

    fn visit_err(&mut self, input: &'a ErrExpression, _: &Self::AdditionalInput) -> Self::Output {
        self.check(input.id);
    }

    fn visit_identifier(&mut self, input: &'a Identifier, _: &Self::AdditionalInput) -> Self::Output {
        self.check(input.id)
    }

    fn visit_literal(&mut self, input: &'a Literal, _: &Self::AdditionalInput) -> Self::Output {
        self.check(input.id())
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, _: &Self::AdditionalInput) -> Self::Output {
        let TernaryExpression { condition, if_true, if_false, id, .. } = input;
        self.visit_expression(condition, &Default::default());
        self.visit_expression(if_true, &Default::default());
        self.visit_expression(if_false, &Default::default());
        self.check(*id);
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression, _: &Self::AdditionalInput) -> Self::Output {
        let TupleExpression { elements, id, .. } = input;
        for element in elements {
            self.visit_expression(element, &Default::default());
        }
        self.check(*id);
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, _: &Self::AdditionalInput) -> Self::Output {
        let UnaryExpression { receiver, id, .. } = input;
        self.visit_expression(receiver, &Default::default());
        self.check(*id);
    }

    fn visit_unit(&mut self, input: &'a UnitExpression, _: &Self::AdditionalInput) -> Self::Output {
        self.check(input.id)
    }
}

impl<'a> StatementVisitor<'a> for CheckUniqueNodeIds<'a> {
    fn visit_assert(&mut self, input: &'a AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => self.visit_expression(expr, &Default::default()),
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default())
            }
        };
        self.check(input.id)
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        self.visit_expression(&input.place, &Default::default());
        self.visit_expression(&input.value, &Default::default());
        self.check(input.id)
    }

    fn visit_block(&mut self, input: &'a Block) {
        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
        self.check(input.id)
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_block(&input.then);
        if let Some(stmt) = input.otherwise.as_ref() {
            self.visit_statement(stmt);
        }
        self.check(input.id)
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.visit_expression(expr, &Default::default());
            }
            ConsoleFunction::AssertEq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default());
            }
            ConsoleFunction::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default());
            }
        };
        self.check(input.id)
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        self.visit_expression(&input.place, &Default::default());
        self.check_ty(&input.type_);
        self.visit_expression(&input.value, &Default::default());
        self.check(input.id)
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
        self.visit_expression(&input.expression, &Default::default());
        self.check(input.id)
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        self.visit_identifier(&input.variable, &Default::default());
        self.check_ty(&input.type_);
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
        self.check(input.id)
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        self.visit_expression(&input.expression, &Default::default());
        if let Some(arguments) = &input.finalize_arguments {
            arguments.iter().for_each(|argument| {
                self.visit_expression(argument, &Default::default());
            })
        }
        self.check(input.id)
    }
}

impl<'a> ProgramVisitor<'a> for CheckUniqueNodeIds<'a> {
    fn visit_struct(&mut self, input: &'a Struct) {
        let Struct { identifier, members, id, .. } = input;
        self.visit_identifier(identifier, &Default::default());
        for Member { identifier, type_, id, .. } in members {
            self.visit_identifier(identifier, &Default::default());
            self.check_ty(type_);
            self.check(*id);
        }
        self.check(*id);
    }

    fn visit_mapping(&mut self, input: &'a Mapping) {
        let Mapping { identifier, key_type, value_type, id, .. } = input;
        self.visit_identifier(identifier, &Default::default());
        self.check_ty(key_type);
        self.check_ty(value_type);
        self.check(*id);
    }

    fn visit_function(&mut self, input: &'a Function) {
        let Function { annotations, identifier, input, output, block, finalize, id, .. } = input;
        // Check the annotations.
        for Annotation { identifier, id, .. } in annotations {
            self.visit_identifier(identifier, &Default::default());
            self.check(*id);
        }
        // Check the function name.
        self.visit_identifier(identifier, &Default::default());
        // Check the inputs.
        for in_ in input {
            match in_ {
                Input::Internal(FunctionInput { identifier, type_, id, .. }) => {
                    self.visit_identifier(identifier, &Default::default());
                    self.check_ty(type_);
                    self.check(*id);
                }
                Input::External(External { identifier, program_name, record, id, .. }) => {
                    self.visit_identifier(identifier, &Default::default());
                    self.visit_identifier(program_name, &Default::default());
                    self.visit_identifier(record, &Default::default());
                    self.check(*id);
                }
            }
        }
        // Check the outputs.
        for out in output {
            match out {
                Output::Internal(FunctionOutput { type_, id, .. }) => {
                    self.check_ty(type_);
                    self.check(*id);
                }
                Output::External(External { identifier, program_name, record, id, .. }) => {
                    self.visit_identifier(identifier, &Default::default());
                    self.visit_identifier(program_name, &Default::default());
                    self.visit_identifier(record, &Default::default());
                    self.check(*id);
                }
            }
        }
        // Check the function body.
        self.visit_block(block);
        // Check the finalize block.
        if let Some(Finalize { identifier, input, output, block, id, .. }) = finalize {
            // Check the finalize name.
            self.visit_identifier(identifier, &Default::default());
            // Check the inputs.
            for in_ in input {
                match in_ {
                    Input::Internal(FunctionInput { identifier, type_, id, .. }) => {
                        self.visit_identifier(identifier, &Default::default());
                        self.check_ty(type_);
                        self.check(*id);
                    }
                    Input::External(External { identifier, program_name, record, id, .. }) => {
                        self.visit_identifier(identifier, &Default::default());
                        self.visit_identifier(program_name, &Default::default());
                        self.visit_identifier(record, &Default::default());
                        self.check(*id);
                    }
                }
            }
            // Check the outputs.
            for out in output {
                match out {
                    Output::Internal(FunctionOutput { type_, id, .. }) => {
                        self.check_ty(type_);
                        self.check(*id);
                    }
                    Output::External(External { identifier, program_name, record, id, .. }) => {
                        self.visit_identifier(identifier, &Default::default());
                        self.visit_identifier(program_name, &Default::default());
                        self.visit_identifier(record, &Default::default());
                        self.check(*id);
                    }
                }
            }
            // Check the function body.
            self.visit_block(block);
            self.check(*id);
        }
        self.check(*id);
    }
}

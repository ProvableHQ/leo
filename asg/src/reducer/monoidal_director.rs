// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use super::*;
use crate::{accesses::*, expression::*, program::*, statement::*};

use std::marker::PhantomData;

pub struct MonoidalDirector<'a, T: Monoid, R: MonoidalReducerExpression<'a, T>> {
    reducer: R,
    _monoid: PhantomData<&'a T>,
}

impl<'a, T: Monoid, R: MonoidalReducerExpression<'a, T>> MonoidalDirector<'a, T, R> {
    pub fn new(reducer: R) -> Self {
        Self {
            reducer,
            _monoid: PhantomData,
        }
    }

    pub fn reducer(self) -> R {
        self.reducer
    }

    pub fn reduce_expression(&mut self, input: &'a Expression<'a>) -> T {
        let value = match input {
            Expression::Err(e) => self.reduce_err(e),
            Expression::ArrayInit(e) => self.reduce_array_init(e),
            Expression::ArrayInline(e) => self.reduce_array_inline(e),
            Expression::Binary(e) => self.reduce_binary(e),
            Expression::Call(e) => self.reduce_call(e),
            Expression::StructInit(e) => self.reduce_struct_init(e),
            Expression::Ternary(e) => self.reduce_ternary_expression(e),
            Expression::Cast(e) => self.reduce_cast_expression(e),
            Expression::Access(e) => self.reduce_access_expression(e),
            Expression::Constant(e) => self.reduce_constant(e),
            Expression::TupleInit(e) => self.reduce_tuple_init(e),
            Expression::Unary(e) => self.reduce_unary(e),
            Expression::VariableRef(e) => self.reduce_variable_ref(e),
        };

        self.reducer.reduce_expression(input, value)
    }

    pub fn reduce_err(&mut self, input: &ErrExpression<'a>) -> T {
        self.reducer.reduce_err(input)
    }

    pub fn reduce_array_init(&mut self, input: &ArrayInitExpression<'a>) -> T {
        let element = self.reduce_expression(input.element.get());

        self.reducer.reduce_array_init(input, element)
    }

    pub fn reduce_array_inline(&mut self, input: &ArrayInlineExpression<'a>) -> T {
        let elements = input
            .elements
            .iter()
            .map(|(x, _)| self.reduce_expression(x.get()))
            .collect();

        self.reducer.reduce_array_inline(input, elements)
    }

    pub fn reduce_binary(&mut self, input: &BinaryExpression<'a>) -> T {
        let left = self.reduce_expression(input.left.get());
        let right = self.reduce_expression(input.right.get());

        self.reducer.reduce_binary(input, left, right)
    }

    pub fn reduce_call(&mut self, input: &CallExpression<'a>) -> T {
        let target = input.target.get().map(|e| self.reduce_expression(e));
        let arguments = input
            .arguments
            .iter()
            .map(|e| self.reduce_expression(e.get()))
            .collect();

        self.reducer.reduce_call(input, target, arguments)
    }

    pub fn reduce_struct_init(&mut self, input: &StructInitExpression<'a>) -> T {
        let values = input
            .values
            .iter()
            .map(|(_, e)| self.reduce_expression(e.get()))
            .collect();

        self.reducer.reduce_struct_init(input, values)
    }

    pub fn reduce_ternary_expression(&mut self, input: &TernaryExpression<'a>) -> T {
        let condition = self.reduce_expression(input.condition.get());
        let if_true = self.reduce_expression(input.if_true.get());
        let if_false = self.reduce_expression(input.if_false.get());

        self.reducer
            .reduce_ternary_expression(input, condition, if_true, if_false)
    }

    pub fn reduce_cast_expression(&mut self, input: &CastExpression<'a>) -> T {
        let inner = self.reduce_expression(input.inner.get());

        self.reducer.reduce_cast_expression(input, inner)
    }

    pub fn reduce_array_access(&mut self, input: &ArrayAccess<'a>) -> T {
        let array = self.reduce_expression(input.array.get());
        let index = self.reduce_expression(input.index.get());

        self.reducer.reduce_array_access(input, array, index)
    }

    pub fn reduce_array_range_access(&mut self, input: &ArrayRangeAccess<'a>) -> T {
        let array = self.reduce_expression(input.array.get());
        let left = input.left.get().map(|e| self.reduce_expression(e));
        let right = input.right.get().map(|e| self.reduce_expression(e));

        self.reducer.reduce_array_range_access(input, array, left, right)
    }

    pub fn reduce_struct_access(&mut self, input: &StructAccess<'a>) -> T {
        let target = input.target.get().map(|e| self.reduce_expression(e));

        self.reducer.reduce_struct_access(input, target)
    }

    pub fn reduce_constant(&mut self, input: &Constant<'a>) -> T {
        self.reducer.reduce_constant(input)
    }

    pub fn reduce_tuple_access(&mut self, input: &TupleAccess<'a>) -> T {
        let tuple_ref = self.reduce_expression(input.tuple_ref.get());

        self.reducer.reduce_tuple_access(input, tuple_ref)
    }

    pub fn reduce_access_expression(&mut self, input: &AccessExpression<'a>) -> T {
        use AccessExpression::*;

        match input {
            Array(a) => self.reduce_array_access(a),
            ArrayRange(a) => self.reduce_array_range_access(a),
            Struct(a) => self.reduce_struct_access(a),
            Tuple(a) => self.reduce_tuple_access(a),
        }
    }

    pub fn reduce_tuple_init(&mut self, input: &TupleInitExpression<'a>) -> T {
        let values = input.elements.iter().map(|e| self.reduce_expression(e.get())).collect();

        self.reducer.reduce_tuple_init(input, values)
    }

    pub fn reduce_unary(&mut self, input: &UnaryExpression<'a>) -> T {
        let inner = self.reduce_expression(input.inner.get());

        self.reducer.reduce_unary(input, inner)
    }

    pub fn reduce_variable_ref(&mut self, input: &VariableRef<'a>) -> T {
        self.reducer.reduce_variable_ref(input)
    }
}

impl<'a, T: Monoid, R: MonoidalReducerStatement<'a, T>> MonoidalDirector<'a, T, R> {
    pub fn reduce_statement(&mut self, input: &'a Statement<'a>) -> T {
        let value = match input {
            Statement::Assign(s) => self.reduce_assign(s),
            Statement::Block(s) => self.reduce_block(s),
            Statement::Conditional(s) => self.reduce_conditional_statement(s),
            Statement::Console(s) => self.reduce_console(s),
            Statement::Definition(s) => self.reduce_definition(s),
            Statement::Expression(s) => self.reduce_expression_statement(s),
            Statement::Iteration(s) => self.reduce_iteration(s),
            Statement::Return(s) => self.reduce_return(s),
            Statement::Empty(_) => T::default(),
        };

        self.reducer.reduce_statement(input, value)
    }

    pub fn reduce_assign_access(&mut self, input: &AssignAccess<'a>) -> T {
        let (left, right) = match input {
            AssignAccess::ArrayRange(left, right) => (
                left.get().map(|e| self.reduce_expression(e)),
                right.get().map(|e| self.reduce_expression(e)),
            ),
            AssignAccess::ArrayIndex(index) => (Some(self.reduce_expression(index.get())), None),
            _ => (None, None),
        };

        self.reducer.reduce_assign_access(input, left, right)
    }

    pub fn reduce_assign(&mut self, input: &AssignStatement<'a>) -> T {
        let accesses = input
            .target_accesses
            .iter()
            .map(|x| self.reduce_assign_access(x))
            .collect();
        let value = self.reduce_expression(input.value.get());

        self.reducer.reduce_assign(input, accesses, value)
    }

    pub fn reduce_block(&mut self, input: &BlockStatement<'a>) -> T {
        let statements = input
            .statements
            .iter()
            .map(|x| self.reduce_statement(x.get()))
            .collect();

        self.reducer.reduce_block(input, statements)
    }

    pub fn reduce_conditional_statement(&mut self, input: &ConditionalStatement<'a>) -> T {
        let condition = self.reduce_expression(input.condition.get());
        let if_true = self.reduce_statement(input.result.get());
        let if_false = input.next.get().map(|s| self.reduce_statement(s));

        self.reducer
            .reduce_conditional_statement(input, condition, if_true, if_false)
    }

    pub fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>) -> T {
        let parameters = input
            .parameters
            .iter()
            .map(|e| self.reduce_expression(e.get()))
            .collect();

        self.reducer.reduce_formatted_string(input, parameters)
    }

    pub fn reduce_console(&mut self, input: &ConsoleStatement<'a>) -> T {
        let argument = match &input.function {
            ConsoleFunction::Assert(e) => self.reduce_expression(e.get()),
            ConsoleFunction::Error(f) | ConsoleFunction::Log(f) => self.reduce_formatted_string(f),
        };

        self.reducer.reduce_console(input, argument)
    }

    pub fn reduce_definition(&mut self, input: &DefinitionStatement<'a>) -> T {
        let value = self.reduce_expression(input.value.get());

        self.reducer.reduce_definition(input, value)
    }

    pub fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>) -> T {
        let value = self.reduce_expression(input.expression.get());

        self.reducer.reduce_expression_statement(input, value)
    }

    pub fn reduce_iteration(&mut self, input: &IterationStatement<'a>) -> T {
        let start = self.reduce_expression(input.start.get());
        let stop = self.reduce_expression(input.stop.get());
        let body = self.reduce_statement(input.body.get());

        self.reducer.reduce_iteration(input, start, stop, body)
    }

    pub fn reduce_return(&mut self, input: &ReturnStatement<'a>) -> T {
        let value = self.reduce_expression(input.expression.get());

        self.reducer.reduce_return(input, value)
    }
}

impl<'a, T: Monoid, R: MonoidalReducerProgram<'a, T>> MonoidalDirector<'a, T, R> {
    pub fn reduce_function(&mut self, input: &'a Function<'a>) -> T {
        let body = input.body.get().map(|s| self.reduce_statement(s)).unwrap_or_default();

        self.reducer.reduce_function(input, body)
    }

    pub fn reduce_struct_member(&mut self, input: &StructMember<'a>) -> T {
        let function = match input {
            StructMember::Function(f) => Some(self.reduce_function(f)),
            _ => None,
        };

        self.reducer.reduce_struct_member(input, function)
    }

    pub fn reduce_struct(&mut self, input: &'a Struct<'a>) -> T {
        let members = input
            .members
            .borrow()
            .iter()
            .map(|(_, member)| self.reduce_struct_member(member))
            .collect();

        self.reducer.reduce_struct(input, members)
    }

    pub fn reduce_program(&mut self, input: &Program<'a>) -> T {
        let imported_modules = input
            .imported_modules
            .iter()
            .map(|(_, import)| self.reduce_program(import))
            .collect();
        let functions = input.functions.iter().map(|(_, f)| self.reduce_function(f)).collect();
        let structs = input.structs.iter().map(|(_, c)| self.reduce_struct(c)).collect();

        self.reducer.reduce_program(input, imported_modules, functions, structs)
    }
}

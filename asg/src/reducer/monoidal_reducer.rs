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

use crate::{accesses::*, expression::*, program::*, statement::*, Monoid, Variable};

#[allow(unused_variables)]
pub trait MonoidalReducerExpression<'a, T: Monoid> {
    fn reduce_expression(&mut self, input: &'a Expression<'a>, value: T) -> T {
        value
    }

    fn reduce_err(&mut self, input: &ErrExpression<'a>) -> T {
        T::default()
    }

    fn reduce_array_init(&mut self, input: &ArrayInitExpression<'a>, element: T) -> T {
        element
    }

    fn reduce_array_inline(&mut self, input: &'a ArrayInlineExpression<'a>, elements: Vec<T>) -> T {
        T::default().append_all(elements.into_iter())
    }

    fn reduce_binary(&mut self, input: &'a BinaryExpression<'a>, left: T, right: T) -> T {
        left.append(right)
    }

    fn reduce_call(&mut self, input: &'a CallExpression<'a>, target: Option<T>, arguments: Vec<T>) -> T {
        target.unwrap_or_default().append_all(arguments.into_iter())
    }

    fn reduce_circuit_init(&mut self, input: &'a CircuitInitExpression<'a>, values: Vec<T>) -> T {
        T::default().append_all(values.into_iter())
    }

    fn reduce_ternary_expression(
        &mut self,
        input: &'a TernaryExpression<'a>,
        condition: T,
        if_true: T,
        if_false: T,
    ) -> T {
        condition.append(if_true).append(if_false)
    }

    fn reduce_cast_expression(&mut self, input: &'a CastExpression<'a>, inner: T) -> T {
        inner
    }

    fn reduce_array_access(&mut self, input: &'a ArrayAccess<'a>, array: T, index: T) -> T {
        array.append(index)
    }

    fn reduce_constant(&mut self, input: &'a Constant<'a>) -> T {
        T::default()
    }

    fn reduce_array_range_access(
        &mut self,
        input: &'a ArrayRangeAccess<'a>,
        array: T,
        left: Option<T>,
        right: Option<T>,
    ) -> T {
        array.append_option(left).append_option(right)
    }

    fn reduce_circuit_access(&mut self, input: &'a CircuitAccess<'a>, target: Option<T>) -> T {
        target.unwrap_or_default()
    }

    fn reduce_tuple_access(&mut self, input: &'a TupleAccess<'a>, tuple_ref: T) -> T {
        tuple_ref
    }

    fn reduce_tuple_init(&mut self, input: &'a TupleInitExpression<'a>, values: Vec<T>) -> T {
        T::default().append_all(values.into_iter())
    }

    fn reduce_unary(&mut self, input: &'a UnaryExpression<'a>, inner: T) -> T {
        inner
    }

    fn reduce_variable(&mut self, input: &'a Variable<'a>) -> T {
        T::default()
    }

    fn reduce_variable_ref(&mut self, input: &'a VariableRef<'a>, variable: T) -> T {
        T::default().append(variable)
    }
}

#[allow(unused_variables)]
pub trait MonoidalReducerStatement<'a, T: Monoid>: MonoidalReducerExpression<'a, T> {
    fn reduce_statement(&mut self, input: &'a Statement<'a>, value: T) -> T {
        value
    }

    // left = Some(ArrayIndex.0) always if AssignAccess::ArrayIndex. if member/tuple, always None
    fn reduce_assign_access(&mut self, input: &AssignAccess<'a>, left: Option<T>, right: Option<T>) -> T {
        left.unwrap_or_default().append_option(right)
    }

    fn reduce_assign(&mut self, input: &AssignStatement<'a>, variable: T, accesses: Vec<T>, value: T) -> T {
        variable.append_all(accesses.into_iter()).append(value)
    }

    fn reduce_block(&mut self, input: &BlockStatement<'a>, statements: Vec<T>) -> T {
        T::default().append_all(statements.into_iter())
    }

    fn reduce_conditional_statement(
        &mut self,
        input: &ConditionalStatement<'a>,
        condition: T,
        if_true: T,
        if_false: Option<T>,
    ) -> T {
        condition.append(if_true).append_option(if_false)
    }

    fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>, parameters: Vec<T>) -> T {
        T::default().append_all(parameters.into_iter())
    }

    fn reduce_console(&mut self, input: &ConsoleStatement<'a>, argument: T) -> T {
        argument
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement<'a>, variables: Vec<T>, value: T) -> T {
        T::default().append_all(variables.into_iter()).append(value)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>, expression: T) -> T {
        expression
    }

    fn reduce_iteration(&mut self, input: &IterationStatement<'a>, variable: T, start: T, stop: T, body: T) -> T {
        variable.append(start).append(stop).append(body)
    }

    fn reduce_return(&mut self, input: &ReturnStatement<'a>, value: T) -> T {
        value
    }
}

#[allow(unused_variables)]
pub trait MonoidalReducerProgram<'a, T: Monoid>: MonoidalReducerStatement<'a, T> {
    fn reduce_function(&mut self, input: &'a Function<'a>, arguments: Vec<T>, body: T) -> T {
        T::default().append_all(arguments.into_iter()).append(body)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMember<'a>, function: Option<T>) -> T {
        function.unwrap_or_default()
    }

    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<T>) -> T {
        T::default().append_all(members.into_iter())
    }

    fn reduce_alias(&mut self, input: &'a Alias<'a>) -> T {
        T::default()
    }

    fn reduce_program(
        &mut self,
        input: &Program,
        imported_modules: Vec<T>,
        aliases: Vec<T>,
        functions: Vec<T>,
        global_consts: Vec<T>,
        circuits: Vec<T>,
    ) -> T {
        T::default()
            .append_all(imported_modules.into_iter())
            .append_all(aliases.into_iter())
            .append_all(functions.into_iter())
            .append_all(global_consts.into_iter())
            .append_all(circuits.into_iter())
    }
}

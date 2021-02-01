// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{expression::*, program::*, statement::*, Monoid};

use std::sync::Arc;

#[allow(unused_variables)]
pub trait MonoidalReducerExpression<T: Monoid> {
    fn reduce_expression(&mut self, input: &Arc<Expression>, value: T) -> T {
        value
    }

    fn reduce_array_access(&mut self, input: &ArrayAccessExpression, array: T, index: T) -> T {
        array.append(index)
    }

    fn reduce_array_init(&mut self, input: &ArrayInitExpression, element: T) -> T {
        element
    }

    fn reduce_array_inline(&mut self, input: &ArrayInlineExpression, elements: Vec<T>) -> T {
        T::default().append_all(elements.into_iter())
    }

    fn reduce_array_range_access(
        &mut self,
        input: &ArrayRangeAccessExpression,
        array: T,
        left: Option<T>,
        right: Option<T>,
    ) -> T {
        array.append_option(left).append_option(right)
    }

    fn reduce_binary(&mut self, input: &BinaryExpression, left: T, right: T) -> T {
        left.append(right)
    }

    fn reduce_call(&mut self, input: &CallExpression, target: Option<T>, arguments: Vec<T>) -> T {
        target.unwrap_or_default().append_all(arguments.into_iter())
    }

    fn reduce_circuit_access(&mut self, input: &CircuitAccessExpression, target: Option<T>) -> T {
        target.unwrap_or_default()
    }

    fn reduce_circuit_init(&mut self, input: &CircuitInitExpression, values: Vec<T>) -> T {
        T::default().append_all(values.into_iter())
    }

    fn reduce_ternary_expression(&mut self, input: &TernaryExpression, condition: T, if_true: T, if_false: T) -> T {
        condition.append(if_true).append(if_false)
    }

    fn reduce_constant(&mut self, input: &Constant) -> T {
        T::default()
    }

    fn reduce_tuple_access(&mut self, input: &TupleAccessExpression, tuple_ref: T) -> T {
        tuple_ref
    }

    fn reduce_tuple_init(&mut self, input: &TupleInitExpression, values: Vec<T>) -> T {
        T::default().append_all(values.into_iter())
    }

    fn reduce_unary(&mut self, input: &UnaryExpression, inner: T) -> T {
        inner
    }

    fn reduce_variable_ref(&mut self, input: &VariableRef) -> T {
        T::default()
    }
}

#[allow(unused_variables)]
pub trait MonoidalReducerStatement<T: Monoid>: MonoidalReducerExpression<T> {
    fn reduce_statement(&mut self, input: &Arc<Statement>, value: T) -> T {
        value
    }

    // left = Some(ArrayIndex.0) always if AssignAccess::ArrayIndex. if member/tuple, always None
    fn reduce_assign_access(&mut self, input: &AssignAccess, left: Option<T>, right: Option<T>) -> T {
        left.unwrap_or_default().append_option(right)
    }

    fn reduce_assign(&mut self, input: &AssignStatement, accesses: Vec<T>, value: T) -> T {
        T::default().append_all(accesses.into_iter()).append(value)
    }

    fn reduce_block(&mut self, input: &BlockStatement, statements: Vec<T>) -> T {
        T::default().append_all(statements.into_iter())
    }

    fn reduce_conditional_statement(
        &mut self,
        input: &ConditionalStatement,
        condition: T,
        if_true: T,
        if_false: Option<T>,
    ) -> T {
        condition.append(if_true).append_option(if_false)
    }

    fn reduce_formatted_string(&mut self, input: &FormattedString, parameters: Vec<T>) -> T {
        T::default().append_all(parameters.into_iter())
    }

    fn reduce_console(&mut self, input: &ConsoleStatement, argument: T) -> T {
        argument
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement, value: T) -> T {
        value
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement, expression: T) -> T {
        expression
    }

    fn reduce_iteration(&mut self, input: &IterationStatement, start: T, stop: T, body: T) -> T {
        start.append(stop).append(body)
    }

    fn reduce_return(&mut self, input: &ReturnStatement, value: T) -> T {
        value
    }
}

#[allow(unused_variables)]
pub trait MonoidalReducerProgram<T: Monoid>: MonoidalReducerStatement<T> {
    fn reduce_function(&mut self, input: &Arc<FunctionBody>, body: T) -> T {
        body
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMemberBody, function: Option<T>) -> T {
        function.unwrap_or_default()
    }

    fn reduce_circuit(&mut self, input: &Arc<CircuitBody>, members: Vec<T>) -> T {
        T::default().append_all(members.into_iter())
    }

    fn reduce_program(
        &mut self,
        input: &InnerProgram,
        imported_modules: Vec<T>,
        test_functions: Vec<T>,
        functions: Vec<T>,
        circuits: Vec<T>,
    ) -> T {
        T::default()
            .append_all(imported_modules.into_iter())
            .append_all(test_functions.into_iter())
            .append_all(functions.into_iter())
            .append_all(circuits.into_iter())
    }
}

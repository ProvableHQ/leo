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

use super::*;
use crate::{expression::*, program::*, statement::*};
use std::{marker::PhantomData, sync::Arc};

pub struct MonoidalDirector<T: Monoid, R: MonoidalReducerExpression<T>> {
    reducer: R,
    _monoid: PhantomData<T>,
}

impl<T: Monoid, R: MonoidalReducerExpression<T>> MonoidalDirector<T, R> {
    pub fn new(reducer: R) -> Self {
        Self {
            reducer,
            _monoid: PhantomData,
        }
    }

    pub fn reducer(self) -> R {
        self.reducer
    }

    pub fn reduce_expression(&mut self, input: &Arc<Expression>) -> T {
        match &**input {
            Expression::ArrayAccess(e) => self.reduce_array_access(e),
            Expression::ArrayInit(e) => self.reduce_array_init(e),
            Expression::ArrayInline(e) => self.reduce_array_inline(e),
            Expression::ArrayRangeAccess(e) => self.reduce_array_range_access(e),
            Expression::Binary(e) => self.reduce_binary(e),
            Expression::Call(e) => self.reduce_call(e),
            Expression::CircuitAccess(e) => self.reduce_circuit_access(e),
            Expression::CircuitInit(e) => self.reduce_circuit_init(e),
            Expression::Ternary(e) => self.reduce_ternary_expression(e),
            Expression::Constant(e) => self.reduce_constant(e),
            Expression::TupleAccess(e) => self.reduce_tuple_access(e),
            Expression::TupleInit(e) => self.reduce_tuple_init(e),
            Expression::Unary(e) => self.reduce_unary(e),
            Expression::VariableRef(e) => self.reduce_variable_ref(e),
        }
    }

    pub fn reduce_array_access(&mut self, input: &ArrayAccessExpression) -> T {
        let array = self.reduce_expression(&input.array);
        let index = self.reduce_expression(&input.index);

        self.reducer.reduce_array_access(input, array, index)
    }

    pub fn reduce_array_init(&mut self, input: &ArrayInitExpression) -> T {
        let element = self.reduce_expression(&input.element);

        self.reducer.reduce_array_init(input, element)
    }

    pub fn reduce_array_inline(&mut self, input: &ArrayInlineExpression) -> T {
        let elements = input.elements.iter().map(|(x, _)| self.reduce_expression(x)).collect();

        self.reducer.reduce_array_inline(input, elements)
    }

    pub fn reduce_array_range_access(&mut self, input: &ArrayRangeAccessExpression) -> T {
        let array = self.reduce_expression(&input.array);
        let left = input.left.as_ref().map(|e| self.reduce_expression(e));
        let right = input.right.as_ref().map(|e| self.reduce_expression(e));

        self.reducer.reduce_array_range_access(input, array, left, right)
    }

    pub fn reduce_binary(&mut self, input: &BinaryExpression) -> T {
        let left = self.reduce_expression(&input.left);
        let right = self.reduce_expression(&input.right);

        self.reducer.reduce_binary(input, left, right)
    }

    pub fn reduce_call(&mut self, input: &CallExpression) -> T {
        let target = input.target.as_ref().map(|e| self.reduce_expression(e));
        let arguments = input.arguments.iter().map(|e| self.reduce_expression(e)).collect();

        self.reducer.reduce_call(input, target, arguments)
    }

    pub fn reduce_circuit_access(&mut self, input: &CircuitAccessExpression) -> T {
        let target = input.target.as_ref().map(|e| self.reduce_expression(e));

        self.reducer.reduce_circuit_access(input, target)
    }

    pub fn reduce_circuit_init(&mut self, input: &CircuitInitExpression) -> T {
        let values = input.values.iter().map(|(_, e)| self.reduce_expression(e)).collect();

        self.reducer.reduce_circuit_init(input, values)
    }

    pub fn reduce_ternary_expression(&mut self, input: &TernaryExpression) -> T {
        let condition = self.reduce_expression(&input.condition);
        let if_true = self.reduce_expression(&input.if_true);
        let if_false = self.reduce_expression(&input.if_false);

        self.reducer
            .reduce_ternary_expression(input, condition, if_true, if_false)
    }

    pub fn reduce_constant(&mut self, input: &Constant) -> T {
        self.reducer.reduce_constant(input)
    }

    pub fn reduce_tuple_access(&mut self, input: &TupleAccessExpression) -> T {
        let tuple_ref = self.reduce_expression(&input.tuple_ref);

        self.reducer.reduce_tuple_access(input, tuple_ref)
    }

    pub fn reduce_tuple_init(&mut self, input: &TupleInitExpression) -> T {
        let values = input.elements.iter().map(|e| self.reduce_expression(e)).collect();

        self.reducer.reduce_tuple_init(input, values)
    }

    pub fn reduce_unary(&mut self, input: &UnaryExpression) -> T {
        let inner = self.reduce_expression(&input.inner);

        self.reducer.reduce_unary(input, inner)
    }

    pub fn reduce_variable_ref(&mut self, input: &VariableRef) -> T {
        self.reducer.reduce_variable_ref(input)
    }
}

impl<T: Monoid, R: MonoidalReducerStatement<T>> MonoidalDirector<T, R> {
    pub fn reduce_statement(&mut self, input: &Arc<Statement>) -> T {
        match &**input {
            Statement::Assign(s) => self.reduce_assign(s),
            Statement::Block(s) => self.reduce_block(s),
            Statement::Conditional(s) => self.reduce_conditional_statement(s),
            Statement::Console(s) => self.reduce_console(s),
            Statement::Definition(s) => self.reduce_definition(s),
            Statement::Expression(s) => self.reduce_expression_statement(s),
            Statement::Iteration(s) => self.reduce_iteration(s),
            Statement::Return(s) => self.reduce_return(s),
        }
    }

    pub fn reduce_assign_access(&mut self, input: &AssignAccess) -> T {
        let (left, right) = match input {
            AssignAccess::ArrayRange(left, right) => (
                left.as_ref().map(|e| self.reduce_expression(e)),
                right.as_ref().map(|e| self.reduce_expression(e)),
            ),
            AssignAccess::ArrayIndex(index) => (Some(self.reduce_expression(index)), None),
            _ => (None, None),
        };

        self.reducer.reduce_assign_access(input, left, right)
    }

    pub fn reduce_assign(&mut self, input: &AssignStatement) -> T {
        let accesses = input
            .target_accesses
            .iter()
            .map(|x| self.reduce_assign_access(x))
            .collect();
        let value = self.reduce_expression(&input.value);

        self.reducer.reduce_assign(input, accesses, value)
    }

    pub fn reduce_block(&mut self, input: &BlockStatement) -> T {
        let statements = input.statements.iter().map(|x| self.reduce_statement(x)).collect();

        self.reducer.reduce_block(input, statements)
    }

    pub fn reduce_conditional_statement(&mut self, input: &ConditionalStatement) -> T {
        let condition = self.reduce_expression(&input.condition);
        let if_true = self.reduce_statement(&input.result);
        let if_false = input.next.as_ref().map(|s| self.reduce_statement(s));

        self.reducer
            .reduce_conditional_statement(input, condition, if_true, if_false)
    }

    pub fn reduce_formatted_string(&mut self, input: &FormattedString) -> T {
        let parameters = input.parameters.iter().map(|e| self.reduce_expression(e)).collect();

        self.reducer.reduce_formatted_string(input, parameters)
    }

    pub fn reduce_console(&mut self, input: &ConsoleStatement) -> T {
        let argument = match &input.function {
            ConsoleFunction::Assert(e) => self.reduce_expression(e),
            ConsoleFunction::Debug(f) | ConsoleFunction::Error(f) | ConsoleFunction::Log(f) => {
                self.reduce_formatted_string(f)
            }
        };

        self.reducer.reduce_console(input, argument)
    }

    pub fn reduce_definition(&mut self, input: &DefinitionStatement) -> T {
        let value = self.reduce_expression(&input.value);

        self.reducer.reduce_definition(input, value)
    }

    pub fn reduce_expression_statement(&mut self, input: &ExpressionStatement) -> T {
        let value = self.reduce_expression(&input.expression);

        self.reducer.reduce_expression_statement(input, value)
    }

    pub fn reduce_iteration(&mut self, input: &IterationStatement) -> T {
        let start = self.reduce_expression(&input.start);
        let stop = self.reduce_expression(&input.stop);
        let body = self.reduce_statement(&input.body);

        self.reducer.reduce_iteration(input, start, stop, body)
    }

    pub fn reduce_return(&mut self, input: &ReturnStatement) -> T {
        let value = self.reduce_expression(&input.expression);

        self.reducer.reduce_return(input, value)
    }
}

#[allow(dead_code)]
impl<T: Monoid, R: MonoidalReducerProgram<T>> MonoidalDirector<T, R> {
    fn reduce_function(&mut self, input: &Arc<FunctionBody>) -> T {
        let body = self.reduce_statement(&input.body);

        self.reducer.reduce_function(input, body)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMemberBody) -> T {
        let function = match input {
            CircuitMemberBody::Function(f) => Some(self.reduce_function(f)),
            _ => None,
        };

        self.reducer.reduce_circuit_member(input, function)
    }

    fn reduce_circuit(&mut self, input: &Arc<CircuitBody>) -> T {
        let members = input
            .members
            .borrow()
            .iter()
            .map(|(_, member)| self.reduce_circuit_member(member))
            .collect();

        self.reducer.reduce_circuit(input, members)
    }

    fn reduce_program(&mut self, input: &Program) -> T {
        let input = input.borrow();
        let imported_modules = input
            .imported_modules
            .iter()
            .map(|(_, import)| self.reduce_program(import))
            .collect();
        let test_functions = input
            .test_functions
            .iter()
            .map(|(_, (f, _))| self.reduce_function(f))
            .collect();
        let functions = input.functions.iter().map(|(_, f)| self.reduce_function(f)).collect();
        let circuits = input.circuits.iter().map(|(_, c)| self.reduce_circuit(c)).collect();

        self.reducer
            .reduce_program(&input, imported_modules, test_functions, functions, circuits)
    }
}

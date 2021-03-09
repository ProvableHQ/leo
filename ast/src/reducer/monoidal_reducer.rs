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

//! This module contains the reducer which iterates through ast nodes - converting them into
//! asg nodes and saving relevant information.

use crate::{
    AssigneeAccess,
    Circuit,
    CircuitMember,
    ConditionalStatement,
    Expression,
    Function,
    FunctionInput,
    FunctionInputVariable,
    GroupCoordinate,
    Identifier,
    ImportStatement,
    IntegerType,
    Monoid,
    Package,
    Packages,
    Program,
    Statement,
    TestFunction,
    Type,
    ValueExpression,
    VariableName,
};
use indexmap::IndexMap;

pub enum TypeMonoidItems<T: Monoid> {
    None,
    Tuple(Vec<T>),
    Array(T),
    Identifier(T),
}

pub enum CircuitMemberMonoidItems<T: Monoid> {
    Variable { identifier: T, type_: T },
    Function(T),
}

pub enum StatementMonoidItems<T: Monoid> {
    Return(T),
    Definition {
        variables: T,
        expression: T,
    },
    Assign {
        assignee: T,
        assignee_accesses: Vec<T>,
        expression: T,
    },
    Conditional(T),
    Iteration {
        identifier: T,
        start: T,
        stop: T,
        statements: Vec<T>,
    },
    ConsoleAssert(T),
    ConsoleFormat(Vec<T>),
    Expression(T),
    Statements(Vec<T>),
}

pub enum AssigneeAccessItem<T: Monoid> {
    Array(RangeItem<T>),
    Tuple,
    Member(T),
}

pub enum RangeItem<T: Monoid> {
    Range(Option<T>, Option<T>),
    Index(T),
}

pub enum ExpressionMonoidItems<T: Monoid> {
    Empty,
    Unary(T),
    Binary(T, T),
    Triary(T, T, T),
    FunctionCall(T, Vec<T>),
    ArrayAccess(T, T),
    Circuit(T, Vec<(T, T)>),
    Var(Vec<T>),
    Value(T),
}

pub enum ConditionStatementNextItem<T: Monoid> {
    Nested(T),
    End(Vec<T>),
}

pub enum FunctionInputItem<T: Monoid> {
    InputKeyword,
    SelfKeyword,
    MutSelfKeyword,
    Variable(T),
}

pub enum ValueExpressionMonoidItems<T: Monoid> {
    Address,
    Boolean,
    Field,
    GroupSingle,
    GroupTuple(T, T),
    Implicit,
    Integer(T),
}

#[allow(unused_variables)]
pub trait MonoidalReducer<T: Monoid> {
    fn reduce_program(
        &mut self,
        program: &Program,
        expected_input: Vec<T>,
        imports: Vec<T>,
        circuits: IndexMap<String, (T, T)>,
        functions: IndexMap<String, (T, T)>,
        tests: IndexMap<String, (T, T)>,
    ) -> T {
        let mut items = T::default()
            .append_all(expected_input.into_iter())
            .append_all(imports.into_iter());

        for (_, (identifier, value)) in circuits.into_iter() {
            items = items.append(identifier).append(value);
        }
        for (_, (identifier, value)) in functions.into_iter() {
            items = items.append(identifier).append(value);
        }
        for (_, (identifier, value)) in tests.into_iter() {
            items = items.append(identifier).append(value);
        }
        items
    }

    fn reduce_function_input(&mut self, input: &FunctionInput, item: FunctionInputItem<T>) -> T {
        match item {
            FunctionInputItem::InputKeyword | FunctionInputItem::SelfKeyword | FunctionInputItem::MutSelfKeyword => {
                T::default()
            }
            FunctionInputItem::Variable(variable) => variable,
        }
    }

    fn reduce_import_statement(&mut self, import: &ImportStatement, package: T) -> T {
        package
    }

    fn reduce_circuit(&mut self, circuit: &Circuit, circuit_name: T, members: Vec<T>) -> T {
        circuit_name.append_all(members.into_iter())
    }

    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: T,
        input: Vec<T>,
        output: Option<T>,
        statements: Vec<T>,
    ) -> T {
        identifier
            .append_all(input.into_iter())
            .append_option(output)
            .append_all(statements.into_iter())
    }

    fn reduce_test_function(&mut self, test_function: &TestFunction, function: T, input_file: Option<T>) -> T {
        function.append_option(input_file)
    }

    fn reduce_identifier(&mut self, identifier: &Identifier) -> T {
        T::default()
    }

    fn reduce_integer_type(&mut self, integer_type: &IntegerType) -> T {
        T::default()
    }

    fn reduce_function_input_variable(
        &mut self,
        function_input_variable: &FunctionInputVariable,
        identifier: T,
        type_: T,
    ) -> T {
        identifier.append(type_)
    }

    fn reduce_type(&mut self, type_: &Type, items: TypeMonoidItems<T>) -> T {
        match items {
            TypeMonoidItems::Tuple(items) => T::default().append_all(items.into_iter()),
            TypeMonoidItems::Array(item) => item,
            TypeMonoidItems::Identifier(item) => item,
            TypeMonoidItems::None => T::default(),
        }
    }

    fn reduce_packages(&mut self, packages: &Packages, name: T) -> T {
        name
    }

    fn reduce_package(&mut self, package: &Package, name: T) -> T {
        name
    }

    fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember, items: CircuitMemberMonoidItems<T>) -> T {
        match items {
            CircuitMemberMonoidItems::Variable { identifier, type_ } => identifier.append(type_),
            CircuitMemberMonoidItems::Function(identifier) => identifier,
        }
    }

    fn reduce_statement(&mut self, statement: &Statement, items: StatementMonoidItems<T>) -> T {
        match items {
            StatementMonoidItems::Return(expression) => expression,
            StatementMonoidItems::Definition { variables, expression } => variables.append(expression),
            StatementMonoidItems::Assign {
                assignee,
                assignee_accesses,
                expression,
            } => assignee.append_all(assignee_accesses.into_iter()).append(expression),
            StatementMonoidItems::Conditional(conditional) => conditional,
            StatementMonoidItems::Iteration {
                identifier,
                start,
                stop,
                statements,
            } => identifier.append(start).append(stop).append_all(statements.into_iter()),
            StatementMonoidItems::ConsoleAssert(expression) => expression,
            StatementMonoidItems::ConsoleFormat(parameters) => T::default().append_all(parameters.into_iter()),
            StatementMonoidItems::Expression(expression) => expression,
            StatementMonoidItems::Statements(statements) => T::default().append_all(statements.into_iter()),
        }
    }

    fn reduce_assignee_access(&mut self, assignee_access: &AssigneeAccess, item: AssigneeAccessItem<T>) -> T {
        match item {
            AssigneeAccessItem::Array(assignee) => match assignee {
                RangeItem::Index(index) => index,
                RangeItem::Range(start, stop) => T::default().append_option(start).append_option(stop),
            },
            AssigneeAccessItem::Tuple => T::default(),
            AssigneeAccessItem::Member(identifier) => identifier,
        }
    }

    fn reduce_conditional_statement(
        &mut self,
        statement: &ConditionalStatement,
        condition: T,
        statements: Vec<T>,
        next: Option<T>,
    ) -> T {
        condition.append_all(statements.into_iter()).append_option(next)
    }

    fn reduce_variable_name(&mut self, variable_name: &VariableName, identifier: T) -> T {
        identifier
    }

    fn reduce_variable_names(&mut self, names: Vec<T>) -> T {
        T::default().append_all(names.into_iter())
    }

    fn reduce_group_coordinate(&mut self, group_coordinate: &GroupCoordinate) -> T {
        T::default()
    }

    fn reduce_value_expression(
        &mut self,
        value_expression: &ValueExpression,
        value: ValueExpressionMonoidItems<T>,
    ) -> T {
        match value {
            ValueExpressionMonoidItems::GroupTuple(x, y) => x.append(y),
            ValueExpressionMonoidItems::Integer(integer_type) => integer_type,
            _ => T::default(),
        }
    }

    // please be careful matching on array access/range expressions, they can be ExpressionMonoidItems::BiTriary or ExpressionMonoidItems::Binary
    fn reduce_expression(&mut self, expression: &Expression, items: ExpressionMonoidItems<T>) -> T {
        match items {
            ExpressionMonoidItems::Empty => T::default(),
            ExpressionMonoidItems::Unary(expression) => expression,
            ExpressionMonoidItems::Binary(left, right) => left.append(right),
            ExpressionMonoidItems::Triary(left, center, right) => left.append(center).append(right),
            ExpressionMonoidItems::ArrayAccess(identifier, index) => identifier.append(index),
            ExpressionMonoidItems::FunctionCall(identifier, arguments) => identifier.append_all(arguments.into_iter()),
            ExpressionMonoidItems::Circuit(identifier, arguments) => {
                let mut out = identifier;
                for (key, value) in arguments.into_iter() {
                    out = out.append(key).append(value);
                }
                out
            }
            ExpressionMonoidItems::Var(items) => T::default().append_all(items.into_iter()),
            ExpressionMonoidItems::Value(value) => value,
        }
    }
}

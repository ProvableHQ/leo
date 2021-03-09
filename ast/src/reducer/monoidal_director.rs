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
    reducer::*,
    AssigneeAccess,
    Circuit,
    CircuitMember,
    ConditionalStatement,
    ConsoleFunction,
    Expression,
    Function,
    FunctionInput,
    FunctionInputVariable,
    GroupCoordinate,
    GroupValue,
    Identifier,
    ImportStatement,
    IntegerType,
    Monoid,
    PackageOrPackages,
    Program,
    SpreadOrExpression,
    Statement,
    Type,
    ValueExpression,
    VariableName,
};
use std::marker::PhantomData;

pub struct MonoidalDirector<T: Monoid, R: MonoidalReducer<T>> {
    reducer: R,
    _monoid: PhantomData<T>,
}

impl<T: Monoid, R: MonoidalReducer<T>> MonoidalDirector<T, R> {
    pub fn new(reducer: R) -> Self {
        Self {
            reducer,
            _monoid: PhantomData,
        }
    }

    pub fn reduce_program(&mut self, program: &Program) -> T {
        let inputs = program
            .expected_input
            .iter()
            .map(|x| self.reduce_function_input(x))
            .collect();
        let imports = program
            .imports
            .iter()
            .map(|x| self.reduce_import_statement(x))
            .collect();
        let circuits = program
            .circuits
            .iter()
            .map(|(identifier, circuit)| {
                (
                    identifier.name.clone(),
                    (self.reduce_identifier(identifier), self.reduce_circuit(circuit)),
                )
            })
            .collect();
        let functions = program
            .functions
            .iter()
            .map(|(identifier, function)| {
                (
                    identifier.name.clone(),
                    (self.reduce_identifier(identifier), self.reduce_function(function)),
                )
            })
            .collect();

        self.reducer
            .reduce_program(program, inputs, imports, circuits, functions)
    }

    pub fn reduce_function_input(&mut self, input: &FunctionInput) -> T {
        let item = match input {
            FunctionInput::InputKeyword(_) => FunctionInputItem::InputKeyword,
            FunctionInput::SelfKeyword(_) => FunctionInputItem::SelfKeyword,
            FunctionInput::MutSelfKeyword(_) => FunctionInputItem::MutSelfKeyword,
            FunctionInput::Variable(function_input_variable) => {
                FunctionInputItem::Variable(self.reduce_function_input_variable(function_input_variable))
            }
        };

        self.reducer.reduce_function_input(input, item)
    }

    pub fn reduce_import_statement(&mut self, import: &ImportStatement) -> T {
        let package = self.reduce_package(&import.package_or_packages);

        self.reducer.reduce_import_statement(import, package)
    }

    pub fn reduce_circuit(&mut self, circuit: &Circuit) -> T {
        let circuit_name = self.reduce_identifier(&circuit.circuit_name);
        let members = circuit.members.iter().map(|x| self.reduce_circuit_member(x)).collect();

        self.reducer.reduce_circuit(circuit, circuit_name, members)
    }

    pub fn reduce_function(&mut self, function: &Function) -> T {
        let identifier = self.reduce_identifier(&function.identifier);
        let input = function.input.iter().map(|x| self.reduce_function_input(x)).collect();
        let output = function.output.as_ref().map(|x| self.reduce_type(x));
        let statements = function
            .block
            .statements
            .iter()
            .map(|x| self.reduce_statement(x))
            .collect();

        self.reducer
            .reduce_function(function, identifier, input, output, statements)
    }

    pub fn reduce_identifier(&mut self, identifier: &Identifier) -> T {
        self.reducer.reduce_identifier(identifier)
    }

    pub fn reduce_integer_type(&mut self, integer_type: &IntegerType) -> T {
        self.reducer.reduce_integer_type(integer_type)
    }

    pub fn reduce_function_input_variable(&mut self, function_input_variable: &FunctionInputVariable) -> T {
        let identifier = self.reduce_identifier(&function_input_variable.identifier);
        let type_ = self.reduce_type(&function_input_variable.type_);

        self.reducer
            .reduce_function_input_variable(function_input_variable, identifier, type_)
    }

    pub fn reduce_type(&mut self, type_: &Type) -> T {
        let items = match type_ {
            Type::Array(type_, _) => TypeMonoidItems::Array(self.reduce_type(type_)),
            Type::Tuple(types) => TypeMonoidItems::Tuple(types.iter().map(|x| self.reduce_type(x)).collect()),
            Type::Circuit(identifier) => TypeMonoidItems::Identifier(self.reduce_identifier(identifier)),
            _ => TypeMonoidItems::None,
        };

        self.reducer.reduce_type(type_, items)
    }

    pub fn reduce_package(&mut self, package_or_packages: &PackageOrPackages) -> T {
        match package_or_packages {
            PackageOrPackages::Package(package) => {
                let name = self.reduce_identifier(&package.name);

                self.reducer.reduce_package(package, name)
            }
            PackageOrPackages::Packages(packages) => {
                let name = self.reduce_identifier(&packages.name);

                self.reducer.reduce_packages(packages, name)
            }
        }
    }

    pub fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember) -> T {
        let items = match circuit_member {
            CircuitMember::CircuitVariable(identifier, type_) => CircuitMemberMonoidItems::Variable {
                identifier: self.reduce_identifier(identifier),
                type_: self.reduce_type(type_),
            },
            CircuitMember::CircuitFunction(function) => {
                CircuitMemberMonoidItems::Function(self.reduce_function(function))
            }
        };

        self.reducer.reduce_circuit_member(circuit_member, items)
    }

    pub fn reduce_statement(&mut self, statement: &Statement) -> T {
        let items = match statement {
            Statement::Return(ret) => StatementMonoidItems::Return(self.reduce_expression(&ret.expression)),
            Statement::Definition(definition) => StatementMonoidItems::Definition {
                variables: self.reduce_variable_names(&definition.variable_names),
                expression: self.reduce_expression(&definition.value),
            },
            Statement::Assign(assign) => StatementMonoidItems::Assign {
                assignee: self.reduce_identifier(&assign.assignee.identifier),
                assignee_accesses: assign
                    .assignee
                    .accesses
                    .iter()
                    .map(|x| self.reduce_assignee_access(x))
                    .collect(),
                expression: self.reduce_expression(&assign.value),
            },
            Statement::Conditional(conditional) => {
                StatementMonoidItems::Conditional(self.reduce_conditional_statement(conditional))
            }
            Statement::Iteration(iteration) => StatementMonoidItems::Iteration {
                identifier: self.reduce_identifier(&iteration.variable),
                start: self.reduce_expression(&iteration.start),
                stop: self.reduce_expression(&iteration.stop),
                statements: iteration
                    .block
                    .statements
                    .iter()
                    .map(|x| self.reduce_statement(x))
                    .collect(),
            },
            Statement::Console(console) => match &console.function {
                ConsoleFunction::Assert(expression) => {
                    StatementMonoidItems::ConsoleAssert(self.reduce_expression(expression))
                }
                ConsoleFunction::Debug(formatted_string)
                | ConsoleFunction::Error(formatted_string)
                | ConsoleFunction::Log(formatted_string) => StatementMonoidItems::ConsoleFormat(
                    formatted_string
                        .parameters
                        .iter()
                        .map(|parameter| self.reduce_expression(&parameter))
                        .collect(),
                ),
            },
            Statement::Expression(statement) => {
                StatementMonoidItems::Expression(self.reduce_expression(&statement.expression))
            }
            Statement::Block(block) => StatementMonoidItems::Statements(
                block
                    .statements
                    .iter()
                    .map(|statement| self.reduce_statement(statement))
                    .collect(),
            ),
        };

        self.reducer.reduce_statement(statement, items)
    }

    pub fn reduce_assignee_access(&mut self, assignee_access: &AssigneeAccess) -> T {
        let item = match assignee_access {
            AssigneeAccess::ArrayRange(start, stop) => {
                let start_item = start.as_ref().map(|x| self.reduce_expression(x));
                let stop_item = stop.as_ref().map(|x| self.reduce_expression(x));
                AssigneeAccessItem::Array(RangeItem::Range(start_item, stop_item))
            }
            AssigneeAccess::ArrayIndex(expression) => {
                AssigneeAccessItem::Array(RangeItem::Index(self.reduce_expression(expression)))
            }
            AssigneeAccess::Tuple(_, _) => AssigneeAccessItem::Tuple,
            AssigneeAccess::Member(identifier) => {
                let identifier = self.reduce_identifier(identifier);
                AssigneeAccessItem::Member(identifier)
            }
        };

        self.reducer.reduce_assignee_access(assignee_access, item)
    }

    pub fn reduce_conditional_statement(&mut self, statement: &ConditionalStatement) -> T {
        let condition = self.reduce_expression(&statement.condition);
        let statements = statement
            .block
            .statements
            .iter()
            .map(|x| self.reduce_statement(x))
            .collect();
        let next = statement.next.as_ref().map(|x| self.reduce_statement(x));

        self.reducer
            .reduce_conditional_statement(statement, condition, statements, next)
    }

    pub fn reduce_variable_name(&mut self, variable_name: &VariableName) -> T {
        let identifier = self.reduce_identifier(&variable_name.identifier);

        self.reducer.reduce_variable_name(variable_name, identifier)
    }

    pub fn reduce_variable_names(&mut self, variable_names: &[VariableName]) -> T {
        let names = variable_names
            .iter()
            .map(|variable_name| self.reduce_variable_name(variable_name))
            .collect();

        self.reducer.reduce_variable_names(names)
    }

    pub fn reduce_group_coordinate(&mut self, group_coordinate: &GroupCoordinate) -> T {
        self.reducer.reduce_group_coordinate(group_coordinate)
    }

    pub fn reduce_value_expression(&mut self, value_expression: &ValueExpression) -> T {
        let item = match value_expression {
            ValueExpression::Address(_, _) => ValueExpressionMonoidItems::Address,
            ValueExpression::Boolean(_, _) => ValueExpressionMonoidItems::Boolean,
            ValueExpression::Field(_, _) => ValueExpressionMonoidItems::Field,
            ValueExpression::Group(group_value) => match group_value.as_ref() {
                GroupValue::Single(_, _) => ValueExpressionMonoidItems::GroupSingle,
                GroupValue::Tuple(tuple) => {
                    let x = self.reduce_group_coordinate(&tuple.x);
                    let y = self.reduce_group_coordinate(&tuple.y);
                    ValueExpressionMonoidItems::GroupTuple(x, y)
                }
            },
            ValueExpression::Implicit(_, _) => ValueExpressionMonoidItems::Implicit,
            ValueExpression::Integer(integer_type, _, _) => {
                ValueExpressionMonoidItems::Integer(self.reduce_integer_type(integer_type))
            }
        };

        self.reducer.reduce_value_expression(value_expression, item)
    }

    pub fn reduce_expression(&mut self, expression: &Expression) -> T {
        let items = match expression {
            Expression::Identifier(identifier) => ExpressionMonoidItems::Unary(self.reduce_identifier(identifier)),
            Expression::Value(value) => ExpressionMonoidItems::Value(self.reduce_value_expression(value)),
            Expression::Binary(binary) => {
                let left = self.reduce_expression(&binary.left);
                let right = self.reduce_expression(&binary.right);
                ExpressionMonoidItems::Binary(left, right)
            }
            Expression::Unary(unary) => ExpressionMonoidItems::Unary(self.reduce_expression(&unary.inner)),
            Expression::Ternary(ternary) => {
                let condition = self.reduce_expression(&ternary.condition);
                let left = self.reduce_expression(&ternary.if_true);
                let right = self.reduce_expression(&ternary.if_false);
                ExpressionMonoidItems::Triary(condition, left, right)
            }
            Expression::ArrayInline(array_inline) => ExpressionMonoidItems::Var(
                array_inline
                    .elements
                    .iter()
                    .map(|x| match x {
                        SpreadOrExpression::Expression(expression) | SpreadOrExpression::Spread(expression) => {
                            self.reduce_expression(expression)
                        }
                    })
                    .collect(),
            ),
            Expression::ArrayInit(array_init) => {
                let element = self.reduce_expression(&array_init.element);
                ExpressionMonoidItems::Unary(element)
            }
            Expression::ArrayAccess(array_access) => {
                let array = self.reduce_expression(&array_access.array);
                let index = self.reduce_expression(&array_access.index);
                ExpressionMonoidItems::ArrayAccess(array, index)
            }
            Expression::ArrayRangeAccess(array_range_access) => {
                let array = self.reduce_expression(&array_range_access.array);

                match (array_range_access.left.as_ref(), array_range_access.right.as_ref()) {
                    (Some(left_expression), Some(right_expression)) => {
                        let left = self.reduce_expression(&left_expression);
                        let right = self.reduce_expression(&right_expression);
                        ExpressionMonoidItems::Triary(array, left, right)
                    }
                    (Some(left_expression), None) => {
                        let left = self.reduce_expression(&left_expression);
                        ExpressionMonoidItems::Binary(array, left)
                    }
                    (None, Some(right_expression)) => {
                        let right = self.reduce_expression(&right_expression);
                        ExpressionMonoidItems::Binary(array, right)
                    }
                    (None, None) => ExpressionMonoidItems::Unary(array),
                }
            }
            Expression::TupleInit(tuple_init) => {
                let element_items = tuple_init.elements.iter().map(|x| self.reduce_expression(x)).collect();
                ExpressionMonoidItems::Var(element_items)
            }
            Expression::TupleAccess(tuple_access) => {
                let tuple_access = self.reduce_expression(&tuple_access.tuple);
                ExpressionMonoidItems::Unary(tuple_access)
            }

            Expression::CircuitInit(circuit_init) => {
                let defined_circuit_name_item = self.reduce_identifier(&circuit_init.name);
                let members = circuit_init
                    .members
                    .iter()
                    .map(|definition| {
                        let definition_identifier = self.reduce_identifier(&definition.identifier);
                        let definition_expression =
                            definition.expression.as_ref().map(|expr| self.reduce_expression(&expr));
                        (definition_identifier, definition_expression)
                    })
                    .collect();

                ExpressionMonoidItems::Circuit(defined_circuit_name_item, members)
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                let declared_circuit_name = self.reduce_expression(&circuit_member_access.circuit);
                let circuit_member_name = self.reduce_identifier(&circuit_member_access.name);

                ExpressionMonoidItems::Binary(declared_circuit_name, circuit_member_name)
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_func_access) => {
                let declared_circuit_name = self.reduce_expression(&circuit_static_func_access.circuit);
                let circuit_static_function_name = self.reduce_identifier(&circuit_static_func_access.name);

                ExpressionMonoidItems::Binary(declared_circuit_name, circuit_static_function_name)
            }

            Expression::Call(call) => {
                let function = self.reduce_expression(&call.function);
                let function_arguments = call.arguments.iter().map(|x| self.reduce_expression(x)).collect();

                ExpressionMonoidItems::FunctionCall(function, function_arguments)
            }

            // TODO casts?
            _ => ExpressionMonoidItems::Empty,
        };
        self.reducer.reduce_expression(expression, items)
    }
}

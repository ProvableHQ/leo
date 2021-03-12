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

use crate::*;

pub struct ReconstructingDirector<R: ReconstructingReducer> {
    reducer: R,
}

impl<R: ReconstructingReducer> ReconstructingDirector<R> {
    pub fn new(reducer: R) -> Self {
        Self { reducer }
    }

    pub fn reduce_type(&mut self, type_: &Type, in_circuit: bool) -> Type {
        let new = match type_ {
            // Data type wrappers
            Type::Array(type_, dimensions) => {
                Type::Array(Box::new(self.reduce_type(type_, in_circuit)), dimensions.clone())
            }
            Type::Tuple(types) => Type::Tuple(types.iter().map(|type_| self.reduce_type(type_, in_circuit)).collect()),
            Type::Circuit(identifier) => Type::Circuit(self.reduce_identifier(identifier)),
            _ => type_.clone(),
        };

        self.reducer.reduce_type(type_, new, in_circuit)
    }

    // Expressions
    pub fn reduce_expression(&mut self, expression: &Expression) -> Expression {
        let new = match expression {
            Expression::Identifier(identifier) => Expression::Identifier(self.reduce_identifier(&identifier)),
            Expression::Value(value) => Expression::Value(self.reduce_value(&value)),
            Expression::Binary(binary) => Expression::Binary(self.reduce_binary(&binary)),
            Expression::Unary(unary) => Expression::Unary(self.reduce_unary(&unary)),
            Expression::Ternary(ternary) => Expression::Ternary(self.reduce_ternary(&ternary)),
            Expression::Cast(cast) => Expression::Cast(self.reduce_cast(&cast)),

            Expression::ArrayInline(array_inline) => Expression::ArrayInline(self.reduce_array_inline(&array_inline)),
            Expression::ArrayInit(array_init) => Expression::ArrayInit(self.reduce_array_init(&array_init)),
            Expression::ArrayAccess(array_access) => Expression::ArrayAccess(self.reduce_array_access(&array_access)),
            Expression::ArrayRangeAccess(array_range_access) => {
                Expression::ArrayRangeAccess(self.reduce_array_range_access(&array_range_access))
            }

            Expression::TupleInit(tuple_init) => Expression::TupleInit(self.reduce_tuple_init(&tuple_init)),
            Expression::TupleAccess(tuple_access) => Expression::TupleAccess(self.reduce_tuple_access(&tuple_access)),

            Expression::CircuitInit(circuit_init) => Expression::CircuitInit(self.reduce_circuit_init(&circuit_init)),
            Expression::CircuitMemberAccess(circuit_member_access) => {
                Expression::CircuitMemberAccess(self.reduce_circuit_member_access(&circuit_member_access))
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_fn_access) => {
                Expression::CircuitStaticFunctionAccess(self.reduce_circuit_static_fn_access(&circuit_static_fn_access))
            }

            Expression::Call(call) => Expression::Call(self.reduce_call(&call)),
        };

        self.reducer.reduce_expression(expression, new)
    }

    pub fn reduce_identifier(&mut self, identifier: &Identifier) -> Identifier {
        self.reducer.reduce_identifier(identifier)
    }

    pub fn reduce_group_tuple(&mut self, group_tuple: &GroupTuple) -> GroupTuple {
        self.reducer.reduce_group_tuple(group_tuple)
    }

    pub fn reduce_group_value(&mut self, group_value: &GroupValue) -> GroupValue {
        let new = match group_value {
            GroupValue::Tuple(group_tuple) => GroupValue::Tuple(self.reduce_group_tuple(&group_tuple)),
            _ => group_value.clone(),
        };

        self.reducer.reduce_group_value(group_value, new)
    }

    pub fn reduce_value(&mut self, value: &ValueExpression) -> ValueExpression {
        let new = match value {
            ValueExpression::Group(group_value) => {
                ValueExpression::Group(Box::new(self.reduce_group_value(&group_value)))
            }
            _ => value.clone(),
        };

        self.reducer.reduce_value(value, new)
    }

    pub fn reduce_binary(&mut self, binary: &BinaryExpression) -> BinaryExpression {
        let left = self.reduce_expression(&binary.left);
        let right = self.reduce_expression(&binary.right);

        self.reducer.reduce_binary(binary, left, right, binary.op.clone())
    }

    pub fn reduce_unary(&mut self, unary: &UnaryExpression) -> UnaryExpression {
        let inner = self.reduce_expression(&unary.inner);

        self.reducer.reduce_unary(unary, inner, unary.op.clone())
    }

    pub fn reduce_ternary(&mut self, ternary: &TernaryExpression) -> TernaryExpression {
        let condition = self.reduce_expression(&ternary.condition);
        let if_true = self.reduce_expression(&ternary.if_true);
        let if_false = self.reduce_expression(&ternary.if_false);

        self.reducer.reduce_ternary(ternary, condition, if_true, if_false)
    }

    pub fn reduce_cast(&mut self, cast: &CastExpression) -> CastExpression {
        let inner = self.reduce_expression(&cast.inner);
        let target_type = cast.target_type.clone(); // TODO reduce

        self.reducer.reduce_cast(cast, inner, target_type)
    }

    pub fn reduce_array_inline(&mut self, array_inline: &ArrayInlineExpression) -> ArrayInlineExpression {
        let elements = array_inline
            .elements
            .iter()
            .map(|element| match element {
                SpreadOrExpression::Expression(expression) => {
                    SpreadOrExpression::Expression(self.reduce_expression(expression))
                }
                SpreadOrExpression::Spread(expression) => {
                    SpreadOrExpression::Spread(self.reduce_expression(expression))
                }
            })
            .collect();

        self.reducer.reduce_array_inline(array_inline, elements)
    }

    pub fn reduce_array_init(&mut self, array_init: &ArrayInitExpression) -> ArrayInitExpression {
        let element = self.reduce_expression(&array_init.element);

        self.reducer.reduce_array_init(array_init, element)
    }

    pub fn reduce_array_access(&mut self, array_access: &ArrayAccessExpression) -> ArrayAccessExpression {
        let array = self.reduce_expression(&array_access.array);
        let index = self.reduce_expression(&array_access.index);

        self.reducer.reduce_array_access(array_access, array, index)
    }

    pub fn reduce_array_range_access(
        &mut self,
        array_range_access: &ArrayRangeAccessExpression,
    ) -> ArrayRangeAccessExpression {
        let array = self.reduce_expression(&array_range_access.array);
        let left = array_range_access
            .left
            .as_ref()
            .map(|left| self.reduce_expression(left));
        let right = array_range_access
            .right
            .as_ref()
            .map(|right| self.reduce_expression(right));

        self.reducer
            .reduce_array_range_access(array_range_access, array, left, right)
    }

    pub fn reduce_tuple_init(&mut self, tuple_init: &TupleInitExpression) -> TupleInitExpression {
        let elements = tuple_init
            .elements
            .iter()
            .map(|expr| self.reduce_expression(expr))
            .collect();

        self.reducer.reduce_tuple_init(tuple_init, elements)
    }

    pub fn reduce_tuple_access(&mut self, tuple_access: &TupleAccessExpression) -> TupleAccessExpression {
        let tuple = self.reduce_expression(&tuple_access.tuple);

        self.reducer.reduce_tuple_access(tuple_access, tuple)
    }

    pub fn reduce_circuit_init(&mut self, circuit_init: &CircuitInitExpression) -> CircuitInitExpression {
        let name = self.reduce_identifier(&circuit_init.name);
        let members = circuit_init
            .members
            .iter()
            .map(|definition| {
                let identifier = self.reduce_identifier(&definition.identifier);
                let expression = definition.expression.as_ref().map(|expr| self.reduce_expression(expr));

                CircuitImpliedVariableDefinition { identifier, expression }
            })
            .collect();

        self.reducer.reduce_circuit_init(circuit_init, name, members)
    }

    pub fn reduce_circuit_member_access(
        &mut self,
        circuit_member_access: &CircuitMemberAccessExpression,
    ) -> CircuitMemberAccessExpression {
        let circuit = self.reduce_expression(&circuit_member_access.circuit);
        let name = self.reduce_identifier(&circuit_member_access.name);

        self.reducer
            .reduce_circuit_member_access(circuit_member_access, circuit, name)
    }

    pub fn reduce_circuit_static_fn_access(
        &mut self,
        circuit_static_fn_access: &CircuitStaticFunctionAccessExpression,
    ) -> CircuitStaticFunctionAccessExpression {
        let circuit = self.reduce_expression(&circuit_static_fn_access.circuit);
        let name = self.reduce_identifier(&circuit_static_fn_access.name);

        self.reducer
            .reduce_circuit_static_fn_access(circuit_static_fn_access, circuit, name)
    }

    pub fn reduce_call(&mut self, call: &CallExpression) -> CallExpression {
        let function = self.reduce_expression(&call.function);
        let arguments = call.arguments.iter().map(|expr| self.reduce_expression(expr)).collect();

        self.reducer.reduce_call(call, function, arguments)
    }

    // Statements
    pub fn reduce_statement(&mut self, statement: &Statement, in_circuit: bool) -> Statement {
        let new = match statement {
            Statement::Return(return_statement) => Statement::Return(self.reduce_return(&return_statement)),
            Statement::Definition(definition) => Statement::Definition(self.reduce_definition(&definition, in_circuit)),
            Statement::Assign(assign) => Statement::Assign(self.reduce_assign(&assign)),
            Statement::Conditional(conditional) => {
                Statement::Conditional(self.reduce_conditional(&conditional, in_circuit))
            }
            Statement::Iteration(iteration) => Statement::Iteration(self.reduce_iteration(&iteration, in_circuit)),
            Statement::Console(console) => Statement::Console(self.reduce_console(&console)),
            Statement::Expression(expression) => Statement::Expression(self.reduce_expression_statement(&expression)),
            Statement::Block(block) => Statement::Block(self.reduce_block(&block, in_circuit)),
        };

        self.reducer.reduce_statement(statement, new, in_circuit)
    }

    pub fn reduce_return(&mut self, return_statement: &ReturnStatement) -> ReturnStatement {
        let expression = self.reduce_expression(&return_statement.expression);

        self.reducer.reduce_return(return_statement, expression)
    }

    pub fn reduce_variable_name(&mut self, variable_name: &VariableName) -> VariableName {
        let identifier = self.reduce_identifier(&variable_name.identifier);

        self.reducer.reduce_variable_name(variable_name, identifier)
    }

    pub fn reduce_definition(&mut self, definition: &DefinitionStatement, in_circuit: bool) -> DefinitionStatement {
        let variable_names = definition
            .variable_names
            .iter()
            .map(|variable_name| self.reduce_variable_name(variable_name))
            .collect();
        let type_ = definition
            .type_
            .as_ref()
            .map(|inner| self.reduce_type(inner, in_circuit));
        let value = self.reduce_expression(&definition.value);

        self.reducer
            .reduce_definition(definition, variable_names, type_, value, in_circuit)
    }

    pub fn reduce_assignee_access(&mut self, access: &AssigneeAccess) -> AssigneeAccess {
        let new = match access {
            AssigneeAccess::ArrayRange(left, right) => AssigneeAccess::ArrayRange(
                left.as_ref().map(|expr| self.reduce_expression(expr)),
                right.as_ref().map(|expr| self.reduce_expression(expr)),
            ),
            AssigneeAccess::ArrayIndex(index) => AssigneeAccess::ArrayIndex(self.reduce_expression(&index)),
            AssigneeAccess::Member(identifier) => AssigneeAccess::Member(self.reduce_identifier(&identifier)),
            _ => access.clone(),
        };

        self.reducer.reduce_assignee_access(access, new)
    }

    pub fn reduce_assignee(&mut self, assignee: &Assignee) -> Assignee {
        let identifier = self.reduce_identifier(&assignee.identifier);
        let accesses = assignee
            .accesses
            .iter()
            .map(|access| self.reduce_assignee_access(access))
            .collect();

        self.reducer.reduce_assignee(assignee, identifier, accesses)
    }

    pub fn reduce_assign(&mut self, assign: &AssignStatement) -> AssignStatement {
        let assignee = self.reduce_assignee(&assign.assignee);
        let value = self.reduce_expression(&assign.value);

        self.reducer.reduce_assign(assign, assignee, value)
    }

    pub fn reduce_conditional(&mut self, conditional: &ConditionalStatement, in_circuit: bool) -> ConditionalStatement {
        let condition = self.reduce_expression(&conditional.condition);
        let block = self.reduce_block(&conditional.block, in_circuit);
        let next = conditional
            .next
            .as_ref()
            .map(|condition| self.reduce_statement(condition, in_circuit));

        self.reducer
            .reduce_conditional(conditional, condition, block, next, in_circuit)
    }

    pub fn reduce_iteration(&mut self, iteration: &IterationStatement, in_circuit: bool) -> IterationStatement {
        let variable = self.reduce_identifier(&iteration.variable);
        let start = self.reduce_expression(&iteration.start);
        let stop = self.reduce_expression(&iteration.stop);
        let block = self.reduce_block(&iteration.block, in_circuit);

        self.reducer
            .reduce_iteration(iteration, variable, start, stop, block, in_circuit)
    }

    pub fn reduce_console(&mut self, console_function_call: &ConsoleStatement) -> ConsoleStatement {
        let function = match &console_function_call.function {
            ConsoleFunction::Assert(expression) => ConsoleFunction::Assert(self.reduce_expression(expression)),
            ConsoleFunction::Debug(format) | ConsoleFunction::Error(format) | ConsoleFunction::Log(format) => {
                let formatted = FormattedString {
                    parts: format.parts.clone(),
                    parameters: format
                        .parameters
                        .iter()
                        .map(|parameter| self.reduce_expression(parameter))
                        .collect(),
                    span: format.span.clone(),
                };
                match &console_function_call.function {
                    ConsoleFunction::Debug(_) => ConsoleFunction::Debug(formatted),
                    ConsoleFunction::Error(_) => ConsoleFunction::Error(formatted),
                    ConsoleFunction::Log(_) => ConsoleFunction::Log(formatted),
                    _ => unimplemented!(), // impossible
                }
            }
        };

        self.reducer.reduce_console(console_function_call, function)
    }

    pub fn reduce_expression_statement(&mut self, expression: &ExpressionStatement) -> ExpressionStatement {
        let inner_expression = self.reduce_expression(&expression.expression);
        self.reducer.reduce_expression_statement(expression, inner_expression)
    }

    pub fn reduce_block(&mut self, block: &Block, in_circuit: bool) -> Block {
        let statements = block
            .statements
            .iter()
            .map(|statement| self.reduce_statement(statement, in_circuit))
            .collect();

        self.reducer.reduce_block(block, statements, in_circuit)
    }

    // Program
    pub fn reduce_program(&mut self, program: &Program) -> Program {
        let inputs = program
            .expected_input
            .iter()
            .map(|input| self.reduce_function_input(input, false))
            .collect();
        let imports = program
            .imports
            .iter()
            .map(|import| self.reduce_import(import))
            .collect();
        let circuits = program
            .circuits
            .iter()
            .map(|(identifier, circuit)| (self.reduce_identifier(identifier), self.reduce_circuit(circuit)))
            .collect();
        let functions = program
            .functions
            .iter()
            .map(|(identifier, function)| {
                (
                    self.reduce_identifier(identifier),
                    self.reduce_function(function, false),
                )
            })
            .collect();

        self.reducer
            .reduce_program(program, inputs, imports, circuits, functions)
    }

    pub fn reduce_function_input_variable(
        &mut self,
        variable: &FunctionInputVariable,
        in_circuit: bool,
    ) -> FunctionInputVariable {
        let identifier = self.reduce_identifier(&variable.identifier);
        let type_ = self.reduce_type(&variable.type_, in_circuit);

        self.reducer
            .reduce_function_input_variable(variable, identifier, type_, in_circuit)
    }

    pub fn reduce_function_input(&mut self, input: &FunctionInput, in_circuit: bool) -> FunctionInput {
        let new = match input {
            FunctionInput::Variable(function_input_variable) => {
                FunctionInput::Variable(self.reduce_function_input_variable(function_input_variable, in_circuit))
            }
            _ => input.clone(),
        };

        self.reducer.reduce_function_input(input, new, in_circuit)
    }

    pub fn reduce_package_or_packages(&mut self, package_or_packages: &PackageOrPackages) -> PackageOrPackages {
        let new = match package_or_packages {
            PackageOrPackages::Package(package) => PackageOrPackages::Package(Package {
                name: self.reduce_identifier(&package.name),
                access: package.access.clone(),
                span: package.span.clone(),
            }),
            PackageOrPackages::Packages(packages) => PackageOrPackages::Packages(Packages {
                name: self.reduce_identifier(&packages.name),
                accesses: packages.accesses.clone(),
                span: packages.span.clone(),
            }),
        };

        self.reducer.reduce_package_or_packages(package_or_packages, new)
    }

    pub fn reduce_import(&mut self, import: &ImportStatement) -> ImportStatement {
        let package_or_packages = self.reduce_package_or_packages(&import.package_or_packages);

        self.reducer.reduce_import(import, package_or_packages)
    }

    pub fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember) -> CircuitMember {
        let new = match circuit_member {
            CircuitMember::CircuitVariable(identifier, type_) => {
                CircuitMember::CircuitVariable(self.reduce_identifier(&identifier), self.reduce_type(&type_, true))
            }
            CircuitMember::CircuitFunction(function) => {
                CircuitMember::CircuitFunction(self.reduce_function(&function, true))
            }
        };

        self.reducer.reduce_circuit_member(circuit_member, new)
    }

    pub fn reduce_circuit(&mut self, circuit: &Circuit) -> Circuit {
        let circuit_name = self.reduce_identifier(&circuit.circuit_name);
        let members = circuit
            .members
            .iter()
            .map(|member| self.reduce_circuit_member(member))
            .collect();

        self.reducer.reduce_circuit(circuit, circuit_name, members)
    }

    fn reduce_annotation(&mut self, annotation: &Annotation) -> Annotation {
        let name = self.reduce_identifier(&annotation.name);

        self.reducer.reduce_annotation(annotation, name)
    }

    pub fn reduce_function(&mut self, function: &Function, in_circuit: bool) -> Function {
        let identifier = self.reduce_identifier(&function.identifier);
        let annotations = function
            .annotations
            .iter()
            .map(|annotation| self.reduce_annotation(annotation))
            .collect();
        let input = function
            .input
            .iter()
            .map(|input| self.reduce_function_input(input, false))
            .collect();
        let output = function
            .output
            .as_ref()
            .map(|output| self.reduce_type(output, in_circuit));
        let block = self.reduce_block(&function.block, false);

        self.reducer
            .reduce_function(function, identifier, annotations, input, output, block, in_circuit)
    }
}

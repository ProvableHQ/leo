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
use indexmap::IndexMap;

pub struct ReconstructingDirector<R: ReconstructingReducer> {
    reducer: R,
}

impl<R: ReconstructingReducer> ReconstructingDirector<R> {
    pub fn new(reducer: R) -> Self {
        Self { reducer }
    }

    pub fn reduce_type(&self, type_: &Type, in_circuit: bool, span: &Span) -> Result<Type, CanonicalizeError> {
        let new = match type_ {
            Type::Array(type_, dimensions) => {
                Type::Array(Box::new(self.reduce_type(type_, in_circuit, span)?), dimensions.clone())
            }
            Type::Tuple(types) => {
                let mut reduced_types = vec![];
                for type_ in types.iter() {
                    reduced_types.push(self.reduce_type(type_, in_circuit, span)?);
                }

                Type::Tuple(reduced_types)
            }
            Type::Circuit(identifier) => Type::Circuit(self.reduce_identifier(identifier)?),
            _ => type_.clone(),
        };

        self.reducer.reduce_type(type_, new, in_circuit, span)
    }

    // Expressions
    pub fn reduce_expression(
        &self,
        expression: &Expression,
        in_circuit: bool,
    ) -> Result<Expression, CanonicalizeError> {
        let new = match expression {
            Expression::Identifier(identifier) => Expression::Identifier(self.reduce_identifier(&identifier)?),
            Expression::Value(value) => Expression::Value(self.reduce_value(&value)?),
            Expression::Binary(binary) => Expression::Binary(self.reduce_binary(&binary, in_circuit)?),
            Expression::Unary(unary) => Expression::Unary(self.reduce_unary(&unary, in_circuit)?),
            Expression::Ternary(ternary) => Expression::Ternary(self.reduce_ternary(&ternary, in_circuit)?),
            Expression::Cast(cast) => Expression::Cast(self.reduce_cast(&cast, in_circuit)?),

            Expression::ArrayInline(array_inline) => {
                Expression::ArrayInline(self.reduce_array_inline(&array_inline, in_circuit)?)
            }
            Expression::ArrayInit(array_init) => {
                Expression::ArrayInit(self.reduce_array_init(&array_init, in_circuit)?)
            }
            Expression::ArrayAccess(array_access) => {
                Expression::ArrayAccess(self.reduce_array_access(&array_access, in_circuit)?)
            }
            Expression::ArrayRangeAccess(array_range_access) => {
                Expression::ArrayRangeAccess(self.reduce_array_range_access(&array_range_access, in_circuit)?)
            }

            Expression::TupleInit(tuple_init) => {
                Expression::TupleInit(self.reduce_tuple_init(&tuple_init, in_circuit)?)
            }
            Expression::TupleAccess(tuple_access) => {
                Expression::TupleAccess(self.reduce_tuple_access(&tuple_access, in_circuit)?)
            }

            Expression::CircuitInit(circuit_init) => {
                Expression::CircuitInit(self.reduce_circuit_init(&circuit_init, in_circuit)?)
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                Expression::CircuitMemberAccess(self.reduce_circuit_member_access(&circuit_member_access, in_circuit)?)
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_fn_access) => {
                Expression::CircuitStaticFunctionAccess(
                    self.reduce_circuit_static_fn_access(&circuit_static_fn_access, in_circuit)?,
                )
            }

            Expression::Call(call) => Expression::Call(self.reduce_call(&call, in_circuit)?),
        };

        self.reducer.reduce_expression(expression, new, in_circuit)
    }

    pub fn reduce_identifier(&self, identifier: &Identifier) -> Result<Identifier, CanonicalizeError> {
        self.reducer.reduce_identifier(identifier)
    }

    pub fn reduce_group_tuple(&self, group_tuple: &GroupTuple) -> Result<GroupTuple, CanonicalizeError> {
        self.reducer.reduce_group_tuple(group_tuple)
    }

    pub fn reduce_group_value(&self, group_value: &GroupValue) -> Result<GroupValue, CanonicalizeError> {
        let new = match group_value {
            GroupValue::Tuple(group_tuple) => GroupValue::Tuple(self.reduce_group_tuple(&group_tuple)?),
            _ => group_value.clone(),
        };

        self.reducer.reduce_group_value(group_value, new)
    }

    pub fn reduce_value(&self, value: &ValueExpression) -> Result<ValueExpression, CanonicalizeError> {
        let new = match value {
            ValueExpression::Group(group_value) => {
                ValueExpression::Group(Box::new(self.reduce_group_value(&group_value)?))
            }
            _ => value.clone(),
        };

        self.reducer.reduce_value(value, new)
    }

    pub fn reduce_binary(
        &self,
        binary: &BinaryExpression,
        in_circuit: bool,
    ) -> Result<BinaryExpression, CanonicalizeError> {
        let left = self.reduce_expression(&binary.left, in_circuit)?;
        let right = self.reduce_expression(&binary.right, in_circuit)?;

        self.reducer
            .reduce_binary(binary, left, right, binary.op.clone(), in_circuit)
    }

    pub fn reduce_unary(
        &self,
        unary: &UnaryExpression,
        in_circuit: bool,
    ) -> Result<UnaryExpression, CanonicalizeError> {
        let inner = self.reduce_expression(&unary.inner, in_circuit)?;

        self.reducer.reduce_unary(unary, inner, unary.op.clone(), in_circuit)
    }

    pub fn reduce_ternary(
        &self,
        ternary: &TernaryExpression,
        in_circuit: bool,
    ) -> Result<TernaryExpression, CanonicalizeError> {
        let condition = self.reduce_expression(&ternary.condition, in_circuit)?;
        let if_true = self.reduce_expression(&ternary.if_true, in_circuit)?;
        let if_false = self.reduce_expression(&ternary.if_false, in_circuit)?;

        self.reducer
            .reduce_ternary(ternary, condition, if_true, if_false, in_circuit)
    }

    pub fn reduce_cast(&self, cast: &CastExpression, in_circuit: bool) -> Result<CastExpression, CanonicalizeError> {
        let inner = self.reduce_expression(&cast.inner, in_circuit)?;
        let target_type = self.reduce_type(&cast.target_type, in_circuit, &cast.span)?;

        self.reducer.reduce_cast(cast, inner, target_type, in_circuit)
    }

    pub fn reduce_array_inline(
        &self,
        array_inline: &ArrayInlineExpression,
        in_circuit: bool,
    ) -> Result<ArrayInlineExpression, CanonicalizeError> {
        let mut elements = vec![];
        for element in array_inline.elements.iter() {
            let reduced_element = match element {
                SpreadOrExpression::Expression(expression) => {
                    SpreadOrExpression::Expression(self.reduce_expression(expression, in_circuit)?)
                }
                SpreadOrExpression::Spread(expression) => {
                    SpreadOrExpression::Spread(self.reduce_expression(expression, in_circuit)?)
                }
            };

            elements.push(reduced_element);
        }

        self.reducer.reduce_array_inline(array_inline, elements, in_circuit)
    }

    pub fn reduce_array_init(
        &self,
        array_init: &ArrayInitExpression,
        in_circuit: bool,
    ) -> Result<ArrayInitExpression, CanonicalizeError> {
        let element = self.reduce_expression(&array_init.element, in_circuit)?;

        self.reducer.reduce_array_init(array_init, element, in_circuit)
    }

    pub fn reduce_array_access(
        &self,
        array_access: &ArrayAccessExpression,
        in_circuit: bool,
    ) -> Result<ArrayAccessExpression, CanonicalizeError> {
        let array = self.reduce_expression(&array_access.array, in_circuit)?;
        let index = self.reduce_expression(&array_access.index, in_circuit)?;

        self.reducer.reduce_array_access(array_access, array, index, in_circuit)
    }

    pub fn reduce_array_range_access(
        &self,
        array_range_access: &ArrayRangeAccessExpression,
        in_circuit: bool,
    ) -> Result<ArrayRangeAccessExpression, CanonicalizeError> {
        let array = self.reduce_expression(&array_range_access.array, in_circuit)?;
        let left = match array_range_access.left.as_ref() {
            Some(left) => Some(self.reduce_expression(left, in_circuit)?),
            None => None,
        };
        let right = match array_range_access.right.as_ref() {
            Some(right) => Some(self.reduce_expression(right, in_circuit)?),
            None => None,
        };

        self.reducer
            .reduce_array_range_access(array_range_access, array, left, right, in_circuit)
    }

    pub fn reduce_tuple_init(
        &self,
        tuple_init: &TupleInitExpression,
        in_circuit: bool,
    ) -> Result<TupleInitExpression, CanonicalizeError> {
        let mut elements = vec![];
        for element in tuple_init.elements.iter() {
            elements.push(self.reduce_expression(element, in_circuit)?);
        }

        self.reducer.reduce_tuple_init(tuple_init, elements, in_circuit)
    }

    pub fn reduce_tuple_access(
        &self,
        tuple_access: &TupleAccessExpression,
        in_circuit: bool,
    ) -> Result<TupleAccessExpression, CanonicalizeError> {
        let tuple = self.reduce_expression(&tuple_access.tuple, in_circuit)?;

        self.reducer.reduce_tuple_access(tuple_access, tuple, in_circuit)
    }

    pub fn reduce_circuit_implied_variable_definition(
        &self,
        variable: &CircuitImpliedVariableDefinition,
        in_circuit: bool,
    ) -> Result<CircuitImpliedVariableDefinition, CanonicalizeError> {
        let identifier = self.reduce_identifier(&variable.identifier)?;
        let expression = match variable.expression.as_ref() {
            Some(expr) => Some(self.reduce_expression(expr, in_circuit)?),
            None => None,
        };

        self.reducer
            .reduce_circuit_implied_variable_definition(variable, identifier, expression, in_circuit)
    }

    pub fn reduce_circuit_init(
        &self,
        circuit_init: &CircuitInitExpression,
        in_circuit: bool,
    ) -> Result<CircuitInitExpression, CanonicalizeError> {
        let name = self.reduce_identifier(&circuit_init.name)?;

        let mut members = vec![];
        for member in circuit_init.members.iter() {
            members.push(self.reduce_circuit_implied_variable_definition(member, in_circuit)?);
        }

        self.reducer
            .reduce_circuit_init(circuit_init, name, members, in_circuit)
    }

    pub fn reduce_circuit_member_access(
        &self,
        circuit_member_access: &CircuitMemberAccessExpression,
        in_circuit: bool,
    ) -> Result<CircuitMemberAccessExpression, CanonicalizeError> {
        let circuit = self.reduce_expression(&circuit_member_access.circuit, in_circuit)?;
        let name = self.reduce_identifier(&circuit_member_access.name)?;

        self.reducer
            .reduce_circuit_member_access(circuit_member_access, circuit, name, in_circuit)
    }

    pub fn reduce_circuit_static_fn_access(
        &self,
        circuit_static_fn_access: &CircuitStaticFunctionAccessExpression,
        in_circuit: bool,
    ) -> Result<CircuitStaticFunctionAccessExpression, CanonicalizeError> {
        let circuit = self.reduce_expression(&circuit_static_fn_access.circuit, in_circuit)?;
        let name = self.reduce_identifier(&circuit_static_fn_access.name)?;

        self.reducer
            .reduce_circuit_static_fn_access(circuit_static_fn_access, circuit, name, in_circuit)
    }

    pub fn reduce_call(&self, call: &CallExpression, in_circuit: bool) -> Result<CallExpression, CanonicalizeError> {
        let function = self.reduce_expression(&call.function, in_circuit)?;

        let mut arguments = vec![];
        for argument in call.arguments.iter() {
            arguments.push(self.reduce_expression(argument, in_circuit)?);
        }

        self.reducer.reduce_call(call, function, arguments, in_circuit)
    }

    // Statements
    pub fn reduce_statement(&self, statement: &Statement, in_circuit: bool) -> Result<Statement, CanonicalizeError> {
        let new = match statement {
            Statement::Return(return_statement) => {
                Statement::Return(self.reduce_return(&return_statement, in_circuit)?)
            }
            Statement::Definition(definition) => {
                Statement::Definition(self.reduce_definition(&definition, in_circuit)?)
            }
            Statement::Assign(assign) => Statement::Assign(self.reduce_assign(&assign, in_circuit)?),
            Statement::Conditional(conditional) => {
                Statement::Conditional(self.reduce_conditional(&conditional, in_circuit)?)
            }
            Statement::Iteration(iteration) => Statement::Iteration(self.reduce_iteration(&iteration, in_circuit)?),
            Statement::Console(console) => Statement::Console(self.reduce_console(&console, in_circuit)?),
            Statement::Expression(expression) => {
                Statement::Expression(self.reduce_expression_statement(&expression, in_circuit)?)
            }
            Statement::Block(block) => Statement::Block(self.reduce_block(&block, in_circuit)?),
        };

        self.reducer.reduce_statement(statement, new, in_circuit)
    }

    pub fn reduce_return(
        &self,
        return_statement: &ReturnStatement,
        in_circuit: bool,
    ) -> Result<ReturnStatement, CanonicalizeError> {
        let expression = self.reduce_expression(&return_statement.expression, in_circuit)?;

        self.reducer.reduce_return(return_statement, expression, in_circuit)
    }

    pub fn reduce_variable_name(&self, variable_name: &VariableName) -> Result<VariableName, CanonicalizeError> {
        let identifier = self.reduce_identifier(&variable_name.identifier)?;

        self.reducer.reduce_variable_name(variable_name, identifier)
    }

    pub fn reduce_definition(
        &self,
        definition: &DefinitionStatement,
        in_circuit: bool,
    ) -> Result<DefinitionStatement, CanonicalizeError> {
        let mut variable_names = vec![];
        for variable_name in definition.variable_names.iter() {
            variable_names.push(self.reduce_variable_name(variable_name)?);
        }

        let type_ = match definition.type_.as_ref() {
            Some(inner) => Some(self.reduce_type(inner, in_circuit, &definition.span)?),
            None => None,
        };

        let value = self.reduce_expression(&definition.value, in_circuit)?;

        self.reducer
            .reduce_definition(definition, variable_names, type_, value, in_circuit)
    }

    pub fn reduce_assignee_access(
        &self,
        access: &AssigneeAccess,
        in_circuit: bool,
    ) -> Result<AssigneeAccess, CanonicalizeError> {
        let new = match access {
            AssigneeAccess::ArrayRange(left, right) => {
                let left = match left.as_ref() {
                    Some(left) => Some(self.reduce_expression(left, in_circuit)?),
                    None => None,
                };
                let right = match right.as_ref() {
                    Some(right) => Some(self.reduce_expression(right, in_circuit)?),
                    None => None,
                };

                AssigneeAccess::ArrayRange(left, right)
            }
            AssigneeAccess::ArrayIndex(index) => {
                AssigneeAccess::ArrayIndex(self.reduce_expression(&index, in_circuit)?)
            }
            AssigneeAccess::Member(identifier) => AssigneeAccess::Member(self.reduce_identifier(&identifier)?),
            _ => access.clone(),
        };

        self.reducer.reduce_assignee_access(access, new, in_circuit)
    }

    pub fn reduce_assignee(&self, assignee: &Assignee, in_circuit: bool) -> Result<Assignee, CanonicalizeError> {
        let identifier = self.reduce_identifier(&assignee.identifier)?;

        let mut accesses = vec![];
        for access in assignee.accesses.iter() {
            accesses.push(self.reduce_assignee_access(access, in_circuit)?);
        }

        self.reducer.reduce_assignee(assignee, identifier, accesses, in_circuit)
    }

    pub fn reduce_assign(
        &self,
        assign: &AssignStatement,
        in_circuit: bool,
    ) -> Result<AssignStatement, CanonicalizeError> {
        let assignee = self.reduce_assignee(&assign.assignee, in_circuit)?;
        let value = self.reduce_expression(&assign.value, in_circuit)?;

        self.reducer.reduce_assign(assign, assignee, value, in_circuit)
    }

    pub fn reduce_conditional(
        &self,
        conditional: &ConditionalStatement,
        in_circuit: bool,
    ) -> Result<ConditionalStatement, CanonicalizeError> {
        let condition = self.reduce_expression(&conditional.condition, in_circuit)?;
        let block = self.reduce_block(&conditional.block, in_circuit)?;
        let next = match conditional.next.as_ref() {
            Some(condition) => Some(self.reduce_statement(condition, in_circuit)?),
            None => None,
        };

        self.reducer
            .reduce_conditional(conditional, condition, block, next, in_circuit)
    }

    pub fn reduce_iteration(
        &self,
        iteration: &IterationStatement,
        in_circuit: bool,
    ) -> Result<IterationStatement, CanonicalizeError> {
        let variable = self.reduce_identifier(&iteration.variable)?;
        let start = self.reduce_expression(&iteration.start, in_circuit)?;
        let stop = self.reduce_expression(&iteration.stop, in_circuit)?;
        let block = self.reduce_block(&iteration.block, in_circuit)?;

        self.reducer
            .reduce_iteration(iteration, variable, start, stop, block, in_circuit)
    }

    pub fn reduce_console(
        &self,
        console_function_call: &ConsoleStatement,
        in_circuit: bool,
    ) -> Result<ConsoleStatement, CanonicalizeError> {
        let function = match &console_function_call.function {
            ConsoleFunction::Assert(expression) => {
                ConsoleFunction::Assert(self.reduce_expression(expression, in_circuit)?)
            }
            ConsoleFunction::Debug(format) | ConsoleFunction::Error(format) | ConsoleFunction::Log(format) => {
                let mut parameters = vec![];
                for parameter in format.parameters.iter() {
                    parameters.push(self.reduce_expression(parameter, in_circuit)?);
                }

                let formatted = FormattedString {
                    parts: format.parts.clone(),
                    parameters,
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

        self.reducer.reduce_console(console_function_call, function, in_circuit)
    }

    pub fn reduce_expression_statement(
        &self,
        expression: &ExpressionStatement,
        in_circuit: bool,
    ) -> Result<ExpressionStatement, CanonicalizeError> {
        let inner_expression = self.reduce_expression(&expression.expression, in_circuit)?;
        self.reducer
            .reduce_expression_statement(expression, inner_expression, in_circuit)
    }

    pub fn reduce_block(&self, block: &Block, in_circuit: bool) -> Result<Block, CanonicalizeError> {
        let mut statements = vec![];
        for statement in block.statements.iter() {
            statements.push(self.reduce_statement(statement, in_circuit)?);
        }

        self.reducer.reduce_block(block, statements, in_circuit)
    }

    // Program
    pub fn reduce_program(&self, program: &Program) -> Result<Program, CanonicalizeError> {
        let mut inputs = vec![];
        for input in program.expected_input.iter() {
            inputs.push(self.reduce_function_input(input, false)?);
        }

        let mut imports = vec![];
        for import in program.imports.iter() {
            imports.push(self.reduce_import(import)?);
        }

        let mut circuits = IndexMap::new();
        for (identifier, circuit) in program.circuits.iter() {
            circuits.insert(self.reduce_identifier(identifier)?, self.reduce_circuit(circuit)?);
        }

        let mut functions = IndexMap::new();
        for (identifier, function) in program.functions.iter() {
            functions.insert(
                self.reduce_identifier(identifier)?,
                self.reduce_function(function, false)?,
            );
        }

        self.reducer
            .reduce_program(program, inputs, imports, circuits, functions)
    }

    pub fn reduce_function_input_variable(
        &self,
        variable: &FunctionInputVariable,
        in_circuit: bool,
    ) -> Result<FunctionInputVariable, CanonicalizeError> {
        let identifier = self.reduce_identifier(&variable.identifier)?;
        let type_ = self.reduce_type(&variable.type_, in_circuit, &variable.span)?;

        self.reducer
            .reduce_function_input_variable(variable, identifier, type_, in_circuit)
    }

    pub fn reduce_function_input(
        &self,
        input: &FunctionInput,
        in_circuit: bool,
    ) -> Result<FunctionInput, CanonicalizeError> {
        let new = match input {
            FunctionInput::Variable(function_input_variable) => {
                FunctionInput::Variable(self.reduce_function_input_variable(function_input_variable, in_circuit)?)
            }
            _ => input.clone(),
        };

        self.reducer.reduce_function_input(input, new, in_circuit)
    }

    pub fn reduce_package_or_packages(
        &self,
        package_or_packages: &PackageOrPackages,
    ) -> Result<PackageOrPackages, CanonicalizeError> {
        let new = match package_or_packages {
            PackageOrPackages::Package(package) => PackageOrPackages::Package(Package {
                name: self.reduce_identifier(&package.name)?,
                access: package.access.clone(),
                span: package.span.clone(),
            }),
            PackageOrPackages::Packages(packages) => PackageOrPackages::Packages(Packages {
                name: self.reduce_identifier(&packages.name)?,
                accesses: packages.accesses.clone(),
                span: packages.span.clone(),
            }),
        };

        self.reducer.reduce_package_or_packages(package_or_packages, new)
    }

    pub fn reduce_import(&self, import: &ImportStatement) -> Result<ImportStatement, CanonicalizeError> {
        let package_or_packages = self.reduce_package_or_packages(&import.package_or_packages)?;

        self.reducer.reduce_import(import, package_or_packages)
    }

    pub fn reduce_circuit_member(&self, circuit_member: &CircuitMember) -> Result<CircuitMember, CanonicalizeError> {
        let new = match circuit_member {
            CircuitMember::CircuitVariable(identifier, type_) => CircuitMember::CircuitVariable(
                self.reduce_identifier(&identifier)?,
                self.reduce_type(&type_, true, &identifier.span)?,
            ),
            CircuitMember::CircuitFunction(function) => {
                CircuitMember::CircuitFunction(self.reduce_function(&function, true)?)
            }
        };

        self.reducer.reduce_circuit_member(circuit_member, new)
    }

    pub fn reduce_circuit(&self, circuit: &Circuit) -> Result<Circuit, CanonicalizeError> {
        let circuit_name = self.reduce_identifier(&circuit.circuit_name)?;

        let mut members = vec![];
        for member in circuit.members.iter() {
            members.push(self.reduce_circuit_member(member)?);
        }

        self.reducer.reduce_circuit(circuit, circuit_name, members)
    }

    fn reduce_annotation(&self, annotation: &Annotation) -> Result<Annotation, CanonicalizeError> {
        let name = self.reduce_identifier(&annotation.name)?;

        self.reducer.reduce_annotation(annotation, name)
    }

    pub fn reduce_function(&self, function: &Function, in_circuit: bool) -> Result<Function, CanonicalizeError> {
        let identifier = self.reduce_identifier(&function.identifier)?;

        let mut annotations = vec![];
        for annotation in function.annotations.iter() {
            annotations.push(self.reduce_annotation(annotation)?);
        }

        let mut inputs = vec![];
        for input in function.input.iter() {
            inputs.push(self.reduce_function_input(input, false)?);
        }

        let output = match function.output.as_ref() {
            Some(type_) => Some(self.reduce_type(type_, in_circuit, &function.span)?),
            None => None,
        };

        let block = self.reduce_block(&function.block, false)?;

        self.reducer
            .reduce_function(function, identifier, annotations, inputs, output, block, in_circuit)
    }
}

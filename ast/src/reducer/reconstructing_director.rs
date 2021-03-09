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
    ArrayAccessExpression,
    ArrayInitExpression,
    ArrayInlineExpression,
    ArrayRangeAccessExpression,
    AssignStatement,
    Assignee,
    AssigneeAccess,
    BinaryExpression,
    Block,
    CallExpression,
    Circuit,
    CircuitImpliedVariableDefinition,
    CircuitInitExpression,
    CircuitMember,
    CircuitMemberAccessExpression,
    CircuitStaticFunctionAccessExpression,
    ConditionalStatement,
    ConsoleFunction,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionStatement,
    FormattedString,
    Function,
    FunctionInput,
    FunctionInputVariable,
    Identifier,
    ImportStatement,
    IterationStatement,
    PackageOrPackages,
    Program,
    ReconstructingReducer,
    ReturnStatement,
    SpreadOrExpression,
    Statement,
    TernaryExpression,
    TestFunction,
    TupleAccessExpression,
    TupleInitExpression,
    Type,
    UnaryExpression,
    VariableName,
};

pub struct ReconstructingDirector<R: ReconstructingReducer> {
    reducer: R,
}

impl<R: ReconstructingReducer> ReconstructingDirector<R> {
    pub fn new(reducer: R) -> Self {
        Self { reducer }
    }

    pub fn reduce_program(&mut self, program: &Program) -> Program {
        let inputs = program
            .expected_input
            .iter()
            .filter_map(|x| self.reduce_function_input(x))
            .collect();
        let imports = program
            .imports
            .iter()
            .filter_map(|x| self.reduce_import_statement(x))
            .collect();
        let circuits = program
            .circuits
            .iter()
            .filter_map(|(identifier, circuit)| {
                Some((self.reduce_identifier(identifier), self.reduce_circuit(circuit)?))
            })
            .collect();
        let functions = program
            .functions
            .iter()
            .filter_map(|(identifier, function)| {
                Some((self.reduce_identifier(identifier), self.reduce_function(function)?))
            })
            .collect();
        let test_functions = program
            .tests
            .iter()
            .filter_map(|(identifier, test_function)| {
                Some((
                    self.reduce_identifier(identifier),
                    self.reduce_test_function(test_function)?,
                ))
            })
            .collect();

        self.reducer
            .reduce_program(program, inputs, imports, circuits, functions, test_functions)
    }

    pub fn reduce_function_input(&mut self, input: &FunctionInput) -> Option<FunctionInput> {
        let item = match input {
            FunctionInput::InputKeyword(input_keyword) => FunctionInput::InputKeyword(input_keyword.clone()),
            FunctionInput::SelfKeyword(self_keyword) => FunctionInput::SelfKeyword(self_keyword.clone()),
            FunctionInput::MutSelfKeyword(mut_self_keyword) => FunctionInput::MutSelfKeyword(mut_self_keyword.clone()),
            FunctionInput::Variable(function_input_variable) => {
                FunctionInput::Variable(self.reduce_function_input_variable(function_input_variable))
            }
        };

        self.reducer.reduce_function_input(input, item)
    }

    pub fn reduce_import_statement(&mut self, import: &ImportStatement) -> Option<ImportStatement> {
        let package = self.reduce_package(&import.package_or_packages);

        self.reducer.reduce_import_statement(import, package)
    }

    pub fn reduce_circuit(&mut self, circuit: &Circuit) -> Option<Circuit> {
        let circuit_name = self.reduce_identifier(&circuit.circuit_name);
        let members = circuit
            .members
            .iter()
            .filter_map(|x| self.reduce_circuit_member(x))
            .collect();

        self.reducer.reduce_circuit(circuit, circuit_name, members)
    }

    pub fn reduce_function(&mut self, function: &Function) -> Option<Function> {
        let identifier = self.reduce_identifier(&function.identifier);
        let input = function
            .input
            .iter()
            .filter_map(|x| self.reduce_function_input(x))
            .collect();
        let output = function.output.as_ref().map(|x| self.reduce_type(x));
        let block = Block {
            statements: function
                .block
                .statements
                .iter()
                .map(|x| self.reduce_statement(x))
                .collect(),
            span: function.block.span.clone(),
        };

        self.reducer.reduce_function(function, identifier, input, output, block)
    }

    pub fn reduce_test_function(&mut self, test_function: &TestFunction) -> Option<TestFunction> {
        let function = self.reduce_function(&test_function.function);
        let input_file = test_function.input_file.as_ref().map(|x| self.reduce_identifier(x));

        self.reducer.reduce_test_function(test_function, function?, input_file)
    }

    pub fn reduce_identifier(&mut self, identifier: &Identifier) -> Identifier {
        self.reducer.reduce_identifier(identifier)
    }

    pub fn reduce_function_input_variable(
        &mut self,
        function_input_variable: &FunctionInputVariable,
    ) -> FunctionInputVariable {
        let identifier = self.reduce_identifier(&function_input_variable.identifier);
        let type_ = self.reduce_type(&function_input_variable.type_);

        self.reducer
            .reduce_function_input_variable(function_input_variable, identifier, type_)
    }

    pub fn reduce_type(&mut self, type_: &Type) -> Type {
        let items = match type_ {
            // Data type wrappers
            Type::Array(type_, dimensions) => Type::Array(Box::new(self.reduce_type(type_)), dimensions.clone()),
            Type::Tuple(types) => Type::Tuple(types.iter().map(|x| self.reduce_type(x)).collect()),
            Type::Circuit(identifier) => Type::Circuit(self.reduce_identifier(identifier)),
            _ => type_.clone(),
        };

        self.reducer.reduce_type(type_, items)
    }

    pub fn reduce_package(&mut self, package_or_packages: &PackageOrPackages) -> PackageOrPackages {
        self.reducer.reduce_package(package_or_packages)
    }

    pub fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember) -> Option<CircuitMember> {
        let items = match circuit_member {
            CircuitMember::CircuitVariable(identifier, type_) => {
                CircuitMember::CircuitVariable(self.reduce_identifier(identifier), self.reduce_type(type_))
            }
            CircuitMember::CircuitFunction(function) => CircuitMember::CircuitFunction(self.reduce_function(function)?),
        };

        self.reducer.reduce_circuit_member(circuit_member, items)
    }

    pub fn reduce_statement(&mut self, statement: &Statement) -> Statement {
        let items = match statement {
            Statement::Return(return_statement) => Statement::Return(ReturnStatement {
                expression: self.reduce_expression(&return_statement.expression),
                span: return_statement.span.clone(),
            }),
            Statement::Definition(definition) => {
                Statement::Definition(DefinitionStatement {
                    declaration_type: definition.declaration_type.clone(),
                    variable_names: definition
                        .variable_names
                        .iter()
                        .map(|variable_name| self.reduce_variable_name(variable_name))
                        .collect(),
                    type_: Some(self.reduce_type(&definition.type_.as_ref().unwrap())), // TODO fix
                    value: self.reduce_expression(&definition.value),
                    span: definition.span.clone(),
                })
            }
            Statement::Assign(assign) => Statement::Assign(AssignStatement {
                operation: assign.operation.clone(),
                assignee: Assignee {
                    identifier: self.reduce_identifier(&assign.assignee.identifier),
                    accesses: assign
                        .assignee
                        .accesses
                        .iter()
                        .filter_map(|x| self.reduce_assignee_access(x))
                        .collect(),
                    span: assign.assignee.span.clone(),
                },
                value: self.reduce_expression(&assign.value),
                span: assign.span.clone(),
            }),
            Statement::Conditional(conditional) => {
                Statement::Conditional(self.reduce_conditional_statement(conditional))
            }
            Statement::Iteration(iteration) => {
                Statement::Iteration(IterationStatement {
                    variable: self.reduce_identifier(&iteration.variable),
                    start: self.reduce_expression(&iteration.start),
                    stop: self.reduce_expression(&iteration.stop),
                    block: Block {
                        statements: iteration
                            .block
                            .statements
                            .iter()
                            .map(|statement| self.reduce_statement(statement))
                            .collect(),
                        span: iteration.block.span.clone(),
                    }, // TODO reduce block that isn't in a statement
                    span: iteration.span.clone(),
                })
            }
            Statement::Console(console_function_call) => {
                let function = match &console_function_call.function {
                    ConsoleFunction::Assert(expression) => ConsoleFunction::Assert(self.reduce_expression(expression)),
                    ConsoleFunction::Debug(format) | ConsoleFunction::Error(format) | ConsoleFunction::Log(format) => {
                        let formatted = FormattedString {
                            string: format.string.clone(),
                            containers: format.containers.clone(),
                            parameters: format
                                .parameters
                                .iter()
                                .map(|parameter| self.reduce_expression(&parameter))
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
                Statement::Console(ConsoleStatement {
                    function,
                    span: console_function_call.span.clone(),
                })
            }
            Statement::Expression(expression) => Statement::Expression(ExpressionStatement {
                expression: self.reduce_expression(&expression.expression),
                span: expression.span.clone(),
            }),
            Statement::Block(block) => Statement::Block(Block {
                statements: block
                    .statements
                    .iter()
                    .map(|statement| self.reduce_statement(statement))
                    .collect(),
                span: block.span.clone(),
            }),
        };

        self.reducer.reduce_statement(statement, items)
    }

    pub fn reduce_assignee_access(&mut self, assignee_access: &AssigneeAccess) -> Option<AssigneeAccess> {
        let item = match assignee_access {
            AssigneeAccess::ArrayRange(start, stop) => {
                let start_item = start.as_ref().map(|x| self.reduce_expression(x));
                let stop_item = stop.as_ref().map(|x| self.reduce_expression(x));
                AssigneeAccess::ArrayRange(start_item, stop_item)
            }
            AssigneeAccess::ArrayIndex(expression) => AssigneeAccess::ArrayIndex(self.reduce_expression(&expression)),
            AssigneeAccess::Tuple(number, span) => AssigneeAccess::Tuple(number.clone(), span.clone()),
            AssigneeAccess::Member(identifier) => {
                let identifier = self.reduce_identifier(identifier);
                AssigneeAccess::Member(identifier)
            }
        };

        self.reducer.reduce_assignee_access(assignee_access, item)
    }

    pub fn reduce_conditional_statement(&mut self, statement: &ConditionalStatement) -> ConditionalStatement {
        let condition = self.reduce_expression(&statement.condition);
        let statements = Block {
            statements: statement
                .block
                .statements
                .iter()
                .map(|x| self.reduce_statement(x))
                .collect(),
            span: statement.block.span.clone(),
        };
        let next = statement.next.as_ref().map(|x| self.reduce_statement(x));

        self.reducer
            .reduce_conditional_statement(statement, condition, statements, next)
    }

    pub fn reduce_variable_name(&mut self, variable_name: &VariableName) -> VariableName {
        let identifier = self.reduce_identifier(&variable_name.identifier);

        self.reducer.reduce_variable_name(variable_name, identifier)
    }

    pub fn reduce_expression(&mut self, expression: &Expression) -> Expression {
        let items = match expression {
            Expression::Identifier(identifier) => Expression::Identifier(self.reduce_identifier(identifier)),
            // Expression::Value(value) => Expression::Value(self.reduce_expression(value.))
            Expression::Binary(binary) => {
                let left = Box::new(self.reduce_expression(&binary.left));
                let right = Box::new(self.reduce_expression(&binary.right));

                Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op: binary.op.clone(),
                    span: binary.span.clone(),
                })
            }
            Expression::Unary(unary) => {
                let inner = Box::new(self.reduce_expression(&unary.inner));

                Expression::Unary(UnaryExpression {
                    inner,
                    op: unary.op.clone(),
                    span: unary.span.clone(),
                })
            }
            Expression::Ternary(ternary) => {
                let condition = Box::new(self.reduce_expression(&ternary.condition));
                let if_true = Box::new(self.reduce_expression(&ternary.if_true));
                let if_false = Box::new(self.reduce_expression(&ternary.if_false));

                Expression::Ternary(TernaryExpression {
                    condition,
                    if_true,
                    if_false,
                    span: ternary.span.clone(),
                })
            }

            Expression::ArrayInline(array_inline) => {
                let elements = array_inline
                    .elements
                    .iter()
                    .map(|x| match x {
                        SpreadOrExpression::Expression(expression) => {
                            SpreadOrExpression::Expression(self.reduce_expression(expression))
                        }
                        SpreadOrExpression::Spread(expression) => {
                            SpreadOrExpression::Spread(self.reduce_expression(expression))
                        }
                    })
                    .collect();

                Expression::ArrayInline(ArrayInlineExpression {
                    elements,
                    span: array_inline.span.clone(),
                })
            }
            Expression::ArrayInit(array_init) => {
                let element = Box::new(self.reduce_expression(&array_init.element));

                Expression::ArrayInit(ArrayInitExpression {
                    element,
                    dimensions: array_init.dimensions.clone(),
                    span: array_init.span.clone(),
                })
            }
            Expression::ArrayAccess(array_access) => {
                let array = Box::new(self.reduce_expression(&array_access.array));
                let index = Box::new(self.reduce_expression(&array_access.index));
                Expression::ArrayAccess(ArrayAccessExpression {
                    array,
                    index,
                    span: array_access.span.clone(),
                })
            }
            Expression::ArrayRangeAccess(array_range_access) => {
                let array = Box::new(self.reduce_expression(&array_range_access.array));
                let left = array_range_access
                    .left
                    .as_ref()
                    .map(|left| Box::new(self.reduce_expression(&left)));
                let right = array_range_access
                    .right
                    .as_ref()
                    .map(|right| Box::new(self.reduce_expression(&right)));

                Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                    array,
                    left,
                    right,
                    span: array_range_access.span.clone(),
                })
            }

            Expression::TupleInit(tuple_init) => {
                let elements = tuple_init.elements.iter().map(|x| self.reduce_expression(x)).collect();

                Expression::TupleInit(TupleInitExpression {
                    elements,
                    span: tuple_init.span.clone(),
                })
            }
            Expression::TupleAccess(tuple_access) => {
                let tuple = Box::new(self.reduce_expression(&tuple_access.tuple));

                Expression::TupleAccess(TupleAccessExpression {
                    tuple,
                    index: tuple_access.index.clone(),
                    span: tuple_access.span.clone(),
                })
            }
            Expression::CircuitInit(circuit_init) => {
                let name = self.reduce_identifier(&circuit_init.name);
                let members = circuit_init
                    .members
                    .iter()
                    .map(|definition| {
                        let identifier = self.reduce_identifier(&definition.identifier);
                        let expression = self.reduce_expression(&definition.expression);

                        CircuitImpliedVariableDefinition { identifier, expression }
                    })
                    .collect();

                Expression::CircuitInit(CircuitInitExpression {
                    name,
                    members,
                    span: circuit_init.span.clone(),
                })
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                let circuit = Box::new(self.reduce_expression(&circuit_member_access.circuit));
                let name = self.reduce_identifier(&circuit_member_access.name);

                Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                    circuit,
                    name,
                    span: circuit_member_access.span.clone(),
                })
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_func_access) => {
                let circuit = Box::new(self.reduce_expression(&circuit_static_func_access.circuit));
                let name = self.reduce_identifier(&circuit_static_func_access.name);

                Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression {
                    circuit,
                    name,
                    span: circuit_static_func_access.span.clone(),
                })
            }
            Expression::Call(call) => {
                let function = Box::new(self.reduce_expression(&call.function));
                let arguments = call.arguments.iter().map(|x| self.reduce_expression(x)).collect();

                Expression::Call(CallExpression {
                    function,
                    arguments,
                    span: call.span.clone(),
                })
            }

            x => x.clone(), // leaf nodes we dont reconstruct
        };

        self.reducer.reduce_expression(expression, items)
    }
}

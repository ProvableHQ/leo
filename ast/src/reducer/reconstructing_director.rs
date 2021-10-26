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
use leo_errors::{AstError, Result, Span};

pub struct ReconstructingDirector<R: ReconstructingReducer> {
    reducer: R,
}

impl<R: ReconstructingReducer> ReconstructingDirector<R> {
    pub fn new(reducer: R) -> Self {
        Self { reducer }
    }

    pub fn reduce_type(&mut self, type_: &Type, span: &Span) -> Result<Type> {
        let new = match type_ {
            Type::Array(type_, dimensions) => Type::Array(Box::new(self.reduce_type(type_, span)?), dimensions.clone()),
            Type::Tuple(types) => {
                let mut reduced_types = vec![];
                for type_ in types.iter() {
                    reduced_types.push(self.reduce_type(type_, span)?);
                }

                Type::Tuple(reduced_types)
            }
            Type::Identifier(identifier) => Type::Identifier(self.reduce_identifier(identifier)?),
            _ => type_.clone(),
        };

        self.reducer.reduce_type(type_, new, span)
    }

    // Expressions
    pub fn reduce_expression(&mut self, expression: &Expression) -> Result<Expression> {
        let new = match expression {
            Expression::Identifier(identifier) => Expression::Identifier(self.reduce_identifier(identifier)?),
            Expression::Value(value) => self.reduce_value(value)?,
            Expression::Binary(binary) => Expression::Binary(self.reduce_binary(binary)?),
            Expression::Unary(unary) => Expression::Unary(self.reduce_unary(unary)?),
            Expression::Ternary(ternary) => Expression::Ternary(self.reduce_ternary(ternary)?),
            Expression::Cast(cast) => Expression::Cast(self.reduce_cast(cast)?),
            Expression::Access(access) => Expression::Access(self.reduce_access(access)?),
            Expression::LengthOf(lengthof) => Expression::LengthOf(lengthof.clone()), // Expression::LengthOf(self.reduce_lengthof(lengthof)?), // TODO: add reducer

            Expression::ArrayInline(array_inline) => Expression::ArrayInline(self.reduce_array_inline(array_inline)?),
            Expression::ArrayInit(array_init) => Expression::ArrayInit(self.reduce_array_init(array_init)?),

            Expression::TupleInit(tuple_init) => Expression::TupleInit(self.reduce_tuple_init(tuple_init)?),

            Expression::CircuitInit(circuit_init) => Expression::CircuitInit(self.reduce_circuit_init(circuit_init)?),

            Expression::Call(call) => Expression::Call(self.reduce_call(call)?),
        };

        self.reducer.reduce_expression(expression, new)
    }

    pub fn reduce_identifier(&mut self, identifier: &Identifier) -> Result<Identifier> {
        self.reducer.reduce_identifier(identifier)
    }

    pub fn reduce_group_tuple(&mut self, group_tuple: &GroupTuple) -> Result<GroupTuple> {
        self.reducer.reduce_group_tuple(group_tuple)
    }

    pub fn reduce_group_value(&mut self, group_value: &GroupValue) -> Result<GroupValue> {
        let new = match group_value {
            GroupValue::Tuple(group_tuple) => GroupValue::Tuple(self.reduce_group_tuple(group_tuple)?),
            _ => group_value.clone(),
        };

        self.reducer.reduce_group_value(group_value, new)
    }

    pub fn reduce_string(&mut self, string: &[Char], span: &Span) -> Result<Expression> {
        self.reducer.reduce_string(string, span)
    }

    pub fn reduce_value(&mut self, value: &ValueExpression) -> Result<Expression> {
        let new = match value {
            ValueExpression::Group(group_value) => {
                Expression::Value(ValueExpression::Group(Box::new(self.reduce_group_value(group_value)?)))
            }
            ValueExpression::String(string, span) => self.reduce_string(string, span)?,
            _ => Expression::Value(value.clone()),
        };

        self.reducer.reduce_value(value, new)
    }

    pub fn reduce_binary(&mut self, binary: &BinaryExpression) -> Result<BinaryExpression> {
        let left = self.reduce_expression(&binary.left)?;
        let right = self.reduce_expression(&binary.right)?;

        self.reducer.reduce_binary(binary, left, right, binary.op.clone())
    }

    pub fn reduce_unary(&mut self, unary: &UnaryExpression) -> Result<UnaryExpression> {
        let inner = self.reduce_expression(&unary.inner)?;

        self.reducer.reduce_unary(unary, inner, unary.op.clone())
    }

    pub fn reduce_ternary(&mut self, ternary: &TernaryExpression) -> Result<TernaryExpression> {
        let condition = self.reduce_expression(&ternary.condition)?;
        let if_true = self.reduce_expression(&ternary.if_true)?;
        let if_false = self.reduce_expression(&ternary.if_false)?;

        self.reducer.reduce_ternary(ternary, condition, if_true, if_false)
    }

    pub fn reduce_cast(&mut self, cast: &CastExpression) -> Result<CastExpression> {
        let inner = self.reduce_expression(&cast.inner)?;
        let target_type = self.reduce_type(&cast.target_type, &cast.span)?;

        self.reducer.reduce_cast(cast, inner, target_type)
    }

    pub fn reduce_array_access(&mut self, array_access: &ArrayAccess) -> Result<ArrayAccess> {
        let array = self.reduce_expression(&array_access.array)?;
        let index = self.reduce_expression(&array_access.index)?;

        self.reducer.reduce_array_access(array_access, array, index)
    }

    pub fn reduce_array_range_access(&mut self, array_range_access: &ArrayRangeAccess) -> Result<ArrayRangeAccess> {
        let array = self.reduce_expression(&array_range_access.array)?;
        let left = array_range_access
            .left
            .as_ref()
            .map(|left| self.reduce_expression(left))
            .transpose()?;
        let right = array_range_access
            .right
            .as_ref()
            .map(|right| self.reduce_expression(right))
            .transpose()?;

        self.reducer
            .reduce_array_range_access(array_range_access, array, left, right)
    }

    pub fn reduce_member_access(&mut self, member_access: &MemberAccess) -> Result<MemberAccess> {
        let inner = self.reduce_expression(&member_access.inner)?;
        let name = self.reduce_identifier(&member_access.name)?;
        let type_ = member_access
            .type_
            .as_ref()
            .map(|type_| self.reduce_type(type_, &member_access.span))
            .transpose()?;

        self.reducer.reduce_member_access(member_access, inner, name, type_)
    }

    pub fn reduce_tuple_access(&mut self, tuple_access: &TupleAccess) -> Result<TupleAccess> {
        let tuple = self.reduce_expression(&tuple_access.tuple)?;

        self.reducer.reduce_tuple_access(tuple_access, tuple)
    }

    pub fn reduce_static_access(&mut self, static_access: &StaticAccess) -> Result<StaticAccess> {
        let value = self.reduce_expression(&static_access.inner)?;
        let name = self.reduce_identifier(&static_access.name)?;
        let type_ = static_access
            .type_
            .as_ref()
            .map(|type_| self.reduce_type(type_, &static_access.span))
            .transpose()?;

        self.reducer.reduce_static_access(static_access, value, type_, name)
    }

    pub fn reduce_access(&mut self, access: &AccessExpression) -> Result<AccessExpression> {
        use AccessExpression::*;

        let new = match access {
            Array(access) => Array(self.reduce_array_access(access)?),
            ArrayRange(access) => ArrayRange(self.reduce_array_range_access(access)?),
            Member(access) => Member(self.reduce_member_access(access)?),
            Tuple(access) => Tuple(self.reduce_tuple_access(access)?),
            Static(access) => Static(self.reduce_static_access(access)?),
        };

        Ok(new)
    }

    pub fn reduce_array_inline(&mut self, array_inline: &ArrayInlineExpression) -> Result<ArrayInlineExpression> {
        let mut elements = vec![];
        for element in array_inline.elements.iter() {
            let reduced_element = match element {
                SpreadOrExpression::Expression(expression) => {
                    SpreadOrExpression::Expression(self.reduce_expression(expression)?)
                }
                SpreadOrExpression::Spread(expression) => {
                    SpreadOrExpression::Spread(self.reduce_expression(expression)?)
                }
            };

            elements.push(reduced_element);
        }

        self.reducer.reduce_array_inline(array_inline, elements)
    }

    pub fn reduce_array_init(&mut self, array_init: &ArrayInitExpression) -> Result<ArrayInitExpression> {
        let element = self.reduce_expression(&array_init.element)?;

        self.reducer.reduce_array_init(array_init, element)
    }

    pub fn reduce_tuple_init(&mut self, tuple_init: &TupleInitExpression) -> Result<TupleInitExpression> {
        let mut elements = vec![];
        for element in tuple_init.elements.iter() {
            elements.push(self.reduce_expression(element)?);
        }

        self.reducer.reduce_tuple_init(tuple_init, elements)
    }

    pub fn reduce_circuit_implied_variable_definition(
        &mut self,
        variable: &CircuitImpliedVariableDefinition,
    ) -> Result<CircuitImpliedVariableDefinition> {
        let identifier = self.reduce_identifier(&variable.identifier)?;
        let expression = variable
            .expression
            .as_ref()
            .map(|expr| self.reduce_expression(expr))
            .transpose()?;

        self.reducer
            .reduce_circuit_implied_variable_definition(variable, identifier, expression)
    }

    pub fn reduce_circuit_init(&mut self, circuit_init: &CircuitInitExpression) -> Result<CircuitInitExpression> {
        let name = self.reduce_identifier(&circuit_init.name)?;

        let mut members = vec![];
        for member in circuit_init.members.iter() {
            members.push(self.reduce_circuit_implied_variable_definition(member)?);
        }

        self.reducer.reduce_circuit_init(circuit_init, name, members)
    }

    pub fn reduce_call(&mut self, call: &CallExpression) -> Result<CallExpression> {
        let function = self.reduce_expression(&call.function)?;

        let mut arguments = vec![];
        for argument in call.arguments.iter() {
            arguments.push(self.reduce_expression(argument)?);
        }

        self.reducer.reduce_call(call, function, arguments)
    }

    // Statements
    pub fn reduce_statement(&mut self, statement: &Statement) -> Result<Statement> {
        let new = match statement {
            Statement::Return(return_statement) => Statement::Return(self.reduce_return(return_statement)?),
            Statement::Definition(definition) => Statement::Definition(self.reduce_definition(definition)?),
            Statement::Assign(assign) => Statement::Assign(Box::new(self.reduce_assign(assign)?)),
            Statement::Conditional(conditional) => Statement::Conditional(self.reduce_conditional(conditional)?),
            Statement::Iteration(iteration) => Statement::Iteration(Box::new(self.reduce_iteration(iteration)?)),
            Statement::Console(console) => Statement::Console(self.reduce_console(console)?),
            Statement::Expression(expression) => Statement::Expression(self.reduce_expression_statement(expression)?),
            Statement::Block(block) => Statement::Block(self.reduce_block(block)?),
        };

        self.reducer.reduce_statement(statement, new)
    }

    pub fn reduce_return(&mut self, return_statement: &ReturnStatement) -> Result<ReturnStatement> {
        let expression = self.reduce_expression(&return_statement.expression)?;

        self.reducer.reduce_return(return_statement, expression)
    }

    pub fn reduce_variable_name(&mut self, variable_name: &VariableName) -> Result<VariableName> {
        let identifier = self.reduce_identifier(&variable_name.identifier)?;

        self.reducer.reduce_variable_name(variable_name, identifier)
    }

    pub fn reduce_definition(&mut self, definition: &DefinitionStatement) -> Result<DefinitionStatement> {
        let mut variable_names = vec![];
        for variable_name in definition.variable_names.iter() {
            variable_names.push(self.reduce_variable_name(variable_name)?);
        }

        let type_ = definition
            .type_
            .as_ref()
            .map(|type_| self.reduce_type(type_, &definition.span))
            .transpose()?;

        let value = self.reduce_expression(&definition.value)?;

        self.reducer.reduce_definition(definition, variable_names, type_, value)
    }

    pub fn reduce_assignee_access(&mut self, access: &AssigneeAccess) -> Result<AssigneeAccess> {
        let new = match access {
            AssigneeAccess::ArrayRange(left, right) => {
                let left = left.as_ref().map(|left| self.reduce_expression(left)).transpose()?;
                let right = right.as_ref().map(|right| self.reduce_expression(right)).transpose()?;

                AssigneeAccess::ArrayRange(left, right)
            }
            AssigneeAccess::ArrayIndex(index) => AssigneeAccess::ArrayIndex(self.reduce_expression(index)?),
            AssigneeAccess::Member(identifier) => AssigneeAccess::Member(self.reduce_identifier(identifier)?),
            _ => access.clone(),
        };

        self.reducer.reduce_assignee_access(access, new)
    }

    pub fn reduce_assignee(&mut self, assignee: &Assignee) -> Result<Assignee> {
        let identifier = self.reduce_identifier(&assignee.identifier)?;

        let mut accesses = vec![];
        for access in assignee.accesses.iter() {
            accesses.push(self.reduce_assignee_access(access)?);
        }

        self.reducer.reduce_assignee(assignee, identifier, accesses)
    }

    pub fn reduce_assign(&mut self, assign: &AssignStatement) -> Result<AssignStatement> {
        let assignee = self.reduce_assignee(&assign.assignee)?;
        let value = self.reduce_expression(&assign.value)?;

        self.reducer.reduce_assign(assign, assignee, value)
    }

    pub fn reduce_conditional(&mut self, conditional: &ConditionalStatement) -> Result<ConditionalStatement> {
        let condition = self.reduce_expression(&conditional.condition)?;
        let block = self.reduce_block(&conditional.block)?;
        let next = conditional
            .next
            .as_ref()
            .map(|condition| self.reduce_statement(condition))
            .transpose()?;

        self.reducer.reduce_conditional(conditional, condition, block, next)
    }

    pub fn reduce_iteration(&mut self, iteration: &IterationStatement) -> Result<IterationStatement> {
        let variable = self.reduce_identifier(&iteration.variable)?;
        let start = self.reduce_expression(&iteration.start)?;
        let stop = self.reduce_expression(&iteration.stop)?;
        let block = self.reduce_block(&iteration.block)?;

        self.reducer.reduce_iteration(iteration, variable, start, stop, block)
    }

    pub fn reduce_console(&mut self, console_function_call: &ConsoleStatement) -> Result<ConsoleStatement> {
        let function = match &console_function_call.function {
            ConsoleFunction::Assert(expression) => ConsoleFunction::Assert(self.reduce_expression(expression)?),
            ConsoleFunction::Error(args) | ConsoleFunction::Log(args) => {
                let mut parameters = vec![];
                for parameter in args.parameters.iter() {
                    parameters.push(self.reduce_expression(parameter)?);
                }

                let formatted = ConsoleArgs {
                    string: args.string.clone(),
                    parameters,
                    span: args.span.clone(),
                };

                match &console_function_call.function {
                    ConsoleFunction::Error(_) => ConsoleFunction::Error(formatted),
                    ConsoleFunction::Log(_) => ConsoleFunction::Log(formatted),
                    _ => return Err(AstError::impossible_console_assert_call(&args.span).into()),
                }
            }
        };

        self.reducer.reduce_console(console_function_call, function)
    }

    pub fn reduce_expression_statement(&mut self, expression: &ExpressionStatement) -> Result<ExpressionStatement> {
        let inner_expression = self.reduce_expression(&expression.expression)?;
        self.reducer.reduce_expression_statement(expression, inner_expression)
    }

    pub fn reduce_block(&mut self, block: &Block) -> Result<Block> {
        let mut statements = vec![];
        for statement in block.statements.iter() {
            statements.push(self.reduce_statement(statement)?);
        }

        self.reducer.reduce_block(block, statements)
    }

    // Program
    pub fn reduce_program(&mut self, program: &Program) -> Result<Program> {
        let mut inputs = vec![];
        for input in program.expected_input.iter() {
            inputs.push(self.reduce_function_input(input)?);
        }

        let mut import_statements = vec![];
        for import in program.import_statements.iter() {
            import_statements.push(self.reduce_import_statement(import)?);
        }

        let mut imports = IndexMap::new();
        for (identifier, program) in program.imports.iter() {
            let (ident, import) = self.reduce_import(identifier, program)?;
            imports.insert(ident, import);
        }

        let mut aliases = IndexMap::new();
        for (name, alias) in program.aliases.iter() {
            let represents = self.reduce_type(&alias.represents, &alias.name.span)?;
            aliases.insert(
                name.clone(),
                Alias {
                    name: alias.name.clone(),
                    span: alias.span.clone(),
                    represents,
                },
            );
        }

        let mut circuits = IndexMap::new();
        self.reducer.swap_in_circuit();
        for (name, circuit) in program.circuits.iter() {
            circuits.insert(name.clone(), self.reduce_circuit(circuit)?);
        }
        self.reducer.swap_in_circuit();

        let mut functions = IndexMap::new();
        for (name, function) in program.functions.iter() {
            functions.insert(name.clone(), self.reduce_function(function)?);
        }

        let mut global_consts = IndexMap::new();
        for (name, definition) in program.global_consts.iter() {
            global_consts.insert(name.clone(), self.reduce_definition(definition)?);
        }

        self.reducer.reduce_program(
            program,
            inputs,
            import_statements,
            imports,
            aliases,
            circuits,
            functions,
            global_consts,
        )
    }

    pub fn reduce_function_input_variable(
        &mut self,
        variable: &FunctionInputVariable,
    ) -> Result<FunctionInputVariable> {
        let identifier = self.reduce_identifier(&variable.identifier)?;
        let type_ = self.reduce_type(&variable.type_, &variable.span)?;

        self.reducer.reduce_function_input_variable(variable, identifier, type_)
    }

    pub fn reduce_function_input(&mut self, input: &FunctionInput) -> Result<FunctionInput> {
        let new = match input {
            FunctionInput::Variable(function_input_variable) => {
                FunctionInput::Variable(self.reduce_function_input_variable(function_input_variable)?)
            }
            _ => input.clone(),
        };

        self.reducer.reduce_function_input(input, new)
    }

    pub fn reduce_package_or_packages(&mut self, package_or_packages: &PackageOrPackages) -> Result<PackageOrPackages> {
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

    pub fn reduce_import_statement(&mut self, import: &ImportStatement) -> Result<ImportStatement> {
        let package_or_packages = self.reduce_package_or_packages(&import.package_or_packages)?;

        self.reducer.reduce_import_statement(import, package_or_packages)
    }

    pub fn reduce_import(&mut self, identifier: &[String], import: &Program) -> Result<(Vec<String>, Program)> {
        let new_identifer = identifier.to_vec();
        let new_import = self.reduce_program(import)?;
        self.reducer.reduce_import(new_identifer, new_import)
    }

    pub fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember) -> Result<CircuitMember> {
        let new = match circuit_member {
            CircuitMember::CircuitConst(identifier, type_, value) => CircuitMember::CircuitConst(
                self.reduce_identifier(identifier)?,
                self.reduce_type(type_, &identifier.span)?,
                self.reduce_expression(value)?,
            ),
            CircuitMember::CircuitVariable(identifier, type_) => CircuitMember::CircuitVariable(
                self.reduce_identifier(identifier)?,
                self.reduce_type(type_, &identifier.span)?,
            ),
            CircuitMember::CircuitFunction(function) => {
                CircuitMember::CircuitFunction(Box::new(self.reduce_function(function)?))
            }
        };

        self.reducer.reduce_circuit_member(circuit_member, new)
    }

    pub fn reduce_circuit(&mut self, circuit: &Circuit) -> Result<Circuit> {
        let circuit_name = self.reduce_identifier(&circuit.circuit_name)?;

        let mut members = vec![];
        for member in circuit.members.iter() {
            members.push(self.reduce_circuit_member(member)?);
        }

        self.reducer.reduce_circuit(circuit, circuit_name, members)
    }

    fn reduce_annotation(&mut self, annotation: &Annotation) -> Result<Annotation> {
        let name = self.reduce_identifier(&annotation.name)?;

        self.reducer.reduce_annotation(annotation, name)
    }

    pub fn reduce_function(&mut self, function: &Function) -> Result<Function> {
        let identifier = self.reduce_identifier(&function.identifier)?;

        let mut annotations = vec![];
        for annotation in function.annotations.iter() {
            annotations.push(self.reduce_annotation(annotation)?);
        }

        let mut inputs = vec![];
        for input in function.input.iter() {
            inputs.push(self.reduce_function_input(input)?);
        }

        let output = function
            .output
            .as_ref()
            .map(|type_| self.reduce_type(type_, &function.span))
            .transpose()?;

        let block = self.reduce_block(&function.block)?;

        self.reducer
            .reduce_function(function, identifier, annotations, inputs, output, block)
    }
}

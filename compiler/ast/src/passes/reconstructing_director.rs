// Copyright (C) 2019-2022 Aleo Systems Inc.
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

//! This module contains a Director for how to map over the AST
//! and applies a reducer call to each node.

use crate::*;

use leo_errors::{AstError, Result};
use leo_span::Span;

use indexmap::IndexMap;

pub struct ReconstructingDirector<R: ReconstructingReducer> {
    reducer: R,
}

impl<R: ReconstructingReducer> ReconstructingDirector<R> {
    pub fn new(reducer: R) -> Self {
        Self { reducer }
    }

    pub fn reduce_type(&mut self, type_: &Type, span: &Span) -> Result<Type> {
        self.reducer.reduce_type(type_, *type_, span)
    }

    // Expressions
    pub fn reduce_expression(&mut self, expression: &Expression) -> Result<Expression> {
        let new = match expression {
            Expression::Identifier(identifier) => Expression::Identifier(self.reduce_identifier(identifier)?),
            Expression::Value(value) => self.reduce_value(value)?,
            Expression::Binary(binary) => Expression::Binary(self.reduce_binary(binary)?),
            Expression::Unary(unary) => Expression::Unary(self.reduce_unary(unary)?),
            Expression::Ternary(ternary) => Expression::Ternary(self.reduce_ternary(ternary)?),
            Expression::Call(call) => Expression::Call(self.reduce_call(call)?),
            Expression::Err(s) => Expression::Err(s.clone()),
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

    pub fn reduce_string(&mut self, string: &str, span: &Span) -> Result<Expression> {
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

        self.reducer.reduce_binary(binary, left, right, binary.op)
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

        let type_ = self.reduce_type(&definition.type_, &definition.span)?;

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
        let type_ = self.reduce_type(&iteration.type_, &iteration.span())?;
        let start = self.reduce_expression(&iteration.start)?;
        let stop = self.reduce_expression(&iteration.stop)?;
        let block = self.reduce_block(&iteration.block)?;

        self.reducer
            .reduce_iteration(iteration, variable, type_, start, stop, block)
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
                    span: args.span,
                };

                match &console_function_call.function {
                    ConsoleFunction::Error(_) => ConsoleFunction::Error(formatted),
                    ConsoleFunction::Log(_) => ConsoleFunction::Log(formatted),
                    _ => return Err(AstError::impossible_console_assert_call(args.span).into()),
                }
            }
        };

        self.reducer.reduce_console(console_function_call, function)
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

        let mut functions = IndexMap::new();
        for (name, function) in program.functions.iter() {
            functions.insert(name.clone(), self.reduce_function(function)?);
        }

        self.reducer.reduce_program(program, inputs, functions)
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
        };

        self.reducer.reduce_function_input(input, new)
    }

    pub fn reduce_function(&mut self, function: &Function) -> Result<Function> {
        let identifier = self.reduce_identifier(&function.identifier)?;

        let mut inputs = vec![];
        for input in function.input.iter() {
            inputs.push(self.reduce_function_input(input)?);
        }

        let output = self.reduce_type(&function.output, &function.span)?;

        let block = self.reduce_block(&function.block)?;

        self.reducer
            .reduce_function(function, identifier, inputs, output, block)
    }
}

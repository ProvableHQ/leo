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

//! Compiles a Leo program from a file path.

use crate::CompilerOptions;
use indexmap::IndexMap;
use leo_asg::{
    ArrayAccessExpression as AsgArrayAccessExpression,
    ArrayInitExpression as AsgArrayInitExpression,
    ArrayInlineExpression as AsgArrayInlineExpression,
    ArrayRangeAccessExpression as AsgArrayRangeAccessExpression,
    AssignAccess as AsgAssignAccess,
    AssignStatement as AsgAssignStatement,
    BinaryExpression as AsgBinaryExpression,
    BlockStatement as AsgBlockStatement,
    CallExpression as AsgCallExpression,
    CastExpression as AsgCastExpression,
    CircuitAccessExpression as AsgCircuitAccessExpression,
    CircuitInitExpression as AsgCircuitInitExpression,
    ConditionalStatement as AsgConditionalStatement,
    ConsoleFunction as AsgConsoleFunction,
    ConsoleStatement as AsgConsoleStatement,
    Constant as AsgConstant,
    DefinitionStatement as AsgDefinitionStatement,
    Expression as AsgExpression,
    ExpressionStatement as AsgExpressionStatement,
    IterationStatement as AsgIterationStatement,
    ReturnStatement as AsgReturnStatement,
    Statement as AsgStatement,
    TernaryExpression as AsgTernaryExpression,
    TupleAccessExpression as AsgTupleAccessExpression,
    TupleInitExpression as AsgTupleInitExpression,
    Type as AsgType,
    UnaryExpression as AsgUnaryExpression,
    VariableRef as AsgVariableRef,
};
use leo_ast::{
    ArrayAccessExpression as AstArrayAccessExpression,
    ArrayDimensions,
    ArrayInitExpression as AstArrayInitExpression,
    ArrayInlineExpression as AstArrayInlineExpression,
    ArrayRangeAccessExpression as AstArrayRangeAccessExpression,
    AssignStatement as AstAssignStatement,
    Assignee,
    AssigneeAccess as AstAssignAccess,
    BinaryExpression as AstBinaryExpression,
    Block as AstBlockStatement,
    CallExpression as AstCallExpression,
    CanonicalizeError,
    CastExpression as AstCastExpression,
    CircuitImpliedVariableDefinition,
    CircuitInitExpression as AstCircuitInitExpression,
    CircuitMemberAccessExpression,
    CircuitStaticFunctionAccessExpression,
    ConditionalStatement as AstConditionalStatement,
    ConsoleFunction as AstConsoleFunction,
    ConsoleStatement as AstConsoleStatement,
    DefinitionStatement as AstDefinitionStatement,
    Expression as AstExpression,
    ExpressionStatement as AstExpressionStatement,
    FormattedString,
    IterationStatement as AstIterationStatement,
    PositiveNumber,
    ReconstructingReducer,
    ReturnStatement as AstReturnStatement,
    Span,
    SpreadOrExpression,
    Statement as AstStatement,
    TernaryExpression as AstTernaryExpression,
    TupleAccessExpression as AstTupleAccessExpression,
    TupleInitExpression as AstTupleInitExpression,
    Type as AstType,
    UnaryExpression as AstUnaryExpression,
    ValueExpression,
};
use tendril::StrTendril;

pub struct CombineAstAsgDirector<R: ReconstructingReducer> {
    ast_reducer: R,
    options: CompilerOptions,
    in_circuit: bool,
}

impl<R: ReconstructingReducer> CombineAstAsgDirector<R> {
    pub fn new(ast_reducer: R, options: CompilerOptions) -> Self {
        Self {
            ast_reducer,
            options,
            in_circuit: false,
        }
    }

    pub fn reduce_type(&mut self, ast: &AstType, asg: &AsgType, span: &Span) -> Result<AstType, CanonicalizeError> {
        let new = match (ast, asg) {
            (AstType::Array(ast_type, ast_dimensions), AsgType::Array(asg_type, asg_dimensions)) => {
                if self.options.type_inference_enabled {
                    AstType::Array(
                        Box::new(self.reduce_type(ast_type, asg_type, span)?),
                        ArrayDimensions(vec![PositiveNumber {
                            value: StrTendril::from(format!("{}", asg_dimensions)),
                        }]),
                    )
                } else {
                    AstType::Array(
                        Box::new(self.reduce_type(ast_type, asg_type, span)?),
                        ast_dimensions.clone(),
                    )
                }
            }
            (AstType::Tuple(ast_types), AsgType::Tuple(asg_types)) => {
                let mut reduced_types = vec![];
                for (ast_type, asg_type) in ast_types.iter().zip(asg_types.iter()) {
                    reduced_types.push(self.reduce_type(ast_type, asg_type, span)?);
                }

                AstType::Tuple(reduced_types)
            }
            _ if !self.options.type_inference_enabled => ast.clone(),
            _ => asg.into(),
        };

        self.ast_reducer.reduce_type(ast, new, self.in_circuit, span)
    }

    pub fn reduce_expression(
        &mut self,
        ast: &AstExpression,
        asg: &AsgExpression,
    ) -> Result<AstExpression, CanonicalizeError> {
        let new = match (ast, asg) {
            // TODO what to do for the following:
            // Ast::Identifier, Ast::Value, Asg::Constant, Asg::ValueRef

            // AsgExpression::Identifier(identifier) => AsgExpression::Identifier(self.reduce_identifier(&identifier)?),
            // AsgExpression::Value(value) => AsgExpression::Value(self.reduce_value(&value)?),
            (AstExpression::Binary(ast), AsgExpression::Binary(asg)) => {
                AstExpression::Binary(self.reduce_binary(&ast, &asg)?)
            }
            (AstExpression::Unary(ast), AsgExpression::Unary(asg)) => {
                AstExpression::Unary(self.reduce_unary(&ast, &asg)?)
            }
            (AstExpression::Ternary(ast), AsgExpression::Ternary(asg)) => {
                AstExpression::Ternary(self.reduce_ternary(&ast, &asg)?)
            }
            (AstExpression::Cast(ast), AsgExpression::Cast(asg)) => AstExpression::Cast(self.reduce_cast(&ast, &asg)?),

            (AstExpression::ArrayInline(ast), AsgExpression::ArrayInline(asg)) => {
                AstExpression::ArrayInline(self.reduce_array_inline(&ast, &asg)?)
            }
            (AstExpression::ArrayInit(ast), AsgExpression::ArrayInit(asg)) => {
                AstExpression::ArrayInit(self.reduce_array_init(&ast, &asg)?)
            }
            (AstExpression::ArrayAccess(ast), AsgExpression::ArrayAccess(asg)) => {
                AstExpression::ArrayAccess(self.reduce_array_access(&ast, &asg)?)
            }
            (AstExpression::ArrayRangeAccess(ast), AsgExpression::ArrayRangeAccess(asg)) => {
                AstExpression::ArrayRangeAccess(self.reduce_array_range_access(&ast, &asg)?)
            }

            (AstExpression::TupleInit(ast), AsgExpression::TupleInit(asg)) => {
                AstExpression::TupleInit(self.reduce_tuple_init(&ast, &asg)?)
            }
            (AstExpression::TupleAccess(ast), AsgExpression::TupleAccess(asg)) => {
                AstExpression::TupleAccess(self.reduce_tuple_access(&ast, &asg)?)
            }

            (AstExpression::CircuitInit(ast), AsgExpression::CircuitInit(asg)) => {
                AstExpression::CircuitInit(self.reduce_circuit_init(&ast, &asg)?)
            }
            (AstExpression::CircuitMemberAccess(ast), AsgExpression::CircuitAccess(asg)) => {
                AstExpression::CircuitMemberAccess(self.reduce_circuit_member_access(&ast, &asg)?)
            }
            (AstExpression::CircuitStaticFunctionAccess(ast), AsgExpression::CircuitAccess(asg)) => {
                AstExpression::CircuitStaticFunctionAccess(self.reduce_circuit_static_fn_access(&ast, &asg)?)
            }

            (AstExpression::Call(ast), AsgExpression::Call(asg)) => AstExpression::Call(self.reduce_call(&ast, &asg)?),
            _ => ast.clone(),
        };

        self.ast_reducer.reduce_expression(ast, new, self.in_circuit)
    }

    pub fn reduce_array_access(
        &mut self,
        ast: &AstArrayAccessExpression,
        asg: &AsgArrayAccessExpression,
    ) -> Result<AstArrayAccessExpression, CanonicalizeError> {
        let array = self.reduce_expression(&ast.array, asg.array.get())?;
        let index = self.reduce_expression(&ast.index, asg.index.get())?;

        self.ast_reducer.reduce_array_access(ast, array, index, self.in_circuit)
    }

    pub fn reduce_array_init(
        &mut self,
        ast: &AstArrayInitExpression,
        asg: &AsgArrayInitExpression,
    ) -> Result<AstArrayInitExpression, CanonicalizeError> {
        let element = self.reduce_expression(&ast.element, asg.element.get())?;

        self.ast_reducer.reduce_array_init(ast, element, self.in_circuit)
    }

    pub fn reduce_array_inline(
        &mut self,
        ast: &AstArrayInlineExpression,
        asg: &AsgArrayInlineExpression,
    ) -> Result<AstArrayInlineExpression, CanonicalizeError> {
        let mut elements = vec![];
        for (ast_element, asg_element) in ast.elements.iter().zip(asg.elements.iter()) {
            let reduced_element = match ast_element {
                SpreadOrExpression::Expression(ast_expression) => {
                    SpreadOrExpression::Expression(self.reduce_expression(ast_expression, asg_element.0.get())?)
                }
                SpreadOrExpression::Spread(ast_expression) => {
                    SpreadOrExpression::Spread(self.reduce_expression(ast_expression, asg_element.0.get())?)
                }
            };

            elements.push(reduced_element);
        }

        self.ast_reducer.reduce_array_inline(ast, elements, self.in_circuit)
    }

    pub fn reduce_array_range_access(
        &mut self,
        ast: &AstArrayRangeAccessExpression,
        asg: &AsgArrayRangeAccessExpression,
    ) -> Result<AstArrayRangeAccessExpression, CanonicalizeError> {
        let array = self.reduce_expression(&ast.array, asg.array.get())?;
        let left = match (ast.left.as_ref(), asg.left.get()) {
            (Some(ast_left), Some(asg_left)) => Some(self.reduce_expression(ast_left, asg_left)?),
            _ => None,
        };
        let right = match (ast.right.as_ref(), asg.right.get()) {
            (Some(ast_right), Some(asg_right)) => Some(self.reduce_expression(ast_right, asg_right)?),
            _ => None,
        };

        self.ast_reducer
            .reduce_array_range_access(ast, array, left, right, self.in_circuit)
    }

    pub fn reduce_binary(
        &mut self,
        ast: &AstBinaryExpression,
        asg: &AsgBinaryExpression,
    ) -> Result<AstBinaryExpression, CanonicalizeError> {
        let left = self.reduce_expression(&ast.left, asg.left.get())?;
        let right = self.reduce_expression(&ast.right, asg.right.get())?;

        self.ast_reducer
            .reduce_binary(ast, left, right, ast.op.clone(), self.in_circuit)
    }

    pub fn reduce_call(
        &mut self,
        ast: &AstCallExpression,
        asg: &AsgCallExpression,
    ) -> Result<AstCallExpression, CanonicalizeError> {
        // TODO FIGURE IT OUT
        // let function = self.reduce_expression(&ast.function, asg.function.get())?;
        // let target = asg.target.get().map(|exp| self.reduce_expression())
        // Is this needed?

        let mut arguments = vec![];
        for (ast_arg, asg_arg) in ast.arguments.iter().zip(asg.arguments.iter()) {
            arguments.push(self.reduce_expression(ast_arg, asg_arg.get())?);
        }

        self.ast_reducer
            .reduce_call(ast, *ast.function.clone(), arguments, self.in_circuit)
    }

    pub fn reduce_cast(
        &mut self,
        ast: &AstCastExpression,
        asg: &AsgCastExpression,
    ) -> Result<AstCastExpression, CanonicalizeError> {
        let inner = self.reduce_expression(&ast.inner, &asg.inner.get())?;
        let target_type = self.reduce_type(&ast.target_type, &asg.target_type, &ast.span)?;

        self.ast_reducer.reduce_cast(ast, inner, target_type, self.in_circuit)
    }

    pub fn reduce_constant(
        &mut self,
        ast: &ValueExpression,
        _asg: &AsgConstant,
    ) -> Result<ValueExpression, CanonicalizeError> {
        // TODO REDUCE GV
        // Is this needed?
        let new = match ast {
            // AstConstant::Group(group_value) => {
            //     AstConstant::Group(Box::new(self.reduce_group_value(&group_value)?))
            // }
            _ => ast.clone(),
        };

        self.ast_reducer.reduce_value(ast, new)
    }

    pub fn reduce_circuit_member_access(
        &mut self,
        ast: &CircuitMemberAccessExpression,
        _asg: &AsgCircuitAccessExpression,
    ) -> Result<CircuitMemberAccessExpression, CanonicalizeError> {
        // TODO FIGURE IT OUT
        // let circuit = self.reduce_expression(&circuit_member_access.circuit)?;
        // let name = self.reduce_identifier(&circuit_member_access.name)?;
        // let target = input.target.get().map(|e| self.reduce_expression(e));
        // does it matter?

        self.ast_reducer
            .reduce_circuit_member_access(ast, *ast.circuit.clone(), ast.name.clone(), self.in_circuit)
    }

    pub fn reduce_circuit_static_fn_access(
        &mut self,
        ast: &CircuitStaticFunctionAccessExpression,
        _asg: &AsgCircuitAccessExpression,
    ) -> Result<CircuitStaticFunctionAccessExpression, CanonicalizeError> {
        // TODO FIGURE IT OUT
        // let circuit = self.reduce_expression(&circuit_member_access.circuit)?;
        // let name = self.reduce_identifier(&circuit_member_access.name)?;
        // let target = input.target.get().map(|e| self.reduce_expression(e));
        // does it matter?

        self.ast_reducer
            .reduce_circuit_static_fn_access(ast, *ast.circuit.clone(), ast.name.clone(), self.in_circuit)
    }

    pub fn reduce_circuit_implied_variable_definition(
        &mut self,
        ast: &CircuitImpliedVariableDefinition,
        asg: &AsgExpression,
    ) -> Result<CircuitImpliedVariableDefinition, CanonicalizeError> {
        let expression = ast
            .expression
            .as_ref()
            .map(|ast_expr| self.reduce_expression(ast_expr, asg))
            .transpose()?;

        self.ast_reducer.reduce_circuit_implied_variable_definition(
            ast,
            ast.identifier.clone(),
            expression,
            self.in_circuit,
        )
    }

    pub fn reduce_circuit_init(
        &mut self,
        ast: &AstCircuitInitExpression,
        asg: &AsgCircuitInitExpression,
    ) -> Result<AstCircuitInitExpression, CanonicalizeError> {
        let mut members = vec![];
        for (ast_member, asg_member) in ast.members.iter().zip(asg.values.iter()) {
            members.push(self.reduce_circuit_implied_variable_definition(ast_member, asg_member.1.get())?);
        }

        self.ast_reducer
            .reduce_circuit_init(ast, ast.name.clone(), members, self.in_circuit)
    }

    pub fn reduce_ternary(
        &mut self,
        ast: &AstTernaryExpression,
        asg: &AsgTernaryExpression,
    ) -> Result<AstTernaryExpression, CanonicalizeError> {
        let condition = self.reduce_expression(&ast.condition, asg.condition.get())?;
        let if_true = self.reduce_expression(&ast.if_true, asg.if_true.get())?;
        let if_false = self.reduce_expression(&ast.if_false, asg.if_false.get())?;

        self.ast_reducer
            .reduce_ternary(ast, condition, if_true, if_false, self.in_circuit)
    }

    pub fn reduce_tuple_access(
        &mut self,
        ast: &AstTupleAccessExpression,
        asg: &AsgTupleAccessExpression,
    ) -> Result<AstTupleAccessExpression, CanonicalizeError> {
        let tuple = self.reduce_expression(&ast.tuple, asg.tuple_ref.get())?;

        self.ast_reducer.reduce_tuple_access(ast, tuple, self.in_circuit)
    }

    pub fn reduce_tuple_init(
        &mut self,
        ast: &AstTupleInitExpression,
        asg: &AsgTupleInitExpression,
    ) -> Result<AstTupleInitExpression, CanonicalizeError> {
        let mut elements = vec![];
        for (ast_element, asg_element) in ast.elements.iter().zip(asg.elements.iter()) {
            let element = self.reduce_expression(ast_element, asg_element.get())?;
            elements.push(element);
        }

        self.ast_reducer.reduce_tuple_init(ast, elements, self.in_circuit)
    }

    pub fn reduce_unary(
        &mut self,
        ast: &AstUnaryExpression,
        asg: &AsgUnaryExpression,
    ) -> Result<AstUnaryExpression, CanonicalizeError> {
        let inner = self.reduce_expression(&ast.inner, asg.inner.get())?;

        self.ast_reducer
            .reduce_unary(ast, inner, ast.op.clone(), self.in_circuit)
    }

    pub fn reduce_variable_ref(
        &mut self,
        ast: &ValueExpression,
        _asg: &AsgVariableRef,
    ) -> Result<ValueExpression, CanonicalizeError> {
        // TODO FIGURE IT OUT
        let new = match ast {
            // ValueExpression::Group(group_value) => {
            //     ValueExpression::Group(Box::new(self.reduce_group_value(&group_value)?))
            // }
            _ => ast.clone(),
        };

        Ok(new)
        // self.ast_reducer.reduce_value(value, new)
    }

    pub fn reduce_statement(
        &mut self,
        ast_statement: &AstStatement,
        asg_statement: &AsgStatement,
    ) -> Result<AstStatement, CanonicalizeError> {
        let new = match (ast_statement, asg_statement) {
            (AstStatement::Assign(ast), AsgStatement::Assign(asg)) => {
                AstStatement::Assign(self.reduce_assign(ast, asg)?)
            }
            (AstStatement::Block(ast), AsgStatement::Block(asg)) => AstStatement::Block(self.reduce_block(ast, asg)?),
            (AstStatement::Conditional(ast), AsgStatement::Conditional(asg)) => {
                AstStatement::Conditional(self.reduce_conditional(ast, asg)?)
            }
            (AstStatement::Console(ast), AsgStatement::Console(asg)) => {
                AstStatement::Console(self.reduce_console(ast, asg)?)
            }
            (AstStatement::Definition(ast), AsgStatement::Definition(asg)) => {
                AstStatement::Definition(self.reduce_definition(ast, asg)?)
            }
            (AstStatement::Expression(ast), AsgStatement::Expression(asg)) => {
                AstStatement::Expression(self.reduce_expression_statement(ast, asg)?)
            }
            // (AstStatement::Iteration(ast), AsgStatement::Iteration(asg)) => AstStatement::Iteration(self.reduce_iteration(ast, asg)?),
            // (AstStatement::Return(ast), AsgStatement::Return(asg)) => AstStatement::Return(self.reduce_return(ast, asg)?),
            _ => ast_statement.clone(),
        };

        self.ast_reducer.reduce_statement(ast_statement, new, self.in_circuit)
    }

    pub fn reduce_assign_access(
        &mut self,
        ast: &AstAssignAccess,
        asg: &AsgAssignAccess,
    ) -> Result<AstAssignAccess, CanonicalizeError> {
        let new = match (ast, asg) {
            (AstAssignAccess::ArrayRange(ast_left, ast_right), AsgAssignAccess::ArrayRange(asg_left, asg_right)) => {
                let left = match (ast_left.as_ref(), asg_left.get()) {
                    (Some(ast_left), Some(asg_left)) => Some(self.reduce_expression(ast_left, asg_left)?),
                    _ => None,
                };
                let right = match (ast_right.as_ref(), asg_right.get()) {
                    (Some(ast_right), Some(asg_right)) => Some(self.reduce_expression(ast_right, asg_right)?),
                    _ => None,
                };

                AstAssignAccess::ArrayRange(left, right)
            }
            (AstAssignAccess::ArrayIndex(ast_index), AsgAssignAccess::ArrayIndex(asg_index)) => {
                let index = self.reduce_expression(&ast_index, asg_index.get())?;
                AstAssignAccess::ArrayIndex(index)
            }
            _ => ast.clone(),
        };

        self.ast_reducer.reduce_assignee_access(ast, new, self.in_circuit)
    }

    pub fn reduce_assignee(
        &mut self,
        ast: &Assignee,
        asg: &Vec<AsgAssignAccess>,
    ) -> Result<Assignee, CanonicalizeError> {
        let mut accesses = vec![];
        for (ast_access, asg_access) in ast.accesses.iter().zip(asg.iter()) {
            accesses.push(self.reduce_assign_access(ast_access, asg_access)?);
        }

        self.ast_reducer
            .reduce_assignee(ast, ast.identifier.clone(), accesses, self.in_circuit)
    }

    pub fn reduce_assign(
        &mut self,
        ast: &AstAssignStatement,
        asg: &AsgAssignStatement,
    ) -> Result<AstAssignStatement, CanonicalizeError> {
        let assignee = self.reduce_assignee(&ast.assignee, &asg.target_accesses)?;
        let value = self.reduce_expression(&ast.value, asg.value.get())?;

        self.ast_reducer.reduce_assign(ast, assignee, value, self.in_circuit)
    }

    pub fn reduce_block(
        &mut self,
        ast: &AstBlockStatement,
        asg: &AsgBlockStatement,
    ) -> Result<AstBlockStatement, CanonicalizeError> {
        let mut statements = vec![];
        for (ast_statement, asg_statement) in ast.statements.iter().zip(asg.statements.iter()) {
            statements.push(self.reduce_statement(ast_statement, asg_statement.get())?);
        }

        self.ast_reducer.reduce_block(ast, statements, self.in_circuit)
    }

    pub fn reduce_conditional(
        &mut self,
        ast: &AstConditionalStatement,
        asg: &AsgConditionalStatement,
    ) -> Result<AstConditionalStatement, CanonicalizeError> {
        let condition = self.reduce_expression(&ast.condition, asg.condition.get())?;
        let block;
        if let AsgStatement::Block(asg_block) = asg.result.get() {
            block = self.reduce_block(&ast.block, asg_block)?;
        } else {
            // TODO throw error?
            block = ast.block.clone();
        }
        let next = match (ast.next.as_ref(), asg.next.get()) {
            (Some(ast_next), Some(asg_next)) => Some(self.reduce_statement(ast_next, asg_next)?),
            _ => None,
        };

        self.ast_reducer
            .reduce_conditional(ast, condition, block, next, self.in_circuit)
    }

    pub fn reduce_console(
        &mut self,
        ast: &AstConsoleStatement,
        asg: &AsgConsoleStatement,
    ) -> Result<AstConsoleStatement, CanonicalizeError> {
        let function = match (&ast.function, &asg.function) {
            (AstConsoleFunction::Assert(ast_expression), AsgConsoleFunction::Assert(asg_expression)) => {
                AstConsoleFunction::Assert(self.reduce_expression(&ast_expression, asg_expression.get())?)
            }
            (AstConsoleFunction::Debug(ast_format), AsgConsoleFunction::Debug(asg_format))
            | (AstConsoleFunction::Error(ast_format), AsgConsoleFunction::Error(asg_format))
            | (AstConsoleFunction::Log(ast_format), AsgConsoleFunction::Log(asg_format)) => {
                let mut parameters = vec![];
                for (ast_parameter, asg_parameter) in ast_format.parameters.iter().zip(asg_format.parameters.iter()) {
                    parameters.push(self.reduce_expression(&ast_parameter, asg_parameter.get())?);
                }

                let formatted = FormattedString {
                    parts: ast_format.parts.clone(),
                    parameters,
                    span: ast_format.span.clone(),
                };

                match &ast.function {
                    AstConsoleFunction::Debug(_) => AstConsoleFunction::Debug(formatted),
                    AstConsoleFunction::Error(_) => AstConsoleFunction::Error(formatted),
                    AstConsoleFunction::Log(_) => AstConsoleFunction::Log(formatted),
                    _ => unimplemented!(), // impossible
                }
            }
            _ => ast.function.clone(),
        };

        self.ast_reducer.reduce_console(ast, function, self.in_circuit)
    }

    pub fn reduce_definition(
        &mut self,
        ast: &AstDefinitionStatement,
        asg: &AsgDefinitionStatement,
    ) -> Result<AstDefinitionStatement, CanonicalizeError> {
        let type_;

        if asg.variables.len() > 1 {
            let mut types = vec![];
            for variable in asg.variables.iter() {
                types.push(variable.borrow().type_.clone());
            }

            type_ = ast
                .type_
                .as_ref()
                .map(|type_| self.reduce_type(type_, &AsgType::Tuple(types), &ast.span))
                .transpose()?;
        } else {
            type_ = ast
                .type_
                .as_ref()
                .map(|type_| self.reduce_type(type_, &asg.variables.first().unwrap().borrow().type_, &ast.span))
                .transpose()?;
        }

        let value = self.reduce_expression(&ast.value, asg.value.get())?;

        self.ast_reducer
            .reduce_definition(ast, ast.variable_names.clone(), type_, value, self.in_circuit)
    }

    pub fn reduce_expression_statement(
        &mut self,
        ast: &AstExpressionStatement,
        asg: &AsgExpressionStatement,
    ) -> Result<AstExpressionStatement, CanonicalizeError> {
        let inner_expression = self.reduce_expression(&ast.expression, asg.expression.get())?;
        self.ast_reducer
            .reduce_expression_statement(ast, inner_expression, self.in_circuit)
    }

    pub fn reduce_iteration(
        &mut self,
        ast: &AstIterationStatement,
        asg: &AsgIterationStatement,
    ) -> Result<AstIterationStatement, CanonicalizeError> {
        let start = self.reduce_expression(&ast.start, asg.start.get())?;
        let stop = self.reduce_expression(&ast.stop, asg.stop.get())?;
        let block;
        if let AsgStatement::Block(asg_block) = asg.body.get() {
            block = self.reduce_block(&ast.block, asg_block)?;
        } else {
            block = ast.block.clone();
        }

        self.ast_reducer
            .reduce_iteration(ast, ast.variable.clone(), start, stop, block, self.in_circuit)
    }

    pub fn reduce_return(
        &mut self,
        ast: &AstReturnStatement,
        asg: &AsgReturnStatement,
    ) -> Result<AstReturnStatement, CanonicalizeError> {
        let expression = self.reduce_expression(&ast.expression, asg.expression.get())?;

        self.ast_reducer.reduce_return(ast, expression, self.in_circuit)
    }

    pub fn reduce_program(
        &mut self,
        ast: &leo_ast::Program,
        asg: &leo_asg::Program,
    ) -> Result<leo_ast::Program, leo_ast::CanonicalizeError> {
        let mut circuits = IndexMap::new();
        for ((asg_ident, asg_circuit), (ast_ident, ast_circuit)) in asg.circuits.iter().zip(&ast.circuits) {
            circuits.insert(
                // self.reduce_identifier(asg_ident, ast_ident),
                ast_ident.clone(),
                // self.reduce_circuit(asg_circuit, ast_circuit)
                ast_circuit.clone(),
            );
        }

        let mut functions = IndexMap::new();
        for ((asg_ident, asg_function), (ast_identifier, ast_function)) in asg.functions.iter().zip(&ast.functions) {
            // etc
            functions.insert(ast_identifier.clone(), ast_function.clone());
        }

        self.ast_reducer.reduce_program(
            ast,
            ast.expected_input.clone(),
            ast.imports.clone(),
            circuits,
            functions,
        )
    }
}

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

use crate::*;

/// Replace Self when it is in a enclosing circuit type.
/// Error when Self is outside an enclosing circuit type.
/// Tuple array types and expressions expand to nested arrays.
/// Tuple array types and expressions error if a size of 0 is given.anyhow
/// Compound operators become simple assignments.
/// Functions missing output type return a empty tuple.
pub struct Canonicalizer {
    // If we are in a circuit keep track of the circuit name.
    circuit_name: Option<Identifier>,
    in_circuit: bool,
}

impl Default for Canonicalizer {
    fn default() -> Self {
        Self {
            circuit_name: None,
            in_circuit: false,
        }
    }
}

impl Canonicalizer {
    pub fn canonicalize_accesses(
        &mut self,
        start: Expression,
        accesses: &[AssigneeAccess],
        span: &Span,
    ) -> Result<Box<Expression>, ReducerError> {
        let mut left = Box::new(start);

        for access in accesses.iter() {
            match self.canonicalize_assignee_access(&access) {
                AssigneeAccess::ArrayIndex(index) => {
                    left = Box::new(Expression::ArrayAccess(ArrayAccessExpression {
                        array: left,
                        index: Box::new(index),
                        span: span.clone(),
                    }));
                }
                AssigneeAccess::Tuple(positive_number, _) => {
                    left = Box::new(Expression::TupleAccess(TupleAccessExpression {
                        tuple: left,
                        index: positive_number,
                        span: span.clone(),
                    }));
                }
                AssigneeAccess::Member(identifier) => {
                    left = Box::new(Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                        circuit: left,
                        name: identifier,
                        span: span.clone(),
                    }));
                }
                _ => return Err(ReducerError::from(CombinerError::illegal_compound_array_range(&span))),
            }
        }

        Ok(left)
    }

    pub fn compound_operation_converstion(
        &mut self,
        operation: &AssignOperation,
    ) -> Result<BinaryOperation, ReducerError> {
        match operation {
            AssignOperation::Assign => unreachable!(),
            AssignOperation::Add => Ok(BinaryOperation::Add),
            AssignOperation::Sub => Ok(BinaryOperation::Sub),
            AssignOperation::Mul => Ok(BinaryOperation::Mul),
            AssignOperation::Div => Ok(BinaryOperation::Div),
            AssignOperation::Pow => Ok(BinaryOperation::Pow),
            AssignOperation::Or => Ok(BinaryOperation::Or),
            AssignOperation::And => Ok(BinaryOperation::And),
            AssignOperation::BitOr => Ok(BinaryOperation::BitOr),
            AssignOperation::BitAnd => Ok(BinaryOperation::BitAnd),
            AssignOperation::BitXor => Ok(BinaryOperation::BitXor),
            AssignOperation::Shr => Ok(BinaryOperation::Shr),
            AssignOperation::ShrSigned => Ok(BinaryOperation::ShrSigned),
            AssignOperation::Shl => Ok(BinaryOperation::Shl),
            AssignOperation::Mod => Ok(BinaryOperation::Mod),
        }
    }

    fn is_self_type(&mut self, type_option: Option<&Type>) -> bool {
        matches!(type_option, Some(Type::SelfType))
    }

    fn canonicalize_expression(&mut self, expression: &Expression) -> Expression {
        match expression {
            Expression::Unary(unary) => {
                let inner = Box::new(self.canonicalize_expression(&unary.inner));

                return Expression::Unary(UnaryExpression {
                    inner,
                    op: unary.op.clone(),
                    span: unary.span.clone(),
                });
            }
            Expression::Binary(binary) => {
                let left = Box::new(self.canonicalize_expression(&binary.left));
                let right = Box::new(self.canonicalize_expression(&binary.right));

                return Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op: binary.op.clone(),
                    span: binary.span.clone(),
                });
            }
            Expression::Ternary(ternary) => {
                let condition = Box::new(self.canonicalize_expression(&ternary.condition));
                let if_true = Box::new(self.canonicalize_expression(&ternary.if_true));
                let if_false = Box::new(self.canonicalize_expression(&ternary.if_false));

                return Expression::Ternary(TernaryExpression {
                    condition,
                    if_true,
                    if_false,
                    span: ternary.span.clone(),
                });
            }

            Expression::Cast(cast) => {
                let inner = Box::new(self.canonicalize_expression(&cast.inner));
                let mut target_type = cast.target_type.clone();

                if matches!(target_type, Type::SelfType) {
                    target_type = Type::Circuit(self.circuit_name.as_ref().unwrap().clone());
                }

                return Expression::Cast(CastExpression {
                    inner,
                    target_type,
                    span: cast.span.clone(),
                });
            }

            Expression::ArrayInline(array_inline) => {
                let elements = array_inline
                    .elements
                    .iter()
                    .map(|element| match element {
                        SpreadOrExpression::Expression(expression) => {
                            SpreadOrExpression::Expression(self.canonicalize_expression(expression))
                        }
                        SpreadOrExpression::Spread(expression) => {
                            SpreadOrExpression::Spread(self.canonicalize_expression(expression))
                        }
                    })
                    .collect();

                return Expression::ArrayInline(ArrayInlineExpression {
                    elements,
                    span: array_inline.span.clone(),
                });
            }

            Expression::ArrayInit(array_init) => {
                let element = Box::new(self.canonicalize_expression(&array_init.element));

                return Expression::ArrayInit(ArrayInitExpression {
                    dimensions: array_init.dimensions.clone(),
                    element,
                    span: array_init.span.clone(),
                });
            }

            Expression::ArrayAccess(array_access) => {
                let array = Box::new(self.canonicalize_expression(&array_access.array));
                let index = Box::new(self.canonicalize_expression(&array_access.index));

                return Expression::ArrayAccess(ArrayAccessExpression {
                    array,
                    index,
                    span: array_access.span.clone(),
                });
            }

            Expression::ArrayRangeAccess(array_range_access) => {
                let array = Box::new(self.canonicalize_expression(&array_range_access.array));
                let left = array_range_access
                    .left
                    .as_ref()
                    .map(|left| Box::new(self.canonicalize_expression(left)));
                let right = array_range_access
                    .right
                    .as_ref()
                    .map(|right| Box::new(self.canonicalize_expression(right)));

                return Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                    array,
                    left,
                    right,
                    span: array_range_access.span.clone(),
                });
            }

            Expression::TupleInit(tuple_init) => {
                let elements = tuple_init
                    .elements
                    .iter()
                    .map(|element| self.canonicalize_expression(element))
                    .collect();

                return Expression::TupleInit(TupleInitExpression {
                    elements,
                    span: tuple_init.span.clone(),
                });
            }

            Expression::TupleAccess(tuple_access) => {
                let tuple = Box::new(self.canonicalize_expression(&tuple_access.tuple));

                return Expression::TupleAccess(TupleAccessExpression {
                    tuple,
                    index: tuple_access.index.clone(),
                    span: tuple_access.span.clone(),
                });
            }

            Expression::CircuitInit(circuit_init) => {
                let mut name = circuit_init.name.clone();
                if name.name.as_ref() == "Self" {
                    name = self.circuit_name.as_ref().unwrap().clone();
                }

                return Expression::CircuitInit(CircuitInitExpression {
                    name,
                    members: circuit_init.members.clone(),
                    span: circuit_init.span.clone(),
                });
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                return Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_member_access.circuit)),
                    name: circuit_member_access.name.clone(),
                    span: circuit_member_access.span.clone(),
                });
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_func_access) => {
                return Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_static_func_access.circuit)),
                    name: circuit_static_func_access.name.clone(),
                    span: circuit_static_func_access.span.clone(),
                });
            }
            Expression::Call(call) => {
                return Expression::Call(CallExpression {
                    function: Box::new(self.canonicalize_expression(&call.function)),
                    arguments: call.arguments.clone(),
                    span: call.span.clone(),
                });
            }
            _ => {}
        }

        expression.clone()
    }

    fn canonicalize_assignee_access(&mut self, access: &AssigneeAccess) -> AssigneeAccess {
        match access {
            AssigneeAccess::ArrayRange(left, right) => {
                let left = left.as_ref().map(|left| self.canonicalize_expression(left));
                let right = right.as_ref().map(|right| self.canonicalize_expression(right));

                AssigneeAccess::ArrayRange(left, right)
            }
            AssigneeAccess::ArrayIndex(index) => AssigneeAccess::ArrayIndex(self.canonicalize_expression(&index)),
            _ => access.clone(),
        }
    }

    fn canonicalize_assignee(&mut self, assignee: &Assignee) -> Assignee {
        let accesses = assignee
            .accesses
            .iter()
            .map(|access| self.canonicalize_assignee_access(access))
            .collect();

        Assignee {
            identifier: assignee.identifier.clone(),
            accesses,
            span: assignee.span.clone(),
        }
    }

    fn canonicalize_block(&mut self, block: &Block) -> Block {
        let statements = block
            .statements
            .iter()
            .map(|block_statement| self.canonicalize_statement(&block_statement))
            .collect();

        Block {
            statements,
            span: block.span.clone(),
        }
    }

    fn canonicalize_statement(&mut self, statement: &Statement) -> Statement {
        match statement {
            Statement::Return(return_statement) => {
                let expression = self.canonicalize_expression(&return_statement.expression);
                Statement::Return(ReturnStatement {
                    expression,
                    span: return_statement.span.clone(),
                })
            }
            Statement::Definition(definition) => {
                let value = self.canonicalize_expression(&definition.value);
                let mut type_ = definition.type_.clone();

                if self.is_self_type(type_.as_ref()) {
                    type_ = Some(Type::Circuit(self.circuit_name.as_ref().unwrap().clone()));
                }

                Statement::Definition(DefinitionStatement {
                    declaration_type: definition.declaration_type.clone(),
                    variable_names: definition.variable_names.clone(),
                    type_,
                    value,
                    span: definition.span.clone(),
                })
            }
            Statement::Assign(assign) => {
                let assignee = self.canonicalize_assignee(&assign.assignee);
                let value = self.canonicalize_expression(&assign.value);

                Statement::Assign(AssignStatement {
                    assignee,
                    value,
                    operation: assign.operation,
                    span: assign.span.clone(),
                })
            }
            Statement::Conditional(conditional) => {
                let condition = self.canonicalize_expression(&conditional.condition);
                let block = self.canonicalize_block(&conditional.block);
                let next = conditional
                    .next
                    .as_ref()
                    .map(|condition| Box::new(self.canonicalize_statement(condition)));

                Statement::Conditional(ConditionalStatement {
                    condition,
                    block,
                    next,
                    span: conditional.span.clone(),
                })
            }
            Statement::Iteration(iteration) => {
                let start = self.canonicalize_expression(&iteration.start);
                let stop = self.canonicalize_expression(&iteration.stop);
                let block = self.canonicalize_block(&iteration.block);

                Statement::Iteration(IterationStatement {
                    variable: iteration.variable.clone(),
                    start,
                    stop,
                    block,
                    span: iteration.span.clone(),
                })
            }
            Statement::Console(console_function_call) => {
                let function = match &console_function_call.function {
                    ConsoleFunction::Assert(expression) => {
                        ConsoleFunction::Assert(self.canonicalize_expression(expression))
                    }
                    ConsoleFunction::Debug(format) | ConsoleFunction::Error(format) | ConsoleFunction::Log(format) => {
                        let parameters = format
                            .parameters
                            .iter()
                            .map(|parameter| self.canonicalize_expression(parameter))
                            .collect();

                        let formatted = FormatString {
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

                Statement::Console(ConsoleStatement {
                    function,
                    span: console_function_call.span.clone(),
                })
            }
            Statement::Expression(expression) => Statement::Expression(ExpressionStatement {
                expression: self.canonicalize_expression(&expression.expression),
                span: expression.span.clone(),
            }),
            Statement::Block(block) => Statement::Block(self.canonicalize_block(block)),
        }
    }

    fn canonicalize_circuit_member(&mut self, circuit_member: &CircuitMember) -> CircuitMember {
        match circuit_member {
            CircuitMember::CircuitVariable(_, _) => {}
            CircuitMember::CircuitFunction(function) => {
                let input = function.input.clone();
                let mut output = function.output.clone();
                let block = self.canonicalize_block(&function.block);

                if self.is_self_type(output.as_ref()) {
                    output = Some(Type::Circuit(self.circuit_name.as_ref().unwrap().clone()));
                }

                return CircuitMember::CircuitFunction(Function {
                    annotations: function.annotations.clone(),
                    identifier: function.identifier.clone(),
                    input,
                    output,
                    block,
                    span: function.span.clone(),
                });
            }
        }

        circuit_member.clone()
    }
}

impl ReconstructingReducer for Canonicalizer {
    fn in_circuit(&self) -> bool {
        self.in_circuit
    }

    fn swap_in_circuit(&mut self) {
        self.in_circuit = !self.in_circuit;
    }

    fn reduce_type(&mut self, _type_: &Type, new: Type, span: &Span) -> Result<Type, ReducerError> {
        match new {
            Type::Array(type_, mut dimensions) => {
                if dimensions.is_zero() {
                    return Err(ReducerError::from(CanonicalizeError::invalid_array_dimension_size(
                        span,
                    )));
                }

                let mut next = Type::Array(type_, ArrayDimensions(vec![dimensions.remove_last().unwrap()]));
                let mut array = next.clone();

                loop {
                    if dimensions.is_empty() {
                        break;
                    }

                    array = Type::Array(Box::new(next), ArrayDimensions(vec![dimensions.remove_last().unwrap()]));
                    next = array.clone();
                }

                Ok(array)
            }
            Type::SelfType if !self.in_circuit => {
                Err(ReducerError::from(CanonicalizeError::big_self_outside_of_circuit(span)))
            }
            _ => Ok(new.clone()),
        }
    }

    fn reduce_string(&mut self, string: &str, span: &Span) -> Result<Expression, ReducerError> {
        let mut elements = Vec::new();
        for character in string.chars() {
            elements.push(SpreadOrExpression::Expression(Expression::Value(
                ValueExpression::Char(character, span.clone()),
            )));
        }

        Ok(Expression::ArrayInline(ArrayInlineExpression {
            elements,
            span: span.clone(),
        }))
    }

    fn reduce_array_init(
        &mut self,
        array_init: &ArrayInitExpression,
        element: Expression,
    ) -> Result<ArrayInitExpression, ReducerError> {
        if array_init.dimensions.is_zero() {
            return Err(ReducerError::from(CanonicalizeError::invalid_array_dimension_size(
                &array_init.span,
            )));
        }

        let element = Box::new(element);

        if array_init.dimensions.0.len() == 1 {
            return Ok(ArrayInitExpression {
                element,
                dimensions: array_init.dimensions.clone(),
                span: array_init.span.clone(),
            });
        }

        let mut dimensions = array_init.dimensions.clone();

        let mut next = Expression::ArrayInit(ArrayInitExpression {
            element,
            dimensions: ArrayDimensions(vec![dimensions.remove_last().unwrap()]),
            span: array_init.span.clone(),
        });

        let mut outer_element = Box::new(next.clone());
        for (index, dimension) in dimensions.0.iter().rev().enumerate() {
            if index == dimensions.0.len() - 1 {
                break;
            }

            next = Expression::ArrayInit(ArrayInitExpression {
                element: outer_element,
                dimensions: ArrayDimensions(vec![dimension.clone()]),
                span: array_init.span.clone(),
            });
            outer_element = Box::new(next.clone());
        }

        Ok(ArrayInitExpression {
            element: outer_element,
            dimensions: ArrayDimensions(vec![dimensions.remove_first().unwrap()]),
            span: array_init.span.clone(),
        })
    }

    fn reduce_assign(
        &mut self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
    ) -> Result<AssignStatement, ReducerError> {
        match value {
            Expression::Binary(binary_expr) if assign.operation != AssignOperation::Assign => {
                let left = self.canonicalize_accesses(
                    Expression::Identifier(assignee.identifier.clone()),
                    &assignee.accesses,
                    &assign.span,
                )?;
                let right = Box::new(Expression::Binary(binary_expr));
                let op = self.compound_operation_converstion(&assign.operation)?;

                let new_value = Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op,
                    span: assign.span.clone(),
                });

                Ok(AssignStatement {
                    operation: AssignOperation::Assign,
                    assignee,
                    value: new_value,
                    span: assign.span.clone(),
                })
            }
            Expression::Value(value_expr) if assign.operation != AssignOperation::Assign => {
                let left = self.canonicalize_accesses(
                    Expression::Identifier(assignee.identifier.clone()),
                    &assignee.accesses,
                    &assign.span,
                )?;
                let right = Box::new(Expression::Value(value_expr));
                let op = self.compound_operation_converstion(&assign.operation)?;

                let new_value = Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op,
                    span: assign.span.clone(),
                });

                Ok(AssignStatement {
                    operation: AssignOperation::Assign,
                    assignee,
                    value: new_value,
                    span: assign.span.clone(),
                })
            }
            _ => Ok(assign.clone()),
        }
    }

    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: Identifier,
        annotations: Vec<Annotation>,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
    ) -> Result<Function, ReducerError> {
        let new_output = match output {
            None => Some(Type::Tuple(vec![])),
            _ => output,
        };

        Ok(Function {
            identifier,
            annotations,
            input,
            output: new_output,
            block,
            span: function.span.clone(),
        })
    }

    fn reduce_circuit(
        &mut self,
        _circuit: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Result<Circuit, ReducerError> {
        self.circuit_name = Some(circuit_name.clone());
        let circ = Circuit {
            circuit_name,
            members: members
                .iter()
                .map(|member| self.canonicalize_circuit_member(member))
                .collect(),
        };
        self.circuit_name = None;
        Ok(circ)
    }
}

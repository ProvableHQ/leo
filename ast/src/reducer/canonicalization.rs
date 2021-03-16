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

pub struct Canonicalizer;

// TODO fix function input array.
// TODO fix test 0 size array.

impl Canonicalizer {
    fn is_self_type(&self, type_option: Option<&Type>) -> bool {
        matches!(type_option, Some(Type::SelfType))
    }

    fn canonicalize_expression(&self, expression: &Expression, circuit_name: &Identifier) -> Expression {
        match expression {
            Expression::Unary(unary) => {
                let inner = Box::new(self.canonicalize_expression(&unary.inner, circuit_name));

                return Expression::Unary(UnaryExpression {
                    inner,
                    op: unary.op.clone(),
                    span: unary.span.clone(),
                });
            }
            Expression::Binary(binary) => {
                let left = Box::new(self.canonicalize_expression(&binary.left, circuit_name));
                let right = Box::new(self.canonicalize_expression(&binary.right, circuit_name));

                return Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op: binary.op.clone(),
                    span: binary.span.clone(),
                });
            }
            Expression::Ternary(ternary) => {
                let condition = Box::new(self.canonicalize_expression(&ternary.condition, circuit_name));
                let if_true = Box::new(self.canonicalize_expression(&ternary.if_true, circuit_name));
                let if_false = Box::new(self.canonicalize_expression(&ternary.if_false, circuit_name));

                return Expression::Ternary(TernaryExpression {
                    condition,
                    if_true,
                    if_false,
                    span: ternary.span.clone(),
                });
            }

            Expression::Cast(cast) => {
                let inner = Box::new(self.canonicalize_expression(&cast.inner, circuit_name));
                let mut target_type = cast.target_type.clone();

                if matches!(target_type, Type::SelfType) {
                    target_type = Type::Circuit(circuit_name.clone());
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
                            SpreadOrExpression::Expression(self.canonicalize_expression(expression, circuit_name))
                        }
                        SpreadOrExpression::Spread(expression) => {
                            SpreadOrExpression::Spread(self.canonicalize_expression(expression, circuit_name))
                        }
                    })
                    .collect();

                return Expression::ArrayInline(ArrayInlineExpression {
                    elements,
                    span: array_inline.span.clone(),
                });
            }

            Expression::ArrayInit(array_init) => {
                let element = Box::new(self.canonicalize_expression(&array_init.element, circuit_name));

                return Expression::ArrayInit(ArrayInitExpression {
                    dimensions: array_init.dimensions.clone(),
                    element,
                    span: array_init.span.clone(),
                });
            }

            Expression::ArrayAccess(array_access) => {
                let array = Box::new(self.canonicalize_expression(&array_access.array, circuit_name));
                let index = Box::new(self.canonicalize_expression(&array_access.index, circuit_name));

                return Expression::ArrayAccess(ArrayAccessExpression {
                    array,
                    index,
                    span: array_access.span.clone(),
                });
            }

            Expression::ArrayRangeAccess(array_range_access) => {
                let array = Box::new(self.canonicalize_expression(&array_range_access.array, circuit_name));
                let left = array_range_access
                    .left
                    .as_ref()
                    .map(|left| Box::new(self.canonicalize_expression(left, circuit_name)));
                let right = array_range_access
                    .right
                    .as_ref()
                    .map(|right| Box::new(self.canonicalize_expression(right, circuit_name)));

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
                    .map(|element| self.canonicalize_expression(element, circuit_name))
                    .collect();

                return Expression::TupleInit(TupleInitExpression {
                    elements,
                    span: tuple_init.span.clone(),
                });
            }

            Expression::TupleAccess(tuple_access) => {
                let tuple = Box::new(self.canonicalize_expression(&tuple_access.tuple, circuit_name));

                return Expression::TupleAccess(TupleAccessExpression {
                    tuple,
                    index: tuple_access.index.clone(),
                    span: tuple_access.span.clone(),
                });
            }

            Expression::CircuitInit(circuit_init) => {
                return Expression::CircuitInit(CircuitInitExpression {
                    name: circuit_name.clone(),
                    members: circuit_init.members.clone(),
                    span: circuit_init.span.clone(),
                });
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                return Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_member_access.circuit, circuit_name)),
                    name: circuit_member_access.name.clone(),
                    span: circuit_member_access.span.clone(),
                });
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_func_access) => {
                return Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_static_func_access.circuit, circuit_name)),
                    name: circuit_static_func_access.name.clone(),
                    span: circuit_static_func_access.span.clone(),
                });
            }
            Expression::Call(call) => {
                return Expression::Call(CallExpression {
                    function: Box::new(self.canonicalize_expression(&call.function, circuit_name)),
                    arguments: call.arguments.clone(),
                    span: call.span.clone(),
                });
            }
            _ => {}
        }

        expression.clone()
    }

    fn canonicalize_assignee_access(&self, access: &AssigneeAccess, circuit_name: &Identifier) -> AssigneeAccess {
        match access {
            AssigneeAccess::ArrayRange(left, right) => {
                let left = match left.as_ref() {
                    Some(left) => Some(self.canonicalize_expression(left, circuit_name)),
                    None => None,
                };
                let right = match right.as_ref() {
                    Some(right) => Some(self.canonicalize_expression(right, circuit_name)),
                    None => None,
                };

                AssigneeAccess::ArrayRange(left, right)
            }
            AssigneeAccess::ArrayIndex(index) => {
                AssigneeAccess::ArrayIndex(self.canonicalize_expression(&index, circuit_name))
            }
            _ => access.clone(),
        }
    }

    fn canonicalize_assignee(&self, assignee: &Assignee, circuit_name: &Identifier) -> Assignee {
        let accesses = assignee
            .accesses
            .iter()
            .map(|access| self.canonicalize_assignee_access(access, circuit_name))
            .collect();

        Assignee {
            identifier: assignee.identifier.clone(),
            accesses,
            span: assignee.span.clone(),
        }
    }

    fn canonicalize_block(&self, block: &Block, circuit_name: &Identifier) -> Block {
        let statements = block
            .statements
            .iter()
            .map(|block_statement| self.canonicalize_statement(&block_statement, circuit_name))
            .collect();

        Block {
            statements,
            span: block.span.clone(),
        }
    }

    fn canonicalize_statement(&self, statement: &Statement, circuit_name: &Identifier) -> Statement {
        match statement {
            Statement::Return(return_statement) => {
                let expression = self.canonicalize_expression(&return_statement.expression, circuit_name);
                Statement::Return(ReturnStatement {
                    expression,
                    span: return_statement.span.clone(),
                })
            }
            Statement::Definition(definition) => {
                let value = self.canonicalize_expression(&definition.value, circuit_name);
                let mut type_ = definition.type_.clone();

                if self.is_self_type(type_.as_ref()) {
                    type_ = Some(Type::Circuit(circuit_name.clone()));
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
                let assignee = self.canonicalize_assignee(&assign.assignee, circuit_name);
                let value = self.canonicalize_expression(&assign.value, circuit_name);

                Statement::Assign(AssignStatement {
                    assignee,
                    value,
                    operation: assign.operation.clone(),
                    span: assign.span.clone(),
                })
            }
            Statement::Conditional(conditional) => {
                let condition = self.canonicalize_expression(&conditional.condition, circuit_name);
                let block = self.canonicalize_block(&conditional.block, circuit_name);
                let next = match conditional.next.as_ref() {
                    Some(condition) => Some(Box::new(self.canonicalize_statement(condition, circuit_name))),
                    None => None,
                };

                Statement::Conditional(ConditionalStatement {
                    condition,
                    block,
                    next,
                    span: conditional.span.clone(),
                })
            }
            Statement::Iteration(iteration) => {
                let start = self.canonicalize_expression(&iteration.start, circuit_name);
                let stop = self.canonicalize_expression(&iteration.stop, circuit_name);
                let block = self.canonicalize_block(&iteration.block, circuit_name);

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
                        ConsoleFunction::Assert(self.canonicalize_expression(expression, circuit_name))
                    }
                    ConsoleFunction::Debug(format) | ConsoleFunction::Error(format) | ConsoleFunction::Log(format) => {
                        let parameters = format
                            .parameters
                            .iter()
                            .map(|parameter| self.canonicalize_expression(parameter, circuit_name))
                            .collect();

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

                Statement::Console(ConsoleStatement {
                    function,
                    span: console_function_call.span.clone(),
                })
            }
            Statement::Expression(expression) => Statement::Expression(ExpressionStatement {
                expression: self.canonicalize_expression(&expression.expression, circuit_name),
                span: expression.span.clone(),
            }),
            Statement::Block(block) => Statement::Block(self.canonicalize_block(block, circuit_name)),
        }
    }

    fn canonicalize_circuit_member(&self, circuit_member: &CircuitMember, circuit_name: &Identifier) -> CircuitMember {
        match circuit_member {
            CircuitMember::CircuitVariable(_, _) => {}
            CircuitMember::CircuitFunction(function) => {
                let input = function.input.clone();
                let mut output = function.output.clone();
                let block = self.canonicalize_block(&function.block, circuit_name);

                if self.is_self_type(output.as_ref()) {
                    output = Some(Type::Circuit(circuit_name.clone()));
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
    fn reduce_type(&self, _type_: &Type, new: Type, in_circuit: bool, span: &Span) -> Result<Type, CanonicalizeError> {
        match new {
            Type::Array(_, mut dimensions) => {
                if dimensions.is_zero() {
                    return Err(CanonicalizeError::invalid_array_dimension_size(span));
                }

                let mut next = Type::Array(
                    Box::new(Type::Group),
                    ArrayDimensions(vec![dimensions.remove_last().unwrap()]),
                );
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
            Type::SelfType if !in_circuit => Err(CanonicalizeError::big_self_outside_of_circuit(span)),
            _ => Ok(new.clone()),
        }
    }

    fn reduce_array_init(
        &self,
        array_init: &ArrayInitExpression,
        element: Expression,
        _in_circuit: bool,
    ) -> Result<ArrayInitExpression, CanonicalizeError> {
        if array_init.dimensions.is_zero() {
            return Err(CanonicalizeError::invalid_array_dimension_size(&array_init.span));
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
        &self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
        _in_circuit: bool,
    ) -> Result<AssignStatement, CanonicalizeError> {
        match value {
            Expression::Value(value) => {
                let left = Box::new(Expression::Identifier(assignee.identifier.clone()));
                let right = Box::new(Expression::Value(value));
                let op = match assign.operation {
                    AssignOperation::Assign => BinaryOperation::Eq,
                    AssignOperation::Add => BinaryOperation::Add,
                    AssignOperation::Sub => BinaryOperation::Sub,
                    AssignOperation::Mul => BinaryOperation::Mul,
                    AssignOperation::Div => BinaryOperation::Div,
                    AssignOperation::Pow => BinaryOperation::Pow,
                    AssignOperation::Or => BinaryOperation::Or,
                    AssignOperation::And => BinaryOperation::And,
                    AssignOperation::BitOr => BinaryOperation::BitOr,
                    AssignOperation::BitAnd => BinaryOperation::BitAnd,
                    AssignOperation::BitXor => BinaryOperation::BitXor,
                    AssignOperation::Shr => BinaryOperation::Shr,
                    AssignOperation::ShrSigned => BinaryOperation::ShrSigned,
                    AssignOperation::Shl => BinaryOperation::Shl,
                    AssignOperation::Mod => BinaryOperation::Mod,
                };

                let value = Expression::Binary(BinaryExpression {
                    left,
                    right,
                    op,
                    span: assign.span.clone(),
                });

                Ok(AssignStatement {
                    operation: assign.operation.clone(),
                    assignee,
                    value,
                    span: assign.span.clone(),
                })
            }
            _ => Ok(assign.clone()),
        }
    }

    fn reduce_function(
        &self,
        function: &Function,
        identifier: Identifier,
        annotations: Vec<Annotation>,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
        _in_circuit: bool,
    ) -> Result<Function, CanonicalizeError> {
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
        &self,
        _circuit: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Result<Circuit, CanonicalizeError> {
        Ok(Circuit {
            circuit_name: circuit_name.clone(),
            members: members
                .iter()
                .map(|member| self.canonicalize_circuit_member(member, &circuit_name))
                .collect(),
        })
    }
}

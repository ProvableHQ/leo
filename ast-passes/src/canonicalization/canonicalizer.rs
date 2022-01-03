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

use indexmap::IndexMap;
use leo_ast::*;
use leo_errors::{AstError, Result, Span};

/// Replace Self when it is in a enclosing circuit type.
/// Error when Self is outside an enclosing circuit type.
/// Tuple array types and expressions expand to nested arrays.
/// Tuple array types and expressions error if a size of 0 is given.
/// Compound operators become simple assignments.
/// Functions missing output type return a empty tuple.
#[derive(Default)]
pub struct Canonicalizer {
    // If we are in a circuit keep track of the circuit name.
    circuit_name: Option<Identifier>,
    in_circuit: bool,
}

impl Canonicalizer {
    pub fn canonicalize_accesses(
        &mut self,
        start: Expression,
        accesses: &[AssigneeAccess],
        span: &Span,
    ) -> Result<Box<Expression>> {
        let mut left = Box::new(start);

        for access in accesses.iter() {
            match self.canonicalize_assignee_access(access) {
                AssigneeAccess::ArrayIndex(index) => {
                    left = Box::new(Expression::Access(AccessExpression::Array(ArrayAccess {
                        array: left,
                        index: Box::new(index),
                        span: span.clone(),
                    })));
                }
                AssigneeAccess::ArrayRange(start, stop) => {
                    left = Box::new(Expression::Access(AccessExpression::ArrayRange(ArrayRangeAccess {
                        array: left,
                        left: start.map(Box::new),
                        right: stop.map(Box::new),
                        span: span.clone(),
                    })));
                }
                AssigneeAccess::Tuple(positive_number, _) => {
                    left = Box::new(Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: left,
                        index: positive_number,
                        span: span.clone(),
                    })));
                }
                AssigneeAccess::Member(identifier) => {
                    left = Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                        inner: left,
                        name: identifier,
                        span: span.clone(),
                        type_: None,
                    })));
                }
            }
        }

        Ok(left)
    }

    pub fn compound_operation_conversion(&mut self, operation: &AssignOperation) -> Result<BinaryOperation> {
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

    fn canonicalize_self_type(&self, type_option: Option<&Type>) -> Option<Type> {
        match type_option {
            Some(type_) => match type_ {
                Type::SelfType => Some(Type::Identifier(self.circuit_name.as_ref().unwrap().clone())),
                Type::Array(type_, dimensions) => Some(Type::Array(
                    Box::new(self.canonicalize_self_type(Some(type_)).unwrap()),
                    dimensions.clone(),
                )),
                Type::Tuple(types) => Some(Type::Tuple(
                    types
                        .iter()
                        .map(|type_| self.canonicalize_self_type(Some(type_)).unwrap())
                        .collect(),
                )),
                _ => Some(type_.clone()),
            },
            None => None,
        }
    }

    fn canonicalize_circuit_implied_variable_definition(
        &mut self,
        member: &CircuitImpliedVariableDefinition,
    ) -> CircuitImpliedVariableDefinition {
        CircuitImpliedVariableDefinition {
            identifier: member.identifier.clone(),
            expression: member
                .expression
                .as_ref()
                .map(|expr| self.canonicalize_expression(expr)),
        }
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
                    op: binary.op,
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
                let target_type = self.canonicalize_self_type(Some(&cast.target_type)).unwrap();

                return Expression::Cast(CastExpression {
                    inner,
                    target_type,
                    span: cast.span.clone(),
                });
            }

            Expression::Access(access) => {
                let access = match access {
                    AccessExpression::Array(array_access) => {
                        let array = Box::new(self.canonicalize_expression(&array_access.array));
                        let index = Box::new(self.canonicalize_expression(&array_access.index));

                        AccessExpression::Array(ArrayAccess {
                            array,
                            index,
                            span: array_access.span.clone(),
                        })
                    }
                    AccessExpression::ArrayRange(array_range_access) => {
                        let array = Box::new(self.canonicalize_expression(&array_range_access.array));
                        let left = array_range_access
                            .left
                            .as_ref()
                            .map(|left| Box::new(self.canonicalize_expression(left)));
                        let right = array_range_access
                            .right
                            .as_ref()
                            .map(|right| Box::new(self.canonicalize_expression(right)));

                        AccessExpression::ArrayRange(ArrayRangeAccess {
                            array,
                            left,
                            right,
                            span: array_range_access.span.clone(),
                        })
                    }
                    AccessExpression::Member(member_access) => AccessExpression::Member(MemberAccess {
                        inner: Box::new(self.canonicalize_expression(&member_access.inner)),
                        name: member_access.name.clone(),
                        span: member_access.span.clone(),
                        type_: None,
                    }),
                    AccessExpression::Tuple(tuple_access) => {
                        let tuple = Box::new(self.canonicalize_expression(&tuple_access.tuple));

                        AccessExpression::Tuple(TupleAccess {
                            tuple,
                            index: tuple_access.index.clone(),
                            span: tuple_access.span.clone(),
                        })
                    }
                    AccessExpression::Static(static_access) => AccessExpression::Static(StaticAccess {
                        inner: Box::new(self.canonicalize_expression(&static_access.inner)),
                        name: static_access.name.clone(),
                        type_: self.canonicalize_self_type(static_access.type_.as_ref()),
                        span: static_access.span.clone(),
                    }),
                };

                return Expression::Access(access);
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

            Expression::CircuitInit(circuit_init) => {
                let mut name = circuit_init.name.clone();
                if name.name.as_ref() == "Self" && self.circuit_name.is_some() {
                    name = self.circuit_name.as_ref().unwrap().clone();
                }

                return Expression::CircuitInit(CircuitInitExpression {
                    name,
                    members: circuit_init
                        .members
                        .iter()
                        .map(|member| self.canonicalize_circuit_implied_variable_definition(member))
                        .collect(),
                    span: circuit_init.span.clone(),
                });
            }
            Expression::Call(call) => {
                return Expression::Call(CallExpression {
                    function: Box::new(self.canonicalize_expression(&call.function)),
                    arguments: call
                        .arguments
                        .iter()
                        .map(|arg| self.canonicalize_expression(arg))
                        .collect(),
                    span: call.span.clone(),
                });
            }
            Expression::Identifier(identifier) => {
                if identifier.name.as_ref() == "Self" && self.circuit_name.is_some() {
                    return Expression::Identifier(self.circuit_name.as_ref().unwrap().clone());
                }
            }
            _ => (),
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
            AssigneeAccess::ArrayIndex(index) => AssigneeAccess::ArrayIndex(self.canonicalize_expression(index)),
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
            .map(|block_statement| self.canonicalize_statement(block_statement))
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
                let type_ = self.canonicalize_self_type(definition.type_.as_ref());

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

                Statement::Assign(Box::new(AssignStatement {
                    assignee,
                    value,
                    operation: assign.operation,
                    span: assign.span.clone(),
                }))
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

                Statement::Iteration(Box::new(IterationStatement {
                    variable: iteration.variable.clone(),
                    start,
                    stop,
                    inclusive: iteration.inclusive,
                    block,
                    span: iteration.span.clone(),
                }))
            }
            Statement::Console(console_function_call) => {
                let function = match &console_function_call.function {
                    ConsoleFunction::Assert(expression) => {
                        ConsoleFunction::Assert(self.canonicalize_expression(expression))
                    }
                    ConsoleFunction::Error(args) | ConsoleFunction::Log(args) => {
                        let parameters = args
                            .parameters
                            .iter()
                            .map(|parameter| self.canonicalize_expression(parameter))
                            .collect();

                        let console_args = ConsoleArgs {
                            string: args.string.clone(),
                            parameters,
                            span: args.span.clone(),
                        };

                        match &console_function_call.function {
                            ConsoleFunction::Error(_) => ConsoleFunction::Error(console_args),
                            ConsoleFunction::Log(_) => ConsoleFunction::Log(console_args),
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

    fn canonicalize_function_input(&mut self, input: &FunctionInput) -> FunctionInput {
        if let FunctionInput::Variable(variable) = input {
            let type_ = self.canonicalize_self_type(Some(&variable.type_)).unwrap();

            return FunctionInput::Variable(FunctionInputVariable {
                identifier: variable.identifier.clone(),
                const_: variable.const_,
                mutable: variable.mutable,
                type_,
                span: variable.span.clone(),
            });
        }

        input.clone()
    }

    fn canonicalize_circuit_member(&mut self, circuit_member: &CircuitMember) -> CircuitMember {
        match circuit_member {
            CircuitMember::CircuitConst(identifier, type_, value) => {
                return CircuitMember::CircuitConst(
                    identifier.clone(),
                    type_.clone(),
                    self.canonicalize_expression(value),
                );
            }
            CircuitMember::CircuitVariable(_, _) => {}
            CircuitMember::CircuitFunction(function) => {
                let input = function
                    .input
                    .iter()
                    .map(|input| self.canonicalize_function_input(input))
                    .collect();
                let output = self.canonicalize_self_type(function.output.as_ref());
                let block = self.canonicalize_block(&function.block);

                return CircuitMember::CircuitFunction(Box::new(Function {
                    annotations: function.annotations.clone(),
                    identifier: function.identifier.clone(),
                    const_: function.const_,
                    input,
                    output,
                    block,
                    core_mapping: function.core_mapping.clone(),
                    span: function.span.clone(),
                }));
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

    fn reduce_type(&mut self, _type_: &Type, new: Type, span: &Span) -> Result<Type> {
        match new.clone() {
            Type::Array(type_, array_dimensions) => {
                // The goal of this part is to reduce `ArrayDimensions::Multi` into nested Array type.
                Ok(match &array_dimensions {
                    ArrayDimensions::Unspecified | ArrayDimensions::Number(_) => Type::Array(type_, array_dimensions),

                    // Only canonicalize multidimensional Arrays
                    ArrayDimensions::Multi(dimensions) => {
                        let mut iter = dimensions.iter().rev();

                        // Get the last Array dimension to place it inside others.
                        if let Some(last) = iter.next() {
                            let mut prev = Type::Array(type_, last.clone());
                            for next in iter {
                                prev = Type::Array(Box::new(prev), next.clone());
                            }
                            prev
                        } else {
                            Type::Array(type_, array_dimensions)
                        }
                    }
                })
            }
            Type::SelfType if !self.in_circuit => Err(AstError::big_self_outside_of_circuit(span).into()),
            _ => Ok(new),
        }
    }

    fn reduce_string(&mut self, string: &[Char], span: &Span) -> Result<Expression> {
        if string.is_empty() {
            return Err(AstError::empty_string(span).into());
        }

        let mut elements = Vec::new();
        let mut col_adder = 0;
        for (index, character) in string.iter().enumerate() {
            let col_start = span.col_start + index + 1 + col_adder; // account for open quote
            let bytes = span.content.clone().into_bytes();
            let col_stop: usize;

            if bytes[col_start - 1] == b'\\' {
                let mut width = 0;

                match bytes[col_start] {
                    b'x' => width += 3,
                    b'u' => {
                        width += 1;
                        let mut index = 1;
                        while bytes[col_start + index] != b'}' {
                            width += 1;
                            index += 1;
                        }
                        width += 1;
                    }
                    _ => width += 1,
                }
                col_adder += width;
                col_stop = col_start + 1 + width;
            } else {
                col_stop = col_start + 1;
            }

            elements.push(SpreadOrExpression::Expression(Expression::Value(
                ValueExpression::Char(CharValue {
                    character: character.clone(),
                    span: Span::new(
                        span.line_start,
                        span.line_stop,
                        col_start,
                        col_stop,
                        span.path.clone(),
                        span.content.clone(),
                    ),
                }),
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
    ) -> Result<ArrayInitExpression> {
        if array_init.dimensions.is_zero() {
            return Err(AstError::invalid_array_dimension_size(&array_init.span).into());
        }

        let element = Box::new(element);
        let mut dimensions = array_init.dimensions.flatten();

        if dimensions.len() == 1 {
            return Ok(ArrayInitExpression {
                element,
                dimensions: dimensions.get(0).unwrap().clone(),
                span: array_init.span.clone(),
            });
        };

        let mut next = Expression::ArrayInit(ArrayInitExpression {
            element,
            dimensions: dimensions.pop().unwrap(),
            span: array_init.span.clone(),
        });

        let mut outer_element = Box::new(next.clone());
        for (index, dimension) in dimensions.iter().rev().enumerate() {
            if index == dimensions.len() - 1 {
                break;
            }

            next = Expression::ArrayInit(ArrayInitExpression {
                element: outer_element,
                dimensions: dimension.clone(),
                span: array_init.span.clone(),
            });

            outer_element = Box::new(next.clone());
        }

        Ok(ArrayInitExpression {
            element: outer_element,
            dimensions: dimensions.remove(0),
            span: array_init.span.clone(),
        })
    }

    fn reduce_assign(
        &mut self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
    ) -> Result<AssignStatement> {
        match value {
            value if assign.operation != AssignOperation::Assign => {
                let left = self.canonicalize_accesses(
                    Expression::Identifier(assignee.identifier.clone()),
                    &assignee.accesses,
                    &assign.span,
                )?;
                let right = Box::new(value);
                let op = self.compound_operation_conversion(&assign.operation)?;

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
            value => Ok(AssignStatement {
                operation: AssignOperation::Assign,
                assignee,
                value,
                span: assign.span.clone(),
            }),
        }
    }

    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: Identifier,
        annotations: IndexMap<String, Annotation>,
        input: Vec<FunctionInput>,
        const_: bool,
        output: Option<Type>,
        block: Block,
    ) -> Result<Function> {
        let new_output = match output {
            None => Some(Type::Tuple(vec![])),
            _ => output,
        };

        Ok(Function {
            identifier,
            annotations,
            input,
            const_,
            output: new_output,
            block,
            core_mapping: function.core_mapping.clone(),
            span: function.span.clone(),
        })
    }

    fn reduce_circuit(
        &mut self,
        _circuit: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Result<Circuit> {
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

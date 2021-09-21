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

use crate::{
    CircuitMember, ConstValue, Expression, ExpressionNode, FromAst, Function, FunctionQualifier, Node, PartialType,
    Scope, Type,
};
pub use leo_ast::{BinaryOperation, Node as AstNode};
use leo_errors::{AsgError, Result, Span};

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Clone)]
pub struct CallExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub function: Rc<RefCell<Function<'a>>>,
    pub target: Cell<Option<&'a Expression<'a>>>,
    pub arguments: Vec<Cell<&'a Expression<'a>>>,
}

impl<'a> Node for CallExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for CallExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        if let Some(target) = self.target.get() {
            target.set_parent(expr);
        }
        self.arguments.iter().for_each(|element| {
            element.get().set_parent(expr);
        })
    }

    fn get_type(&self) -> Option<Type<'a>> {
        Some(self.function.borrow().output.clone())
    }

    fn is_mut_ref(&self) -> bool {
        true
    }

    fn const_value(&self) -> Option<ConstValue> {
        // static function const evaluation
        None
    }

    fn is_consty(&self) -> bool {
        self.target.get().map(|x| x.is_consty()).unwrap_or(true) && self.arguments.iter().all(|x| x.get().is_consty())
    }
}

impl<'a> FromAst<'a, leo_ast::CallExpression> for CallExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::CallExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<CallExpression<'a>> {
        let (target, function): (Option<&Expression>, Function) = match &*value.function {
            leo_ast::Expression::Identifier(name) => (
                None,
                scope
                    .resolve_function(&name.name)
                    .ok_or_else(|| AsgError::unresolved_function(&name.name, &name.span))?
                    .clone(),
            ),
            leo_ast::Expression::Access(access) => match access {
                leo_ast::AccessExpression::CircuitMember(leo_ast::accesses::CircuitMemberAccess {
                    circuit: ast_circuit,
                    name,
                    span,
                    ..
                }) => {
                    let target = <&Expression<'a>>::from_ast(scope, &**ast_circuit, None)?;
                    let circuit = match target.get_type() {
                        Some(Type::Circuit(circuit)) => circuit,
                        type_ => {
                            return Err(AsgError::unexpected_type(
                                "circuit",
                                type_.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                                span,
                            )
                            .into());
                        }
                    };
                    let circuit_name = circuit.name.borrow().name.clone();
                    let member = circuit.members.borrow();
                    let member = member
                        .get(name.name.as_ref())
                        .ok_or_else(|| AsgError::unresolved_circuit_member(&circuit_name, &name.name, span))?;
                    match member {
                        CircuitMember::Function(body) => {
                            if body.qualifier == FunctionQualifier::Static {
                                return Err(
                                    AsgError::circuit_static_call_invalid(&circuit_name, &name.name, span).into()
                                );
                            } else if body.qualifier == FunctionQualifier::MutSelfRef && !target.is_mut_ref() {
                                return Err(
                                    AsgError::circuit_member_mut_call_invalid(circuit_name, &name.name, span).into(),
                                );
                            }
                            (Some(target), (*body).clone())
                        }
                        CircuitMember::Variable(_) => {
                            return Err(AsgError::circuit_variable_call(circuit_name, &name.name, span).into());
                        }
                    }
                }
                leo_ast::AccessExpression::CircuitStaticFunction(leo_ast::accesses::CircuitStaticFunctionAccess {
                    circuit: ast_circuit,
                    name,
                    span,
                }) => {
                    let circuit = if let leo_ast::Expression::Identifier(circuit_name) = &**ast_circuit {
                        scope
                            .resolve_circuit(&circuit_name.name)
                            .ok_or_else(|| AsgError::unresolved_circuit(&circuit_name.name, &circuit_name.span))?
                    } else {
                        return Err(AsgError::unexpected_type("circuit", "unknown", span).into());
                    };
                    let circuit_name = circuit.name.borrow().name.clone();

                    let member = circuit.members.borrow();
                    let member = member
                        .get(name.name.as_ref())
                        .ok_or_else(|| AsgError::unresolved_circuit_member(&circuit_name, &name.name, span))?;
                    match member {
                        CircuitMember::Function(body) => {
                            if body.qualifier != FunctionQualifier::Static {
                                return Err(
                                    AsgError::circuit_member_call_invalid(circuit_name, &name.name, span).into()
                                );
                            }
                            (None, (*body).clone())
                        }
                        CircuitMember::Variable(_) => {
                            return Err(AsgError::circuit_variable_call(circuit_name, &name.name, span).into());
                        }
                    }
                }
                leo_ast::AccessExpression::Value(leo_ast::accesses::ValueAccess { value, access, span }) => {
                    let target = <&Expression<'a>>::from_ast(scope, &**value, None)?;

                    let values = if let Some(function) = scope.resolve_function(access.name.as_ref()) {
                        (Some(target), function.clone())
                    } else {
                        // TODO put better error here
                        return Err(AsgError::illegal_ast_structure(
                            "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                            span,
                        )
                        .into());
                    };

                    values
                }
                leo_ast::AccessExpression::Named(leo_ast::accesses::NamedTypeAccess {
                    named_type,
                    access,
                    span,
                }) => {
                    let target = <&Expression<'a>>::from_ast(scope, &**named_type, None)?;

                    let values = if let Some(function) = scope.resolve_function(access.name.as_ref()) {
                        if let Expression::NamedType(crate::NamedTypeExpression { named_type, .. }) = target {
                            // this is a temporary hack to get correct output type till generics
                            let _type_ = named_type.name.to_string();

                            /* function.output = match type_.as_str() {
                                "u8" => Type::Integer(leo_ast::IntegerType::U8),
                                "u16" => Type::Integer(leo_ast::IntegerType::U16),
                                "u32" => Type::Integer(leo_ast::IntegerType::U32),
                                "u64" => Type::Integer(leo_ast::IntegerType::U64),
                                "u128" => Type::Integer(leo_ast::IntegerType::U128),
                                "i8" => Type::Integer(leo_ast::IntegerType::I8),
                                "i16" => Type::Integer(leo_ast::IntegerType::I16),
                                "i32" => Type::Integer(leo_ast::IntegerType::I32),
                                "i64" => Type::Integer(leo_ast::IntegerType::I64),
                                "i128" => Type::Integer(leo_ast::IntegerType::I128),
                                _ => unimplemented!(),
                            }; */
                            (Some(target), function.clone())
                        } else {
                            return Err(AsgError::illegal_ast_structure(
                                "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                                span,
                            )
                            .into());
                        }
                    } else {
                        // TODO put better error here
                        return Err(AsgError::illegal_ast_structure(
                            "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                            span,
                        )
                        .into());
                    };

                    values
                }
                _ => {
                    return Err(AsgError::illegal_ast_structure(
                        "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess/ValueAccess as call target",
                        access.span(),
                    )
                    .into());
                }
            },
            _ => {
                return Err(AsgError::illegal_ast_structure(
                    "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                    &value.span,
                )
                .into());
            }
        };
        if let Some(expected) = expected_type {
            let output: Type = function.output.clone();
            if !expected.matches(&output) {
                return Err(AsgError::unexpected_type(expected, output, &value.span).into());
            }
        }
        if value.arguments.len() != function.arguments.len() {
            return Err(AsgError::unexpected_call_argument_count(
                function.arguments.len(),
                value.arguments.len(),
                &value.span,
            )
            .into());
        }

        let arguments = value
            .arguments
            .iter()
            .zip(function.arguments.iter())
            .map(|(expr, (_, argument))| {
                let argument = argument.get().borrow();
                let converted = <&Expression<'a>>::from_ast(scope, expr, Some(argument.type_.clone().partial()))?;
                if argument.const_ && !converted.is_consty() {
                    return Err(AsgError::unexpected_nonconst(expr.span()).into());
                }
                Ok(Cell::new(converted))
            })
            .collect::<Result<Vec<_>>>()?;

        if function.is_test() {
            return Err(AsgError::call_test_function(&value.span).into());
        }
        Ok(CallExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            arguments,
            function: Rc::new(RefCell::new(function)),
            target: Cell::new(target),
        })
    }
}

impl<'a> Into<leo_ast::CallExpression> for &CallExpression<'a> {
    fn into(self) -> leo_ast::CallExpression {
        let target_function = if let Some(target) = self.target.get() {
            target.into()
        } else {
            let circuit = self.function.borrow().circuit.get();
            if let Some(circuit) = circuit {
                leo_ast::Expression::Access(leo_ast::AccessExpression::CircuitStaticFunction(
                    leo_ast::accesses::CircuitStaticFunctionAccess {
                        circuit: Box::new(leo_ast::Expression::Identifier(circuit.name.borrow().clone())),
                        name: self.function.borrow().name.borrow().clone(),
                        span: self.span.clone().unwrap_or_default(),
                    },
                ))
            } else {
                leo_ast::Expression::Identifier(self.function.borrow().name.borrow().clone())
            }
        };
        leo_ast::CallExpression {
            function: Box::new(target_function),
            arguments: self.arguments.iter().map(|arg| arg.get().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}

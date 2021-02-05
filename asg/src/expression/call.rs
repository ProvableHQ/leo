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
    AsgConvertError,
    CircuitMember,
    ConstValue,
    Expression,
    ExpressionNode,
    FromAst,
    Function,
    FunctionQualifier,
    Node,
    PartialType,
    Scope,
    Span,
    Type,
};
pub use leo_ast::BinaryOperation;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct CallExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub function: Arc<Function>,
    pub target: Option<Arc<Expression>>,
    pub arguments: Vec<Arc<Expression>>,
}

impl Node for CallExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for CallExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        if let Some(target) = self.target.as_ref() {
            target.set_parent(Arc::downgrade(expr));
        }
        self.arguments.iter().for_each(|element| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(self.function.output.clone().into())
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        // static function const evaluation
        None
    }

    fn is_consty(&self) -> bool {
        self.target.as_ref().map(|x| x.is_consty()).unwrap_or(true) && self.arguments.iter().all(|x| x.is_consty())
    }
}

impl FromAst<leo_ast::CallExpression> for CallExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::CallExpression,
        expected_type: Option<PartialType>,
    ) -> Result<CallExpression, AsgConvertError> {
        let (target, function) = match &*value.function {
            leo_ast::Expression::Identifier(name) => (
                None,
                scope
                    .borrow()
                    .resolve_function(&name.name)
                    .ok_or_else(|| AsgConvertError::unresolved_function(&name.name, &name.span))?,
            ),
            leo_ast::Expression::CircuitMemberAccess(leo_ast::CircuitMemberAccessExpression {
                circuit: ast_circuit,
                name,
                span,
            }) => {
                let target = Arc::<Expression>::from_ast(scope, &**ast_circuit, None)?;
                let circuit = match target.get_type() {
                    Some(Type::Circuit(circuit)) => circuit,
                    type_ => {
                        return Err(AsgConvertError::unexpected_type(
                            "circuit",
                            type_.map(|x| x.to_string()).as_deref(),
                            &span,
                        ));
                    }
                };
                let circuit_name = circuit.name.borrow().name.clone();
                let member = circuit.members.borrow();
                let member = member
                    .get(&name.name)
                    .ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit_name, &name.name, &span))?;
                match member {
                    CircuitMember::Function(body) => {
                        if body.qualifier == FunctionQualifier::Static {
                            return Err(AsgConvertError::circuit_static_call_invalid(
                                &circuit_name,
                                &name.name,
                                &span,
                            ));
                        } else if body.qualifier == FunctionQualifier::MutSelfRef && !target.is_mut_ref() {
                            return Err(AsgConvertError::circuit_member_mut_call_invalid(
                                &circuit_name,
                                &name.name,
                                &span,
                            ));
                        }
                        (Some(target), body.clone())
                    }
                    CircuitMember::Variable(_) => {
                        return Err(AsgConvertError::circuit_variable_call(&circuit_name, &name.name, &span));
                    }
                }
            }
            leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                circuit: ast_circuit,
                name,
                span,
            }) => {
                let circuit = if let leo_ast::Expression::Identifier(circuit_name) = &**ast_circuit {
                    scope
                        .borrow()
                        .resolve_circuit(&circuit_name.name)
                        .ok_or_else(|| AsgConvertError::unresolved_circuit(&circuit_name.name, &circuit_name.span))?
                } else {
                    return Err(AsgConvertError::unexpected_type("circuit", None, &span));
                };
                let circuit_name = circuit.name.borrow().name.clone();

                let member = circuit.members.borrow();
                let member = member
                    .get(&name.name)
                    .ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit_name, &name.name, &span))?;
                match member {
                    CircuitMember::Function(body) => {
                        if body.qualifier != FunctionQualifier::Static {
                            return Err(AsgConvertError::circuit_member_call_invalid(
                                &circuit_name,
                                &name.name,
                                &span,
                            ));
                        }
                        (None, body.clone())
                    }
                    CircuitMember::Variable(_) => {
                        return Err(AsgConvertError::circuit_variable_call(&circuit_name, &name.name, &span));
                    }
                }
            }
            _ => {
                return Err(AsgConvertError::illegal_ast_structure(
                    "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                ));
            }
        };
        if let Some(expected) = expected_type {
            let output: Type = function.output.clone().into();
            if !expected.matches(&output) {
                return Err(AsgConvertError::unexpected_type(
                    &expected.to_string(),
                    Some(&*output.to_string()),
                    &value.span,
                ));
            }
        }
        if value.arguments.len() != function.argument_types.len() {
            return Err(AsgConvertError::unexpected_call_argument_count(
                function.argument_types.len(),
                value.arguments.len(),
                &value.span,
            ));
        }

        Ok(CallExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            arguments: value
                .arguments
                .iter()
                .zip(function.argument_types.iter())
                .map(|(expr, argument)| {
                    Arc::<Expression>::from_ast(scope, expr, Some(argument.clone().strong().partial()))
                })
                .collect::<Result<Vec<_>, AsgConvertError>>()?,
            function,
            target,
        })
    }
}

impl Into<leo_ast::CallExpression> for &CallExpression {
    fn into(self) -> leo_ast::CallExpression {
        let target_function = if let Some(target) = &self.target {
            target.as_ref().into()
        } else {
            let circuit = self.function.circuit.borrow().as_ref().map(|x| x.upgrade()).flatten();
            if let Some(circuit) = circuit {
                leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(leo_ast::Expression::Identifier(circuit.name.borrow().clone())),
                    name: self.function.name.borrow().clone(),
                    span: self.span.clone().unwrap_or_default(),
                })
            } else {
                leo_ast::Expression::Identifier(self.function.name.borrow().clone())
            }
        };
        leo_ast::CallExpression {
            function: Box::new(target_function),
            arguments: self.arguments.iter().map(|arg| arg.as_ref().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}

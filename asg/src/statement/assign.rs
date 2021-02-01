// Copyright (C) 2019-2020 Aleo Systems Inc.
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
    ConstInt,
    ConstValue,
    Expression,
    ExpressionNode,
    FromAst,
    Identifier,
    IntegerType,
    Node,
    PartialType,
    Scope,
    Span,
    Statement,
    Type,
    Variable,
};
pub use leo_ast::AssignOperation;
use leo_ast::AssigneeAccess as AstAssigneeAccess;

use std::sync::{Arc, Weak};

pub enum AssignAccess {
    ArrayRange(Option<Arc<Expression>>, Option<Arc<Expression>>),
    ArrayIndex(Arc<Expression>),
    Tuple(usize),
    Member(Identifier),
}

pub struct AssignStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub operation: AssignOperation,
    pub target_variable: Variable,
    pub target_accesses: Vec<AssignAccess>,
    pub value: Arc<Expression>,
}

impl Node for AssignStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::AssignStatement> for Arc<Statement> {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::AssignStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Arc<Statement>, AsgConvertError> {
        let (name, span) = (&statement.assignee.identifier.name, &statement.assignee.identifier.span);

        let variable = if name == "input" {
            if let Some(function) = scope.borrow().resolve_current_function() {
                if !function.has_input {
                    return Err(AsgConvertError::unresolved_reference(name, span));
                }
            } else {
                return Err(AsgConvertError::unresolved_reference(name, span));
            }
            if let Some(input) = scope.borrow().resolve_input() {
                input.container
            } else {
                return Err(AsgConvertError::InternalError(
                    "attempted to reference input when none is in scope".to_string(),
                ));
            }
        } else {
            scope
                .borrow()
                .resolve_variable(&name)
                .ok_or_else(|| AsgConvertError::unresolved_reference(name, span))?
        };

        if !variable.borrow().mutable {
            return Err(AsgConvertError::immutable_assignment(&name, &statement.span));
        }
        let mut target_type: Option<PartialType> = Some(variable.borrow().type_.clone().into());

        let mut target_accesses = vec![];
        for access in statement.assignee.accesses.iter() {
            target_accesses.push(match access {
                AstAssigneeAccess::ArrayRange(left, right) => {
                    let index_type = Some(PartialType::Integer(None, Some(IntegerType::U32)));
                    let left = left
                        .as_ref()
                        .map(
                            |left: &leo_ast::Expression| -> Result<Arc<Expression>, AsgConvertError> {
                                Arc::<Expression>::from_ast(scope, left, index_type.clone())
                            },
                        )
                        .transpose()?;
                    let right = right
                        .as_ref()
                        .map(
                            |right: &leo_ast::Expression| -> Result<Arc<Expression>, AsgConvertError> {
                                Arc::<Expression>::from_ast(scope, right, index_type)
                            },
                        )
                        .transpose()?;

                    match &target_type {
                        Some(PartialType::Array(item, len)) => {
                            if let (Some(left), Some(right)) = (
                                left.as_ref()
                                    .map(|x| x.const_value())
                                    .unwrap_or_else(|| Some(ConstValue::Int(ConstInt::U32(0)))),
                                right
                                    .as_ref()
                                    .map(|x| x.const_value())
                                    .unwrap_or_else(|| Some(ConstValue::Int(ConstInt::U32(len.map(|x| x as u32)?)))),
                            ) {
                                let left = match left {
                                    ConstValue::Int(x) => x.to_usize().ok_or_else(|| {
                                        AsgConvertError::invalid_assign_index(&name, &x.to_string(), &statement.span)
                                    })?,
                                    _ => unimplemented!(),
                                };
                                let right = match right {
                                    ConstValue::Int(x) => x.to_usize().ok_or_else(|| {
                                        AsgConvertError::invalid_assign_index(&name, &x.to_string(), &statement.span)
                                    })?,
                                    _ => unimplemented!(),
                                };
                                if right >= left {
                                    target_type = Some(PartialType::Array(item.clone(), Some((right - left) as usize)))
                                } else {
                                    return Err(AsgConvertError::invalid_backwards_assignment(
                                        &name,
                                        left,
                                        right,
                                        &statement.span,
                                    ));
                                }
                            }
                        }
                        _ => return Err(AsgConvertError::index_into_non_array(&name, &statement.span)),
                    }

                    AssignAccess::ArrayRange(left, right)
                }
                AstAssigneeAccess::ArrayIndex(index) => {
                    target_type = match target_type.clone() {
                        Some(PartialType::Array(item, _)) => item.map(|x| *x),
                        _ => return Err(AsgConvertError::index_into_non_array(&name, &statement.span)),
                    };
                    AssignAccess::ArrayIndex(Arc::<Expression>::from_ast(
                        scope,
                        index,
                        Some(PartialType::Integer(None, Some(IntegerType::U32))),
                    )?)
                }
                AstAssigneeAccess::Tuple(index, _) => {
                    let index = index
                        .value
                        .parse::<usize>()
                        .map_err(|_| AsgConvertError::parse_index_error())?;
                    target_type = match target_type {
                        Some(PartialType::Tuple(types)) => types
                            .get(index)
                            .cloned()
                            .ok_or_else(|| AsgConvertError::tuple_index_out_of_bounds(index, &statement.span))?,
                        _ => return Err(AsgConvertError::index_into_non_tuple(&name, &statement.span)),
                    };
                    AssignAccess::Tuple(index)
                }
                AstAssigneeAccess::Member(name) => {
                    target_type = match target_type {
                        Some(PartialType::Type(Type::Circuit(circuit))) => {
                            let circuit = circuit;

                            let members = circuit.members.borrow();
                            let member = members.get(&name.name).ok_or_else(|| {
                                AsgConvertError::unresolved_circuit_member(
                                    &circuit.name.borrow().name,
                                    &name.name,
                                    &statement.span,
                                )
                            })?;

                            let x = match &member {
                                CircuitMember::Variable(type_) => type_.clone(),
                                CircuitMember::Function(_) => {
                                    return Err(AsgConvertError::illegal_function_assign(&name.name, &statement.span));
                                }
                            };
                            Some(x.strong().partial())
                        }
                        _ => {
                            return Err(AsgConvertError::index_into_non_tuple(
                                &statement.assignee.identifier.name,
                                &statement.span,
                            ));
                        }
                    };
                    AssignAccess::Member(name.clone())
                }
            });
        }
        let value = Arc::<Expression>::from_ast(scope, &statement.value, target_type)?;

        let statement = Arc::new(Statement::Assign(AssignStatement {
            parent: None,
            span: Some(statement.span.clone()),
            operation: statement.operation.clone(),
            target_variable: variable.clone(),
            target_accesses,
            value,
        }));

        {
            let mut variable = variable.borrow_mut();
            variable.assignments.push(Arc::downgrade(&statement));
        }

        Ok(statement)
    }
}

impl Into<leo_ast::AssignStatement> for &AssignStatement {
    fn into(self) -> leo_ast::AssignStatement {
        leo_ast::AssignStatement {
            operation: self.operation.clone(),
            assignee: leo_ast::Assignee {
                identifier: self.target_variable.borrow().name.clone(),
                accesses: self
                    .target_accesses
                    .iter()
                    .map(|access| match access {
                        AssignAccess::ArrayRange(left, right) => AstAssigneeAccess::ArrayRange(
                            left.as_ref().map(|e| e.as_ref().into()),
                            right.as_ref().map(|e| e.as_ref().into()),
                        ),
                        AssignAccess::ArrayIndex(index) => AstAssigneeAccess::ArrayIndex(index.as_ref().into()),
                        AssignAccess::Tuple(index) => AstAssigneeAccess::Tuple(
                            leo_ast::PositiveNumber {
                                value: index.to_string(),
                            },
                            self.span.clone().unwrap_or_default(),
                        ),
                        AssignAccess::Member(name) => AstAssigneeAccess::Member(name.clone()),
                    })
                    .collect(),
                span: self.span.clone().unwrap_or_default(),
            },
            value: self.value.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}

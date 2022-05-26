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

use leo_ast::*;
use leo_errors::{emitter::Handler, TypeCheckerError};

use crate::{SymbolTable, TypeChecker};

pub(crate) struct Director<'a> {
    visitor: TypeChecker<'a>,
}

impl<'a> Director<'a> {
    pub(crate) fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            visitor: TypeChecker::new(symbol_table, handler),
        }
    }
}

impl<'a> VisitorDirector<'a> for Director<'a> {
    type Visitor = TypeChecker<'a>;

    fn visitor(self) -> Self::Visitor {
        self.visitor
    }

    fn visitor_ref(&mut self) -> &mut Self::Visitor {
        &mut self.visitor
    }
}

fn return_incorrect_type(t1: Option<Type>, t2: Option<Type>, expected: Option<Type>) -> Option<Type> {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => Some(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if t1 != expected {
                    Some(t1)
                } else {
                    Some(t2)
                }
            } else {
                Some(t1)
            }
        }
        (None, Some(_)) | (Some(_), None) | (None, None) => None,
    }
}

impl<'a> ExpressionVisitorDirector<'a> for Director<'a> {
    type Output = Type;

    fn visit_expression(&mut self, input: &'a Expression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_expression(input).0 {
            return match input {
                Expression::Identifier(expr) => self.visit_identifier(expr),
                Expression::Value(expr) => self.visit_value(expr),
                Expression::Binary(expr) => self.visit_binary(expr),
                Expression::Unary(expr) => self.visit_unary(expr),
                Expression::Ternary(expr) => self.visit_ternary(expr),
                Expression::Call(expr) => self.visit_call(expr),
                Expression::Err(expr) => self.visit_err(expr),
            };
        }

        None
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> Option<Self::Output> {
        self.visitor.visit_identifier(input).1
    }

    fn visit_value(&mut self, input: &'a ValueExpression) -> Option<Self::Output> {
        self.visitor.visit_value(input).1
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> Option<Self::Output> {
        match self.visitor.visit_binary(input) {
            (VisitResult::VisitChildren, expected) => {
                let t1 = self.visit_expression(&input.left);
                let t2 = self.visit_expression(&input.right);

                return_incorrect_type(t1, t2, self.visitor.expected_type)
            }
            _ => None,
        }
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression) -> Option<Self::Output> {
        match input.op {
            UnaryOperation::Not => {
                self.visitor.assert_type(Type::Boolean, self.visitor.expected_type);
                self.visit_expression(&input.inner)
            }
            UnaryOperation::Negate => {
                let prior_negate_state = self.visitor.negate;
                self.visitor.negate = true;

                let type_ = self.visit_expression(&input.inner);
                self.visitor.negate = prior_negate_state;
                match type_.as_ref() {
                    Some(
                        Type::IntegerType(
                            IntegerType::I8
                            | IntegerType::I16
                            | IntegerType::I32
                            | IntegerType::I64
                            | IntegerType::I128,
                        )
                        | Type::Field
                        | Type::Group,
                    ) => {}
                    Some(t) => self
                        .visitor
                        .handler
                        .emit_err(TypeCheckerError::type_is_not_negatable(t, input.inner.span()).into()),
                    _ => {}
                };
                type_
            }
        }
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_ternary(input).0 {
            let prev_expected_type = self.visitor.expected_type;
            self.visitor.expected_type = Some(Type::Boolean);
            self.visit_expression(&input.condition);
            self.visitor.expected_type = prev_expected_type;

            let t1 = self.visit_expression(&input.if_true);
            let t2 = self.visit_expression(&input.if_false);

            return return_incorrect_type(t1, t2, self.visitor.expected_type);
        }

        None
    }

    fn visit_call(&mut self, input: &'a CallExpression) -> Option<Self::Output> {
        match &*input.function {
            Expression::Identifier(ident) => {
                if let Some(func) = self.visitor.symbol_table.clone().lookup_fn(&ident.name) {
                    let ret = self.visitor.assert_type(func.output, self.visitor.expected_type);

                    if func.input.len() != input.arguments.len() {
                        self.visitor.handler.emit_err(
                            TypeCheckerError::incorrect_num_args_to_call(
                                func.input.len(),
                                input.arguments.len(),
                                input.span(),
                            )
                            .into(),
                        );
                    }

                    func.input
                        .iter()
                        .zip(input.arguments.iter())
                        .for_each(|(expected, argument)| {
                            let prev_expected_type = self.visitor.expected_type;
                            self.visitor.expected_type = Some(expected.get_variable().type_);
                            self.visit_expression(argument);
                            self.visitor.expected_type = prev_expected_type;
                        });

                    Some(ret)
                } else {
                    self.visitor
                        .handler
                        .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                    None
                }
            }
            expr => self.visit_expression(expr),
        }
    }

    fn visit_err(&mut self, input: &'a ErrExpression) -> Option<Self::Output> {
        self.visitor.visit_err(input).1
    }
}

impl<'a> StatementVisitorDirector<'a> for Director<'a> {}

impl<'a> ProgramVisitorDirector<'a> for Director<'a> {}

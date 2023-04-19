// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::DeadCodeEliminator;

use leo_ast::{
    AccessExpression,
    AssociatedFunction,
    Expression,
    ExpressionReconstructor,
    Identifier,
    MemberAccess,
    StructExpression,
    StructVariableInitializer,
    TupleAccess,
    Type,
};
use leo_span::sym;

impl ExpressionReconstructor for DeadCodeEliminator {
    type AdditionalOutput = ();

    /// Reconstructs the components of an access expression.
    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(match input {
                AccessExpression::AssociatedFunction(function) => {
                    // If the associated function manipulates a mapping, mark the statement as necessary.
                    match (&function.ty, function.name.name) {
                        (Type::Identifier(Identifier { name: sym::Mapping, .. }), sym::get)
                        | (Type::Identifier(Identifier { name: sym::Mapping, .. }), sym::get_or_init)
                        | (Type::Identifier(Identifier { name: sym::Mapping, .. }), sym::set) => {
                            self.is_necessary = true;
                        }
                        _ => {}
                    };
                    // Reconstruct the access expression.
                    let result = AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: function.ty,
                        name: function.name,
                        arguments: function
                            .arguments
                            .into_iter()
                            .map(|arg| self.reconstruct_expression(arg).0)
                            .collect(),
                        span: function.span,
                    });
                    // Unset `self.is_necessary`.
                    self.is_necessary = false;
                    result
                }
                AccessExpression::Member(member) => AccessExpression::Member(MemberAccess {
                    inner: Box::new(self.reconstruct_expression(*member.inner).0),
                    name: member.name,
                    span: member.span,
                }),
                AccessExpression::Tuple(tuple) => AccessExpression::Tuple(TupleAccess {
                    tuple: Box::new(self.reconstruct_expression(*tuple.tuple).0),
                    index: tuple.index,
                    span: tuple.span,
                }),
                AccessExpression::AssociatedConstant(constant) => AccessExpression::AssociatedConstant(constant),
            }),
            Default::default(),
        )
    }

    /// Reconstruct the components of the struct init expression.
    /// This is necessary since the reconstructor does not explicitly visit each component of the expression.
    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Struct(StructExpression {
                name: input.name,
                // Reconstruct each of the struct members.
                members: input
                    .members
                    .into_iter()
                    .map(|member| StructVariableInitializer {
                        identifier: member.identifier,
                        expression: match member.expression {
                            Some(expression) => Some(self.reconstruct_expression(expression).0),
                            None => unreachable!("Static single assignment ensures that the expression always exists."),
                        },
                    })
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    /// Marks identifiers as used.
    /// This is necessary to determine which statements can be eliminated from the program.
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        // Add the identifier to `self.used_variables`.
        if self.is_necessary {
            self.used_variables.insert(input.name);
        }
        // Return the identifier as is.
        (Expression::Identifier(input), Default::default())
    }
}

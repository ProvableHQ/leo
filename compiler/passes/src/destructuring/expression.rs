// Copyright (C) 2019-2025 Provable Inc.
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

use super::DestructuringVisitor;

use leo_ast::{
    ArrayAccess,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IntegerType,
    Literal,
    Node as _,
    Statement,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    Type,
};

use itertools::izip;

impl ExpressionReconstructor for DestructuringVisitor<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Replaces a tuple access expression with the appropriate expression.
    fn reconstruct_tuple_access(&mut self, input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        let Expression::Identifier(identifier) = &input.tuple else {
            panic!("SSA guarantees that subexpressions are identifiers or literals.");
        };

        // Look up the expression in the tuple map.
        match self.tuples.get(&identifier.name).and_then(|tuple_names| tuple_names.get(input.index.value())) {
            Some(id) => ((*id).into(), Default::default()),
            None => {
                if !matches!(self.state.type_table.get(&identifier.id), Some(Type::Future(_))) {
                    panic!("Type checking guarantees that all tuple accesses are declared and indices are valid.");
                }

                let index = Literal::integer(
                    IntegerType::U32,
                    input.index.to_string(),
                    input.span,
                    self.state.node_builder.next_id(),
                );
                self.state.type_table.insert(index.id(), Type::Integer(IntegerType::U32));

                let expr =
                    ArrayAccess { array: (*identifier).into(), index: index.into(), span: input.span, id: input.id }
                        .into();

                (expr, Default::default())
            }
        }
    }

    /// If this is a ternary expression on tuples of length `n`, we'll need to change it into
    /// `n` ternary expressions on the members.
    fn reconstruct_ternary(&mut self, mut input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (if_true, mut statements) = self.reconstruct_expression_tuple(std::mem::take(&mut input.if_true));
        let (if_false, statements2) = self.reconstruct_expression_tuple(std::mem::take(&mut input.if_false));
        statements.extend(statements2);

        match (if_true, if_false) {
            (Expression::Tuple(tuple_true), Expression::Tuple(tuple_false)) => {
                // Aleo's `ternary` opcode doesn't know about tuples, so we have to handle this.
                let Some(Type::Tuple(tuple_type)) = self.state.type_table.get(&tuple_true.id()) else {
                    panic!("Should have tuple type");
                };

                // We'll be reusing `input.condition`, so assign it to a variable.
                let cond = if let Expression::Identifier(..) = input.condition {
                    input.condition
                } else {
                    let place = Identifier::new(
                        self.state.assigner.unique_symbol("cond", "$$"),
                        self.state.node_builder.next_id(),
                    );

                    let definition = self.state.assigner.simple_definition(
                        place,
                        input.condition,
                        self.state.node_builder.next_id(),
                    );

                    statements.push(definition);

                    self.state.type_table.insert(place.id(), Type::Boolean);

                    Expression::Identifier(place)
                };

                // These will be the `elements` of our resulting tuple.
                let mut elements = Vec::with_capacity(tuple_true.elements.len());

                // Create an individual `ternary` for each tuple member and assign the
                // result to a new variable.
                for (i, (lhs, rhs, ty)) in
                    izip!(tuple_true.elements, tuple_false.elements, tuple_type.elements()).enumerate()
                {
                    let identifier = Identifier::new(
                        self.state.assigner.unique_symbol(format_args!("ternary_{i}"), "$$"),
                        self.state.node_builder.next_id(),
                    );

                    let expression: Expression = TernaryExpression {
                        condition: cond.clone(),
                        if_true: lhs,
                        if_false: rhs,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();

                    self.state.type_table.insert(identifier.id(), ty.clone());
                    self.state.type_table.insert(expression.id(), ty.clone());

                    let definition = self.state.assigner.simple_definition(
                        identifier,
                        expression,
                        self.state.node_builder.next_id(),
                    );

                    statements.push(definition);
                    elements.push(identifier.into());
                }

                let expr: Expression =
                    TupleExpression { elements, span: Default::default(), id: self.state.node_builder.next_id() }
                        .into();

                self.state.type_table.insert(expr.id(), Type::Tuple(tuple_type.clone()));

                (expr, statements)
            }
            (if_true, if_false) => {
                // This isn't a tuple. Just rebuild it and otherwise leave it alone.
                (TernaryExpression { if_true, if_false, ..input }.into(), statements)
            }
        }
    }
}

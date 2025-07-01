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

use crate::CompilerState;

use leo_ast::{
    AstReconstructor,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    Identifier,
    Node as _,
    Statement,
    TupleExpression,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct DestructuringVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// A mapping between variables and tuple elements.
    pub tuples: IndexMap<Symbol, Vec<Identifier>>,
    /// Whether or not we are currently traversing an async function block.
    pub is_async: bool,
}

impl DestructuringVisitor<'_> {
    /// Similar to `reconstruct_expression`, except that if `expression` is of tuple type, returns it as a tuple
    /// literal `(mem1, mem2, mem3, ...)`.
    pub fn reconstruct_expression_tuple(&mut self, expression: Expression) -> (Expression, Vec<Statement>) {
        let Type::Tuple(tuple_type) =
            self.state.type_table.get(&expression.id()).expect("Expressions should have types.")
        else {
            // It's not a tuple, so there's no more to do.
            return self.reconstruct_expression(expression);
        };

        let (new_expression, mut statements) = self.reconstruct_expression(expression);

        match new_expression {
            Expression::Identifier(identifier) => {
                // It's a variable name, so just get the member identifiers we've already made.
                let identifiers = self.tuples.get(&identifier.name).expect("Tuples should have been found");
                let elements: Vec<Expression> =
                    identifiers.iter().map(|identifier| Expression::Identifier(*identifier)).collect();

                let tuple: Expression =
                    TupleExpression { elements, span: Default::default(), id: self.state.node_builder.next_id() }
                        .into();

                self.state.type_table.insert(tuple.id(), Type::Tuple(tuple_type.clone()));

                (tuple, statements)
            }

            tuple @ Expression::Tuple(..) => {
                // It's already a tuple literal.
                (tuple, statements)
            }

            expr @ Expression::Call(..) => {
                // It's a call, so we'll need to make a new definition for the variables.
                let definition_stmt = self.assign_tuple(expr, Symbol::intern("destructure"));
                let Statement::Definition(DefinitionStatement {
                    place: DefinitionPlace::Multiple(identifiers), ..
                }) = &definition_stmt
                else {
                    panic!("`assign_tuple` always creates a definition with `Multiple`");
                };

                let elements = identifiers.iter().map(|identifier| Expression::Identifier(*identifier)).collect();

                let expr = Expression::Tuple(TupleExpression {
                    elements,
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                });

                self.state.type_table.insert(expr.id(), Type::Tuple(tuple_type.clone()));

                statements.push(definition_stmt);

                (expr, statements)
            }

            _ => panic!("Tuples may only be identifiers, tuple literals, or calls."),
        }
    }

    // Given the `expression` of tuple type, make a definition assigning variable to its members.
    //
    // That is, `let (mem1, mem2, mem3...) = expression;`
    pub fn assign_tuple(&mut self, expression: Expression, name: Symbol) -> Statement {
        let Type::Tuple(tuple_type) =
            self.state.type_table.get(&expression.id()).expect("Expressions should have types.")
        else {
            panic!("assign_tuple should only be called for tuple types.");
        };

        let new_identifiers: Vec<Identifier> = (0..tuple_type.length())
            .map(|i| {
                let new_symbol = self.state.assigner.unique_symbol(name, format_args!("#{i}#"));
                Identifier::new(new_symbol, self.state.node_builder.next_id())
            })
            .collect();

        Statement::Definition(DefinitionStatement {
            place: DefinitionPlace::Multiple(new_identifiers),
            type_: Some(Type::Tuple(tuple_type.clone())),
            value: expression,
            span: Default::default(),
            id: self.state.node_builder.next_id(),
        })
    }
}

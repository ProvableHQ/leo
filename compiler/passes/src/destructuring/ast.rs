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
use leo_ast::*;
use leo_span::Symbol;

use itertools::{Itertools, izip};

impl AstReconstructor for DestructuringVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /// Reconstructs a binary expression, expanding equality and inequality over
    /// tuples into elementwise comparisons. When both sides are tuples and the
    /// operator is `==` or `!=`, it generates per-element comparisons and folds
    /// them with AND/OR; otherwise the expression is rebuilt normally.
    ///
    /// Example: `(a, b) == (c, d)` → `(a == c) && (b == d)`
    /// Example: `(a, b, c) != (x, y, z)` → `(a != x) || (b != y) || (c != z)`
    fn reconstruct_binary(
        &mut self,
        input: BinaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (left, mut statements) = self.reconstruct_expression_tuple(input.left);
        let (right, statements2) = self.reconstruct_expression_tuple(input.right);
        statements.extend(statements2);

        use BinaryOperation::*;

        // Tuple equality / inequality expansion
        if let (Expression::Tuple(tuple_left), Expression::Tuple(tuple_right)) = (&left, &right)
            && matches!(input.op, Eq | Neq)
        {
            assert_eq!(tuple_left.elements.len(), tuple_right.elements.len());

            // Directly build elementwise (l OP r)
            let pieces: Vec<Expression> = tuple_left
                .elements
                .iter()
                .zip(&tuple_right.elements)
                .map(|(l, r)| {
                    let expr: Expression = BinaryExpression {
                        op: input.op,
                        left: l.clone(),
                        right: r.clone(),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();

                    self.state.type_table.insert(expr.id(), Type::Boolean);
                    expr
                })
                .collect();

            // Fold appropriately
            let op = match input.op {
                Eq => BinaryOperation::And,
                Neq => BinaryOperation::Or,
                _ => unreachable!(),
            };

            return (self.fold_with_op(op, pieces.into_iter()), statements);
        }

        // Fallback
        (BinaryExpression { op: input.op, left, right, ..input }.into(), Default::default())
    }

    /// Replaces a tuple access expression with the appropriate expression.
    fn reconstruct_tuple_access(
        &mut self,
        input: TupleAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let Expression::Path(path) = &input.tuple else {
            panic!("SSA guarantees that subexpressions are identifiers or literals.");
        };

        // Look up the expression in the tuple map.
        match self.tuples.get(&path.identifier().name).and_then(|tuple_names| tuple_names.get(input.index.value())) {
            Some(id) => (Path::from(*id).into_absolute().into(), Default::default()),
            None => {
                if !matches!(self.state.type_table.get(&path.id), Some(Type::Future(_))) {
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
                    ArrayAccess { array: path.clone().into(), index: index.into(), span: input.span, id: input.id }
                        .into();

                (expr, Default::default())
            }
        }
    }

    /// If this is a ternary expression on tuples of length `n`, we'll need to change it into
    /// `n` ternary expressions on the members.
    fn reconstruct_ternary(
        &mut self,
        mut input: TernaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (condition, mut statements) =
            self.reconstruct_expression(std::mem::take(&mut input.condition), &Default::default());
        let (if_true, statements2) = self.reconstruct_expression_tuple(std::mem::take(&mut input.if_true));
        statements.extend(statements2);
        let (if_false, statements3) = self.reconstruct_expression_tuple(std::mem::take(&mut input.if_false));
        statements.extend(statements3);

        match (if_true, if_false) {
            (Expression::Tuple(tuple_true), Expression::Tuple(tuple_false)) => {
                // Aleo's `ternary` opcode doesn't know about tuples, so we have to handle this.
                let Some(Type::Tuple(tuple_type)) = self.state.type_table.get(&tuple_true.id()) else {
                    panic!("Should have tuple type");
                };

                // We'll be reusing `condition`, so assign it to a variable.
                let cond = if let Expression::Path(..) = condition {
                    condition
                } else {
                    let place = Identifier::new(
                        self.state.assigner.unique_symbol("cond", "$$"),
                        self.state.node_builder.next_id(),
                    );

                    let definition =
                        self.state.assigner.simple_definition(place, condition, self.state.node_builder.next_id());

                    statements.push(definition);

                    self.state.type_table.insert(place.id(), Type::Boolean);

                    Expression::Path(Path::from(place).into_absolute())
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
                    elements.push(Path::from(identifier).into_absolute().into());
                }

                let expr: Expression =
                    TupleExpression { elements, span: Default::default(), id: self.state.node_builder.next_id() }
                        .into();

                self.state.type_table.insert(expr.id(), Type::Tuple(tuple_type.clone()));

                (expr, statements)
            }
            (if_true, if_false) => {
                // This isn't a tuple. Just rebuild it and otherwise leave it alone.
                (TernaryExpression { condition, if_true, if_false, ..input }.into(), statements)
            }
        }
    }

    /* Statements */
    /// `assert_eq` and `assert_neq` comparing tuples should be expanded to as many asserts as
    /// the length of each tuple.
    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        match input.variant {
            AssertVariant::Assert(expr) => {
                // Simple assert, just reconstruct the expression.
                let (expr, _) = self.reconstruct_expression(expr, &Default::default());
                (AssertStatement { variant: AssertVariant::Assert(expr), ..input }.into(), Default::default())
            }
            AssertVariant::AssertEq(ref left, ref right) | AssertVariant::AssertNeq(ref left, ref right) => {
                let (left, mut statements) = self.reconstruct_expression_tuple(left.clone());
                let (right, statements2) = self.reconstruct_expression_tuple(right.clone());
                statements.extend(statements2);

                match (&left, &right) {
                    (Expression::Tuple(tuple_left), Expression::Tuple(tuple_right)) => {
                        // Ensure the tuple lengths match
                        assert_eq!(tuple_left.elements.len(), tuple_right.elements.len());

                        for (l, r) in tuple_left.elements.iter().zip(&tuple_right.elements) {
                            let assert_variant = match input.variant {
                                AssertVariant::AssertEq(_, _) => AssertVariant::AssertEq(l.clone(), r.clone()),
                                AssertVariant::AssertNeq(_, _) => AssertVariant::AssertNeq(l.clone(), r.clone()),
                                _ => unreachable!(),
                            };

                            let stmt = AssertStatement { variant: assert_variant, ..input.clone() }.into();
                            statements.push(stmt);
                        }

                        // We don't need the original statement, just the ones we've created.
                        (Statement::dummy(), statements)
                    }
                    _ => {
                        // Not tuples, just keep the original assert
                        let variant = match input.variant {
                            AssertVariant::AssertEq(_, _) => AssertVariant::AssertEq(left, right),
                            AssertVariant::AssertNeq(_, _) => AssertVariant::AssertNeq(left, right),
                            _ => unreachable!(),
                        };
                        (AssertStatement { variant, ..input }.into(), Default::default())
                    }
                }
            }
        }
    }

    /// Modify assignments to tuples to become assignments to the corresponding variables.
    ///
    /// There are two cases we handle:
    /// 1. An assignment to a tuple x, like `x = rhs;`.
    ///    This we need to transform into individual assignments
    ///    `x_i = rhs_i;`
    ///    of the variables corresponding to members of `x` and `rhs`.
    /// 2. An assignment to a tuple member, like `x.2[i].member = rhs;`.
    ///    This we need to change into
    ///    `x_2[i].member = rhs;`
    ///    where `x_2` is the variable corresponding to `x.2`.
    fn reconstruct_assign(&mut self, mut assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(assign.value, &());

        if let Expression::Path(path) = &assign.place
            && let Type::Tuple(..) = self.state.type_table.get(&value.id()).expect("Expressions should have types.")
        {
            // This is the first case, assigning to a variable of tuple type.
            let identifiers = self.tuples.get(&path.identifier().name).expect("Tuple should have been encountered.");

            let Expression::Path(rhs) = value else {
                panic!("SSA should have ensured this is an identifier.");
            };

            let rhs_identifiers = self.tuples.get(&rhs.identifier().name).expect("Tuple should have been encountered.");

            // Again, make an assignment for each identifier.
            for (&identifier, &rhs_identifier) in identifiers.iter().zip_eq(rhs_identifiers) {
                let stmt = AssignStatement {
                    place: Path::from(identifier).into_absolute().into(),
                    value: Path::from(rhs_identifier).into_absolute().into(),
                    id: self.state.node_builder.next_id(),
                    span: Default::default(),
                }
                .into();

                statements.push(stmt);
            }

            // We don't need the original assignment, just the ones we've created.
            return (Statement::dummy(), statements);
        }

        // We need to check for case 2, so we loop and see if we find a tuple access.

        assign.value = value;
        let mut place = &mut assign.place;

        loop {
            // Loop through the places in the assignment to the top-level expression until an identifier or tuple access is reached.
            match place {
                Expression::TupleAccess(access) => {
                    // We're assigning to a tuple member, case 2 mentioned above.
                    let Expression::Path(path) = &access.tuple else {
                        panic!("SSA should have ensured this is an identifier.");
                    };

                    let tuple_ids =
                        self.tuples.get(&path.identifier().name).expect("Tuple should have been encountered.");

                    // This is the corresponding variable name of the member we're assigning to.
                    let identifier = tuple_ids[access.index.value()];

                    *place = Path::from(identifier).into_absolute().into();

                    return (assign.into(), statements);
                }

                Expression::ArrayAccess(access) => {
                    // We need to investigate the array, as maybe it's inside a tuple access, like `tupl.0[1u8]`.
                    place = &mut access.array;
                }

                Expression::MemberAccess(access) => {
                    // We need to investigate the struct, as maybe it's inside a tuple access, like `tupl.0.mem`.
                    place = &mut access.inner;
                }

                Expression::Path(..) => {
                    // There was no tuple access, so this is neither case 1 nor 2; there's nothing to do.
                    return (assign.into(), statements);
                }

                _ => panic!("Type checking should have prevented this."),
            }
        }
    }

    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Reconstruct the statements in the block, accumulating any additional statements.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            if !reconstructed_statement.is_empty() {
                statements.push(reconstructed_statement);
            }
        }

        (Block { statements, ..block }, Default::default())
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition, &());
        let (then, statements2) = self.reconstruct_block(input.then);
        statements.extend(statements2);
        let otherwise = input.otherwise.map(|oth| {
            let (expr, statements3) = self.reconstruct_statement(*oth);
            statements.extend(statements3);
            Box::new(expr)
        });
        (ConditionalStatement { condition, then, otherwise, ..input }.into(), statements)
    }

    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        use DefinitionPlace::*;

        let make_identifiers = |slf: &mut Self, single: Symbol, count: usize| -> Vec<Identifier> {
            (0..count)
                .map(|i| {
                    Identifier::new(
                        slf.state.assigner.unique_symbol(format_args!("{single}#tuple{i}"), "$"),
                        slf.state.node_builder.next_id(),
                    )
                })
                .collect()
        };

        let (value, mut statements) = self.reconstruct_expression(definition.value, &());
        let ty = self.state.type_table.get(&value.id()).expect("Expressions should have a type.");
        match (definition.place, value, ty) {
            (Single(identifier), Expression::Path(rhs), Type::Tuple(tuple_type)) => {
                // We need to give the members new names, in case they are assigned to.
                let identifiers = make_identifiers(self, identifier.name, tuple_type.length());

                let rhs_identifiers = self.tuples.get(&rhs.identifier().name).unwrap();

                for (identifier, rhs_identifier, ty) in izip!(&identifiers, rhs_identifiers, tuple_type.elements()) {
                    // Make a definition for each.
                    let stmt = DefinitionStatement {
                        place: Single(*identifier),
                        type_: Some(ty.clone()),
                        value: Expression::Path(Path::from(*rhs_identifier).into_absolute()),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);

                    // Put each into the type table.
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                // Put the identifier in `self.tuples`. We don't need to keep our definition.
                self.tuples.insert(identifier.name, identifiers);
                (Statement::dummy(), statements)
            }
            (Single(identifier), Expression::Tuple(tuple), Type::Tuple(tuple_type)) => {
                // Name each of the expressions on the right.
                let identifiers = make_identifiers(self, identifier.name, tuple_type.length());

                for (identifier, expr, ty) in izip!(&identifiers, tuple.elements, tuple_type.elements()) {
                    // Make a definition for each.
                    let stmt = DefinitionStatement {
                        place: Single(*identifier),
                        type_: Some(ty.clone()),
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);

                    // Put each into the type table.
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                // Put the identifier in `self.tuples`. We don't need to keep our definition.
                self.tuples.insert(identifier.name, identifiers);
                (Statement::dummy(), statements)
            }
            (Single(identifier), rhs @ Expression::Call(..), Type::Tuple(tuple_type)) => {
                let definition_stmt = self.assign_tuple(rhs, identifier.name);

                let Statement::Definition(DefinitionStatement {
                    place: DefinitionPlace::Multiple(identifiers), ..
                }) = &definition_stmt
                else {
                    panic!("assign_tuple creates `Multiple`.");
                };

                // Put it into `self.tuples`.
                self.tuples.insert(identifier.name, identifiers.clone());

                // Put each into the type table.
                for (identifier, ty) in identifiers.iter().zip(tuple_type.elements()) {
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                (definition_stmt, statements)
            }
            (Multiple(identifiers), Expression::Tuple(tuple), Type::Tuple(..)) => {
                // Just make a definition for each tuple element.
                for (identifier, expr) in identifiers.into_iter().zip_eq(tuple.elements) {
                    let stmt = DefinitionStatement {
                        place: Single(identifier),
                        type_: None,
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (Multiple(identifiers), Expression::Path(rhs), Type::Tuple(..)) => {
                // Again, make a definition for each tuple element.
                let rhs_identifiers =
                    self.tuples.get(&rhs.identifier().name).expect("We should have encountered this tuple by now");
                for (identifier, rhs_identifier) in identifiers.into_iter().zip_eq(rhs_identifiers.iter()) {
                    let stmt = DefinitionStatement {
                        place: Single(identifier),
                        type_: None,
                        value: Expression::Path(Path::from(*rhs_identifier).into_absolute()),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (m @ Multiple(..), value @ Expression::Call(..), Type::Tuple(..)) => {
                // Just reconstruct the statement.
                let stmt =
                    DefinitionStatement { place: m, type_: None, value, span: definition.span, id: definition.id }
                        .into();
                (stmt, statements)
            }
            (_, _, Type::Tuple(..)) => {
                panic!("Expressions of tuple type can only be tuple literals, identifiers, or calls.");
            }
            (s @ Single(..), rhs, _) => {
                // This isn't a tuple. Just build the definition again.
                let stmt = DefinitionStatement {
                    place: s,
                    type_: None,
                    value: rhs,
                    span: Default::default(),
                    id: definition.id,
                }
                .into();
                (stmt, statements)
            }
            (Multiple(_), _, _) => panic!("A definition with multiple identifiers must have tuple type"),
        }
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression_tuple(input.expression);
        (ReturnStatement { expression, ..input }.into(), statements)
    }
}

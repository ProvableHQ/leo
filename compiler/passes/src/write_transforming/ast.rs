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

use super::WriteTransformingVisitor;
use leo_ast::*;
use leo_span::Symbol;

impl WriteTransformingVisitor<'_> {
    pub fn get_array_member(&self, array_name: Symbol, index: &Expression) -> Option<Identifier> {
        let members = self.array_members.get(&array_name)?;
        let Expression::Literal(lit) = index else {
            panic!("Const propagation should have ensured this is a literal.");
        };
        let index = lit
            .as_u32()
            .expect("Const propagation should have ensured this is in range, and consequently a valid u32.")
            as usize;
        Some(members[index])
    }

    pub fn get_struct_member(&self, struct_name: Symbol, field_name: Symbol) -> Option<Identifier> {
        let members = self.struct_members.get(&struct_name)?;
        members.get(&field_name).cloned()
    }
}

impl WriteTransformingVisitor<'_> {
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Vec<Statement>) {
        let ty = self.state.type_table.get(&input.id()).unwrap();
        let mut statements = Vec::new();
        if let Some(array_members) = self.array_members.get(&input.name) {
            // Build the array expression from the members.
            let id = self.state.node_builder.next_id();
            self.state.type_table.insert(id, ty.clone());
            let expr = ArrayExpression {
                elements: array_members
                    // This clone is unfortunate, but both `array_members` and the closure below borrow self.
                    .clone()
                    .iter()
                    .map(|identifier| {
                        let (expr, statements2) = self.reconstruct_identifier(*identifier);
                        statements.extend(statements2);
                        expr
                    })
                    .collect(),
                span: Default::default(),
                id,
            };
            let statement = AssignStatement {
                place: Path::from(input).into_absolute().into(),
                value: expr.into(),
                span: Default::default(),
                id: self.state.node_builder.next_id(),
            };
            statements.push(statement.into());
            (Path::from(input).into_absolute().into(), statements)
        } else if let Some(struct_members) = self.struct_members.get(&input.name) {
            // Build the struct expression from the members.
            let id = self.state.node_builder.next_id();
            self.state.type_table.insert(id, ty.clone());
            let Type::Composite(comp_type) = ty else {
                panic!("The type of a struct init should be a composite.");
            };
            let expr = StructExpression {
                const_arguments: Vec::new(), // All const arguments should have been resolved by now
                members: struct_members
                    // This clone is unfortunate, but both `struct_members` and the closure below borrow self.
                    .clone()
                    .iter()
                    .map(|(field_name, ident)| {
                        let (expr, statements2) = self.reconstruct_identifier(*ident);
                        statements.extend(statements2);
                        StructVariableInitializer {
                            identifier: Identifier::new(*field_name, self.state.node_builder.next_id()),
                            expression: Some(expr),
                            span: Default::default(),
                            id: self.state.node_builder.next_id(),
                        }
                    })
                    .collect(),
                path: comp_type.path,
                span: Default::default(),
                id,
            };
            let statement = AssignStatement {
                place: Path::from(input).into_absolute().into(),
                value: expr.into(),
                span: Default::default(),
                id: self.state.node_builder.next_id(),
            };
            statements.push(statement.into());
            (Path::from(input).into_absolute().into(), statements)
        } else {
            // This is not a struct or array whose members are written to, so there's nothing to do.
            (Path::from(input).into_absolute().into(), Default::default())
        }
    }
}

impl AstReconstructor for WriteTransformingVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /* Expressions */
    fn reconstruct_path(&mut self, input: Path, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        if input.qualifier().is_empty() {
            self.reconstruct_identifier(Identifier { name: input.identifier().name, span: input.span, id: input.id })
        } else {
            (input.into(), Default::default())
        }
    }

    fn reconstruct_array_access(
        &mut self,
        input: ArrayAccess,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let Expression::Path(ref array_name) = input.array else {
            panic!("SSA ensures that this is a Path.");
        };
        if let Some(member) = self.get_array_member(array_name.identifier().name, &input.index) {
            self.reconstruct_identifier(member)
        } else {
            (input.into(), Default::default())
        }
    }

    fn reconstruct_member_access(
        &mut self,
        input: MemberAccess,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let Expression::Path(ref struct_name) = input.inner else {
            panic!("SSA ensures that this is a Path.");
        };
        if let Some(member) = self.get_struct_member(struct_name.identifier().name, input.name.name) {
            self.reconstruct_identifier(member)
        } else {
            (input.into(), Default::default())
        }
    }

    // The rest of the methods below don't do anything but traverse - we only modify their default implementations
    // to combine the `Vec<Statement>` outputs.

    fn reconstruct_associated_function(
        &mut self,
        mut input: AssociatedFunctionExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for arg in input.arguments.iter_mut() {
            let (expr, statements2) = self.reconstruct_expression(std::mem::take(arg), &());
            statements.extend(statements2);
            *arg = expr;
        }
        (input.into(), statements)
    }

    fn reconstruct_tuple_access(
        &mut self,
        _input: TupleAccess,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        panic!("`TupleAccess` should not be in the AST at this point.");
    }

    fn reconstruct_array(
        &mut self,
        mut input: ArrayExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for element in input.elements.iter_mut() {
            let (expr, statements2) = self.reconstruct_expression(std::mem::take(element), &());
            statements.extend(statements2);
            *element = expr;
        }
        (input.into(), statements)
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        let (left, mut statements) = self.reconstruct_expression(input.left, &());
        let (right, statements2) = self.reconstruct_expression(input.right, &());
        statements.extend(statements2);
        (BinaryExpression { left, right, ..input }.into(), statements)
    }

    fn reconstruct_call(&mut self, mut input: CallExpression, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for arg in input.arguments.iter_mut() {
            let (expr, statements2) = self.reconstruct_expression(std::mem::take(arg), &());
            statements.extend(statements2);
            *arg = expr;
        }
        (input.into(), statements)
    }

    fn reconstruct_cast(&mut self, input: CastExpression, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression, &());
        (CastExpression { expression, ..input }.into(), statements)
    }

    fn reconstruct_struct_init(
        &mut self,
        mut input: StructExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for member in input.members.iter_mut() {
            assert!(member.expression.is_some());
            let (expr, statements2) = self.reconstruct_expression(member.expression.take().unwrap(), &());
            statements.extend(statements2);
            member.expression = Some(expr);
        }

        (input.into(), statements)
    }

    fn reconstruct_err(
        &mut self,
        _input: leo_ast::ErrExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        std::panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_literal(
        &mut self,
        input: leo_ast::Literal,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition, &());
        let (if_true, statements2) = self.reconstruct_expression(input.if_true, &());
        let (if_false, statements3) = self.reconstruct_expression(input.if_false, &());
        statements.extend(statements2);
        statements.extend(statements3);
        (TernaryExpression { condition, if_true, if_false, ..input }.into(), statements)
    }

    fn reconstruct_tuple(
        &mut self,
        input: leo_ast::TupleExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // This should ony appear in a return statement.
        let mut statements = Vec::new();
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (expr, statements2) = self.reconstruct_expression(element, &());
                statements.extend(statements2);
                expr
            })
            .collect();
        (TupleExpression { elements, ..input }.into(), statements)
    }

    fn reconstruct_unary(
        &mut self,
        input: leo_ast::UnaryExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (receiver, statements) = self.reconstruct_expression(input.receiver, &());
        (UnaryExpression { receiver, ..input }.into(), statements)
    }

    fn reconstruct_unit(
        &mut self,
        input: leo_ast::UnitExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    /* Statements */
    /// This is the only reconstructing function where we do anything other than traverse and combine statements,
    /// by calling `reconstruct_assign_place` and `reconstruct_assign_recurse`.
    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value, &());
        let place = self.reconstruct_assign_place(input.place);
        self.reconstruct_assign_recurse(place, value, &mut statements);
        (Statement::dummy(), statements)
    }

    fn reconstruct_assert(&mut self, input: leo_ast::AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        let stmt = AssertStatement {
            variant: match input.variant {
                AssertVariant::Assert(expr) => {
                    let (expr, statements2) = self.reconstruct_expression(expr, &());
                    statements.extend(statements2);
                    AssertVariant::Assert(expr)
                }
                AssertVariant::AssertEq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left, &());
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right, &());
                    statements.extend(statements3);
                    AssertVariant::AssertEq(left, right)
                }
                AssertVariant::AssertNeq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left, &());
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right, &());
                    statements.extend(statements3);
                    AssertVariant::AssertNeq(left, right)
                }
            },
            ..input
        }
        .into();
        (stmt, Default::default())
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

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value, &());
        input.value = value;
        match input.place.clone() {
            DefinitionPlace::Single(identifier) => {
                statements.push(input.into());
                self.define_variable_members(identifier, &mut statements);
            }
            DefinitionPlace::Multiple(identifiers) => {
                statements.push(input.into());
                for &identifier in identifiers.iter() {
                    self.define_variable_members(identifier, &mut statements);
                }
            }
        }
        (Statement::dummy(), statements)
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression, &());
        (ExpressionStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this point.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression, &());
        (ReturnStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_conditional(&mut self, input: leo_ast::ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
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
}

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

use leo_ast::{
    ArrayAccess,
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    ExpressionStatement,
    Identifier,
    IntegerType,
    IterationStatement,
    Literal,
    MemberAccess,
    Node,
    ReturnStatement,
    Statement,
    StatementReconstructor,
    Type,
};

impl WriteTransformingVisitor<'_> {
    /// If `name` is a struct or array whose members are written to, make
    /// `DefinitionStatement`s for each of its variables that will correspond to
    /// the members. Note that we create them for all members; unnecessary ones
    /// will be removed by DCE.
    pub fn define_variable_members(&mut self, name: Identifier, accumulate: &mut Vec<Statement>) {
        // The `cloned` here and in the branch below are unfortunate but we need
        // to mutably borrow `self` again below.
        if let Some(members) = self.array_members.get(&name.name).cloned() {
            for (i, member) in members.iter().cloned().enumerate() {
                // Create a definition for each array index.
                let index = Literal::integer(
                    IntegerType::U8,
                    i.to_string(),
                    Default::default(),
                    self.state.node_builder.next_id(),
                );
                self.state.type_table.insert(index.id(), Type::Integer(IntegerType::U32));
                let access = ArrayAccess {
                    array: name.into(),
                    index: index.into(),
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(access.id(), self.state.type_table.get(&member.id()).unwrap().clone());
                let def = DefinitionStatement {
                    place: DefinitionPlace::Single(member),
                    type_: None,
                    value: access.into(),
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                };
                accumulate.push(def.into());
                // And recurse - maybe its members are also written to.
                self.define_variable_members(member, accumulate);
            }
        } else if let Some(members) = self.struct_members.get(&name.name) {
            for (&field_name, &member) in members.clone().iter() {
                // Create a definition for each field.
                let access = MemberAccess {
                    inner: name.into(),
                    name: Identifier::new(field_name, self.state.node_builder.next_id()),
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(access.id(), self.state.type_table.get(&member.id()).unwrap().clone());
                let def = DefinitionStatement {
                    place: DefinitionPlace::Single(member),
                    type_: None,
                    value: access.into(),
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                };
                accumulate.push(def.into());
                // And recurse - maybe its members are also written to.
                self.define_variable_members(member, accumulate);
            }
        }
    }
}

impl WriteTransformingVisitor<'_> {
    /// If we're assigning to a struct or array member, find the variable name we're actually writing to,
    /// recursively if necessary.
    /// That is, if we have
    /// `arr[0u32][1u32] = ...`,
    /// we find the corresponding variable `arr_0_1`.
    pub fn reconstruct_assign_place(&mut self, input: Expression) -> Identifier {
        use Expression::*;
        match input {
            ArrayAccess(array_access) => {
                let identifier = self.reconstruct_assign_place(array_access.array);
                self.get_array_member(identifier.name, &array_access.index).expect("We have visited all array writes.")
            }
            Identifier(identifier) => identifier,
            MemberAccess(member_access) => {
                let identifier = self.reconstruct_assign_place(member_access.inner);
                self.get_struct_member(identifier.name, member_access.name.name)
                    .expect("We have visited all struct writes.")
            }
            TupleAccess(_) => panic!("TupleAccess writes should have been removed by Destructuring"),
            _ => panic!("Type checking should have ensured there are no other places for assignments"),
        }
    }

    /// If we're assigning to a struct or array, create assignments to the individual members, if applicable.
    fn reconstruct_assign_recurse(&self, place: Identifier, value: Expression, accumulate: &mut Vec<Statement>) {
        if let Some(array_members) = self.array_members.get(&place.name) {
            if let Expression::Array(value_array) = value {
                // This was an assignment like
                // `arr = [a, b, c];`
                // Change it to this:
                // `arr_0 = a; arr_1 = b; arr_2 = c`
                for (&member, rhs_element) in array_members.iter().zip(value_array.elements) {
                    self.reconstruct_assign_recurse(member, rhs_element, accumulate);
                }
            } else {
                // This was an assignment like
                // `arr = x;`
                // Change it to this:
                // `arr = x; arr_0 = x[0]; arr_1 = x[1]; arr_2 = x[2];`
                let one_assign = AssignStatement {
                    place: place.into(),
                    value,
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                }
                .into();
                accumulate.push(one_assign);
                for (i, &member) in array_members.iter().enumerate() {
                    let access = ArrayAccess {
                        array: place.into(),
                        index: Literal::integer(
                            IntegerType::U32,
                            format!("{i}u32"),
                            Default::default(),
                            self.state.node_builder.next_id(),
                        )
                        .into(),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    };
                    self.reconstruct_assign_recurse(member, access.into(), accumulate);
                }
            }
        } else if let Some(struct_members) = self.struct_members.get(&place.name) {
            if let Expression::Struct(value_struct) = value {
                // This was an assignment like
                // `struc = S { field0: a, field1: b };`
                // Change it to this:
                // `struc_field0 = a; struc_field1 = b;`
                for initializer in value_struct.members.into_iter() {
                    let member_name = struct_members.get(&initializer.identifier.name).expect("Member should exist.");
                    let rhs_expression =
                        initializer.expression.expect("This should have been normalized to have a value.");
                    self.reconstruct_assign_recurse(*member_name, rhs_expression, accumulate);
                }
            } else {
                // This was an assignment like
                // `struc = x;`
                // Change it to this:
                // `struc = x; struc_field0 = x.field0; struc_field1 = x.field1;`
                let one_assign = AssignStatement {
                    place: place.into(),
                    value,
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                }
                .into();
                accumulate.push(one_assign);
                for (field, member_name) in struct_members.iter() {
                    let access = MemberAccess {
                        inner: place.into(),
                        name: Identifier::new(*field, self.state.node_builder.next_id()),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    };
                    self.reconstruct_assign_recurse(*member_name, access.into(), accumulate);
                }
            }
        } else {
            let stmt = AssignStatement {
                value,
                place: place.into(),
                id: self.state.node_builder.next_id(),
                span: Default::default(),
            }
            .into();
            accumulate.push(stmt);
        }
    }
}

impl StatementReconstructor for WriteTransformingVisitor<'_> {
    /// This is the only reconstructing function where we do anything other than traverse and combine statements,
    /// by calling `reconstruct_assign_place` and `reconstruct_assign_recurse`.
    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value);
        let place = self.reconstruct_assign_place(input.place);
        self.reconstruct_assign_recurse(place, value, &mut statements);
        (Statement::dummy(), statements)
    }

    fn reconstruct_assert(&mut self, input: leo_ast::AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        let stmt = AssertStatement {
            variant: match input.variant {
                AssertVariant::Assert(expr) => {
                    let (expr, statements2) = self.reconstruct_expression(expr);
                    statements.extend(statements2);
                    AssertVariant::Assert(expr)
                }
                AssertVariant::AssertEq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left);
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right);
                    statements.extend(statements3);
                    AssertVariant::AssertEq(left, right)
                }
                AssertVariant::AssertNeq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left);
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right);
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
        let (value, mut statements) = self.reconstruct_expression(input.value);
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
        let (expression, statements) = self.reconstruct_expression(input.expression);
        (ExpressionStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this point.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression);
        (ReturnStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_conditional(&mut self, input: leo_ast::ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition);
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

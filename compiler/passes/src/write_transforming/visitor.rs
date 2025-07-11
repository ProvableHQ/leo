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
    ArrayAccess,
    AssignStatement,
    AstVisitor,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    Identifier,
    IntegerType,
    Literal,
    Location,
    MemberAccess,
    Node as _,
    Program,
    Statement,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexMap;

/// This visitor associates a variable for each member of a struct or array that is
/// written to. Whenever a member of the struct or array is written to, we change the
/// assignment to access the variable instead. Whenever the struct or array itself
/// is accessed, we first rebuild the struct or array from its variables.
pub struct WriteTransformingVisitor<'a> {
    pub state: &'a mut CompilerState,

    /// For any struct whose members are written to, a map of its field names to variables
    /// corresponding to the members.
    pub struct_members: IndexMap<Symbol, IndexMap<Symbol, Identifier>>,

    /// For any array whose members are written to, a vec containing the variables for each index.
    pub array_members: IndexMap<Symbol, Vec<Identifier>>,

    pub program: Symbol,
}

impl<'a> WriteTransformingVisitor<'a> {
    pub fn new(state: &'a mut CompilerState, program: &Program) -> Self {
        let visitor = WriteTransformingVisitor {
            state,
            struct_members: Default::default(),
            array_members: Default::default(),
            program: Symbol::intern(""),
        };

        // We need to do an initial pass through the AST to identify all arrays and structs that are written to.
        let mut wtf = WriteTransformingFiller(visitor);
        wtf.fill(program);
        wtf.0
    }

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
    pub fn reconstruct_assign_recurse(&self, place: Identifier, value: Expression, accumulate: &mut Vec<Statement>) {
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

struct WriteTransformingFiller<'a>(WriteTransformingVisitor<'a>);

impl AstVisitor for WriteTransformingFiller<'_> {
    type AdditionalInput = ();
    type Output = ();

    /* Expressions */
    fn visit_expression(&mut self, _input: &Expression, _additional: &Self::AdditionalInput) -> Self::Output {}

    /* Statements */
    fn visit_assign(&mut self, input: &AssignStatement) {
        self.access_recurse(&input.place);
    }
}

impl WriteTransformingFiller<'_> {
    fn fill(&mut self, program: &Program) {
        for (_, scope) in program.program_scopes.iter() {
            for (_, function) in scope.functions.iter() {
                self.0.program = scope.program_id.name.name;
                self.visit_block(&function.block);
            }
        }
    }

    /// Find assignments to arrays and structs and populate `array_members` and `struct_members` with new
    /// variables names.
    fn access_recurse(&mut self, place: &Expression) -> Identifier {
        match place {
            Expression::Identifier(identifier) => *identifier,
            Expression::ArrayAccess(array_access) => {
                let array_name = self.access_recurse(&array_access.array);
                let members = self.0.array_members.entry(array_name.name).or_insert_with(|| {
                    let ty = self.0.state.type_table.get(&array_access.array.id()).unwrap();
                    let Type::Array(arr) = ty else { panic!("Type checking should have prevented this.") };
                    (0..arr.length.as_u32().expect("length should be known at this point"))
                        .map(|i| {
                            let id = self.0.state.node_builder.next_id();
                            let symbol = self.0.state.assigner.unique_symbol(format_args!("{array_name}#{i}"), "$");
                            self.0.state.type_table.insert(id, arr.element_type().clone());
                            Identifier::new(symbol, id)
                        })
                        .collect()
                });
                let Expression::Literal(lit) = &array_access.index else {
                    panic!("Const propagation should have ensured this is a literal.");
                };
                members[lit
                    .as_u32()
                    .expect("Const propagation should have ensured this is in range, and consequently a valid u32.")
                    as usize]
            }
            Expression::MemberAccess(member_access) => {
                let struct_name = self.access_recurse(&member_access.inner);
                let members = self.0.struct_members.entry(struct_name.name).or_insert_with(|| {
                    let ty = self.0.state.type_table.get(&member_access.inner.id()).unwrap();
                    let Type::Composite(comp) = ty else {
                        panic!("Type checking should have prevented this.");
                    };
                    let struct_ = self
                        .0
                        .state
                        .symbol_table
                        .lookup_struct(comp.id.name)
                        .or_else(|| {
                            self.0
                                .state
                                .symbol_table
                                .lookup_record(Location::new(comp.program.unwrap_or(self.0.program), comp.id.name))
                        })
                        .unwrap();
                    struct_
                        .members
                        .iter()
                        .map(|member| {
                            let name = member.name();
                            let id = self.0.state.node_builder.next_id();
                            let symbol = self.0.state.assigner.unique_symbol(format_args!("{struct_name}#{name}"), "$");
                            self.0.state.type_table.insert(id, member.type_.clone());
                            (member.name(), Identifier::new(symbol, id))
                        })
                        .collect()
                });
                *members.get(&member_access.name.name).expect("Type checking should have ensured this is valid.")
            }
            Expression::TupleAccess(_) => panic!("TupleAccess writes should have been removed by Destructuring"),
            _ => panic!("Type checking should have ensured there are no other places for assignments"),
        }
    }
}

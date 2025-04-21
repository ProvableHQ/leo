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
    AssignStatement,
    Expression,
    ExpressionVisitor,
    Identifier,
    Location,
    Node as _,
    Program,
    StatementVisitor,
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
}

struct WriteTransformingFiller<'a>(WriteTransformingVisitor<'a>);

// We don't actually need to visit expressions here; we're only implementing
// `ExpressionVisitor` because `StatementVisitor` requires it.
impl ExpressionVisitor for WriteTransformingFiller<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_expression(&mut self, _input: &Expression, _additional: &Self::AdditionalInput) -> Self::Output {}
}

// All we actually need is `visit_assign`; we're just using `StatementVisitor`'s
// default traversal.
impl StatementVisitor for WriteTransformingFiller<'_> {
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
                    (0..arr.length())
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

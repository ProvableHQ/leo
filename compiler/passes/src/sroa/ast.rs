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

use super::ScalarReplacementOfAggregatesVisitor;

use leo_ast::*;

impl AstReconstructor for ScalarReplacementOfAggregatesVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        if let DefinitionPlace::Single(id) = input.place {
            match &input.value {
                Expression::Struct(struct_expr) => {
                    // Direct struct initializer — record it
                    self.struct_defs.insert(id.name, struct_expr.clone());
                }
                Expression::Path(path) => {
                    // Is this a direct alias to another struct var?
                    let source_name = path.identifier().name;
                    if let Some(struct_expr) = self.struct_defs.get(&source_name) {
                        // Propagate struct definition info to this alias
                        self.struct_defs.insert(id.name, struct_expr.clone());
                    }
                }
                _ => { /* Do nothing */ }
            }
        }

        (
            DefinitionStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                value: self.reconstruct_expression(input.value, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_member_access(
        &mut self,
        input: MemberAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        use leo_ast::{Expression, Path};

        // Step 1: Check if `input.inner` is a simple path like `s`
        if let Expression::Path(ref path) = input.inner {
            let var_name = path.identifier();
            // Step 2: See if this variable `s` was previously assigned to a struct literal
            if let Some(struct_expr) = self.struct_defs.get(&var_name.name) {
                // Step 3: Find the corresponding field
                if let Some(field) = struct_expr.members.iter().find(|member| member.identifier.name == input.name.name)
                {
                    let replaced_expr = if let Some(expr) = &field.expression {
                        expr.clone()
                    } else {
                        // Field was written as `baz`, so desugar to `Expression::Path("baz")`
                        let field_name = field.identifier.name;
                        let field_ident = Identifier::new(field_name, field.id);
                        Expression::Path(Path::from(field_ident))
                    };

                    return (replaced_expr, Default::default());
                }
            }
        }

        // Fallback: Keep the member access as-is
        (Expression::MemberAccess(Box::new(input)), Default::default())
    }
}

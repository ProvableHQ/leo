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

use super::PathResolutionVisitor;
use leo_ast::{
    AstReconstructor,
    CallExpression,
    CompositeExpression,
    CompositeFieldInitializer,
    CompositeType,
    ErrExpression,
    Expression,
    Path,
    Type,
};

impl AstReconstructor for PathResolutionVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_composite_type(&mut self, mut input: CompositeType) -> (Type, Self::AdditionalOutput) {
        if !input.path.is_resolved() {
            input.path = input.path.with_module_prefix(self.program, self.module.clone());
        }
        (
            Type::Composite(CompositeType {
                path: input.path,
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &()).0)
                    .collect(),
                ..input
            }),
            Default::default(),
        )
    }

    fn reconstruct_err(&mut self, input: ErrExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        (input.into(), ())
    }

    fn reconstruct_call(
        &mut self,
        mut input: CallExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        if !input.function.is_resolved() {
            input.function = input.function.with_module_prefix(self.program, self.module.clone());
        }
        (
            CallExpression {
                function: input.function,
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &()).0)
                    .collect(),
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_composite_init(
        &mut self,
        mut input: CompositeExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        if !input.path.is_resolved() {
            input.path = input.path.with_module_prefix(self.program, self.module.clone());
        }
        (
            CompositeExpression {
                path: input.path,
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &()).0)
                    .collect(),
                members: input
                    .members
                    .into_iter()
                    .map(|member| CompositeFieldInitializer {
                        identifier: member.identifier,
                        expression: member.expression.map(|expr| self.reconstruct_expression(expr, &()).0),
                        span: member.span,
                        id: member.id,
                    })
                    .collect(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_path(&mut self, mut input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        debug_assert!(!input.is_resolved(), "reconstruct_path expects an unresolved Path");

        let has_qualifier = !input.qualifier().is_empty();

        // Case 1: Cannot be local → must be global
        if has_qualifier {
            input = input.with_module_prefix(self.program, self.module.clone());
        }
        // Case 2: Could be local → check globals first
        else {
            // Build a tentative global location in the current module
            let potentially_global = input.clone().with_module_prefix(self.program, self.module.clone());

            if let Some(_) = self.state.symbol_table.lookup_global(potentially_global.expect_global()) {
                // It exists as a global → resolve as global
                dbg!("resolved global:", &potentially_global);
                input = potentially_global;
            } else {
                // Not a global → resolve as local
                let name = input.identifier().name;
                input = input.with_local(name);
            }
        }

        (input.into(), Default::default())
    }
}

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
    CompositeType,
    ErrExpression,
    Expression,
    Path,
    StructExpression,
    StructVariableInitializer,
    Type,
};

impl AstReconstructor for PathResolutionVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_composite_type(&mut self, mut input: CompositeType) -> (Type, Self::AdditionalOutput) {
        if input.path.try_absolute_path().is_none() {
            input.path = input.path.with_module_prefix(&self.module);
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
        if input.function.try_absolute_path().is_none() {
            input.function = input.function.with_module_prefix(&self.module);
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

    fn reconstruct_struct_init(
        &mut self,
        mut input: StructExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        if input.path.try_absolute_path().is_none() {
            input.path = input.path.with_module_prefix(&self.module)
        }
        (
            StructExpression {
                path: input.path,
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &()).0)
                    .collect(),
                members: input
                    .members
                    .into_iter()
                    .map(|member| StructVariableInitializer {
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
        // Because some paths may be paths to global consts, we have to prefix all paths at this
        // stage because we don't have semantic information just yet.
        if input.try_absolute_path().is_none() {
            input = input.with_module_prefix(&self.module);
        }
        (input.into(), Default::default())
    }
}

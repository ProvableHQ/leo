// Copyright (C) 2019-2026 Provable Inc.
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
use crate::{VariableSymbol, VariableType};
use leo_ast::{
    AstReconstructor,
    Block,
    CallExpression,
    CompositeExpression,
    CompositeFieldInitializer,
    CompositeType,
    ConstDeclaration,
    DefinitionPlace,
    DefinitionStatement,
    ErrExpression,
    Expression,
    IterationStatement,
    Path,
    Statement,
    Type,
};
use leo_errors::TypeCheckerError;

impl AstReconstructor for PathResolutionVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_composite_type(&mut self, mut input: CompositeType) -> (Type, Self::AdditionalOutput) {
        if !input.path.is_resolved() {
            input.path = input.path.resolve_as_global_in_module(self.program, self.module.clone());
        }
        (
            Type::Composite(CompositeType {
                path: input.path,
                const_arguments: input
                    .const_arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg, &()).0)
                    .collect(),
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
            input.function = input.function.resolve_as_global_in_module(self.program, self.module.clone());
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
            input.path = input.path.resolve_as_global_in_module(self.program, self.module.clone());
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
        let has_qualifier = !input.qualifier().is_empty();

        if has_qualifier {
            // paths with qualifiers must refer to a global.
            input = input.resolve_as_global_in_module(self.program, self.module.clone());
        } else {
            let potentially_global = input.clone().resolve_as_global_in_module(self.program, self.module.clone());

            if self
                .state
                .symbol_table
                .lookup_global(self.program, potentially_global.expect_global_location())
                .is_some()
            {
                // If already inserted as a global variable in the symbol table, just resolve it to global.
                input = potentially_global;
            } else if self.state.symbol_table.lookup_local(input.identifier().name).is_some() {
                // If already inserted as a local variable in the symbol table, just resolve it to local
                input = input.to_local();
            } else {
                // Otherwise, unknown path.
                self.state.handler.emit_err(TypeCheckerError::unknown_sym("variable", input.clone(), input.span()));
            }
        }

        // Convert to Expression BEFORE consuming input
        let expr: Expression = input.clone().into();
        (expr, Default::default())
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let reconstructed_type = self.reconstruct_type(input.type_).0;
        let reconstructed_value = self.reconstruct_expression(input.value, &Default::default()).0;

        // Are we in a global scope? If not, then this is a local `const`. Insert it as a local in
        // the symbol table.
        if !self.state.symbol_table.global_scope()
            && let Err(err) =
                self.state.symbol_table.insert_variable(self.program, &[input.place.name], VariableSymbol {
                    type_: None,
                    span: input.place.span,
                    declaration: VariableType::Const,
                })
        {
            self.state.handler.emit_err(err);
        }

        (ConstDeclaration { type_: reconstructed_type, value: reconstructed_value, ..input }.into(), Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let reconstructed_type = input.type_.map(|ty| self.reconstruct_type(ty).0);
        let reconstructed_value = self.reconstruct_expression(input.value, &Default::default()).0;

        match &input.place {
            DefinitionPlace::Single(identifier) => {
                if let Err(err) =
                    self.state.symbol_table.insert_variable(self.program, &[identifier.name], VariableSymbol {
                        type_: None,
                        span: identifier.span,
                        declaration: VariableType::Mut,
                    })
                {
                    self.state.handler.emit_err(err);
                }
            }
            DefinitionPlace::Multiple(identifiers) => {
                // Now just insert each tuple element as a separate variable
                for identifier in identifiers.iter() {
                    if let Err(err) =
                        self.state.symbol_table.insert_variable(self.program, &[identifier.name], VariableSymbol {
                            type_: None,
                            span: identifier.span,
                            declaration: VariableType::Mut,
                        })
                    {
                        self.state.handler.emit_err(err);
                    }
                }
            }
        }

        (
            DefinitionStatement { type_: reconstructed_type, value: reconstructed_value, ..input }.into(),
            Default::default(),
        )
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id, |slf| {
            (
                Block {
                    statements: input.statements.into_iter().map(|s| slf.reconstruct_statement(s).0).collect(),
                    span: input.span,
                    id: input.id,
                },
                Default::default(),
            )
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let reconstructed_type = input.type_.map(|ty| self.reconstruct_type(ty).0);
        let reconstructed_start = self.reconstruct_expression(input.start, &Default::default()).0;
        let reconstructed_stop = self.reconstruct_expression(input.stop, &Default::default()).0;

        self.in_scope(input.id, |slf| {
            // Insert the iterator into the symbol table.
            if let Err(err) =
                slf.state.symbol_table.insert_variable(slf.program, &[input.variable.name], VariableSymbol {
                    type_: None,
                    span: input.variable.span,
                    declaration: VariableType::Const,
                })
            {
                slf.state.handler.emit_err(err);
            }

            (
                IterationStatement {
                    type_: reconstructed_type,
                    start: reconstructed_start,
                    stop: reconstructed_stop,
                    block: slf.reconstruct_block(input.block).0,
                    ..input
                }
                .into(),
                Default::default(),
            )
        })
    }
}

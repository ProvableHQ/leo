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

use super::StorageReadForwardingVisitor;

use leo_ast::*;

impl AstReconstructor for StorageReadForwardingVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_path(&mut self, input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        if let Some(alias) = input.try_local_symbol().and_then(|name| self.local_alias(name)) {
            let ty = self.state.type_table.get(&input.id());
            let path = Path::from(Identifier::new(alias, self.state.node_builder.next_id())).to_local();
            if let Some(ty) = ty {
                self.state.type_table.insert(path.id(), ty);
            }
            (path.into(), ())
        } else {
            (input.into(), ())
        }
    }

    fn reconstruct_intrinsic(
        &mut self,
        mut input: IntrinsicExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        input.arguments = input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect();
        if Self::is_effect_boundary(&input) {
            self.clear_reads();
        }
        (input.into(), ())
    }

    fn reconstruct_call(
        &mut self,
        mut input: CallExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        input.arguments = input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect();
        self.clear_reads();
        (input.into(), ())
    }

    fn reconstruct_dynamic_op(
        &mut self,
        mut input: DynamicOpExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        input.interface = self.reconstruct_type(input.interface).0;
        input.target_program = self.reconstruct_expression(input.target_program, &()).0;
        input.network = input.network.map(|network| self.reconstruct_expression(network, &()).0);
        match &mut input.kind {
            DynamicOpKind::Call { arguments, .. } | DynamicOpKind::Op { arguments, .. } => {
                *arguments =
                    std::mem::take(arguments).into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect();
            }
            DynamicOpKind::Read { .. } => {}
        }
        self.clear_reads();
        (input.into(), ())
    }

    fn reconstruct_async(
        &mut self,
        mut input: AsyncExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let previous_context = self.in_finalize_context;
        self.in_finalize_context = true;
        self.clear_function_state();
        input.block = self.reconstruct_block(input.block).0;
        self.clear_function_state();
        self.in_finalize_context = previous_context;
        (input.into(), ())
    }

    fn reconstruct_conditional(&mut self, mut input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        input.condition = self.reconstruct_expression(input.condition, &()).0;
        let join_condition = input.condition.clone();

        let aliases = self.aliases.clone();
        self.clear_reads();
        self.clear_join_aliases();
        self.aliases = aliases.clone();
        input.then = self.reconstruct_block(input.then).0;
        let then_aliases = self.aliases.clone();

        self.clear_reads();
        self.clear_join_aliases();
        self.aliases = aliases.clone();
        input.otherwise = input.otherwise.map(|statement| Box::new(self.reconstruct_statement(*statement).0));
        let otherwise_aliases = self.aliases.clone();

        self.clear_reads();
        self.aliases = aliases;

        self.then_join_aliases = then_aliases
            .into_iter()
            .filter(|(alias, target)| self.aliases.get(alias).copied() != Some(*target))
            .collect();
        self.otherwise_join_aliases = otherwise_aliases
            .into_iter()
            .filter(|(alias, target)| self.aliases.get(alias).copied() != Some(*target))
            .collect();
        self.join_condition =
            (!self.then_join_aliases.is_empty() || !self.otherwise_join_aliases.is_empty()).then_some(join_condition);

        (input.into(), ())
    }

    fn reconstruct_ternary(
        &mut self,
        mut input: TernaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        if !self.same_join_condition(&input.condition) {
            self.clear_join_aliases();
            input.condition = self.reconstruct_expression(input.condition, &()).0;
            input.if_true = self.reconstruct_expression(input.if_true, &()).0;
            input.if_false = self.reconstruct_expression(input.if_false, &()).0;
            return (input.into(), ());
        }

        let aliases = self.aliases.clone();
        input.condition = self.reconstruct_expression(input.condition, &()).0;

        self.aliases.extend(self.then_join_aliases.clone());
        input.if_true = self.reconstruct_expression(input.if_true, &()).0;
        self.aliases = aliases.clone();

        self.aliases.extend(self.otherwise_join_aliases.clone());
        input.if_false = self.reconstruct_expression(input.if_false, &()).0;
        self.aliases = aliases;

        (input.into(), ())
    }

    fn reconstruct_assert(&mut self, mut input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        self.clear_join_aliases();
        input.variant = match input.variant {
            AssertVariant::Assert(expr) => AssertVariant::Assert(self.reconstruct_expression(expr, &()).0),
            AssertVariant::AssertEq(left, right) => AssertVariant::AssertEq(
                self.reconstruct_expression(left, &()).0,
                self.reconstruct_expression(right, &()).0,
            ),
            AssertVariant::AssertNeq(left, right) => AssertVariant::AssertNeq(
                self.reconstruct_expression(left, &()).0,
                self.reconstruct_expression(right, &()).0,
            ),
        };

        self.clear_reads();
        (input.into(), ())
    }

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        self.clear_join_aliases();
        input.type_ = self.reconstruct_type(input.type_).0;
        input.value = self.reconstruct_expression(input.value, &()).0;
        (input.into(), ())
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        if !self.is_matching_join_ternary(&input.value) {
            self.clear_join_aliases();
        }

        input.value = self.reconstruct_expression(input.value, &()).0;

        let DefinitionPlace::Single(place) = &input.place else {
            return (input.into(), ());
        };

        if self.in_finalize_context
            && let Expression::Path(path) = &input.value
            && let Some(target) = path.try_local_symbol()
        {
            self.insert_alias(place.name, target);
        }

        if self.in_finalize_context
            && let Expression::Intrinsic(intrinsic) = &input.value
            && let Some(read) = self.storage_read(intrinsic)
        {
            if let Some(existing) = self.reads.get(&read).copied() {
                let existing = self.canonical_local(existing);
                self.insert_alias(place.name, existing);
                input.value = self.local_expression_like(existing, &input.value);
                return (input.into(), ());
            }
            self.reads.insert(read, place.name);
        }

        (input.into(), ())
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        self.clear_join_aliases();
        input.expression = self.reconstruct_expression(input.expression, &()).0;
        (input.into(), ())
    }

    fn reconstruct_return(&mut self, mut input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        self.clear_join_aliases();
        input.expression = self.reconstruct_expression(input.expression, &()).0;
        (input.into(), ())
    }

    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.");
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not exist in the AST at this phase of compilation.");
    }
}

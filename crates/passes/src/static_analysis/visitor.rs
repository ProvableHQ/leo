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

use crate::{CompilerState, ConditionalTreeNode, static_analysis::await_checker::AwaitChecker};

use crate::errors::static_analyzer;
use leo_ast::*;
use leo_span::{Span, Symbol};

pub(super) struct ExternalRecordInput {
    pub(super) variable: Symbol,
    pub(super) aliases: Vec<Symbol>,
    pub(super) tuple_aliases: Vec<(Symbol, usize)>,
    pub(super) record_program: Symbol,
    pub(super) consumed_at: Option<Span>,
}

pub struct StaticAnalyzingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Struct to store the state relevant to checking all futures are awaited.
    pub await_checker: AwaitChecker,
    /// The current program name.
    pub current_unit: Symbol,
    /// The variant of the function that we are currently traversing.
    pub variant: Option<Variant>,
    /// Whether or not a non-async external call has been seen in this function.
    pub non_async_external_call_seen: bool,
    /// Concrete records received from another program.
    pub external_record_inputs: Vec<ExternalRecordInput>,
    /// The nesting depth of conditional control flow.
    pub conditional_depth: usize,
}

impl StaticAnalyzingVisitor<'_> {
    pub fn emit_err(&self, err: leo_errors::Formatted) {
        self.state.handler.emit_err(err);
    }

    /// Emits a type checker warning
    pub fn emit_warning(&self, warning: leo_errors::Formatted) {
        self.state.handler.emit_warning(warning);
    }

    fn expression_uses_alias(record: &ExternalRecordInput, expression: &Expression) -> bool {
        match expression {
            Expression::Path(path) => path.try_local_symbol().is_some_and(|symbol| record.aliases.contains(&symbol)),
            Expression::TupleAccess(access) => match &access.tuple {
                Expression::Path(path) => path
                    .try_local_symbol()
                    .is_some_and(|symbol| record.tuple_aliases.contains(&(symbol, access.index.value()))),
                Expression::Tuple(tuple) => tuple
                    .elements
                    .get(access.index.value())
                    .is_some_and(|element| Self::expression_uses_alias(record, element)),
                _ => false,
            },
            _ => false,
        }
    }

    fn clear_record_value(&mut self, target: Symbol) {
        if self.conditional_depth == 0 {
            for record in &mut self.external_record_inputs {
                record.aliases.retain(|alias| *alias != target);
                record.tuple_aliases.retain(|(alias, _)| *alias != target);
            }
        }
    }

    fn assign_record_value(&mut self, target: Symbol, source: &Expression) {
        let origins = self
            .external_record_inputs
            .iter()
            .enumerate()
            .filter_map(|(index, record)| Self::expression_uses_alias(record, source).then_some(index))
            .collect::<Vec<_>>();

        if origins.is_empty() {
            self.clear_record_value(target);
            return;
        }

        self.clear_record_value(target);

        for index in origins {
            let aliases = &mut self.external_record_inputs[index].aliases;
            if !aliases.contains(&target) {
                aliases.push(target);
            }
        }
    }

    fn assign_tuple_value(&mut self, target: Symbol, tuple: &TupleExpression) {
        self.clear_record_value(target);
        for (tuple_index, element) in tuple.elements.iter().enumerate() {
            for record in &mut self.external_record_inputs {
                if Self::expression_uses_alias(record, element)
                    && !record.tuple_aliases.contains(&(target, tuple_index))
                {
                    record.tuple_aliases.push((target, tuple_index));
                }
            }
        }
    }

    pub(super) fn type_contains_record_from_different_external_program(
        state: &CompilerState,
        current_unit: Symbol,
        type_: &TypeKind,
        program: Symbol,
    ) -> bool {
        match type_ {
            TypeKind::Composite(composite) => {
                let location = composite.path.expect_global_location();
                location.program != current_unit
                    && location.program != program
                    && state.symbol_table.lookup_record(current_unit, location).is_some()
            }
            TypeKind::Tuple(tuple) => tuple.elements().iter().any(|element| {
                Self::type_contains_record_from_different_external_program(state, current_unit, element, program)
            }),
            _ => false,
        }
    }

    /// Type checks the awaiting of a future.
    pub fn assert_future_await(&mut self, future: &Option<&Expression>, span: Span) {
        // Make sure that it is an identifier expression.
        let future_variable = match future {
            Some(Expression::Path(path)) => path,
            _ => {
                return self.emit_err(static_analyzer::invalid_run_call(span));
            }
        };

        // Make sure that the future is defined.
        match self.state.type_table.get(&future_variable.id).map(|t| self.state.types.resolve(t)) {
            Some(type_) => {
                if !matches!(type_, TypeKind::Future(_)) {
                    self.emit_err(static_analyzer::expected_final(type_, future_variable.span()));
                }
                // Mark the future as consumed.
                // If the call returns true, it means that a future was not awaited in the order of the input list, emit a warning.
                if self.await_checker.remove(&future_variable.identifier().name) {
                    self.emit_warning(static_analyzer::final_not_awaited_in_order(
                        future_variable,
                        future_variable.span(),
                    ));
                }
            }
            None => {
                self.emit_err(static_analyzer::expected_final(future_variable, future_variable.span()));
            }
        }
    }
}

impl AstVisitor for StaticAnalyzingVisitor<'_> {
    /* Expressions */
    type AdditionalInput = ();
    type Output = ();

    fn visit_intrinsic(&mut self, input: &IntrinsicExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Check `Future::await` core functions.
        if let Some(Intrinsic::FinalRun) = Intrinsic::from_symbol(input.name, &input.type_parameters) {
            self.assert_future_await(&input.arguments.first(), input.span());
        }
        input.arguments.iter().for_each(|argument| {
            self.visit_expression(argument, &Default::default());
        });
    }

    fn visit_call(&mut self, input: &CallExpression, _: &Self::AdditionalInput) -> Self::Output {
        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(self.current_unit, input.function.expect_global_location())
            .expect("Type checking guarantees functions exist.");

        if func_symbol.function.variant == Variant::EntryPoint && !func_symbol.function.has_final_output() {
            self.non_async_external_call_seen = true;
        }

        let call_program = input.function.expect_global_location().program;
        for record in &mut self.external_record_inputs {
            if call_program == record.record_program
                && input.arguments.iter().any(|argument| Self::expression_uses_alias(record, argument))
            {
                record.consumed_at.get_or_insert(input.span);
            }
        }

        // if we're passing finals to a final fn, they get run and checked there
        if func_symbol.function.variant == Variant::FinalFn {
            for (param, arg) in func_symbol.function.input.iter().zip(input.arguments.iter()) {
                if matches!(param.type_.kind(), TypeKind::Future(_))
                    && let Expression::Path(path) = arg
                {
                    self.await_checker.remove(&path.identifier().name);
                }
            }
        }

        input.const_arguments.iter().for_each(|argument| {
            self.visit_expression(argument, &Default::default());
        });
        input.arguments.iter().for_each(|argument| {
            self.visit_expression(argument, &Default::default());
        });
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        self.visit_expression(&input.place, &Default::default());
        self.visit_expression(&input.value, &Default::default());

        if let Expression::Path(target) = &input.place
            && let Some(target) = target.try_local_symbol()
        {
            match &input.value {
                Expression::Tuple(tuple) => self.assign_tuple_value(target, tuple),
                source => self.assign_record_value(target, source),
            }
        }
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        match (&input.place, &input.value) {
            (DefinitionPlace::Single(target), Expression::Tuple(tuple)) => {
                self.assign_tuple_value(target.name, tuple);
            }
            (DefinitionPlace::Single(target), source) => {
                self.assign_record_value(target.name, source);
            }
            (DefinitionPlace::Multiple(targets), Expression::Tuple(sources)) => {
                for (target, source) in targets.iter().zip(&sources.elements) {
                    self.assign_record_value(target.name, source);
                }
            }
            _ => {}
        }
        if let Some(type_) = &input.type_ {
            self.visit_type(type_.kind());
        }
        self.visit_expression(&input.value, &Default::default());
    }

    /* Statements */
    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());

        // Create scope for checking awaits in `then` branch of conditional.
        let current_bst_nodes: Vec<ConditionalTreeNode> = match self
            .await_checker
            .create_then_scope(self.variant.is_some_and(|v| v.is_finalize_context()), input.span)
        {
            Ok(nodes) => nodes,
            Err(warn) => return self.emit_warning(warn),
        };

        self.conditional_depth += 1;

        // Visit block.
        self.visit_block(&input.then);

        // Exit scope for checking awaits in `then` branch of conditional.
        let saved_paths = self
            .await_checker
            .exit_then_scope(self.variant.is_some_and(|v| v.is_finalize_context()), current_bst_nodes);

        if let Some(otherwise) = &input.otherwise {
            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.visit_block(stmt);
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }
        }

        self.conditional_depth -= 1;

        // Update the set of all possible BST paths.
        self.await_checker.exit_statement_scope(self.variant.is_some_and(|v| v.is_finalize_context()), saved_paths);
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        if let Some(type_) = &input.type_ {
            self.visit_type(type_.kind());
        }
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.conditional_depth += 1;
        self.visit_block(&input.block);
        self.conditional_depth -= 1;
    }
}

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

use super::StaticAnalyzingVisitor;

use leo_ast::{Type, *};
use leo_errors::{StaticAnalyzerError, StaticAnalyzerWarning};

impl ProgramVisitor for StaticAnalyzingVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Set the current program name.
        self.current_program = input.program_id.name.name;
        // Do the default implementation for visiting the program scope.
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));
        input.composites.iter().for_each(|(_, c)| self.visit_composite(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
        input.storage_variables.iter().for_each(|(_, c)| self.visit_storage_variable(c));
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
        if let Some(c) = input.constructor.as_ref() {
            self.visit_constructor(c);
        }
    }

    fn visit_function(&mut self, function: &Function) {
        function.const_parameters.iter().for_each(|input| self.visit_type(&input.type_));
        function.input.iter().for_each(|input| self.visit_type(&input.type_));
        function.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&function.output_type);

        // Set the function name and variant.
        self.variant = Some(function.variant);

        // Set `non_async_external_call_seen` to false.
        self.non_async_external_call_seen = false;

        if self.variant.is_some_and(|v| v.is_onchain()) | function.has_final_output() {
            super::future_checker::future_check_function(function, &self.state.type_table, &self.state.handler);
        }

        // If the function is an async function, initialize the await checker.
        if self.variant.is_some_and(|v| v.is_onchain()) {
            // Initialize the list of input futures. Each one must be awaited before the end of the function.
            self.await_checker.set_futures(
                function
                    .input
                    .iter()
                    .filter_map(|input| {
                        if let Type::Future(_) = input.type_.clone() { Some(input.identifier.name) } else { None }
                    })
                    .collect(),
            );
        }

        self.visit_block(&function.block);

        // Check that all futures were awaited exactly once.
        if self.variant.is_some_and(|v| v.is_onchain()) {
            // Throw error if not all futures awaits even appear once.
            if !self.await_checker.static_to_await.is_empty() {
                self.emit_err(StaticAnalyzerError::final_runs_missing(
                    self.await_checker
                        .static_to_await
                        .clone()
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    function.span(),
                ));
            } else if !self.await_checker.to_await.is_empty() {
                // Tally up number of paths that are unawaited and number of paths that are awaited more than once.
                let (num_paths_unawaited, num_paths_duplicate_awaited, num_perfect) =
                    self.await_checker.to_await.iter().fold((0, 0, 0), |(unawaited, duplicate, perfect), path| {
                        (
                            unawaited + if !path.elements.is_empty() { 1 } else { 0 },
                            duplicate + if path.counter > 0 { 1 } else { 0 },
                            perfect + if path.counter > 0 || !path.elements.is_empty() { 0 } else { 1 },
                        )
                    });

                // Throw error if there does not exist a path in which all futures are awaited exactly once.
                if num_perfect == 0 {
                    self.emit_err(StaticAnalyzerError::no_path_runs_all_finals_exactly_once(
                        self.await_checker.to_await.len(),
                        function.span(),
                    ));
                }

                // Throw warning if not all futures are awaited in some paths.
                if num_paths_unawaited > 0 {
                    self.emit_warning(StaticAnalyzerWarning::some_paths_do_not_run_all_finals(
                        self.await_checker.to_await.len(),
                        num_paths_unawaited,
                        function.span(),
                    ));
                }

                // Throw warning if some futures are awaited more than once in some paths.
                if num_paths_duplicate_awaited > 0 {
                    self.emit_warning(StaticAnalyzerWarning::some_paths_contain_duplicate_final_runs(
                        self.await_checker.to_await.len(),
                        num_paths_duplicate_awaited,
                        function.span(),
                    ));
                }
            }
        }
    }

    fn visit_constructor(&mut self, _: &Constructor) {
        // Do nothing, since constructors do not have awaits or futures.
    }
}

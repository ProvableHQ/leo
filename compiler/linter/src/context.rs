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

use leo_errors::Lint;
use leo_passes::CompilerState;

use crate::diagnostics::DiagnosticReport;

/// Context for the Late lint pass.
#[derive(Clone, Copy)]
pub struct LateContext<'ctx> {
    #[expect(dead_code)]
    state: &'ctx CompilerState,
    report: &'ctx DiagnosticReport,
}

impl<'ctx> LateContext<'ctx> {
    pub fn new(report: &'ctx DiagnosticReport, state: &'ctx CompilerState) -> LateContext<'ctx> {
        Self { state, report }
    }

    pub fn emit_lint(&self, lint: Lint) {
        self.report.emit_lint(lint);
    }
}

/// Context for the early lint pass.
#[derive(Clone, Copy)]
pub struct EarlyContext<'ctx> {
    report: &'ctx DiagnosticReport,
}

impl<'ctx> EarlyContext<'ctx> {
    pub fn new(report: &'ctx DiagnosticReport) -> EarlyContext<'ctx> {
        Self { report }
    }

    pub fn emit_lint(&self, lint: Lint) {
        self.report.emit_lint(lint);
    }
}

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

use std::cell::RefCell;

/// All the triggered lints are collected here, as we traverse
/// the syntax trees and perform various lints.
#[derive(Default)]
pub struct DiagnosticReport {
    inner: RefCell<DiagnosticReportInner>,
}

impl DiagnosticReport {
    pub fn emit_lint(&self, lint: Lint) {
        self.inner.borrow_mut().emit_lint(lint);
    }

    pub fn consume(self) -> Vec<Lint> {
        self.inner.into_inner().collected
    }
}

#[derive(Default)]
struct DiagnosticReportInner {
    collected: Vec<Lint>,
}

impl DiagnosticReportInner {
    fn emit_lint(&mut self, lint: Lint) {
        self.collected.push(lint);
    }
}

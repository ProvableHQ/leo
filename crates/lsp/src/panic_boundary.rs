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

//! Panic containment for long-lived `leo-lsp` operations.
//!
//! Language servers are expected to stay alive across many requests,
//! notifications, and background jobs. A single panic in one of those flows
//! should not tear down the whole editor session when it can instead be
//! isolated, logged, and treated as a failed operation.
//!
//! This module provides the small boundary used around request dispatch,
//! notification dispatch, and worker execution to capture that failure context
//! in a structured way. Callers supply an operation name together with optional
//! document and generation metadata, then decide how to surface the resulting
//! [`PanicReport`]:
//!
//! - request handlers convert it into an internal-error response
//! - notification handlers log it and continue serving
//! - worker jobs report it back to the main loop without crashing the process
//!
//! The design is intentionally narrow. It is not meant to hide bugs or replace
//! normal error handling; it exists to keep a long-lived server resilient while
//! still emitting enough context to debug the underlying panic.
//!
//! This boundary should only be used at top-level task boundaries owned by the
//! `leo-lsp` runtime, where a single panicking request, notification, or worker
//! job would otherwise tear down the whole process. It should not be threaded
//! through ordinary library code or used as a substitute for returning
//! `Result`s, because a contained panic still indicates a bug that should be
//! fixed rather than normalized.

use lsp_types::Uri;
use std::{backtrace::Backtrace, panic::AssertUnwindSafe};

const BUG_REPORT_URL: &str =
    "https://github.com/ProvableHQ/leo/issues/new?labels=bug,panic&template=bug.md&title=[Bug]";

/// Structured panic report captured at a crate-internal task boundary.
///
/// This report exists so the `leo-lsp` binary can preserve the editor session
/// long enough to log actionable context and surface an internal-error result.
/// It should almost never cross beyond the narrow request, notification, and
/// worker boundaries defined by this crate.
#[derive(Debug, Clone)]
pub(crate) struct PanicReport {
    operation: &'static str,
    thread_name: Option<Box<str>>,
    document_uri: Option<Box<str>>,
    generation: Option<u64>,
    payload: Box<str>,
    backtrace: Box<str>,
}

impl PanicReport {
    /// Log this panic report through the `leo-lsp` tracing pipeline.
    pub(crate) fn log(&self) {
        tracing::error!(
            operation = self.operation,
            thread_name = self.thread_name.as_deref().unwrap_or("<unnamed>"),
            document_uri = self.document_uri.as_deref().unwrap_or("<none>"),
            generation = self.generation.unwrap_or_default(),
            payload = %self.payload,
            backtrace = %self.backtrace,
            bug_report_url = BUG_REPORT_URL,
            "INTERNAL PANIC: this indicates a bug in the Leo compiler or language server implementation. Please report it at {BUG_REPORT_URL}",
        );
    }
}

/// Execute `f` inside a crate-internal panic boundary and convert panics into reports.
///
/// This should only be used at the outermost task boundaries in `leo-lsp`
/// where crashing the whole process would be worse than failing one isolated
/// unit of work. New call-sites should justify that tradeoff with a local
/// comment so this helper does not become a blanket escape hatch.
pub(crate) fn catch_unwind<R>(
    operation: &'static str,
    document_uri: Option<&Uri>,
    generation: Option<u64>,
    f: impl FnOnce() -> R,
) -> Result<R, PanicReport> {
    std::panic::catch_unwind(AssertUnwindSafe(f)).map_err(|payload| PanicReport {
        operation,
        thread_name: std::thread::current().name().map(|name| name.to_owned().into_boxed_str()),
        document_uri: document_uri.map(|uri| uri.to_string().into_boxed_str()),
        generation,
        // Normalize the handful of panic payload shapes Rust code commonly
        // emits so the log output stays readable.
        payload: panic_payload_to_box_str(payload.as_ref()),
        backtrace: format!("{:#}", Backtrace::force_capture()).into_boxed_str(),
    })
}

fn panic_payload_to_box_str(payload: &(dyn std::any::Any + Send)) -> Box<str> {
    // Preserve the common string payload shapes directly and fall back to a
    // stable message for everything else.
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).into()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone().into_boxed_str()
    } else {
        "panic payload was not a string".into()
    }
}

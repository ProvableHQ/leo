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

//! Leo Language Server Protocol runtime.
//!
//! This crate exposes the stdio entrypoint used by the standalone `leo-lsp`
//! binary together with a testable server runner for in-process tests.

mod compiler_bridge;
mod document_store;
mod features;
mod panic_boundary;
mod project_model;
mod scheduler;
mod semantics;
mod server;
mod syntax_semantics;

use anyhow::{Context, Result};
use lsp_server::Connection;
use std::process::ExitCode;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Run the Leo language server over stdio.
///
/// This always joins the helper transport threads before returning, even when
/// the server loop exits with an error, so library callers do not leak stdio
/// reader or writer threads on failure paths.
pub fn run_stdio() -> Result<ExitCode> {
    init_logging();

    let (connection, io_threads) = Connection::stdio();
    let server_result = run_server(connection);
    let join_result = io_threads.join().context("failed to join leo-lsp stdio threads");

    match (server_result, join_result) {
        (Ok(exit_code), Ok(())) => Ok(exit_code),
        (Ok(_), Err(join_error)) => Err(join_error),
        (Err(server_error), Ok(())) => Err(server_error),
        (Err(server_error), Err(join_error)) => {
            tracing::error!(error = %join_error, "failed to join stdio threads after server error");
            Err(server_error)
        }
    }
}

/// Run the Leo language server using the provided LSP transport connection.
pub fn run_server(connection: Connection) -> Result<ExitCode> {
    server::run(connection)
}

/// Standalone `leo-lsp` entrypoint logic.
///
/// This keeps the binary wrapper thin while preserving the library-oriented
/// `Result`-returning entrypoints for tests, embeddings, and future plugins.
pub fn run_standalone() -> ExitCode {
    match run_stdio() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::from(1)
        }
    }
}

fn init_logging() {
    // LSP servers must keep stdout reserved for protocol traffic, so route all
    // diagnostics through a best-effort stderr subscriber.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_ansi(false).with_target(false).without_time().with_writer(std::io::stderr));

    let _ = subscriber.try_init();
}

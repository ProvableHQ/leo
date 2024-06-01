// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod add;
pub use add::Add;

pub mod account;
pub use account::Account;

pub mod build;
pub use build::Build;

pub mod clean;
pub use clean::Clean;

pub mod deploy;
pub use deploy::Deploy;

pub mod example;
pub use example::Example;

pub mod execute;
pub use execute::Execute;

pub mod query;
pub use query::Query;

pub mod new;
pub use new::New;

// pub mod node;
// pub use node::Node;

pub mod remove;
pub use remove::Remove;

pub mod run;
pub use run::Run;

pub mod update;
pub use update::Update;

use super::*;
use crate::cli::helpers::context::*;
use leo_errors::{emitter::Handler, CliError, PackageError, Result};
use leo_package::{build::*, outputs::OutputsDirectory, package::*};

use clap::Parser;
use colored::Colorize;
use tracing::span::Span;

/// Base trait for the Leo CLI, see methods and their documentation for details.
pub trait Command {
    /// If the current command requires running another command beforehand
    /// and needs its output result, this is where the result type is defined.
    /// Example: type Input: <CommandA as Command>::Out
    type Input;

    /// Defines the output of this command, which may be used as `Input` for another
    /// command. If this command is not used as a prelude for another command,
    /// this field may be left empty.
    type Output;

    /// Adds a span to the logger via `tracing::span`.
    /// Because of the specifics of the macro implementation, it is not possible
    /// to set the span name with a non-literal i.e. a dynamic variable even if this
    /// variable is a &'static str.
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Runs the prelude and returns the Input of the current command.
    fn prelude(&self, context: Context) -> Result<Self::Input>
    where
        Self: std::marker::Sized;

    /// Runs the main operation of this command. This function is run within
    /// context of 'execute' function, which sets logging and timers.
    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// A wrapper around the `apply` method.
    /// This function sets up tracing, timing, and the context.
    fn execute(self, context: Context) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        let input = self.prelude(context.clone())?;

        // Create the span for this command.
        let span = self.log_span();
        let span = span.enter();

        // Calculate the execution time for this command.
        let out = self.apply(context, input);

        drop(span);

        out
    }

    /// Executes command but empty the result. Comes in handy where there's a
    /// need to make match arms compatible while keeping implementation-specific
    /// output possible. Errors however are all of the type Error
    fn try_execute(self, context: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.execute(context).map(|_| Ok(()))?
    }
}

/// Compiler Options wrapper for Build command. Also used by other commands which
/// require Build command output as their input.
#[derive(Parser, Clone, Debug)]
pub struct BuildOptions {
    #[clap(long, help = "Endpoint to retrieve network state from.", default_value = "http://api.explorer.aleo.org/v1")]
    pub endpoint: String,
    #[clap(long, help = "Network to broadcast to. Defaults to mainnet.", default_value = "mainnet")]
    pub(crate) network: String,
    #[clap(long, help = "Does not recursively compile dependencies.")]
    pub non_recursive: bool,
    #[clap(long, help = "Enables offline mode.")]
    pub offline: bool,
    #[clap(long, help = "Enable spans in AST snapshots.")]
    pub enable_symbol_table_spans: bool,
    #[clap(long, help = "Enables dead code elimination in the compiler.")]
    pub enable_initial_symbol_table_snapshot: bool,
    #[clap(long, help = "Writes symbol table snapshot of the type checked symbol table.")]
    pub enable_type_checked_symbol_table_snapshot: bool,
    #[clap(long, help = "Writes symbol table snapshot of the unrolled symbol table.")]
    pub enable_unrolled_symbol_table_snapshot: bool,
    #[clap(long, help = "Enable spans in AST snapshots.")]
    pub enable_ast_spans: bool,
    #[clap(long, help = "Enable spans in symbol table snapshots.")]
    pub enable_dce: bool,
    #[clap(long, help = "Writes all AST snapshots for the different compiler phases.")]
    pub enable_all_ast_snapshots: bool,
    #[clap(long, help = "Writes AST snapshot of the initial parse.")]
    pub enable_initial_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the unrolled AST.")]
    pub enable_unrolled_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the SSA AST.")]
    pub enable_ssa_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the flattened AST.")]
    pub enable_flattened_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the destructured AST.")]
    pub enable_destructured_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the inlined AST.")]
    pub enable_inlined_ast_snapshot: bool,
    #[clap(long, help = "Writes AST snapshot of the dead code eliminated (DCE) AST.")]
    pub enable_dce_ast_snapshot: bool,
    #[clap(long, help = "Max depth to type check nested conditionals.", default_value = "10")]
    pub conditional_block_max_depth: usize,
    #[clap(long, help = "Disable type checking of nested conditional branches in finalize scope.")]
    pub disable_conditional_branch_type_checking: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            endpoint: "http://api.explorer.aleo.org/v1".to_string(),
            network: "mainnet".to_string(),
            non_recursive: false,
            offline: false,
            enable_symbol_table_spans: false,
            enable_initial_symbol_table_snapshot: false,
            enable_type_checked_symbol_table_snapshot: false,
            enable_unrolled_symbol_table_snapshot: false,
            enable_ast_spans: false,
            enable_dce: false,
            enable_all_ast_snapshots: false,
            enable_initial_ast_snapshot: false,
            enable_unrolled_ast_snapshot: false,
            enable_ssa_ast_snapshot: false,
            enable_flattened_ast_snapshot: false,
            enable_destructured_ast_snapshot: false,
            enable_inlined_ast_snapshot: false,
            enable_dce_ast_snapshot: false,
            conditional_block_max_depth: 10,
            disable_conditional_branch_type_checking: false,
        }
    }
}

/// On Chain Execution Options to set preferences for keys, fees and networks.
/// Used by Execute and Deploy commands.
#[derive(Parser, Clone, Debug)]
pub struct FeeOptions {
    #[clap(long, help = "Priority fee in microcredits. Defaults to 0.", default_value = "0")]
    pub(crate) priority_fee: String,
    #[clap(long, help = "Private key to authorize fee expenditure.")]
    pub(crate) private_key: Option<String>,
    #[clap(
        short,
        help = "Record to pay for fee privately. If one is not specified, a public fee will be taken.",
        long
    )]
    record: Option<String>,
}

impl Default for FeeOptions {
    fn default() -> Self {
        Self { priority_fee: "0".to_string(), private_key: None, record: None }
    }
}

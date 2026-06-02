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

mod abi;
pub use abi::LeoAbi;

mod add;
pub use add::{DependencySource, LeoAdd};

mod account;
pub use account::Account;

mod build;
pub use build::LeoBuild;

mod clean;
pub use clean::LeoClean;

mod common;
pub use common::*;

mod deploy;
pub use deploy::LeoDeploy;
use deploy::{Task, compute_deployment_stats, print_deployment_plan, print_deployment_summary};

mod devnet;
pub use devnet::LeoDevnet;

mod devnode;
pub use devnode::LeoDevnode;

mod execute;
pub use execute::LeoExecute;

pub mod query;
pub use query::LeoQuery;

mod new;
pub use new::LeoNew;

mod remove;
pub use remove::LeoRemove;

mod run;
pub use run::LeoRun;

mod synthesize;
pub use synthesize::LeoSynthesize;

mod test;
pub use test::LeoTest;

mod update;
pub use update::LeoUpdate;

pub mod upgrade;
pub use upgrade::LeoUpgrade;

use super::*;
use crate::cli::{helpers::context::*, query::QueryCommands};

use leo_errors::Result;
use snarkvm::{
    console::network::Network,
    prelude::{Address, Ciphertext, Plaintext, PrivateKey, Record, ViewKey, block::Transaction},
};

use clap::{Args, Parser};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::{iter, str::FromStr};
use tracing::span::Span;
use ureq::http::Uri;

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

// `parse_input` and its internal validators live in
// `leo_cli_core::commands::util`. `common/util.rs` re-exports them so
// `cli/commands/*.rs` callsites using `super::*` continue to find the
// symbol unchanged.

// `validate_cli_literal` and friends moved to
// `leo_cli_core::commands::util`. The unit tests below exercise the
// re-exported `parse_input` for regression coverage.

// Tests moved to leo-cli-core::commands::util (along with parse_input).

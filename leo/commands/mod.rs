// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::context::{get_context, Context};

use anyhow::Result;
use std::time::Instant;
use tracing::span::Span;

// local program commands
pub mod build;
pub use build::Build;

pub mod clean;
pub use clean::Clean;

pub mod deploy;
pub use deploy::Deploy;

pub mod init;
pub use init::Init;

pub mod lint;
pub use lint::Lint;

pub mod new;
pub use new::New;

pub mod prove;
pub use prove::Prove;

pub mod run;
pub use run::Run;

pub mod setup;
pub use setup::Setup;

pub mod test;
pub use test::Test;

pub mod update;
pub use update::{Automatic as UpdateAutomatic, Update};

pub mod watch;
pub use watch::Watch;

// Aleo PM related commands
pub mod package;

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

    /// Returns the project context, which is defined as the current directory.
    fn context(&self) -> Result<Context> {
        get_context()
    }

    /// Adds a span to the logger via `tracing::span`.
    /// Because of the specifics of the macro implementation, it is not possible
    /// to set the span name with a non-literal i.e. a dynamic variable even if this
    /// variable is a &'static str.
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Runs the prelude and returns the Input of the current command.
    fn prelude(&self) -> Result<Self::Input>
    where
        Self: std::marker::Sized;

    /// Runs the main operation of this command. This function is run within
    /// context of 'execute' function, which sets logging and timers.
    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// A wrapper around the `apply` method.
    /// This function sets up tracing, timing, and the context.
    fn execute(self) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        let input = self.prelude()?;

        // Create the span for this command.
        let span = self.log_span();
        let span = span.enter();

        // Calculate the execution time for this command.
        let timer = Instant::now();

        let context = self.context()?;
        let out = self.apply(context, input);

        drop(span);

        // Use the done context to print the execution time for this command.
        tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
            tracing::info!("Finished in {} milliseconds \n", timer.elapsed().as_millis());
        });

        out
    }

    /// Executes command but empty the result. Comes in handy where there's a
    /// need to make match arms compatible while keeping implementation-specific
    /// output possible. Errors however are all of the type Error
    fn try_execute(self) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.execute().map(|_| Ok(()))?
    }
}

// Copyright (C) 2019-2020 Aleo Systems Inc.
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
pub mod clean;
pub mod init;
pub mod new;
pub mod prove;
pub mod run;
pub mod setup;
pub mod test;
pub mod watch;

// aleo pm related commands
pub mod package;

// not implemented
pub mod deploy;
pub mod lint;

/// Base trait for Leo CLI, see methods and their documentation for details
pub trait Cmd {
    /// If current command requires running another command before
    /// and needs its output results, this is the place to set.
    /// Example: type Input: <CommandA as Cmd>::Out
    type Input;

    /// Define output of the command to be reused as an Input for another
    /// command. If this command is not used as a prelude for another, keep empty
    type Output;

    /// Returns project context, currently keeping it simple but it is possible
    /// that in the future leo will not depend on current directory, and we're keeping
    /// option for extending current core
    fn context(&self) -> Result<Context> {
        get_context()
    }

    /// Add span to the logger tracing::span.
    /// Due to specifics of macro implementation it is impossible to set
    /// span name with non-literal i.e. dynamic variable even if this
    /// variable is &'static str
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Run prelude and get Input for current command. As simple as that.
    /// But due to inability to pass default implementation of a type, this
    /// method must be present in every trait implementation.
    fn prelude(&self) -> Result<Self::Input>
    where
        Self: std::marker::Sized;

    /// Core of the execution - do what is necessary. This function is run within
    /// context of 'execute' function, which sets logging and timers
    fn apply(self, ctx: Context, input: Self::Input) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// Wrapper around apply function, sets up tracing, time tracking and context
    fn execute(self) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        let input = self.prelude()?;

        // create span for this command
        let span = self.log_span();
        let span = span.enter();

        // calculate execution time for each run
        let timer = Instant::now();

        let context = self.context()?;
        let out = self.apply(context, input);

        drop(span);

        // use done context to print time
        tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
            tracing::info!("Finished in {} milliseconds \n", timer.elapsed().as_millis());
        });

        out
    }

    /// Execute command but empty the result. Comes in handy where there's a
    /// need to make match arms compatible while keeping implementation-specific
    /// output possible. Errors however are all of the type Error
    fn try_execute(self) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.execute().map(|_| Ok(()))?
    }
}

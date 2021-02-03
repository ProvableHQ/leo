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

/// Leo command
pub trait Cmd {
    type Output;

    /// Returns project context.
    fn context(&self) -> Result<Context> {
        get_context()
    }

    /// Add span to the logger tracing::span.
    /// Due to specifics of macro implementation it is impossible to set
    /// span name with non-literal i.e. dynamic variable even if this
    /// variable is &'static str.  
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Apply command with given context.
    fn apply(self, ctx: Context) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// Wrapper around apply function, sets up tracing for
    /// each command
    fn run(self) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        // create span for this command
        let span = self.log_span();
        let span = span.enter();

        // calculate command spead on each run
        let timer = Instant::now();

        let context = self.context()?;
        let out = self.apply(context);

        drop(span);

        // use done context to print time
        tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
            tracing::info!("Finished in {} milliseconds", timer.elapsed().as_millis());
        });

        out
    }

    /// No-result wrapper for run function. Used to make match-arms  
    /// compatible in command matching in entrypoint. Only errors get
    /// through this facade.
    fn execute(self) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.run().map(|_| Ok(()))?
    }
}

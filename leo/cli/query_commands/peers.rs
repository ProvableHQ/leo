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

use super::*;

use crate::cli::context::Context;
use clap::Parser;

// Query information about network peers.
#[derive(Parser, Debug)]
pub struct Peers {
    #[arg(short, long, help = "Get all peer metrics", default_value = "false", conflicts_with("count"))]
    pub(crate) metrics: bool,
    #[arg(
        short,
        long,
        help = "Get the count of all participating peers",
        default_value = "false",
        conflicts_with("metrics")
    )]
    pub(crate) count: bool,
}

impl Command for Peers {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Build custom url to fetch from based on the flags and user's input.
        let url = if self.metrics {
            "peers/all/metrics".to_string()
        } else if self.count {
            "peers/count".to_string()
        } else {
            "peers/all".to_string()
        };

        Ok(url)
    }
}

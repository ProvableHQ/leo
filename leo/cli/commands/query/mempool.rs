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

// Query transactions and transmissions from the memory pool.
#[derive(Parser, Debug)]
pub struct Mempool {
    #[arg(
        short,
        long,
        help = "Get the memory pool transactions",
        default_value = "false",
        required_unless_present = "transmissions",
        conflicts_with("transmissions")
    )]
    pub(crate) transactions: bool,
    #[arg(
        short,
        long,
        help = "Get the memory pool transmissions",
        default_value = "false",
        required_unless_present = "transactions",
        conflicts_with("transactions")
    )]
    pub(crate) transmissions: bool,
}

impl Command for Mempool {
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
        let url = if self.transactions {
            "memoryPool/transactions".to_string()
        } else if self.transmissions {
            "memoryPool/transmissions".to_string()
        } else {
            unreachable!("All cases are covered")
        };

        Ok(url)
    }
}

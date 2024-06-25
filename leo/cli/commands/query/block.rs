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

// Query on-chain information related to blocks.
#[derive(Parser, Debug)]
pub struct Block {
    #[clap(help = "Fetch a block by specifying its height or hash", required_unless_present_any = &["latest", "latest_hash", "latest_height", "range"])]
    pub(crate) id: Option<String>,
    #[arg(short, long, help = "Get the latest block", default_value = "false", conflicts_with_all(["latest_hash", "latest_height", "range", "transactions", "to_height"]))]
    pub(crate) latest: bool,
    #[arg(short, long, help = "Get the latest block hash", default_value = "false", conflicts_with_all(["latest", "latest_height", "range", "transactions", "to_height"]))]
    pub(crate) latest_hash: bool,
    #[arg(short, long, help = "Get the latest block height", default_value = "false", conflicts_with_all(["latest", "latest_hash", "range", "transactions", "to_height"]))]
    pub(crate) latest_height: bool,
    #[arg(short, long, help = "Get up to 50 consecutive blocks", number_of_values = 2, value_names = &["START_HEIGHT", "END_HEIGHT"], conflicts_with_all(["latest", "latest_hash", "latest_height", "transactions", "to_height"]))]
    pub(crate) range: Option<Vec<String>>,
    #[arg(
        short,
        long,
        help = "Get all transactions at the specified block height",
        conflicts_with("to_height"),
        default_value = "false"
    )]
    pub(crate) transactions: bool,
    #[arg(short, long, help = "Lookup the block height corresponding to a hash value", default_value = "false")]
    pub(crate) to_height: bool,
}

impl Command for Block {
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
        let url = if self.latest_height {
            "block/height/latest".to_string()
        } else if self.latest_hash {
            "block/hash/latest".to_string()
        } else if self.latest {
            "block/latest".to_string()
        } else if let Some(range) = self.range {
            // Make sure the range is composed of valid numbers.
            is_valid_numerical_input(&range[0])?;
            is_valid_numerical_input(&range[1])?;

            // Parse the range values.
            let end = &range[1].parse::<u32>().map_err(|_| UtilError::invalid_bound(&range[1]))?;
            let start = &range[0].parse::<u32>().map_err(|_| UtilError::invalid_bound(&range[0]))?;
            // Make sure the range is not too large.
            if end - start > 50 {
                return Err(UtilError::invalid_range().into());
            }
            format!("blocks?start={}&end={}", range[0], range[1])
        } else if self.transactions {
            is_valid_numerical_input(&self.id.clone().unwrap())?;
            format!("block/{}/transactions", self.id.unwrap()).to_string()
        } else if self.to_height {
            let id = self.id.unwrap();
            is_valid_hash(&id)?;
            format!("height/{}", id).to_string()
        } else if let Some(id) = self.id {
            is_valid_height_or_hash(&id)?;
            format!("block/{}", id)
        } else {
            unreachable!("All cases are covered")
        };

        Ok(url)
    }
}

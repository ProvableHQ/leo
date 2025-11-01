// Copyright (C) 2019-2025 Provable Inc.
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
use serde_json::json;


// Advance the devnode ledger by a specified number of blocks.  The default value is 1.
#[derive(Parser, Debug)]
pub struct Advance{
    #[clap(help = "The number of blocks to advance the ledger by", default_value = "1")]
    pub num_blocks: u32,
}

impl Command for Advance {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let network: NetworkName = get_network(&context.env_override.network)?;
        let endpoint = get_endpoint(&context.env_override.endpoint)?;
        handle_advance_devnode(context, network, &endpoint, self.num_blocks).await
    }
}

async fn handle_advance_devnode(
    context: Context,
    network: NetworkName,
    endpoint: &str,
    num_blocks: u32,
) -> Result<()> {
    tracing::info!(
        "Advancing the devnode ledger by {} block(s) on network '{}' at endpoint '{}'",
        num_blocks,
        network,
        endpoint
    );

    for i in 0..num_blocks {
        // Call the REST API to advance the ledger by one block.
        let client = reqwest::blocking::Client::new();
        let payload = json!({
            private_key: context.env_override.private_key,
        });

        let response = client
            .post("http://localhost:3030/testnet/block/create")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
    }

    Ok(())
}
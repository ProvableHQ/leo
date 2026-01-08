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

use super::*;
use serde_json::json;

// Advance the Devnode ledger by a specified number of blocks.
#[derive(Parser, Debug)]
pub struct Advance {
    #[clap(help = "The number of blocks to advance the ledger by", default_value = "1")]
    pub num_blocks: u32,
    #[clap(long, help = "devnode REST API server address", default_value = "127.0.0.1:3030")]
    pub(crate) socket_addr: String,
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

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        tokio::runtime::Runtime::new().unwrap().block_on(async { handle_advance_devnode(self).await })
    }
}

async fn handle_advance_devnode(command: Advance) -> Result<<Advance as Command>::Output> {
    tracing::info!("Advancing the Devnode ledger by {} block(s)", command.num_blocks,);

    // Call the REST API to advance the ledger by the specified number of blocks.
    let client = reqwest::blocking::Client::new();
    let payload = json!({
        "num_blocks": command.num_blocks,
    });

    let _response = client
        .post(format!("http://{}/testnet/block/create", command.socket_addr))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send();

    Ok(())
}

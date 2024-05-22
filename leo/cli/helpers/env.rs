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

use snarkvm::prelude::{anyhow, Network, PrivateKey, Result};

fn env_template() -> String {
    r#"
NETWORK=testnet
PRIVATE_KEY={{PASTE_YOUR_PRIVATE_KEY_HERE}}
"#
    .to_string()
}

/// Loads the environment variables from the .env file.
fn dotenv_load() -> Result<()> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv().map_err(|_| {
        anyhow!(
            "Missing a '.env' file. Create the '.env' file in your package's root directory with the following:\n\n{}\n",
            env_template()
        )
    })?;
    Ok(())
}

/// Returns the private key from the environment.
pub fn dotenv_private_key<N: Network>() -> Result<PrivateKey<N>> {
    if cfg!(test) {
        let rng = &mut snarkvm::utilities::TestRng::fixed(123456789);
        PrivateKey::<N>::new(rng)
    } else {
        use std::str::FromStr;
        dotenv_load()?;
        // Load the private key from the environment.
        let private_key = dotenvy::var("PRIVATE_KEY").map_err(|e| anyhow!("Missing PRIVATE_KEY - {e}"))?;
        // Parse the private key.
        PrivateKey::<N>::from_str(&private_key)
    }
}

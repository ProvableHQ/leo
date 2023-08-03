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
use snarkvm::prelude::{Address, PrivateKey, ViewKey};

use rand::SeedableRng;
use rand_chacha::ChaChaRng;
/// Commands to manage Aleo accounts.
#[derive(Parser, Debug)]
pub enum Account {
    /// Generates a new Aleo account
    New {
        /// Seed the RNG with a numeric value.
        #[clap(short = 's', long)]
        seed: Option<u64>,
    },
}

impl Command for Account {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Aleo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<Self::Output>
    where
        Self: Sized,
    {
        match self {
            Account::New { seed } => {
                let private_key = match seed {
                    // Recover the field element deterministically.
                    Some(seed) => PrivateKey::new(&mut ChaChaRng::seed_from_u64(seed)),
                    // Sample a random field element.
                    None => PrivateKey::new(&mut ChaChaRng::from_entropy()),
                }
                .map_err(CliError::failed_to_parse_seed)?;

                let view_key = ViewKey::try_from(&private_key)?;
                let address = Address::<CurrentNetwork>::try_from(&view_key)?;

                // Print keys as formatted string without log level.
                println!(
                    "\n {:>12}  {private_key}\n {:>12}  {view_key}\n {:>12}  {address}",
                    "Private Key".cyan().bold(),
                    "View Key".cyan().bold(),
                    "Address".cyan().bold(),
                );

                //todo: save keys to .env file with --save flag
            }
        }
        Ok(())
    }
}

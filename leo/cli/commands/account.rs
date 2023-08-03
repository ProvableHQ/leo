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
use leo_package::root::Env;
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
        /// Write the private key to the .env file.
        #[clap(short = 'w', long)]
        write: bool,
    },
    /// Derive an Aleo account from a private key.
    Import {
        /// Private key plaintext
        private_key: PrivateKey<CurrentNetwork>,
        /// Write the private key to the .env file.
        #[clap(short = 'w', long)]
        write: bool,
    },
}

impl Command for Account {
    type Input = ();
    type Output = ();

    fn prelude(&self, _: Context) -> Result<Self::Input>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn apply(self, ctx: Context, _: Self::Input) -> Result<Self::Output>
    where
        Self: Sized,
    {
        match self {
            Account::New { seed, write } => {
                // Sample a new Aleo account.
                let private_key = match seed {
                    // Recover the field element deterministically.
                    Some(seed) => PrivateKey::new(&mut ChaChaRng::seed_from_u64(seed)),
                    // Sample a random field element.
                    None => PrivateKey::new(&mut ChaChaRng::from_entropy()),
                }
                .map_err(CliError::failed_to_parse_seed)?;

                // Derive the view key and address and print to stdout.
                print_keys(private_key)?;

                // Save key data to .env file.
                if write {
                    write_to_env_file(private_key, &ctx)?;
                }
            }
            Account::Import { private_key, write } => {
                // Derive the view key and address and print to stdout.
                print_keys(private_key)?;

                // Save key data to .env file.
                if write {
                    write_to_env_file(private_key, &ctx)?;
                }
            }
        }
        Ok(())
    }
}

// Helper functions

// Print keys as a formatted string without log level.
fn print_keys(private_key: PrivateKey<CurrentNetwork>) -> Result<()> {
    let view_key = ViewKey::try_from(&private_key)?;
    let address = Address::<CurrentNetwork>::try_from(&view_key)?;

    println!(
        "\n {:>12}  {private_key}\n {:>12}  {view_key}\n {:>12}  {address}\n",
        "Private Key".cyan().bold(),
        "View Key".cyan().bold(),
        "Address".cyan().bold(),
    );
    Ok(())
}

// Write the network and private key to the .env file in project directory.
fn write_to_env_file(private_key: PrivateKey<CurrentNetwork>, ctx: &Context) -> Result<()> {
    let data = format!("NETWORK=testnet3\nPRIVATE_KEY={private_key}\n");
    let program_dir = ctx.dir()?;
    Env::<CurrentNetwork>::from(data).write_to(&program_dir)?;
    tracing::info!("✅ Private Key written to {}", program_dir.join(".env").display());
    Ok(())
}

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

use crossterm::ExecutableCommand;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::io::{self, Read, Write};

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
        /// Print sensitive information (such as private key) discreetly to an alternate screen
        #[clap(long)]
        discreet: bool,
    },
    /// Derive an Aleo account from a private key.
    Import {
        /// Private key plaintext
        private_key: PrivateKey<CurrentNetwork>,
        /// Write the private key to the .env file.
        #[clap(short = 'w', long)]
        write: bool,
        /// Print sensitive information (such as private key) discreetly to an alternate screen
        #[clap(long)]
        discreet: bool,
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
            Account::New { seed, write, discreet } => {
                // Sample a new Aleo account.
                let private_key = match seed {
                    // Recover the field element deterministically.
                    Some(seed) => PrivateKey::new(&mut ChaChaRng::seed_from_u64(seed)),
                    // Sample a random field element.
                    None => PrivateKey::new(&mut ChaChaRng::from_entropy()),
                }
                .map_err(CliError::failed_to_parse_seed)?;

                // Derive the view key and address and print to stdout.
                print_keys(private_key, discreet)?;

                // Save key data to .env file.
                if write {
                    write_to_env_file(private_key, &ctx)?;
                }
            }
            Account::Import { private_key, write, discreet } => {
                // Derive the view key and address and print to stdout.
                print_keys(private_key, discreet)?;

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
fn print_keys(private_key: PrivateKey<CurrentNetwork>, discreet: bool) -> Result<()> {
    let view_key = ViewKey::try_from(&private_key)?;
    let address = Address::<CurrentNetwork>::try_from(&view_key)?;

    if !discreet {
        println!(
            "\n {:>12}  {private_key}\n {:>12}  {view_key}\n {:>12}  {address}\n",
            "Private Key".cyan().bold(),
            "View Key".cyan().bold(),
            "Address".cyan().bold(),
        );
        return Ok(());
    }
    display_string_discreetly(
        &private_key.to_string(),
        "### Do not share or lose this private key! Press any key to complete. ###",
    )?;
    println!("\n {:>12}  {view_key}\n {:>12}  {address}\n", "View Key".cyan().bold(), "Address".cyan().bold(),);
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

/// Print the string to an alternate screen, so that the string won't been printed to the terminal.
fn display_string_discreetly(discreet_string: &str, continue_message: &str) -> Result<()> {
    use crossterm::{
        style::Print,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    };
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    // print msg on the alternate screen
    stdout.execute(Print(format!("{discreet_string}\n{continue_message}"))).unwrap();
    stdout.flush().unwrap();
    wait_for_keypress();
    stdout.execute(LeaveAlternateScreen).unwrap();
    Ok(())
}

fn wait_for_keypress() {
    let mut single_key = [0u8];
    std::io::stdin().read_exact(&mut single_key).unwrap();
}

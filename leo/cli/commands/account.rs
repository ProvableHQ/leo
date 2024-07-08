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
use leo_errors::UtilError;
use leo_package::root::Env;
use snarkvm::{
    console::program::{Signature, ToFields, Value},
    prelude::{Address, PrivateKey, ViewKey},
};

use crossterm::ExecutableCommand;
use leo_retriever::NetworkName;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use snarkvm::prelude::{CanaryV0, MainnetV0, Network, TestnetV0};
use std::{
    io::{self, Read, Write},
    path::PathBuf,
    str::FromStr,
};

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
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: String,
        #[clap(
            short = 'e',
            long,
            help = "Endpoint to retrieve network state from.",
            default_value = "https://api.explorer.aleo.org/v1"
        )]
        endpoint: String,
    },
    /// Derive an Aleo account from a private key.
    Import {
        /// Private key plaintext
        private_key: Option<String>,
        /// Write the private key to the .env file.
        #[clap(short = 'w', long)]
        write: bool,
        /// Print sensitive information (such as private key) discreetly to an alternate screen
        #[clap(long)]
        discreet: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: String,
        #[clap(
            short = 'e',
            long,
            help = "Endpoint to retrieve network state from.",
            default_value = "https://api.explorer.aleo.org/v1"
        )]
        endpoint: String,
    },
    /// Sign a message using your Aleo private key.
    Sign {
        /// Specify the account private key of the node
        #[clap(long = "private-key")]
        private_key: Option<String>,
        /// Specify the path to a file containing the account private key of the node
        #[clap(long = "private-key-file")]
        private_key_file: Option<String>,
        /// Message (Aleo value) to sign
        #[clap(short = 'm', long)]
        message: String,
        /// Seed the RNG with a numeric value
        #[clap(short = 's', long)]
        seed: Option<u64>,
        /// When enabled, parses the message as bytes instead of Aleo literals
        #[clap(short = 'r', long)]
        raw: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: String,
    },
    /// Verify a message from an Aleo address.
    Verify {
        /// Address to use for verification
        #[clap(short = 'a', long)]
        address: String,
        /// Signature to verify
        #[clap(short = 's', long)]
        signature: String,
        /// Message (Aleo value) to verify the signature against
        #[clap(short = 'm', long)]
        message: String,
        /// When enabled, parses the message as bytes instead of Aleo literals
        #[clap(short = 'r', long)]
        raw: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: String,
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
            Account::New { seed, write, discreet, network, endpoint } => {
                // Parse the network.
                let network = NetworkName::try_from(network.as_str())?;
                match network {
                    NetworkName::MainnetV0 => generate_new_account::<MainnetV0>(seed, write, discreet, &ctx, endpoint),
                    NetworkName::TestnetV0 => generate_new_account::<TestnetV0>(seed, write, discreet, &ctx, endpoint),
                    NetworkName::CanaryV0 => generate_new_account::<CanaryV0>(seed, write, discreet, &ctx, endpoint),
                }?
            }
            Account::Import { private_key, write, discreet, network, endpoint } => {
                // Parse the network.
                let network = NetworkName::try_from(network.as_str())?;
                match network {
                    NetworkName::MainnetV0 => import_account::<MainnetV0>(private_key, write, discreet, &ctx, endpoint),
                    NetworkName::TestnetV0 => import_account::<TestnetV0>(private_key, write, discreet, &ctx, endpoint),
                    NetworkName::CanaryV0 => import_account::<CanaryV0>(private_key, write, discreet, &ctx, endpoint),
                }?
            }
            Self::Sign { message, seed, raw, private_key, private_key_file, network } => {
                // Parse the network.
                let network = NetworkName::try_from(network.as_str())?;
                let result = match network {
                    NetworkName::MainnetV0 => {
                        sign_message::<MainnetV0>(message, seed, raw, private_key, private_key_file)
                    }
                    NetworkName::TestnetV0 => {
                        sign_message::<TestnetV0>(message, seed, raw, private_key, private_key_file)
                    }
                    NetworkName::CanaryV0 => {
                        sign_message::<MainnetV0>(message, seed, raw, private_key, private_key_file)
                    }
                }?;
                println!("{result}")
            }
            Self::Verify { address, signature, message, raw, network } => {
                // Parse the network.
                let network = NetworkName::try_from(network.as_str())?;
                let result = match network {
                    NetworkName::MainnetV0 => verify_message::<MainnetV0>(address, signature, message, raw),
                    NetworkName::TestnetV0 => verify_message::<TestnetV0>(address, signature, message, raw),
                    NetworkName::CanaryV0 => verify_message::<CanaryV0>(address, signature, message, raw),
                }?;
                println!("{result}")
            }
        }
        Ok(())
    }
}

// Helper functions

// Generate a new account.
fn generate_new_account<N: Network>(
    seed: Option<u64>,
    write: bool,
    discreet: bool,
    ctx: &Context,
    endpoint: String,
) -> Result<()> {
    // Sample a new Aleo account.
    let private_key = match seed {
        // Recover the field element deterministically.
        Some(seed) => PrivateKey::<N>::new(&mut ChaChaRng::seed_from_u64(seed)),
        // Sample a random field element.
        None => PrivateKey::new(&mut ChaChaRng::from_entropy()),
    }
    .map_err(CliError::failed_to_parse_seed)?;

    // Derive the view key and address and print to stdout.
    print_keys(private_key, discreet)?;

    // Save key data to .env file.
    if write {
        write_to_env_file(private_key, ctx, endpoint)?;
    }
    Ok(())
}

// Import an account.
fn import_account<N: Network>(
    private_key: Option<String>,
    write: bool,
    discreet: bool,
    ctx: &Context,
    endpoint: String,
) -> Result<()> {
    let priv_key = match discreet {
        true => {
            let private_key_input = rpassword::prompt_password("Please enter your private key: ").unwrap();
            FromStr::from_str(&private_key_input).map_err(CliError::failed_to_parse_private_key)?
        }
        false => match private_key {
            Some(private_key) => FromStr::from_str(&private_key).map_err(CliError::failed_to_parse_private_key)?,
            None => {
                return Err(CliError::failed_to_execute_account(
                    "PRIVATE_KEY shouldn't be empty when --discreet is false",
                )
                .into());
            }
        },
    };

    // Derive the view key and address and print to stdout.
    print_keys::<N>(priv_key, discreet)?;

    // Save key data to .env file.
    if write {
        write_to_env_file::<N>(priv_key, ctx, endpoint)?;
    }

    Ok(())
}

// Print keys as a formatted string without log level.
fn print_keys<N: Network>(private_key: PrivateKey<N>, discreet: bool) -> Result<()> {
    let view_key = ViewKey::try_from(&private_key)?;
    let address = Address::<N>::try_from(&view_key)?;

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

// Sign a message with an Aleo private key
pub(crate) fn sign_message<N: Network>(
    message: String,
    seed: Option<u64>,
    raw: bool,
    private_key: Option<String>,
    private_key_file: Option<String>,
) -> Result<String> {
    let private_key = match (private_key, private_key_file) {
        (Some(private_key), None) => PrivateKey::<N>::from_str(private_key.trim())
            .map_err(|e| CliError::cli_invalid_input(format!("could not parse private key: {e}")))?,
        (None, Some(private_key_file)) => {
            let path = private_key_file
                .parse::<PathBuf>()
                .map_err(|e| CliError::cli_invalid_input(format!("invalid path - {e}")))?;
            let key_str = std::fs::read_to_string(path).map_err(UtilError::failed_to_read_file)?;
            PrivateKey::<N>::from_str(key_str.trim())
                .map_err(|e| CliError::cli_invalid_input(format!("could not parse private key: {e}")))?
        }
        (None, None) => {
            // Attempt to pull private key from env, then .env file
            match dotenvy::var("PRIVATE_KEY") {
                Ok(key) => PrivateKey::<N>::from_str(key.trim())
                    .map_err(|e| CliError::cli_invalid_input(format!("could not parse private key: {e}")))?,
                Err(_) => Err(CliError::cli_invalid_input(
                    "missing the '--private-key', '--private-key-file', PRIVATE_KEY env, or .env",
                ))?,
            }
        }
        (Some(_), Some(_)) => {
            Err(CliError::cli_invalid_input("cannot specify both the '--private-key' and '--private-key-file' flags"))?
        }
    };
    // Recover the seed.
    let mut rng = match seed {
        // Recover the field element deterministically.
        Some(seed) => ChaChaRng::seed_from_u64(seed),
        // Sample a random field element.
        None => ChaChaRng::from_entropy(),
    };

    // Sign the message
    let signature = if raw {
        private_key.sign_bytes(message.as_bytes(), &mut rng)
    } else {
        let fields = Value::<N>::from_str(&message)?
            .to_fields()
            .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid Aleo value"))?;
        private_key.sign(&fields, &mut rng)
    }
    .map_err(|_| CliError::cli_runtime_error("Failed to sign the message"))?
    .to_string();
    // Return the signature as a string
    Ok(signature)
}

// Verify a signature with an Aleo address
pub(crate) fn verify_message<N: Network>(
    address: String,
    signature: String,
    message: String,
    raw: bool,
) -> Result<String> {
    // Parse the address.
    let address = Address::<N>::from_str(&address)?;

    let signature = Signature::<N>::from_str(&signature)
        .map_err(|e| CliError::cli_invalid_input(format!("Failed to parse a valid signature: {e}")))?;

    // Verify the signature
    let verified = if raw {
        signature.verify_bytes(&address, message.as_bytes())
    } else {
        let fields = Value::<N>::from_str(&message)?
            .to_fields()
            .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid Aleo value"))?;
        signature.verify(&address, &fields)
    };

    // Return the verification result
    match verified {
        true => Ok("✅ The signature is valid".to_string()),
        false => Err(CliError::cli_runtime_error("❌ The signature is invalid"))?,
    }
}

// Write the network and private key to the .env file in project directory.
fn write_to_env_file<N: Network>(private_key: PrivateKey<N>, ctx: &Context, endpoint: String) -> Result<()> {
    let program_dir = ctx.dir()?;
    Env::<N>::new(Some(private_key), endpoint)?.write_to(&program_dir)?;
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

#[cfg(test)]
mod tests {
    use super::{sign_message, verify_message};

    type CurrentNetwork = snarkvm::prelude::MainnetV0;

    #[test]
    fn test_signature_raw() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "Hello, world!".to_string();
        assert!(sign_message::<CurrentNetwork>(message, None, true, Some(key), None).is_ok());
    }

    #[test]
    fn test_signature() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "5field".to_string();
        assert!(sign_message::<CurrentNetwork>(message, None, false, Some(key), None).is_ok());
    }

    #[test]
    fn test_signature_fail() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "not a literal value".to_string();
        assert!(sign_message::<CurrentNetwork>(message, None, false, Some(key), None).is_err());
    }

    #[test]
    fn test_seeded_signature_raw() {
        let seed = Some(38868010450269069);
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "Hello, world!".to_string();
        let expected = "sign175pmqldmkqw2nwp7wz7tfmpyqdnvzaq06mh8t2g22frsmrdtuvpf843p0wzazg27rwrjft8863vwn5a5cqgr97ldw69cyq53l0zlwqhesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkevd26g";
        let actual = sign_message::<CurrentNetwork>(message, seed, true, Some(key), None).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_seeded_signature() {
        let seed = Some(38868010450269069);
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "5field".to_string();
        let expected = "sign1ad29myqy8gv6xve2r6tuly39m63l2mpfpyvqkwdl2umxqek6q5qxmy63zmhjx75x90sqxq69u5ntzp25kp59e0hp4hj8l8085sg7vqlesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qk7v46re";
        let actual = sign_message::<CurrentNetwork>(message, seed, false, Some(key), None).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_verify_raw() {
        // test signature of "Hello, world!"
        let address = "aleo1zecnqchckrzw7dlsyf65g6z5le2rmys403ecwmcafrag0e030yxqrnlg8j".to_string();
        let signature = "sign1nnvrjlksrkxdpwsrw8kztjukzhmuhe5zf3srk38h7g32u4kqtqpxn3j5a6k8zrqcfx580a96956nsjvluzt64cqf54pdka9mgksfqp8esm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkwsnaqq".to_string();
        let message = "Hello, world!".to_string();
        assert!(verify_message::<CurrentNetwork>(address.clone(), signature, message, true).is_ok());

        // test signature of "Hello, world!" against the message "Different Message"
        let signature = "sign1nnvrjlksrkxdpwsrw8kztjukzhmuhe5zf3srk38h7g32u4kqtqpxn3j5a6k8zrqcfx580a96956nsjvluzt64cqf54pdka9mgksfqp8esm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkwsnaqq".to_string();
        let message = "Different Message".to_string();
        assert!(verify_message::<CurrentNetwork>(address.clone(), signature, message, true).is_err());

        // test signature of "Hello, world!" against the wrong address
        let signature = "sign1nnvrjlksrkxdpwsrw8kztjukzhmuhe5zf3srk38h7g32u4kqtqpxn3j5a6k8zrqcfx580a96956nsjvluzt64cqf54pdka9mgksfqp8esm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkwsnaqq".to_string();
        let message = "Hello, world!".to_string();
        let wrong_address = "aleo1uxl69laseuv3876ksh8k0nd7tvpgjt6ccrgccedpjk9qwyfensxst9ftg5".to_string();
        assert!(verify_message::<CurrentNetwork>(wrong_address, signature, message, true).is_err());

        // test a valid signature of "Different Message"
        let signature = "sign1424ztyt9hcm77nq450gvdszrvtg9kvhc4qadg4nzy9y0ah7wdqq7t36cxal42p9jj8e8pjpmc06lfev9nvffcpqv0cxwyr0a2j2tjqlesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qk3yrr50".to_string();
        let message = "Different Message".to_string();
        assert!(verify_message::<CurrentNetwork>(address, signature, message, true).is_ok());
    }

    #[test]
    fn test_verify() {
        // test signature of 5u8
        let address = "aleo1zecnqchckrzw7dlsyf65g6z5le2rmys403ecwmcafrag0e030yxqrnlg8j".to_string();
        let signature = "sign1j7swjfnyujt2vme3ulu88wdyh2ddj85arh64qh6c6khvrx8wvsp8z9wtzde0sahqj2qwz8rgzt803c0ceega53l4hks2mf5sfsv36qhesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkdetews".to_string();
        let message = "5field".to_string();
        assert!(verify_message::<CurrentNetwork>(address.clone(), signature, message, false).is_ok());

        // test signature of 5u8 against the message 10u8
        let signature = "sign1j7swjfnyujt2vme3ulu88wdyh2ddj85arh64qh6c6khvrx8wvsp8z9wtzde0sahqj2qwz8rgzt803c0ceega53l4hks2mf5sfsv36qhesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkdetews".to_string();
        let message = "10field".to_string();
        assert!(verify_message::<CurrentNetwork>(address.clone(), signature, message, false).is_err());

        // test signature of 5u8 against the wrong address
        let signature = "sign1j7swjfnyujt2vme3ulu88wdyh2ddj85arh64qh6c6khvrx8wvsp8z9wtzde0sahqj2qwz8rgzt803c0ceega53l4hks2mf5sfsv36qhesm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qkdetews".to_string();
        let message = "5field".to_string();
        let wrong_address = "aleo1uxl69laseuv3876ksh8k0nd7tvpgjt6ccrgccedpjk9qwyfensxst9ftg5".to_string();
        assert!(verify_message::<CurrentNetwork>(wrong_address, signature, message, false).is_err());

        // test a valid signature of 10u8
        let signature = "sign1t9v2t5tljk8pr5t6vkcqgkus0a3v69vryxmfrtwrwg0xtj7yv5qj2nz59e5zcyl50w23lhntxvt6vzeqfyu6dt56698zvfj2l6lz6q0esm5elrqqunzqzmac7kzutl6zk7mqht3c0m9kg4hklv7h2js0qmxavwnpuwyl4lzldl6prs4qeqy9wxyp8y44nnydg3h8sg6ue99qk8rh9kt".to_string();
        let message = "10field".to_string();
        assert!(verify_message::<CurrentNetwork>(address, signature, message, false).is_ok());
    }
}

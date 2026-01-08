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
use leo_ast::NetworkName;
use leo_errors::UtilError;

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    console::program::{Signature, ToFields, Value},
    prelude::{Address, Network, PrivateKey, TestnetV0, ViewKey},
};

use crossterm::ExecutableCommand;
use itertools::Itertools;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
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
        /// Write the private key to the .env file in the current directory.
        #[clap(short = 'w', long)]
        write: bool,
        /// Print sensitive information (such as private key) discreetly to an alternate screen
        #[clap(long)]
        discreet: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: NetworkName,
        #[clap(
            short = 'e',
            long,
            help = "Endpoint to retrieve network state from.",
            default_value = "https://api.explorer.provable.com/v1"
        )]
        endpoint: String,
    },
    /// Derive an Aleo account from a private key.
    Import {
        /// Private key plaintext
        private_key: Option<String>,
        /// Write the private key to the .env file in the current directory.
        #[clap(short = 'w', long)]
        write: bool,
        /// Print sensitive information (such as private key) discreetly to an alternate screen
        #[clap(long)]
        discreet: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: NetworkName,
        #[clap(
            short = 'e',
            long,
            help = "Endpoint to retrieve network state from.",
            default_value = "https://api.explorer.provable.com/v1"
        )]
        endpoint: String,
    },
    /// Sign a message using your Aleo private key.
    Sign {
        /// Specify the account private key
        #[clap(long = "private-key")]
        private_key: Option<String>,
        /// Specify the path to a file containing the account private key
        #[clap(long = "private-key-file")]
        private_key_file: Option<String>,
        /// Message (Aleo value) to sign
        #[clap(short = 'm', long)]
        message: String,
        /// When enabled, parses the message as bytes instead of Aleo literals
        #[clap(short = 'r', long)]
        raw: bool,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: NetworkName,
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
        network: NetworkName,
    },
    /// Decrupt record ciphertext using your Aleo private key or view key.
    Decrypt {
        /// Specify the account key
        #[clap(short = 'k', help = "Private key or view key to use for decryption")]
        key: Option<String>,
        /// Specify the path to a file containing the account private key
        #[clap(short = 'f', help = "Path to a file containing the private key or view key")]
        key_file: Option<String>,
        /// The ciphertext to decrypt
        #[clap(short = 'c', long)]
        ciphertext: String,
        #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
        network: NetworkName,
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
            Account::New { seed, write, discreet, network, endpoint } => match network {
                NetworkName::TestnetV0 => {
                    generate_new_account::<TestnetV0>(network, seed, write, discreet, &ctx, endpoint)
                }
                NetworkName::MainnetV0 => {
                    #[cfg(feature = "only_testnet")]
                    panic!("Mainnet chosen with only_testnet feature");
                    #[cfg(not(feature = "only_testnet"))]
                    generate_new_account::<MainnetV0>(network, seed, write, discreet, &ctx, endpoint)
                }
                NetworkName::CanaryV0 => {
                    #[cfg(feature = "only_testnet")]
                    panic!("Canary chosen with only_testnet feature");
                    #[cfg(not(feature = "only_testnet"))]
                    generate_new_account::<CanaryV0>(network, seed, write, discreet, &ctx, endpoint)
                }
            }?,
            Account::Import { private_key, write, discreet, network, endpoint } => match network {
                NetworkName::TestnetV0 => {
                    import_account::<TestnetV0>(network, private_key, write, discreet, &ctx, endpoint)
                }
                NetworkName::MainnetV0 => {
                    #[cfg(feature = "only_testnet")]
                    panic!("Mainnet chosen with only_testnet feature");
                    #[cfg(not(feature = "only_testnet"))]
                    import_account::<MainnetV0>(network, private_key, write, discreet, &ctx, endpoint)
                }
                NetworkName::CanaryV0 => {
                    #[cfg(feature = "only_testnet")]
                    panic!("Canary chosen with only_testnet feature");
                    #[cfg(not(feature = "only_testnet"))]
                    import_account::<CanaryV0>(network, private_key, write, discreet, &ctx, endpoint)
                }
            }?,
            Self::Sign { message, raw, private_key, private_key_file, network } => {
                let result = match network {
                    NetworkName::TestnetV0 => sign_message::<TestnetV0>(message, raw, private_key, private_key_file),
                    NetworkName::MainnetV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Mainnet chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        sign_message::<MainnetV0>(message, raw, private_key, private_key_file)
                    }
                    NetworkName::CanaryV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Canary chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        sign_message::<CanaryV0>(message, raw, private_key, private_key_file)
                    }
                }?;
                println!("{result}")
            }
            Self::Verify { address, signature, message, raw, network } => {
                let result = match network {
                    NetworkName::TestnetV0 => verify_message::<TestnetV0>(address, signature, message, raw),
                    NetworkName::MainnetV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Mainnet chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        verify_message::<MainnetV0>(address, signature, message, raw)
                    }
                    NetworkName::CanaryV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Canary chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        verify_message::<CanaryV0>(address, signature, message, raw)
                    }
                }?;
                println!("{result}")
            }
            Self::Decrypt { key, key_file, ciphertext, network } => {
                let result = match network {
                    NetworkName::TestnetV0 => decrypt_ciphertext::<TestnetV0>(key, key_file, &ciphertext),
                    NetworkName::MainnetV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Mainnet chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        decrypt_ciphertext::<MainnetV0>(key, key_file, &ciphertext)
                    }
                    NetworkName::CanaryV0 => {
                        #[cfg(feature = "only_testnet")]
                        panic!("Canary chosen with only_testnet feature");
                        #[cfg(not(feature = "only_testnet"))]
                        decrypt_ciphertext::<CanaryV0>(key, key_file, &ciphertext)
                    }
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
    network: NetworkName,
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
        write_to_env_file(network, private_key, ctx, endpoint)?;
    }
    Ok(())
}

// Import an account.
fn import_account<N: Network>(
    network: NetworkName,
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
                    "Missing private key argument. Provide a private key or use the '--discreet' flag to enter it securely.",
                )
                .into());
            }
        },
    };

    // Derive the view key and address and print to stdout.
    print_keys::<N>(priv_key, discreet)?;

    // Save key data to .env file.
    if write {
        write_to_env_file::<N>(network, priv_key, ctx, endpoint)?;
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
    raw: bool,
    private_key: Option<String>,
    private_key_file: Option<String>,
) -> Result<String> {
    // Get the private key string.
    let private_key_string = get_key_string(private_key, private_key_file, &["PRIVATE_KEY"])?;

    // Parse the private key.
    let private_key_string = private_key_string.trim();
    let private_key = PrivateKey::<N>::from_str(private_key_string)
        .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid private key"))?;

    // Sample a random field element.
    let mut rng = ChaChaRng::from_entropy();

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

// Decrypt a record ciphertext using a private key or view key.
pub(crate) fn decrypt_ciphertext<N: Network>(
    key: Option<String>,
    key_file: Option<String>,
    ciphertext: &str,
) -> Result<String> {
    // Get the key string.
    let key_string = get_key_string(key, key_file, &["PRIVATE_KEY", "VIEW_KEY"])?;

    // Parse the key.
    let key_string = key_string.trim();
    let view_key = if key_string.starts_with("APrivateKey1") {
        // If the key starts with "APrivateKey1", treat it as a private key.
        let private_key = PrivateKey::<N>::from_str(key_string)
            .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid private key"))?;
        // Convert the private key to a view key.
        ViewKey::<N>::try_from(&private_key)
            .map_err(|_| CliError::cli_invalid_input("Failed to convert private key to view key"))?
    } else if key_string.starts_with("AViewKey1") {
        // If the key starts with "AViewKey1", treat it as a view key.
        ViewKey::<N>::from_str(key_string)
            .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid view key"))?
    } else {
        // If the key is neither, return an error.
        Err(CliError::cli_invalid_input("Invalid key format. Expected a private or view key."))?
    };

    // Parse the ciphertext as record ciphertext.
    let record_ciphertext = Record::<N, Ciphertext<N>>::from_str(ciphertext)
        .map_err(|_| CliError::cli_invalid_input("Failed to parse a valid record ciphertext"))?;

    // Decrypt the record.
    let decrypted_value = record_ciphertext
        .decrypt(&view_key)
        .map_err(|_| CliError::cli_runtime_error("Failed to decrypt the record ciphertext"))?;

    // Return the decrypted value as a string.
    Ok(decrypted_value.to_string())
}

// A helper function to get the key string from the environment or file.
fn get_key_string(key: Option<String>, key_file: Option<String>, env_vars: &[&'static str]) -> Result<String> {
    match (key, key_file) {
        (Some(key), None) => Ok(key),
        (None, Some(key_file)) => {
            let path =
                key_file.parse::<PathBuf>().map_err(|e| CliError::cli_invalid_input(format!("Invalid path - {e}")))?;
            std::fs::read_to_string(path).map_err(|e| UtilError::failed_to_read_file(e).into())
        }
        (None, None) => {
            // Attempt to pull any of the environment variables
            env_vars.iter().find_map(|&var| std::env::var(var).ok()).ok_or_else(|| {
                CliError::cli_invalid_input(format!(
                    "Missing the '--key', '--key-file', or the following environment variables: '{}'",
                    env_vars.iter().format(",")
                ))
                .into()
            })
        }
        (Some(_), Some(_)) => {
            Err(CliError::cli_invalid_input("Cannot specify both the '--key' and '--key-file' flags").into())
        }
    }
}

// Write the network and private key to an .env file in project directory.
fn write_to_env_file<N: Network>(
    network: NetworkName,
    private_key: PrivateKey<N>,
    ctx: &Context,
    endpoint: String,
) -> Result<()> {
    let program_dir = ctx.dir()?;
    let env_path = program_dir.join(".env");
    std::fs::write(env_path, format!("NETWORK={network}\nPRIVATE_KEY={private_key}\nENDPOINT={endpoint}\n"))
        .map_err(PackageError::io_error_env_file)?;
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
    use super::{decrypt_ciphertext, sign_message, verify_message};
    use snarkvm::{
        prelude::{
            Address,
            Identifier,
            Network,
            Plaintext,
            PrivateKey,
            Process,
            ProgramID,
            Record,
            Scalar,
            TestRng,
            U8,
            Uniform,
            ViewKey,
        },
        synthesizer::program::StackTrait,
    };
    use std::str::FromStr;

    type CurrentNetwork = snarkvm::prelude::MainnetV0;

    #[test]
    fn test_signature_raw() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "Hello, world!".to_string();
        assert!(sign_message::<CurrentNetwork>(message, true, Some(key), None).is_ok());
    }

    #[test]
    fn test_signature() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "5field".to_string();
        assert!(sign_message::<CurrentNetwork>(message, false, Some(key), None).is_ok());
    }

    #[test]
    fn test_signature_fail() {
        let key = "APrivateKey1zkp61PAYmrYEKLtRWeWhUoDpFnGLNuHrCciSqN49T86dw3p".to_string();
        let message = "not a literal value".to_string();
        assert!(sign_message::<CurrentNetwork>(message, false, Some(key), None).is_err());
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

    #[test]
    fn test_decrypt() -> anyhow::Result<()> {
        // Initialize an RNG.
        let mut rng = &mut TestRng::default();

        // Test decryption with a private key
        let private_key =
            PrivateKey::<CurrentNetwork>::from_str("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")?;
        let private_key_string = private_key.to_string();
        let view_key = ViewKey::<CurrentNetwork>::try_from(&private_key)?;
        let view_key_string = view_key.to_string();
        let address = Address::<CurrentNetwork>::try_from(&view_key)?;

        // Create a random record.
        let process = Process::<CurrentNetwork>::load()?;
        let stack = process.get_stack(ProgramID::from_str("credits.aleo")?)?;
        let randomizer = Scalar::<CurrentNetwork>::rand(rng);
        let nonce = CurrentNetwork::g_scalar_multiply(&randomizer);
        let record = stack.sample_record(&address, &Identifier::from_str("credits").unwrap(), nonce, &mut rng)?;
        let record = Record::<CurrentNetwork, Plaintext<CurrentNetwork>>::from_plaintext(
            record.owner().clone(),
            record.data().clone(),
            nonce,
            U8::new(u8::rand(rng) % 2),
        )?;
        let record_string = record.to_string();
        let ciphertext = record.encrypt(randomizer)?;
        let ciphertext_string = ciphertext.to_string();

        // Test decryption with the private key
        let candidate = decrypt_ciphertext::<CurrentNetwork>(Some(private_key_string), None, &ciphertext_string)?;
        assert_eq!(candidate, record_string);

        // Test decryption with a view key
        let candidate = decrypt_ciphertext::<CurrentNetwork>(Some(view_key_string), None, &ciphertext_string)?;
        assert_eq!(candidate, record_string);

        Ok(())
    }
}

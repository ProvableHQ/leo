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

use std::fs;

/// Initialize a new Leo example.
#[derive(Parser, Debug)]
pub struct Example {
    #[clap(name = "NAME", help = "The example to initialize.")]
    pub(crate) name: String,
    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,
    #[clap(
        short = 'e',
        long,
        help = "Endpoint to retrieve network state from.",
        default_value = "https://api.explorer.aleo.org/v1"
    )]
    pub(crate) endpoint: String,
}

impl Command for Example {
    type Input = <New as Command>::Output;
    type Output = ();

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Run leo new <name> --network <network>
        (New { name: self.name.clone(), network: self.network.clone(), endpoint: self.endpoint.clone() })
            .execute(context)
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output>
    where
        Self: Sized,
    {
        let package_dir = context.dir()?;

        // Parse the example variant.
        let example_variant =
            ExampleVariant::try_from(self.name.as_str()).map_err(|_| CliError::invalid_example(self.name.as_str()))?;

        // Write the main file.
        let main_file_path = package_dir.join("src").join("main.leo");
        fs::write(main_file_path, example_variant.main_file_string()).map_err(CliError::failed_to_write_file)?;

        // Write the README file.
        let readme_file_path = package_dir.join("README.md");
        let readme_file_path_string = readme_file_path.display().to_string();
        fs::write(readme_file_path, example_variant.readme_file_string()).map_err(CliError::failed_to_write_file)?;

        // Write the run.sh file.
        let run_file_path = package_dir.join("run.sh");
        fs::write(run_file_path, example_variant.run_file_string()).map_err(CliError::failed_to_write_file)?;

        tracing::info!(
            "🚀 To run the '{}' program follow the instructions at {}",
            example_variant.name().bold(),
            readme_file_path_string
        );

        Ok(())
    }
}

/// The example programs that can be generated.
#[derive(Parser, Debug, Copy, Clone)]
pub enum ExampleVariant {
    #[clap(name = "lottery", about = "A public lottery program")]
    Lottery,
    #[clap(name = "tictactoe", about = "A standard tic-tac-toe game program")]
    TicTacToe,
    #[clap(name = "token", about = "A transparent & shielded custom token program")]
    Token,
}

impl ExampleVariant {
    fn name(&self) -> String {
        match self {
            Self::Lottery => "lottery".to_string(),
            Self::TicTacToe => "tictactoe".to_string(),
            Self::Token => "token".to_string(),
        }
    }

    fn main_file_string(&self) -> String {
        match self {
            Self::Lottery => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/lottery/src/main.leo")).to_string()
            }
            Self::TicTacToe => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/tictactoe/src/main.leo")).to_string()
            }
            Self::Token => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/token/src/main.leo")).to_string()
            }
        }
    }

    fn readme_file_string(&self) -> String {
        match self {
            Self::Lottery => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/lottery/README.md")).to_string()
            }
            Self::TicTacToe => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/tictactoe/README.md")).to_string()
            }
            Self::Token => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/token/README.md")).to_string(),
        }
    }

    fn run_file_string(&self) -> String {
        match self {
            Self::Lottery => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/lottery/run.sh")).to_string(),
            Self::TicTacToe => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/tictactoe/run.sh")).to_string()
            }
            Self::Token => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/token/run.sh")).to_string(),
        }
    }
}

impl TryFrom<&str> for ExampleVariant {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "lottery" => Ok(Self::Lottery),
            "tictactoe" => Ok(Self::TicTacToe),
            "token" => Ok(Self::Token),
            _ => Err(()),
        }
    }
}

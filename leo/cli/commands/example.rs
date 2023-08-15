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

/// The example programs that can be generated.
#[derive(Parser, Debug)]
pub enum Example {
    #[clap(name = "lottery", about = "A public lottery program")]
    Lottery,
    #[clap(name = "tictactoe", about = "A standard tic-tac-toe game program")]
    TicTacToe,
    #[clap(name = "token", about = "A transparent & shielded custom token program")]
    Token,
}

impl Command for Example {
    type Input = <New as Command>::Output;
    type Output = ();

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Run leo new EXAMPLE_NAME
        (New { name: self.name() }).execute(context)
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output>
    where
        Self: Sized,
    {
        let package_dir = context.dir()?;

        // Write the main file.
        let main_file_path = package_dir.join("src").join("main.leo");
        fs::write(main_file_path, self.main_file_string()).map_err(CliError::failed_to_write_file)?;

        // Write the input file.
        let input_file_path = package_dir.join("inputs").join(format!("{}.in", self.name()));
        fs::write(input_file_path, self.input_file_string()).map_err(CliError::failed_to_write_file)?;

        // Write the README file.
        let readme_file_path = package_dir.join("README.md");
        let readme_file_path_string = readme_file_path.display().to_string();
        fs::write(readme_file_path, self.readme_file_string()).map_err(CliError::failed_to_write_file)?;

        // Write the run.sh file.
        let run_file_path = package_dir.join("run.sh");
        fs::write(run_file_path, self.run_file_string()).map_err(CliError::failed_to_write_file)?;

        tracing::info!(
            "🚀 To run the '{}' program follow the instructions at {}",
            self.name().bold(),
            readme_file_path_string
        );

        Ok(())
    }
}

impl Example {
    fn name(&self) -> String {
        match self {
            Example::Lottery => "lottery".to_string(),
            Example::TicTacToe => "tictactoe".to_string(),
            Example::Token => "token".to_string(),
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

    fn input_file_string(&self) -> String {
        match self {
            Self::Lottery => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/lottery/inputs/lottery.in")).to_string()
            }
            Self::TicTacToe => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/tictactoe/inputs/tictactoe.in")).to_string()
            }
            Self::Token => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/token/inputs/token.in")).to_string()
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

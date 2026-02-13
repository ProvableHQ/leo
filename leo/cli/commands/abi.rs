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

use leo_ast::NetworkName;
use leo_errors::CliError;

use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

use std::path::PathBuf;

/// Generate ABI from an Aleo bytecode file.
#[derive(Parser, Debug)]
pub struct LeoAbi {
    /// Path to the .aleo file
    #[clap(value_name = "FILE")]
    file: PathBuf,

    /// Network for parsing (mainnet, testnet, canary)
    #[clap(long, short, default_value = "testnet")]
    network: NetworkName,

    /// Output file path (defaults to stdout)
    #[clap(long, short)]
    output: Option<PathBuf>,
}

impl Command for LeoAbi {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        // Validate file exists.
        if !self.file.exists() {
            return Err(CliError::cli_invalid_input(format!("File not found: {}", self.file.display())).into());
        }

        // Validate file has .aleo extension.
        match self.file.extension().and_then(|s| s.to_str()) {
            Some("aleo") => {}
            _ => {
                return Err(CliError::cli_invalid_input(format!(
                    "Expected a .aleo file, got: {}",
                    self.file.display()
                ))
                .into());
            }
        }

        // Read the file content.
        let content = std::fs::read_to_string(&self.file).map_err(CliError::cli_io_error)?;

        // Get the file name for error messages.
        let file_name = self.file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

        // Disassemble and generate ABI based on network type.
        let aleo_program = match self.network {
            NetworkName::MainnetV0 => leo_disassembler::disassemble_from_str::<MainnetV0>(file_name, &content),
            NetworkName::TestnetV0 => leo_disassembler::disassemble_from_str::<TestnetV0>(file_name, &content),
            NetworkName::CanaryV0 => leo_disassembler::disassemble_from_str::<CanaryV0>(file_name, &content),
        }
        .map_err(|e| CliError::failed_to_parse_aleo_file(file_name, e))?;

        // Generate ABI from the disassembled program.
        let abi = leo_abi::aleo::generate(&aleo_program);

        // Serialize to JSON.
        let json = serde_json::to_string_pretty(&abi).map_err(|e| CliError::failed_to_serialize_abi(e.to_string()))?;

        // Write to output file or stdout.
        match self.output {
            Some(path) => {
                std::fs::write(&path, &json).map_err(CliError::failed_to_write_abi)?;
                tracing::info!("ABI written to '{}'.", path.display());
            }
            None => {
                println!("{json}");
            }
        }

        Ok(())
    }
}

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

use clap::Parser;

/// Query program source code and live mapping values.
#[derive(Parser, Debug)]
pub struct LeoProgram {
    #[clap(name = "NAME", help = "The name of the program to fetch")]
    pub(crate) name: String,
    #[arg(
        long,
        help = "An optional edition to use when fetching the program source. If not specified, the latest edition will be used."
    )]
    pub(crate) edition: Option<u16>,
    #[arg(
        long,
        help = "Get all mappings defined in the latest edition of the program",
        default_value = "false",
        conflicts_with = "mapping_value"
    )]
    pub(crate) mappings: bool,
    #[arg(
        long,
        help = "Get the value corresponding to the specified mapping and key.",
        number_of_values = 2,
        value_names = &["MAPPING", "KEY"],
        conflicts_with = "mappings"
    )]
    pub(crate) mapping_value: Option<Vec<String>>,
}

impl Command for LeoProgram {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        // Check that the program name is valid.
        let program = if self.name.ends_with(".aleo") { self.name.clone() } else { format!("{}.aleo", self.name) };
        if !leo_package::is_valid_aleo_name(&program) {
            return Err(CliError::invalid_program_name(program).into());
        }
        // Build custom url to fetch from based on the flags and user's input.
        let url = if let Some(mapping_info) = self.mapping_value {
            format!("program/{program}/mapping/{}/{}", mapping_info[0], mapping_info[1])
        } else if self.mappings {
            format!("program/{program}/mappings")
        } else {
            // When no edition is specified, omit the edition from the URL to get the latest one.
            match self.edition {
                Some(edition) => format!("program/{program}/{edition}"),
                None => format!("program/{program}"),
            }
        };

        Ok(url)
    }
}

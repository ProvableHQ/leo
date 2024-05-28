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

use clap::Parser;
use leo_package::package::Package;

/// Query program source code and live mapping values.
#[derive(Parser, Debug)]
pub struct Program {
    #[clap(name = "NAME", help = "The name of the program to fetch")]
    pub(crate) name: String,
    #[arg(
        short,
        long,
        help = "Get all mappings defined in the program",
        default_value = "false",
        conflicts_with = "mapping_value"
    )]
    pub(crate) mappings: bool,
    #[arg(short, long, help = "Get the value corresponding to the specified mapping and key.", number_of_values = 2, value_names = &["MAPPING", "KEY"], conflicts_with = "mappings")]
    pub(crate) mapping_value: Option<Vec<String>>,
}

impl Command for Program {
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
        let program = check_valid_program_name(self.name);
        // Build custom url to fetch from based on the flags and user's input.
        let url = if let Some(mapping_info) = self.mapping_value {
            // Check that the mapping name is valid.
            Package::is_aleo_name_valid(&mapping_info[0]);
            format!("program/{}/mapping/{}/{}", program, mapping_info[0], mapping_info[1])
        } else if self.mappings {
            format!("program/{}/mappings", program)
        } else {
            format!("program/{}", program)
        };

        Ok(url)
    }
}

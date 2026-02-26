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

use clap::Parser;

/// Query transaction information.
#[derive(Parser, Debug)]
#[command(group(
    // Ensure exactly one source is specified.
    clap::ArgGroup::new("source").required(true).multiple(false)
))]
#[command(group(
    // Ensure at most one mode is specified and ony if using "id" as a source.
    // The `conflicts_with_all` here should not be required but it looks like Clap is getting a little confused
    clap::ArgGroup::new("mode")
        .required(false)
        .multiple(false)
        .requires("ID").conflicts_with_all(["from_io", "from_transition", "from_program"]
    )
))]
pub struct LeoTransaction {
    #[clap(name = "ID", help = "The ID of the transaction to fetch", group = "source")]
    pub(crate) id: Option<String>,
    #[arg(short, long, help = "Return more information about a confirmed transaction", group = "mode")]
    pub(crate) confirmed: bool,
    #[arg(short, long, help = "Get the original (unconfirmed) transaction", group = "mode")]
    pub(crate) unconfirmed: bool,
    #[arg(
        value_name = "INPUT_OR_OUTPUT_ID",
        long,
        help = "Get the ID of the transaction that an input or output ID occurred in",
        group = "source"
    )]
    pub(crate) from_io: Option<String>,
    #[arg(
        value_name = "TRANSITION_ID",
        long,
        help = "Get the ID of the transaction containing the specified transition",
        group = "source"
    )]
    pub(crate) from_transition: Option<String>,
    #[arg(
        value_name = "PROGRAM",
        long,
        help = "Get the ID of the transaction that the specified program was deployed in",
        group = "source"
    )]
    pub(crate) from_program: Option<String>,
}

impl Command for LeoTransaction {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        // Build custom url to fetch from based on the flags and user's input.
        let url = if let Some(io_id) = self.from_io {
            let field = is_valid_field(&io_id)?;
            format!("find/transitionID/{field}")
        } else if let Some(transition) = self.from_transition {
            is_valid_transition_id(&transition)?;
            format!("find/transactionID/{transition}")
        } else if let Some(program) = self.from_program {
            // Check that the program name is valid.
            if !leo_package::is_valid_aleo_name(&program) {
                return Err(CliError::invalid_program_name(program).into());
            }
            format!("find/transactionID/deployment/{program}")
        } else if let Some(id) = self.id {
            is_valid_transaction_id(&id)?;
            if self.confirmed {
                format!("transaction/confirmed/{id}")
            } else if self.unconfirmed {
                format!("transaction/unconfirmed/{id}")
            } else {
                format!("transaction/{id}")
            }
        } else {
            unreachable!("All command paths covered.")
        };

        Ok(url)
    }
}

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

/// Query transaction information.
#[derive(Parser, Debug)]
pub struct Transaction {
    #[clap(name = "ID", help = "The id of the transaction to fetch", required_unless_present_any = &["from_program", "from_transition", "from_io", "range"])]
    pub(crate) id: Option<String>,
    #[arg(short, long, help = "Get the transaction only if it has been confirmed", default_value = "false", conflicts_with_all(["from_io", "from_transition", "from_program"]))]
    pub(crate) confirmed: bool,
    #[arg(value_name = "INPUT_OR_OUTPUT_ID", short, long, help = "Get the transition id that an input or output id occurred in", conflicts_with_all(["from_program", "from_transition", "confirmed", "id"]))]
    pub(crate) from_io: Option<String>,
    #[arg(value_name = "TRANSITION_ID", short, long, help = "Get the id of the transaction containing the specified transition", conflicts_with_all(["from_io", "from_program", "confirmed", "id"]))]
    pub(crate) from_transition: Option<String>,
    #[arg(value_name = "PROGRAM", short, long, help = "Get the id of the transaction id that the specified program was deployed in", conflicts_with_all(["from_io", "from_transition", "confirmed", "id"]))]
    pub(crate) from_program: Option<String>,
}

impl Command for Transaction {
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
            format!("find/transactionID/deployment/{}", check_valid_program_name(program))
        } else if let Some(id) = self.id {
            is_valid_transaction_id(&id)?;
            if self.confirmed { format!("transaction/confirmed/{}", id) } else { format!("transaction/{}", id) }
        } else {
            unreachable!("All command paths covered.")
        };

        Ok(url)
    }
}

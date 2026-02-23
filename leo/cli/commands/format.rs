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

/// Format Leo source files.
#[derive(Parser, Debug)]
pub struct LeoFormat {
    /// Shared formatting flags/paths from `leo-fmt`.
    #[clap(flatten)]
    args: leo_fmt::FormatCliArgs,
}

impl Command for LeoFormat {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let LeoFormat { args } = self;
        let base_dir = context.dir()?;
        let has_unformatted = leo_fmt::run_format_cli(&args, &base_dir).map_err(|error| -> leo_errors::LeoError {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                CliError::cli_invalid_input(error).into()
            } else {
                CliError::cli_io_error(error).into()
            }
        })?;

        if args.check && has_unformatted {
            std::process::exit(1);
        }

        Ok(())
    }
}

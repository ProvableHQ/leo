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

use leo_fmt::Formatter;

use std::path::PathBuf;

// Formats the leo code in a directory.
#[derive(Parser, Debug)]
pub struct LeoFormat {
    #[clap(long)]
    check: bool,
}

impl Command for LeoFormat {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        let _manifest = context.open_manifest()?; // just so that we can be sure that we are in a valid leo project directory.
        let path = context.dir()?;

        let context_provider = || Formatter::default_format_context();

        let source_dir = path.join(leo_package::SOURCE_DIRECTORY);
        let main = source_dir.join(leo_package::MAIN_FILENAME);
        Formatter::format_directory(main, Some(source_dir), context_provider, self.check)?;

        let tests_dir = path.join(leo_package::TESTS_DIRECTORY);
        let tests = leo_package::Package::files_with_extension(&tests_dir, "leo");
        for test in tests {
            Formatter::format_directory(test, None::<PathBuf>, context_provider, self.check)?;
        }

        Ok(())
    }
}

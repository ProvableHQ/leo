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

/// Clean outputs folder command
#[derive(Parser, Debug)]
pub struct LeoClean {}

impl Command for LeoClean {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let path = context.dir()?;

        // Removes the outputs/ directory.
        let outputs_path = path.join(leo_package::OUTPUTS_DIRECTORY);
        std::fs::remove_dir_all(&outputs_path)
            .map_err(|e| PackageError::failed_to_remove_directory(outputs_path.display(), e))?;
        tracing::info!("🧹 Cleaned the outputs directory {}", outputs_path.display().to_string().dimmed());

        // Removes the build/ directory.
        let build_path = path.join(leo_package::BUILD_DIRECTORY);
        std::fs::remove_dir_all(&build_path)
            .map_err(|e| PackageError::failed_to_remove_directory(build_path.display(), e))?;
        tracing::info!("🧹 Cleaned the build directory {}", build_path.display().to_string().dimmed());

        Ok(())
    }
}

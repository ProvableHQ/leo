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

use leo_package::Workspace;

/// Create new Leo project
#[derive(Parser, Debug)]
pub struct LeoNew {
    #[clap(name = "NAME", help = "Set package name")]
    pub(crate) name: String,
    #[clap(long, help = "Create the package as a library instead of a program", conflicts_with = "workspace")]
    pub(crate) library: bool,
    #[clap(long, help = "Create a workspace skeleton instead of a package", conflicts_with = "library")]
    pub(crate) workspace: bool,
}

impl Command for LeoNew {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Derive the location of the parent directory to the project.
        let package_path = context.parent_dir()?;

        // Change the cwd to the parent directory so subsequent commands resolve sensibly.
        std::env::set_current_dir(&package_path)
            .map_err(|err| crate::errors::failed_to_set_cwd(package_path.display(), err))?;

        if self.workspace {
            let full_path = Workspace::initialize_skeleton(&self.name, &package_path)?;
            println!("Created workspace {} at `{}`.", self.name.bold(), full_path.display());
        } else {
            let full_path = leo_cli_core::package_init::initialize_package(&self.name, &package_path, self.library)?;
            println!("Created program {} at `{}`.", self.name.bold(), full_path.display());

            if Workspace::auto_register_member(&full_path)? {
                println!("Added {} to the enclosing workspace.", self.name.bold());
            }
        }

        Ok(())
    }
}

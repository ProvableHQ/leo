// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{CorePackage, CoreSymbolList, UNSTABLE_CORE_PACKAGE_KEYWORD};
use leo_typed::PackageAccess;

/// A list of core package dependencies.
/// This struct is created when the compiler parses a core import statement.
#[derive(Debug)]
pub struct CorePackageList {
    packages: Vec<CorePackage>,
}

impl CorePackageList {
    pub(crate) fn new() -> Self {
        Self { packages: vec![] }
    }

    pub(crate) fn push(&mut self, package: CorePackage) {
        self.packages.push(package);
    }

    // Parse all dependencies after `core.`
    pub fn from_package_access(access: PackageAccess) -> Self {
        let mut new = Self::new();

        package_access_helper(&mut new, access, false);

        new
    }

    // Return a list of all symbols that need to be stored in the current function
    pub fn to_symbols(&self) -> CoreSymbolList {
        let mut symbols = CoreSymbolList::new();

        for package in &self.packages {
            package.append_symbols(&mut symbols);
        }

        symbols
    }
}

fn package_access_helper(list: &mut CorePackageList, access: PackageAccess, is_unstable: bool) {
    match access {
        PackageAccess::Symbol(_symbol) => unimplemented!("cannot import a symbol directly from Leo core"),
        PackageAccess::Multiple(core_functions) => {
            for access in core_functions {
                package_access_helper(list, access, is_unstable);
            }
        }
        PackageAccess::SubPackage(package) => {
            // Set the `unstable` flag to true if we are importing an unstable core package
            if package.name.name.eq(UNSTABLE_CORE_PACKAGE_KEYWORD) {
                package_access_helper(list, package.access, true);
            } else {
                let mut core_package = CorePackage::from(*package);

                if is_unstable {
                    core_package.set_unstable()
                }

                list.push(core_package);
            }
        }
        PackageAccess::Star(_) => unimplemented!("cannot import star from Leo core"),
    }
}

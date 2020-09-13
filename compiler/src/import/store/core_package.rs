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

use crate::{errors::ImportError, ConstrainedProgram, GroupType};
use leo_typed::{Identifier, ImportSymbol, Package, PackageAccess};

use snarkos_models::curves::{Field, PrimeField};
use std::collections::HashMap;

static UNSTABLE_CORE_PACKAGE_KEYWORD: &str = "unstable";

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_core_package(&mut self, package: Package) {
        println!("storing core package: {}", package);
        // create core package list
        println!("creating core package list");
        let list = CorePackageList::from_package_access(package.access);

        println!("{:?}", list);

        // fetch packages from `leo-core`
        // println!("fetching packages from leo core");

        // store packages
        // println!("storing dependencies from leo core into leo program");
    }
}

/// A list of core package dependencies
#[derive(Debug)]
pub struct CorePackageList {
    packages: Vec<CorePackage>,
}

/// A core package dependency to be imported into a Leo program
#[derive(Debug)]
pub struct CorePackage {
    name: Identifier,
    unstable: bool,
    symbols: Vec<ImportSymbol>,
}

impl CorePackageList {
    pub(crate) fn new() -> Self {
        Self { packages: vec![] }
    }

    pub(crate) fn push(&mut self, package: CorePackage) {
        self.packages.push(package);
    }

    // Parse all dependencies after `core.`
    pub(crate) fn from_package_access(access: PackageAccess) -> Self {
        let mut new = Self::new();

        match access {
            PackageAccess::Symbol(_symbol) => unimplemented!("cannot import a symbol directly from Leo core"),
            PackageAccess::Multiple(_) => unimplemented!("multiple imports not yet implemented for Leo core"),
            PackageAccess::SubPackage(package) => {
                println!("importing package access {}", *package);

                let core_package = CorePackage::from(*package);

                new.push(core_package);
            }
            PackageAccess::Star(_) => unimplemented!("cannot import star from Leo core"),
        }

        new
    }
}

impl CorePackage {
    pub(crate) fn new(name: Identifier) -> Self {
        Self {
            name,
            unstable: false,
            symbols: vec![],
        }
    }

    // Set the `unstable` flag to true if we are importing an unstable core package
    pub(crate) fn set_unstable(&mut self, identifier: &Identifier) {
        if identifier.name.eq(UNSTABLE_CORE_PACKAGE_KEYWORD) {
            self.unstable = true;
        }
    }

    // Recursively set all symbols we are importing from a core package
    pub(crate) fn set_symbols(&mut self, access: PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => {
                self.set_unstable(&package.name);
                self.set_symbols(package.access);
            }
            PackageAccess::Star(_) => unimplemented!("cannot import star from core package"),
            PackageAccess::Multiple(accesses) => {
                for access in accesses {
                    self.set_symbols(access);
                }
            }
            PackageAccess::Symbol(symbol) => self.symbols.push(symbol),
        }
    }
}

impl From<Package> for CorePackage {
    fn from(package: Package) -> Self {
        // Name of core package
        let mut core_package = Self::new(package.name);

        core_package.set_symbols(package.access);

        core_package
    }
}

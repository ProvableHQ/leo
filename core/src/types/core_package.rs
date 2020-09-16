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

use crate::{
    unstable::blake2s::{Blake2sCircuit, CORE_UNSTABLE_BLAKE2S_NAME},
    CoreCircuit,
    CoreSymbolList,
};
use leo_typed::{Identifier, ImportSymbol, Package, PackageAccess};

/// A core package dependency to be imported into a Leo program
#[derive(Debug, Clone)]
pub struct CorePackage {
    name: Identifier,
    unstable: bool,
    symbols: Vec<ImportSymbol>,
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
    pub(crate) fn set_unstable(&mut self) {
        self.unstable = true;
    }

    // Recursively set all symbols we are importing from a core package
    pub(crate) fn set_symbols(&mut self, access: PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => {
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

    // Resolve import symbols into core circuits and store them in the program context
    pub(crate) fn append_symbols(&self, symbols: &mut CoreSymbolList) {
        for symbol in &self.symbols {
            let symbol_name = symbol.symbol.name.as_str();
            let span = symbol.span.clone();

            // take the alias if it is present
            let id = symbol.alias.clone().unwrap_or(symbol.symbol.clone());
            let name = id.name.clone();

            let circuit = if self.unstable {
                // match unstable core circuit
                match symbol_name {
                    CORE_UNSTABLE_BLAKE2S_NAME => Blake2sCircuit::ast(symbol.symbol.clone(), span),
                    _ => unimplemented!("unstable core circuit `{}` not implemented", symbol_name),
                }
            } else {
                // match core circuit
                match symbol_name {
                    _ => unimplemented!("core circuit `{}` not implemented", symbol_name),
                }
            };

            symbols.push(name, circuit)
        }
    }
}

impl From<Package> for CorePackage {
    fn from(package: Package) -> Self {
        // Create new core package
        let mut core_package = Self::new(package.name);

        // Fetch all circuit symbols imported from core package
        core_package.set_symbols(package.access);

        core_package
    }
}

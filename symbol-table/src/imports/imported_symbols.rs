// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_ast::{ImportStatement, ImportSymbol, Package, PackageAccess};

/// Stores the the package file name and imported symbol from an import statement
#[derive(Debug)]
pub struct ImportedSymbols {
    pub symbols: Vec<(String, ImportSymbol)>,
}

impl ImportedSymbols {
    pub fn new(import: &ImportStatement) -> Self {
        let mut imported_symbols = Self::default();

        imported_symbols.push_package(&import.package);

        imported_symbols
    }

    fn push_package(&mut self, package: &Package) {
        self.push_package_access(package.name.name.clone(), &package.access);
    }

    fn push_package_access(&mut self, package: String, access: &PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => self.push_package(package),
            PackageAccess::Star(span) => {
                let star = ImportSymbol::star(span);
                self.symbols.push((package, star));
            }
            PackageAccess::Symbol(symbol) => self.symbols.push((package, symbol.clone())),
            PackageAccess::Multiple(packages) => packages
                .iter()
                .for_each(|access| self.push_package_access(package.clone(), access)),
        }
    }
}

impl Default for ImportedSymbols {
    fn default() -> Self {
        Self { symbols: Vec::new() }
    }
}

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

use leo_typed::{ImportStatement, ImportSymbol, Package, PackageAccess};

/// Stores the the package file name and imported symbol from an import statement
#[derive(Debug)]
pub struct ImportedSymbols {
    pub symbols: Vec<(String, ImportSymbol)>,
}

impl ImportedSymbols {
    fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub fn from(import: &ImportStatement) -> Self {
        let mut symbols = Self::new();

        symbols.from_package(&import.package);

        symbols
    }

    fn from_package(&mut self, package: &Package) {
        self.from_package_access(package.name.name.clone(), &package.access);
    }

    fn from_package_access(&mut self, package: String, access: &PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => self.from_package(package),
            PackageAccess::Star(span) => {
                let star = ImportSymbol::star(span);
                self.symbols.push((package, star));
            }
            PackageAccess::Symbol(symbol) => self.symbols.push((package, symbol.clone())),
            PackageAccess::Multiple(packages) => packages
                .iter()
                .for_each(|access| self.from_package_access(package.clone(), access)),
        }
    }
}

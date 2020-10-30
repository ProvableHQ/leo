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

use crate::{new_scope, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_core_ast::Package;

use leo_core_packages::{CorePackageList, LeoCorePackageError};
use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_core_package(&mut self, scope: &str, package: Package) -> Result<(), LeoCorePackageError> {
        // Create list of imported core packages.
        let list = CorePackageList::from_package_access(package.access)?;

        // Fetch core packages from `leo-core`.
        let symbol_list = list.to_symbols()?;

        for (symbol, circuit) in symbol_list.symbols() {
            let symbol_name = new_scope(scope, symbol);

            // store packages
            self.store(symbol_name, ConstrainedValue::CircuitDefinition(circuit.to_owned()))
        }

        Ok(())
    }
}

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
use leo_typed::{Identifier, ImportSymbol, Package, PackageAccess};

use leo_core::{blake2s::unstable::hash::Blake2sFunction, CorePackageList};
use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_core_package(&mut self, scope: String, package: Package) {
        println!("storing core package: {}", package);
        // create core package list
        println!("creating core package list");
        let list = CorePackageList::from_package_access(package.access);

        println!("{:?}", list);

        // fetch packages from `leo-core`
        println!("fetching packages from leo core");
        let symbol_list = list.to_symbols();

        for (symbol, circuit) in symbol_list.symbols() {
            let symbol_name = new_scope(scope.clone(), symbol);

            // store packages
            println!("storing dependencies from leo core into leo program");
            println!("{}", symbol_name);
            self.store(symbol_name, ConstrainedValue::CircuitDefinition(circuit))
        }
    }
}

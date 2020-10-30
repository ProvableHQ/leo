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
    CoreCircuitStructList,
    CorePackageError,
};
use leo_core_ast::{Identifier, ImportSymbol, Package, PackageAccess};
use std::convert::TryFrom;

/// A core package dependency to be imported into a Leo program.
/// Each `CorePackage` contains one or more `CoreCircuit`s that can be accessed by name.
#[derive(Debug, Clone)]
pub struct CorePackage {
    name: Identifier,
    unstable: bool,
    circuits: Vec<ImportSymbol>,
}

impl CorePackage {
    pub(crate) fn new(name: Identifier) -> Self {
        Self {
            name,
            unstable: false,
            circuits: vec![],
        }
    }

    // Set the `unstable` flag to true if we are importing an unstable core package
    pub(crate) fn set_unstable(&mut self) {
        self.unstable = true;
    }

    // Stores all `CoreCircuit` names that are being accessed in the current `CorePackage`
    fn get_circuit_names(&mut self, access: PackageAccess) -> Result<(), CorePackageError> {
        match access {
            PackageAccess::SubPackage(package) => return self.get_circuit_names(package.access),
            PackageAccess::Star(span) => return Err(CorePackageError::core_package_star(span)),
            PackageAccess::Multiple(accesses) => {
                for access in accesses {
                    self.get_circuit_names(access)?;
                }
            }
            PackageAccess::Symbol(symbol) => self.circuits.push(symbol),
        }
        Ok(())
    }

    // Stores all `CoreCircuit` structs that are being accessed in the current `CorePackage`
    pub(crate) fn get_circuit_structs(
        &self,
        circuit_structs: &mut CoreCircuitStructList,
    ) -> Result<(), CorePackageError> {
        for circuit in &self.circuits {
            let circuit_name = circuit.symbol.name.as_str();
            let span = circuit.span.clone();

            // take the alias if it is present
            let id = circuit.alias.clone().unwrap_or_else(|| circuit.symbol.clone());
            let name = id.name.clone();

            let circuit = if self.unstable {
                // match unstable core circuit
                match circuit_name {
                    CORE_UNSTABLE_BLAKE2S_NAME => Blake2sCircuit::ast(circuit.symbol.clone(), span),
                    name => {
                        return Err(CorePackageError::undefined_unstable_core_circuit(
                            name.to_string(),
                            span,
                        ));
                    }
                }
            } else {
                // match core circuit
                return Err(CorePackageError::undefined_core_circuit(circuit_name.to_string(), span));
            };

            circuit_structs.push(name, circuit)
        }

        Ok(())
    }
}

impl TryFrom<Package> for CorePackage {
    type Error = CorePackageError;

    fn try_from(package: Package) -> Result<Self, Self::Error> {
        // Create new core package
        let mut core_package = Self::new(package.name);

        // Fetch all circuit symbols imported from core package
        core_package.get_circuit_names(package.access)?;

        Ok(core_package)
    }
}

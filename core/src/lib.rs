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

pub mod core_circuit;
pub use self::core_circuit::*;

pub mod errors;
pub use self::errors::*;

pub mod unstable;
pub use self::unstable::*;

use crate::unstable::blake2s::Blake2sFunction;
use leo_gadgets::signed_integer::*;
use leo_typed::{Circuit, Identifier, ImportSymbol, Package, PackageAccess, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            boolean::Boolean,
            uint::{UInt128, UInt16, UInt32, UInt64, UInt8},
        },
    },
};

static UNSTABLE_CORE_PACKAGE_KEYWORD: &str = "unstable";

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
            // take the alias if it is present
            let id = symbol.alias.clone().unwrap_or(symbol.symbol.clone());

            let name = id.name.clone();
            let span = symbol.span.clone();

            // todo: remove hardcoded blake2s circuit
            let blake2s_circuit = Blake2sFunction::ast(symbol.symbol.clone(), span);

            symbols.push(name, blake2s_circuit)
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

/// A list of core package dependencies
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

    // Return a list of all symbols that need to be stored in the current function
    pub fn to_symbols(&self) -> CoreSymbolList {
        let mut symbols = CoreSymbolList::new();

        for package in &self.packages {
            package.append_symbols(&mut symbols);
        }

        symbols
    }

    // Parse all dependencies after `core.`
    pub fn from_package_access(access: PackageAccess) -> Self {
        let mut new = Self::new();

        package_access_helper(&mut new, access, false);

        new
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

/// List of imported core function circuits
pub struct CoreSymbolList {
    /// [(circuit_name, circuit_struct)]
    symbols: Vec<(String, Circuit)>,
}

impl CoreSymbolList {
    pub(crate) fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub(crate) fn push(&mut self, name: String, circuit: Circuit) {
        self.symbols.push((name, circuit))
    }

    pub fn symbols(&self) -> Vec<(String, Circuit)> {
        self.symbols.clone()
    }
}

/// Calls a core function by it's given name.
/// This function should be called by the compiler when enforcing the result of calling a core circuit function.
pub fn call_core_function<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    function_name: String,
    arguments: Vec<Value>,
    span: Span, // todo: return errors using `leo-typed` span
) -> Vec<Value> {
    // Match core function name here
    if function_name.ne("core_blake2s_unstable") {
        // todo: convert this to a real error
        println!("core dne error");
    }
    // Hardcode blake2s core function call
    let res = Blake2sFunction::call(cs, arguments, span);

    return res;
}

/// An intermediate value format that can be converted into a `ConstrainedValue` for the compiler
/// Todo: implement other constrained values
#[derive(Clone)]
pub enum Value {
    Boolean(Boolean),

    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),

    I8(Int8),
    I16(Int16),
    I32(Int32),
    I64(Int64),
    I128(Int128),

    Array(Vec<Value>),
    Tuple(Vec<Value>),
}

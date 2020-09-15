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

use crate::blake2s::unstable::hash::Blake2sFunction;
use leo_typed::{
    Circuit,
    CircuitMember,
    Expression,
    Function,
    FunctionInput,
    Identifier,
    ImportSymbol,
    InputVariable,
    IntegerType,
    Package,
    PackageAccess,
    Statement,
    Type,
};

pub mod blake2s;
pub use self::blake2s::*;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::uint::UInt8},
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

    // Resolve import symbols into core circuits and store them in the program context
    pub(crate) fn append_symbols(&self, symbols: &mut CoreSymbolList) {
        for symbol in &self.symbols {
            //todo: resolve symbol alias if any
            let name = symbol.symbol.name.clone();
            let span = symbol.span.clone();

            /* Hardcode blake2s circuit for now
             * circuit Blake2s {
             *     static function hash(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
             *         // call `check_eval_gadget` in snarkOS
             *         return check_eval_gadget(seed, message)
             *     }
             */
            let blake2s_circuit = Circuit {
                circuit_name: symbol.symbol.clone(),
                members: vec![CircuitMember::CircuitFunction(
                    true, // static function
                    Function {
                        identifier: Identifier {
                            name: "hash".to_owned(),
                            span: span.clone(),
                        },
                        input: vec![
                            InputVariable::FunctionInput(FunctionInput {
                                identifier: Identifier {
                                    name: "seed".to_owned(),
                                    span: span.clone(),
                                },
                                mutable: false,
                                type_: Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize]),
                                span: span.clone(),
                            }),
                            InputVariable::FunctionInput(FunctionInput {
                                identifier: Identifier {
                                    name: "message".to_owned(),
                                    span: span.clone(),
                                },
                                mutable: false,
                                type_: Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize]),
                                span: span.clone(),
                            }),
                        ],
                        returns: Some(Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize])),
                        statements: vec![Statement::Return(
                            Expression::CoreFunctionCall("core_blake2s_unstable".to_owned(), vec![
                                Expression::Identifier(Identifier {
                                    name: "seed".to_owned(),
                                    span: span.clone(),
                                }),
                                Expression::Identifier(Identifier {
                                    name: "message".to_owned(),
                                    span: span.clone(),
                                }),
                            ]),
                            span.clone(),
                        )],
                        span: span.clone(),
                    },
                )],
            };

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

        match access {
            PackageAccess::Symbol(_symbol) => unimplemented!("cannot import a symbol directly from Leo core"),
            PackageAccess::Multiple(_) => unimplemented!("multiple imports not yet implemented for Leo core"),
            PackageAccess::SubPackage(package) => {
                let core_package = CorePackage::from(*package);

                new.push(core_package);
            }
            PackageAccess::Star(_) => unimplemented!("cannot import star from Leo core"),
        }

        new
    }
}

/// List of imported core function symbols and methods
pub struct CoreSymbolList {
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

pub struct CoreFunctionArgument(pub Vec<UInt8>);

/// Calls a core function by it's given name.
/// This function should be called by the compiler when enforcing the result of calling a core circuit function.
pub fn call_core_function<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    function_name: String,
    arguments: Vec<CoreFunctionArgument>,
    //_span: Span // todo: return errors using `leo-typed` span
) -> Vec<UInt8> {
    // Match core function name here
    if function_name.ne("core_blake2s_unstable") {
        // todo: convert this to a real error
        println!("core dne error");
    }
    // Hardcode blake2s core function call
    let res = Blake2sFunction::hash(cs, arguments);

    return res;
}

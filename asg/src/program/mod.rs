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

//! This module defines the program node for an asg.
//!
//!

mod circuit;
pub use circuit::*;

mod function;
pub use function::*;

use crate::{AsgConvertError, ImportResolver, InnerScope, Input, Scope};
use leo_ast::{Identifier, PackageAccess, PackageType, Span};

use indexmap::IndexMap;
use std::{cell::RefCell, sync::Arc};
use uuid::Uuid;

/// Stores the Leo program abstract semantic graph (ASG).
#[derive(Clone)]
pub struct InnerProgram {
    /// The unique id of the program.
    pub id: Uuid,

    /// The program file name.
    pub name: String,

    /// The packages imported by this program.
    /// these should generally not be accessed directly, but through scoped imports
    pub imported_modules: IndexMap<String, Program>,

    /// Maps test name => test code block.
    pub test_functions: IndexMap<String, (Arc<FunctionBody>, Option<Identifier>)>, // identifier = test input file

    /// Maps function name => function code block.
    pub functions: IndexMap<String, Arc<FunctionBody>>,

    /// Maps circuit name => circuit code block.
    pub circuits: IndexMap<String, Arc<CircuitBody>>,

    /// Bindings for names and additional program context.
    pub scope: Scope,
}

pub type Program = Arc<RefCell<InnerProgram>>;

/// Enumerates what names are imported from a package.
enum ImportSymbol {
    /// Import the symbol by name.
    Direct(String),

    /// Import the symbol by name and store it under an alias.
    Alias(String, String), // from remote -> to local

    /// Import all symbols from the package.
    All,
}

fn resolve_import_package(
    output: &mut Vec<(Vec<String>, ImportSymbol, Span)>,
    mut package_segments: Vec<String>,
    package_type: &PackageType,
) {
    match package_type {
        PackageType::Package(package) => {
            package_segments.push(package.name.name.clone());
            resolve_import_package_access(output, package_segments, &package.access);
        }
        PackageType::Packages(packages) => {
            package_segments.push(packages.name.name.clone());
            for access in packages.accesses.clone() {
                resolve_import_package_access(output, package_segments.clone(), &access);
            }
        }
    }
}

fn resolve_import_package_access(
    output: &mut Vec<(Vec<String>, ImportSymbol, Span)>,
    mut package_segments: Vec<String>,
    package: &PackageAccess,
) {
    match package {
        PackageAccess::Star(span) => {
            output.push((package_segments, ImportSymbol::All, span.clone()));
        }
        PackageAccess::SubPackage(subpackage) => {
            resolve_import_package(output, package_segments, &PackageType::Package(*(*subpackage).clone()));
        }
        PackageAccess::Symbol(symbol) => {
            let span = symbol.symbol.span.clone();
            let symbol = if let Some(alias) = symbol.alias.as_ref() {
                ImportSymbol::Alias(symbol.symbol.name.clone(), alias.name.clone())
            } else {
                ImportSymbol::Direct(symbol.symbol.name.clone())
            };
            output.push((package_segments, symbol, span));
        }
        PackageAccess::Multiple(packages) => {
            package_segments.push(packages.name.name.clone());
            for subaccess in packages.accesses.iter() {
                resolve_import_package_access(output, package_segments.clone(), &subaccess);
            }
        }
    }
}

impl InnerProgram {
    /// Returns a new Leo program asg from the given Leo program ast and imports.
    ///
    /// stages:
    /// 1. resolve imports into super scope
    /// 2. finalize declared types
    /// 3. finalize declared functions
    /// 4. resolve all asg nodes
    ///
    pub fn new<T: ImportResolver + 'static>(
        value: &leo_ast::Program,
        import_resolver: &mut T,
    ) -> Result<Program, AsgConvertError> {
        // Recursively extract imported symbols.
        let mut imported_symbols: Vec<(Vec<String>, ImportSymbol, Span)> = vec![];
        for import in value.imports.iter() {
            resolve_import_package(&mut imported_symbols, vec![], &import.package_type);
        }

        // Create package list.
        let mut deduplicated_imports: IndexMap<Vec<String>, Span> = IndexMap::new();
        for (package, _symbol, span) in imported_symbols.iter() {
            deduplicated_imports.insert(package.clone(), span.clone());
        }

        let mut wrapped_resolver = crate::CoreImportResolver(import_resolver);

        // Load imported programs.
        let mut resolved_packages: IndexMap<Vec<String>, Program> = IndexMap::new();
        for (package, span) in deduplicated_imports.iter() {
            let pretty_package = package.join(".");

            let resolved_package =
                match wrapped_resolver.resolve_package(&package.iter().map(|x| &**x).collect::<Vec<_>>()[..], span)? {
                    Some(x) => x,
                    None => return Err(AsgConvertError::unresolved_import(&*pretty_package, &Span::default())),
                };

            resolved_packages.insert(package.clone(), resolved_package);
        }

        let mut imported_functions: IndexMap<String, Arc<FunctionBody>> = IndexMap::new();
        let mut imported_circuits: IndexMap<String, Arc<CircuitBody>> = IndexMap::new();

        // Prepare locally relevant scope of imports.
        for (package, symbol, span) in imported_symbols.into_iter() {
            let pretty_package = package.join(".");

            let resolved_package = resolved_packages
                .get(&package)
                .expect("could not find preloaded package");
            let resolved_package = resolved_package.borrow();
            match symbol {
                ImportSymbol::All => {
                    imported_functions.extend(resolved_package.functions.clone().into_iter());
                    imported_circuits.extend(resolved_package.circuits.clone().into_iter());
                }
                ImportSymbol::Direct(name) => {
                    if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(name.clone(), function.clone());
                    } else if let Some(function) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(name.clone(), function.clone());
                    } else {
                        return Err(AsgConvertError::unresolved_import(
                            &*format!("{}.{}", pretty_package, name),
                            &span,
                        ));
                    }
                }
                ImportSymbol::Alias(name, alias) => {
                    if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(alias.clone(), function.clone());
                    } else if let Some(function) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(alias.clone(), function.clone());
                    } else {
                        return Err(AsgConvertError::unresolved_import(
                            &*format!("{}.{}", pretty_package, name),
                            &span,
                        ));
                    }
                }
            }
        }

        let import_scope = Arc::new(RefCell::new(InnerScope {
            id: uuid::Uuid::new_v4(),
            parent_scope: None,
            circuit_self: None,
            variables: IndexMap::new(),
            functions: imported_functions
                .iter()
                .map(|(name, func)| (name.clone(), func.function.clone()))
                .collect(),
            circuits: imported_circuits
                .iter()
                .map(|(name, circuit)| (name.clone(), circuit.circuit.clone()))
                .collect(),
            function: None,
            input: None,
        }));

        // Prepare header-like scope entries.
        let mut proto_circuits = IndexMap::new();
        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = Circuit::init(circuit);

            proto_circuits.insert(name.name.clone(), asg_circuit);
        }

        let scope = Arc::new(RefCell::new(InnerScope {
            input: Some(Input::new(&import_scope)), // we use import_scope to avoid recursive scope ref here
            id: uuid::Uuid::new_v4(),
            parent_scope: Some(import_scope),
            circuit_self: None,
            variables: IndexMap::new(),
            functions: IndexMap::new(),
            circuits: proto_circuits
                .iter()
                .map(|(name, circuit)| (name.clone(), circuit.clone()))
                .collect(),
            function: None,
        }));

        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = proto_circuits.get(&name.name).unwrap();

            asg_circuit.clone().from_ast(&scope, &circuit)?;
        }

        let mut proto_test_functions = IndexMap::new();
        for (name, test_function) in value.tests.iter() {
            assert_eq!(name.name, test_function.function.identifier.name);
            let function = Arc::new(Function::from_ast(&scope, &test_function.function)?);

            proto_test_functions.insert(name.name.clone(), function);
        }

        let mut proto_functions = IndexMap::new();
        for (name, function) in value.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let asg_function = Arc::new(Function::from_ast(&scope, function)?);

            scope
                .borrow_mut()
                .functions
                .insert(name.name.clone(), asg_function.clone());
            proto_functions.insert(name.name.clone(), asg_function);
        }

        // Load concrete definitions.
        let mut test_functions = IndexMap::new();
        for (name, test_function) in value.tests.iter() {
            assert_eq!(name.name, test_function.function.identifier.name);
            let function = proto_test_functions.get(&name.name).unwrap();

            let body = Arc::new(FunctionBody::from_ast(
                &scope,
                &test_function.function,
                function.clone(),
            )?);
            function.body.replace(Arc::downgrade(&body));

            test_functions.insert(name.name.clone(), (body, test_function.input_file.clone()));
        }

        let mut functions = IndexMap::new();
        for (name, function) in value.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let asg_function = proto_functions.get(&name.name).unwrap();

            let body = Arc::new(FunctionBody::from_ast(&scope, function, asg_function.clone())?);
            asg_function.body.replace(Arc::downgrade(&body));

            functions.insert(name.name.clone(), body);
        }

        let mut circuits = IndexMap::new();
        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = proto_circuits.get(&name.name).unwrap();
            let body = Arc::new(CircuitBody::from_ast(&scope, circuit, asg_circuit.clone())?);
            asg_circuit.body.replace(Arc::downgrade(&body));

            circuits.insert(name.name.clone(), body);
        }

        Ok(Arc::new(RefCell::new(InnerProgram {
            id: Uuid::new_v4(),
            name: value.name.clone(),
            test_functions,
            functions,
            circuits,
            imported_modules: resolved_packages
                .into_iter()
                .map(|(package, program)| (package.join("."), program))
                .collect(),
            scope,
        })))
    }

    pub(crate) fn set_core_mapping(&self, mapping: &str) {
        for (_, circuit) in self.circuits.iter() {
            circuit.circuit.core_mapping.replace(Some(mapping.to_string()));
        }
    }
}

struct InternalIdentifierGenerator {
    next: usize,
}

impl Iterator for InternalIdentifierGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let out = format!("$_{}_", self.next);
        self.next += 1;
        Some(out)
    }
}
/// Returns an ast from the given asg program.
pub fn reform_ast(program: &Program) -> leo_ast::Program {
    let mut all_programs: IndexMap<String, Program> = IndexMap::new();
    let mut program_stack = program.borrow().imported_modules.clone();
    while let Some((module, program)) = program_stack.pop() {
        if all_programs.contains_key(&module) {
            continue;
        }
        all_programs.insert(module, program.clone());
        program_stack.extend(program.borrow().imported_modules.clone());
    }
    all_programs.insert("".to_string(), program.clone());
    let core_programs: Vec<_> = all_programs
        .iter()
        .filter(|(module, _)| module.starts_with("core."))
        .map(|(module, program)| (module.clone(), program.clone()))
        .collect();
    all_programs.retain(|module, _| !module.starts_with("core."));

    let mut all_circuits: IndexMap<String, Arc<CircuitBody>> = IndexMap::new();
    let mut all_functions: IndexMap<String, Arc<FunctionBody>> = IndexMap::new();
    let mut all_test_functions: IndexMap<String, (Arc<FunctionBody>, Option<Identifier>)> = IndexMap::new();
    let mut identifiers = InternalIdentifierGenerator { next: 0 };
    for (_, program) in all_programs.into_iter() {
        let program = program.borrow();
        for (name, circuit) in program.circuits.iter() {
            let identifier = format!("{}{}", identifiers.next().unwrap(), name);
            circuit.circuit.name.borrow_mut().name = identifier.clone();
            all_circuits.insert(identifier, circuit.clone());
        }
        for (name, function) in program.functions.iter() {
            let identifier = if name == "main" {
                "main".to_string()
            } else {
                format!("{}{}", identifiers.next().unwrap(), name)
            };
            function.function.name.borrow_mut().name = identifier.clone();
            all_functions.insert(identifier, function.clone());
        }
        for (name, function) in program.test_functions.iter() {
            let identifier = format!("{}{}", identifiers.next().unwrap(), name);
            function.0.function.name.borrow_mut().name = identifier.clone();
            all_test_functions.insert(identifier, function.clone());
        }
    }

    leo_ast::Program {
        name: "ast_aggregate".to_string(),
        imports: core_programs
            .iter()
            .map(|(module, _)| leo_ast::ImportStatement {
                package_type: leo_ast::PackageType::Package(leo_ast::Package {
                    name: Identifier::new(module.clone()),
                    access: leo_ast::PackageAccess::Star(Span::default()),
                    span: Default::default(),
                }),
                span: Span::default(),
            })
            .collect(),
        expected_input: vec![],
        tests: all_test_functions
            .into_iter()
            .map(|(_, (function, ident))| {
                (function.function.name.borrow().clone(), leo_ast::TestFunction {
                    function: function.function.as_ref().into(),
                    input_file: ident,
                })
            })
            .collect(),
        functions: all_functions
            .into_iter()
            .map(|(_, function)| {
                (
                    function.function.name.borrow().clone(),
                    function.function.as_ref().into(),
                )
            })
            .collect(),
        circuits: all_circuits
            .into_iter()
            .map(|(_, circuit)| (circuit.circuit.name.borrow().clone(), circuit.circuit.as_ref().into()))
            .collect(),
    }
}

impl Into<leo_ast::Program> for &InnerProgram {
    fn into(self) -> leo_ast::Program {
        leo_ast::Program {
            name: self.name.clone(),
            imports: vec![],
            expected_input: vec![],
            circuits: self
                .circuits
                .iter()
                .map(|(_, circuit)| (circuit.circuit.name.borrow().clone(), circuit.circuit.as_ref().into()))
                .collect(),
            functions: self
                .functions
                .iter()
                .map(|(_, function)| {
                    (
                        function.function.name.borrow().clone(),
                        function.function.as_ref().into(),
                    )
                })
                .collect(),
            tests: self
                .test_functions
                .iter()
                .map(|(_, function)| {
                    (function.0.function.name.borrow().clone(), leo_ast::TestFunction {
                        function: function.0.function.as_ref().into(),
                        input_file: function.1.clone(),
                    })
                })
                .collect(),
        }
    }
}

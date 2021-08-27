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

mod alias;
pub use alias::*;

mod circuit;
pub use circuit::*;

mod function;
pub use function::*;

use crate::{node::FromAst, ArenaNode, AsgContext, DefinitionStatement, Input, Scope, Statement};
use leo_ast::{PackageAccess, PackageOrPackages};
use leo_errors::{AsgError, Result, Span};

use indexmap::IndexMap;
use std::cell::{Cell, RefCell};

/// Stores the Leo program abstract semantic graph (ASG).
#[derive(Clone)]
pub struct Program<'a> {
    pub context: AsgContext<'a>,

    /// The unique id of the program.
    pub id: u32,

    /// The program file name.
    pub name: String,

    /// The packages imported by this program.
    /// these should generally not be accessed directly, but through scoped imports
    pub imported_modules: IndexMap<String, Program<'a>>,

    /// Maps alias name => alias definition.
    pub aliases: IndexMap<String, &'a Alias<'a>>,

    /// Maps function name => function code block.
    pub functions: IndexMap<String, &'a Function<'a>>,

    /// Maps global constant name => global const code block.
    pub global_consts: IndexMap<String, &'a DefinitionStatement<'a>>,

    /// Maps circuit name => circuit code block.
    pub circuits: IndexMap<String, &'a Circuit<'a>>,

    pub scope: &'a Scope<'a>,
}

/// Enumerates what names are imported from a package.
#[derive(Clone)]
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
    package_or_packages: &PackageOrPackages,
) {
    match package_or_packages {
        PackageOrPackages::Package(package) => {
            package_segments.push(package.name.name.to_string());
            resolve_import_package_access(output, package_segments, &package.access);
        }
        PackageOrPackages::Packages(packages) => {
            package_segments.push(packages.name.name.to_string());
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
        PackageAccess::Star { span } => {
            output.push((package_segments, ImportSymbol::All, span.clone()));
        }
        PackageAccess::SubPackage(subpackage) => {
            resolve_import_package(
                output,
                package_segments,
                &PackageOrPackages::Package(*(*subpackage).clone()),
            );
        }
        PackageAccess::Symbol(symbol) => {
            let span = symbol.symbol.span.clone();
            let symbol = if let Some(alias) = symbol.alias.as_ref() {
                ImportSymbol::Alias(symbol.symbol.name.to_string(), alias.name.to_string())
            } else {
                ImportSymbol::Direct(symbol.symbol.name.to_string())
            };
            output.push((package_segments, symbol, span));
        }
        PackageAccess::Multiple(packages) => {
            package_segments.push(packages.name.name.to_string());
            for subaccess in packages.accesses.iter() {
                resolve_import_package_access(output, package_segments.clone(), subaccess);
            }
        }
    }
}

impl<'a> Program<'a> {
    /// Returns a new Leo program ASG from the given Leo program AST and its imports.
    ///
    /// Stages:
    /// 1. resolve imports into super scope
    /// 2. finalize declared types
    /// 3. finalize declared functions
    /// 4. resolve all asg nodes
    ///
    pub fn new(context: AsgContext<'a>, program: &leo_ast::Program) -> Result<Program<'a>> {
        // Convert each sub AST.
        let mut imported_modules: IndexMap<Vec<String>, Program> = IndexMap::new();
        for (package, program) in program.imports.iter() {
            imported_modules.insert(package.clone(), Program::new(context, program)?);
        }

        let mut imported_symbols: Vec<(Vec<String>, ImportSymbol, Span)> = vec![];
        for import_statement in program.import_statements.iter() {
            resolve_import_package(&mut imported_symbols, vec![], &import_statement.package_or_packages);
        }

        let mut deduplicated_imports: IndexMap<Vec<String>, Span> = IndexMap::new();
        for (package, _symbol, span) in imported_symbols.iter() {
            deduplicated_imports.insert(package.clone(), span.clone());
        }

        let mut imported_aliases: IndexMap<String, &'a Alias<'a>> = IndexMap::new();
        let mut imported_functions: IndexMap<String, &'a Function<'a>> = IndexMap::new();
        let mut imported_circuits: IndexMap<String, &'a Circuit<'a>> = IndexMap::new();
        let mut imported_global_consts: IndexMap<String, &'a DefinitionStatement<'a>> = IndexMap::new();

        for (package, symbol, span) in imported_symbols.into_iter() {
            let pretty_package = package.join(".");

            let resolved_package = imported_modules
                .get_mut(&package)
                .expect("could not find preloaded package");

            match symbol {
                ImportSymbol::All => {
                    imported_aliases.extend(resolved_package.aliases.clone().into_iter());
                    imported_functions.extend(resolved_package.functions.clone().into_iter());
                    imported_circuits.extend(resolved_package.circuits.clone().into_iter());
                    imported_global_consts.extend(resolved_package.global_consts.clone().into_iter());
                }
                ImportSymbol::Direct(name) => {
                    if let Some(alias) = resolved_package.aliases.get(&name) {
                        imported_aliases.insert(name.clone(), *alias);
                    } else if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(name.clone(), *function);
                    } else if let Some(circuit) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(name.clone(), *circuit);
                    } else if let Some(global_const) = resolved_package.global_consts.get(&name) {
                        imported_global_consts.insert(name.clone(), *global_const);
                    } else {
                        return Err(AsgError::unresolved_import(pretty_package, &span).into());
                    }
                }
                ImportSymbol::Alias(name, alias) => {
                    if let Some(type_alias) = resolved_package.aliases.get(&name) {
                        imported_aliases.insert(alias.clone(), *type_alias);
                    } else if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(alias.clone(), *function);
                    } else if let Some(circuit) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(alias.clone(), *circuit);
                    } else if let Some(global_const) = resolved_package.global_consts.get(&name) {
                        imported_global_consts.insert(alias.clone(), *global_const);
                    } else {
                        return Err(AsgError::unresolved_import(pretty_package, &span).into());
                    }
                }
            }
        }

        let import_scope = match context.arena.alloc(ArenaNode::Scope(Box::new(Scope {
            context,
            id: context.get_id(),
            parent_scope: Cell::new(None),
            variables: RefCell::new(IndexMap::new()),
            aliases: RefCell::new(imported_aliases),
            functions: RefCell::new(imported_functions),
            global_consts: RefCell::new(imported_global_consts),
            circuits: RefCell::new(imported_circuits),
            function: Cell::new(None),
            input: Cell::new(None),
        }))) {
            ArenaNode::Scope(c) => c,
            _ => unimplemented!(),
        };

        let scope = import_scope.context.alloc_scope(Scope {
            context,
            input: Cell::new(Some(Input::new(import_scope))), // we use import_scope to avoid recursive scope ref here
            id: context.get_id(),
            parent_scope: Cell::new(Some(import_scope)),
            variables: RefCell::new(IndexMap::new()),
            aliases: RefCell::new(IndexMap::new()),
            functions: RefCell::new(IndexMap::new()),
            global_consts: RefCell::new(IndexMap::new()),
            circuits: RefCell::new(IndexMap::new()),
            function: Cell::new(None),
        });

        // Prepare header-like scope entries.
        // Have to do aliases first.
        for (name, alias) in program.aliases.iter() {
            assert_eq!(name.name, alias.name.name);

            let asg_alias = Alias::init(scope, alias)?;
            scope.aliases.borrow_mut().insert(name.name.to_string(), asg_alias);
        }

        for (names, global_const) in program.global_consts.iter() {
            let gc = <&Statement<'a>>::from_ast(scope, global_const, None)?;
            if let Statement::Definition(def) = gc {
                let name = names
                    .iter()
                    .enumerate()
                    .map(|(i, name)| {
                        assert_eq!(name.name, def.variables.get(i).unwrap().borrow().name.name);
                        name.name.to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(",");

                scope.global_consts.borrow_mut().insert(name, def);
            }
        }

        for (name, circuit) in program.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = Circuit::init(scope, circuit)?;

            scope.circuits.borrow_mut().insert(name.name.to_string(), asg_circuit);
        }

        // Second pass for circuit members.
        for (name, circuit) in program.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = Circuit::init_member(scope, circuit)?;

            scope.circuits.borrow_mut().insert(name.name.to_string(), asg_circuit);
        }

        for (name, function) in program.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let function = Function::init(scope, function)?;

            scope.functions.borrow_mut().insert(name.name.to_string(), function);
        }

        // Load concrete definitions.
        let mut aliases = IndexMap::new();
        let mut global_consts = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut circuits = IndexMap::new();

        /* let check_global_shadowing = |name: String, span: &Span| -> Result<()> {
            if aliases.contains_key(&name) {
                return Err(AsgError::duplicate_alias_definition(name, span).into());
            } else if global_consts.contains_key(&name) {
                return Err(AsgError::duplicate_global_const_definition(name, span).into());
            } else if functions.contains_key(&name) {
                return Err(AsgError::duplicate_function_definition(name, span).into());
            } else if circuits.contains_key(&name) {
                return Err(AsgError::duplicate_circuit_definition(name, span).into());
            } else {
                Ok(())
            }
        }; */

        for (name, alias) in program.aliases.iter() {
            assert_eq!(name.name, alias.name.name);
            let asg_alias = *scope.aliases.borrow().get(name.name.as_ref()).unwrap();

            let name = name.name.to_string();

            if aliases.contains_key(&name) {
                return Err(AsgError::duplicate_alias_definition(name, &alias.span).into());
            }

            aliases.insert(name, asg_alias);
        }

        for (names, global_const) in program.global_consts.iter() {
            for (identifier, variable) in names.iter().zip(global_const.variable_names.iter()) {
                assert_eq!(identifier.name, variable.identifier.name);

                let name = identifier.name.to_string();
                let asg_global_const = *scope.global_consts.borrow().get(&name).unwrap();

                if global_consts.contains_key(&name) {
                    return Err(AsgError::duplicate_global_const_definition(name, &global_const.span).into());
                }

                global_consts.insert(name.clone(), asg_global_const);
            }
        }

        for (name, function) in program.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let asg_function = *scope.functions.borrow().get(name.name.as_ref()).unwrap();

            asg_function.fill_from_ast(function)?;

            let name = name.name.to_string();

            if functions.contains_key(&name) {
                return Err(AsgError::duplicate_function_definition(name, &function.span).into());
            }

            functions.insert(name, asg_function);
        }

        for (name, circuit) in program.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = *scope.circuits.borrow().get(name.name.as_ref()).unwrap();

            asg_circuit.fill_from_ast(circuit)?;

            circuits.insert(name.name.to_string(), asg_circuit);
        }

        Ok(Program {
            context,
            id: context.get_id(),
            name: program.name.clone(),
            aliases,
            functions,
            global_consts,
            circuits,
            imported_modules: imported_modules
                .into_iter()
                .map(|(package, program)| (package.join("."), program))
                .collect(),
            scope,
        })
    }
}

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

use crate::resolver::*;

use leo_ast::*;
use leo_errors::{AstError, Result, Span};

use indexmap::IndexMap;

pub struct Importer<T>
where
    T: ImportResolver,
{
    import_resolver: T,
}

impl<T> Importer<T>
where
    T: ImportResolver,
{
    pub fn new(import_resolver: T) -> Self {
        Self { import_resolver }
    }

    pub fn do_pass(ast: Program, importer: T) -> Result<Ast> {
        Ok(Ast::new(
            ReconstructingDirector::new(Importer::new(importer)).reduce_program(&ast)?,
        ))
    }
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

impl<T> ReconstructingReducer for Importer<T>
where
    T: ImportResolver,
{
    fn in_circuit(&self) -> bool {
        false
    }

    fn swap_in_circuit(&mut self) {}

    fn reduce_program(
        &mut self,
        program: &Program,
        expected_input: Vec<FunctionInput>,
        import_statements: Vec<ImportStatement>,
        empty_imports: IndexMap<String, Program>,
        aliases: IndexMap<Identifier, Alias>,
        circuits: IndexMap<Identifier, Circuit>,
        functions: IndexMap<Identifier, Function>,
        global_consts: IndexMap<String, DefinitionStatement>,
    ) -> Result<Program> {
        if !empty_imports.is_empty() {
            return Err(AstError::injected_programs(empty_imports.len()).into());
        }

        let mut imported_symbols: Vec<(Vec<String>, ImportSymbol, Span)> = vec![];
        for import_statement in import_statements.iter() {
            resolve_import_package(&mut imported_symbols, vec![], &import_statement.package_or_packages);
        }

        let mut deduplicated_imports: IndexMap<Vec<String>, Span> = IndexMap::new();
        for (package, _symbol, span) in imported_symbols.iter() {
            deduplicated_imports.insert(package.clone(), span.clone());
        }

        let mut wrapped_resolver = CoreImportResolver::new(&mut self.import_resolver);

        let mut resolved_packages: IndexMap<Vec<String>, Program> = IndexMap::new();
        for (package, span) in deduplicated_imports {
            let pretty_package = package.join(".");

            let resolved_package =
                match wrapped_resolver.resolve_package(&package.iter().map(|x| &**x).collect::<Vec<_>>()[..], &span)? {
                    Some(x) => x,
                    None => return Err(AstError::unresolved_import(pretty_package, &span).into()),
                };

            resolved_packages.insert(package.clone(), resolved_package);
        }

        // TODO copyable AST.
        /* for (package, symbol, span) in imported_symbols.into_iter() {
            let pretty_package = package.join(".");

            let resolved_package = resolved_packages
                .get_mut(&package)
                .expect("could not find preloaded package");

            match symbol {
                ImportSymbol::All => {
                    aliases.extend(resolved_package.aliases.clone().into_iter());
                    functions.extend(resolved_package.functions.clone().into_iter());
                    circuits.extend(resolved_package.circuits.clone().into_iter());
                    global_consts.extend(resolved_package.global_consts.clone().into_iter());
                }
                ImportSymbol::Direct(name) => {
                    if let Some(alias) = resolved_package.aliases.get(&name) {
                        aliases.insert(name.clone(), alias.clone());
                    } else if let Some(function) = resolved_package.functions.get(&name) {
                        functions.insert(name.clone(), function.clone());
                    } else if let Some(circuit) = resolved_package.circuits.get(&name) {
                        circuits.insert(name.clone(), circuit.clone());
                    } else if let Some(global_const) = resolved_package.global_consts.get(&name) {
                        global_consts.insert(name.clone(), global_const.clone());
                    } else {
                        return Err(AstError::unresolved_import(pretty_package, &span).into());
                    }
                }
                ImportSymbol::Alias(name, alias) => {
                    if let Some(type_alias) = resolved_package.aliases.get(&name) {
                        aliases.insert(alias.clone(), type_alias.clone());
                    } else if let Some(function) = resolved_package.functions.get(&name) {
                        functions.insert(alias.clone(), function.clone());
                    } else if let Some(circuit) = resolved_package.circuits.get(&name) {
                        circuits.insert(alias.clone(), circuit.clone());
                    } else if let Some(global_const) = resolved_package.global_consts.get(&name) {
                        global_consts.insert(alias.clone(), global_const.clone());
                    } else {
                        return Err(AstError::unresolved_import(pretty_package, &span).into());
                    }
                }
            }
        } */

        Ok(Program {
            name: program.name.clone(),
            expected_input,
            import_statements,
            imports: resolved_packages
                .into_iter()
                .map(|(package, program)| (package.join("."), program))
                .collect(),
            aliases,
            circuits,
            functions,
            global_consts,
        })
    }
}

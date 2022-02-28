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

pub struct Importer {}

impl Importer {
    pub fn do_pass<T>(program: Program, importer: &mut T) -> Result<Ast>
    where
        T: ImportResolver,
    {
        let mut ast = program.clone();
        ast.imports.extend(leo_stdlib::resolve_prelude_modules()?);

        let mut imported_symbols: Vec<(Vec<String>, ImportSymbol, Span)> = vec![];
        for import_statement in program.import_statements.iter() {
            resolve_import_package(&mut imported_symbols, vec![], &import_statement.package_or_packages);
        }

        let mut deduplicated_imports: IndexMap<Vec<String>, Span> = IndexMap::new();
        for (package, _symbol, span) in imported_symbols.iter() {
            deduplicated_imports.insert(package.clone(), span.clone());
        }

        let mut wrapped_resolver = CoreImportResolver::new(importer);

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

        ast.imports.extend(resolved_packages);

        Ok(Ast::new(ast))
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

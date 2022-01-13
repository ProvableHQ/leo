// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::Program;
use leo_ast_passes::ImportResolver;
use leo_errors::emitter::Handler;
use leo_errors::{ImportError, LeoError, Result};
use leo_span::{Span, Symbol};

use indexmap::{IndexMap, IndexSet};
use std::path::PathBuf;

/// Stores imported packages.
///
/// A program can import one or more packages. A package can be found locally in the source
/// directory, foreign in the imports directory, or part of the core package list.
#[derive(Clone)]
pub struct ImportParser<'a> {
    pub(crate) handler: &'a Handler,
    program_path: PathBuf,
    partial_imports: IndexSet<String>,
    imports: IndexMap<String, Program>,
    pub imports_map: IndexMap<String, String>,
}

impl<'a> ImportParser<'a> {
    pub fn default(handler: &'a Handler) -> Self {
        Self::new(handler, <_>::default(), <_>::default())
    }

    pub fn new(handler: &'a Handler, program_path: PathBuf, imports_map: IndexMap<String, String>) -> Self {
        ImportParser {
            handler,
            program_path,
            partial_imports: Default::default(),
            imports: Default::default(),
            imports_map,
        }
    }
}

impl ImportResolver for ImportParser<'_> {
    fn handler(&self) -> &Handler {
        self.handler
    }

    fn resolve_package(&mut self, package_segments: &[Symbol], span: &Span) -> Result<Option<Program>> {
        let package_segments = package_segments
            .iter()
            .map(|s| s.as_str().to_string())
            .collect::<Vec<_>>();
        let package_segments = package_segments.iter().map(|s| &**s).collect::<Vec<_>>();

        let full_path = package_segments.join(".");
        if self.partial_imports.contains(&full_path) {
            return Err(ImportError::recursive_imports(full_path, span).into());
        }

        if let Some(program) = self.imports.get(&full_path) {
            return Ok(Some(program.clone()));
        }

        let path = self.program_path.clone();
        self.partial_imports.insert(full_path.clone());
        let mut imports = self.clone(); // Self::default() was previously
        let program = imports
            .parse_package(path, &package_segments, span)
            .map_err(|x| -> LeoError { x })?;

        self.partial_imports.remove(&full_path);
        self.imports.insert(full_path, program.clone());

        Ok(Some(program))
    }
}

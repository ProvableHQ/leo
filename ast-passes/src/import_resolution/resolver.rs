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

use leo_ast::Program;
use leo_errors::{Result, Span};
use leo_stdlib::resolve_stdlib_module;

use indexmap::IndexMap;

pub trait ImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>>;
}

pub struct NullImportResolver;

impl ImportResolver for NullImportResolver {
    fn resolve_package(&mut self, _package_segments: &[&str], _span: &Span) -> Result<Option<Program>> {
        Ok(None)
    }
}

pub struct CoreImportResolver<'a, T: ImportResolver> {
    inner: &'a mut T,
    _curve: &'a str,
}

impl<'a, T: ImportResolver> CoreImportResolver<'a, T> {
    pub fn new(inner: &'a mut T, curve: &'a str) -> Self {
        CoreImportResolver { inner, _curve: curve }
    }
}

impl<'a, T: ImportResolver> ImportResolver for CoreImportResolver<'a, T> {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>> {
        if !package_segments.is_empty() && package_segments.get(0).unwrap() == &"std" {
            Ok(Some(resolve_stdlib_module(&*package_segments[1..].join("."))?))
        } else {
            self.inner.resolve_package(package_segments, span)
        }
    }
}

pub struct MockedImportResolver {
    pub packages: IndexMap<String, Program>,
}

impl ImportResolver for MockedImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], _span: &Span) -> Result<Option<Program>> {
        Ok(self.packages.get(&package_segments.join(".")).cloned())
    }
}

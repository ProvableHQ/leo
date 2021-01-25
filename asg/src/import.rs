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

use crate::{AsgConvertError, Program, Span};
use indexmap::IndexMap;

pub trait ImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>, AsgConvertError>;
}

pub struct NullImportResolver;

impl ImportResolver for NullImportResolver {
    fn resolve_package(
        &mut self,
        _package_segments: &[&str],
        _span: &Span,
    ) -> Result<Option<Program>, AsgConvertError> {
        Ok(None)
    }
}

pub struct CoreImportResolver<'a, T: ImportResolver + 'static>(pub &'a mut T);

impl<'a, T: ImportResolver + 'static> ImportResolver for CoreImportResolver<'a, T> {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>, AsgConvertError> {
        if !package_segments.is_empty() && package_segments.get(0).unwrap() == &"core" {
            Ok(crate::resolve_core_module(&*package_segments[1..].join("."))?)
        } else {
            self.0.resolve_package(package_segments, span)
        }
    }
}

pub struct StandardImportResolver;

impl ImportResolver for StandardImportResolver {
    fn resolve_package(
        &mut self,
        _package_segments: &[&str],
        _span: &Span,
    ) -> Result<Option<Program>, AsgConvertError> {
        Ok(None)
    }
}

pub struct MockedImportResolver {
    pub packages: IndexMap<String, Program>,
}

impl ImportResolver for MockedImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], _span: &Span) -> Result<Option<Program>, AsgConvertError> {
        Ok(self.packages.get(&package_segments.join(".")).cloned())
    }
}

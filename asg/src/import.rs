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

//! Helper methods for resolving imported packages.

use std::marker::PhantomData;

use crate::{AsgContext, Program};
use leo_errors::{Result, Span};

use indexmap::IndexMap;

pub trait ImportResolver<'a> {
    fn resolve_package(
        &mut self,
        context: AsgContext<'a>,
        package_segments: &[&str],
        span: &Span,
    ) -> Result<Option<Program<'a>>>;
}

pub struct NullImportResolver;

impl<'a> ImportResolver<'a> for NullImportResolver {
    fn resolve_package(
        &mut self,
        _context: AsgContext<'a>,
        _package_segments: &[&str],
        _span: &Span,
    ) -> Result<Option<Program<'a>>> {
        Ok(None)
    }
}

pub struct CoreImportResolver<'a, 'b, T: ImportResolver<'b>> {
    inner: &'a mut T,
    lifetime: PhantomData<&'b ()>,
}

impl<'a, 'b, T: ImportResolver<'b>> CoreImportResolver<'a, 'b, T> {
    pub fn new(inner: &'a mut T) -> Self {
        CoreImportResolver {
            inner,
            lifetime: PhantomData,
        }
    }
}

impl<'a, 'b, T: ImportResolver<'b>> ImportResolver<'b> for CoreImportResolver<'a, 'b, T> {
    fn resolve_package(
        &mut self,
        context: AsgContext<'b>,
        package_segments: &[&str],
        span: &Span,
    ) -> Result<Option<Program<'b>>> {
        if !package_segments.is_empty() && package_segments.get(0).unwrap() == &"core" {
            Ok(crate::resolve_core_module(context, &*package_segments[1..].join("."))?)
        } else {
            self.inner.resolve_package(context, package_segments, span)
        }
    }
}

pub struct MockedImportResolver<'a> {
    pub packages: IndexMap<String, Program<'a>>,
}

impl<'a> ImportResolver<'a> for MockedImportResolver<'a> {
    fn resolve_package(
        &mut self,
        _context: AsgContext<'a>,
        package_segments: &[&str],
        _span: &Span,
    ) -> Result<Option<Program<'a>>> {
        Ok(self.packages.get(&package_segments.join(".")).cloned())
    }
}

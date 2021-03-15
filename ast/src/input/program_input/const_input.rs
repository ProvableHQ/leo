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

use crate::InputValue;
use leo_input::{definitions::Definition, InputParserError};

use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ConstInput {
    input: IndexMap<String, Option<InputValue>>,
}

#[allow(clippy::len_without_is_empty)]
impl ConstInput {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let mut input = self.input.clone();

        input.iter_mut().for_each(|(_name, value)| {
            *value = None;
        });

        Self { input }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn insert(&mut self, key: String, value: Option<InputValue>) {
        self.input.insert(key, value);
    }

    /// Parses constant input definitions and stores them in `self`.
    pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        for definition in definitions {
            let name = definition.parameter.variable.value;
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            self.insert(name, Some(value));
        }

        Ok(())
    }

    /// Returns an `Option` of the constant main function input at `name`.
    pub fn get(&self, name: &str) -> Option<Option<InputValue>> {
        self.input.get(name).cloned()
    }
}

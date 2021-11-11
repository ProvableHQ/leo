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

/// Constructs an input section to store data parsed from a Leo input file.
/// Constructs sections that pass variables to the main function through the input keyword.
#[macro_export]
macro_rules! record_input_section {
    ($($name: ident), *) => ($(

        /// An input section declared in an input file with `[$name]`.
        #[derive(Clone, PartialEq, Eq, Default)]
        pub struct $name {
            is_present: bool,
            values: IndexMap<Parameter, Option<InputValue>>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            /// Returns an empty version of this struct with `None` values.
            /// Called during constraint synthesis to provide private input variables.
            pub fn empty(&self) -> Self {
                let is_present = self.is_present;
                let mut values = self.values.clone();

                values.iter_mut().for_each(|(_parameter, value)| {
                    *value = None;
                });

                Self { is_present, values }
            }

            /// Returns `true` if the main function contains the `$name` variable.
            pub fn is_present(&self) -> bool {
                self.is_present
            }

            /// Parses register input definitions and stores them in `self`.
            /// This function is called if the main function input contains the `$name` variable.
            pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
                self.is_present = true;

                for definition in definitions {
                    let value = InputValue::from_expression(definition.parameter.type_.clone(), definition.expression)?;
                    let parameter = Parameter::from(definition.parameter);

                    self.values.insert(parameter, Some(value));
                }

                Ok(())
            }

            /// Returns this section's [IndexMap] of values.
            pub fn values(&self) -> IndexMap<Parameter, Option<InputValue>> {
                self.values.clone()
            }

            /// a list of all defined name -> type pairs
            pub fn types(&self) -> Vec<(String, crate::Type)> {
                self.values.iter()
                    .map(|(parameter, _)| (parameter.variable.name.to_string(), parameter.type_.clone()))
                    .collect()
            }

            /// a map of all defined name -> value pairs, if present
            pub fn raw_values(&self) -> IndexMap<String, InputValue> {
                self.values.iter()
                    .filter(|(_, value)| value.is_some())
                    .map(|(parameter, value)| (parameter.variable.name.to_string(), value.as_ref().unwrap().clone()))
                    .collect()
            }
        }
    )*)
}

/// Constructs an input section to store data parsed from a Leo input file.
/// Constructs sections that pass variables directly to the main function.
#[macro_export]
macro_rules! main_input_section {
    ($($name: ident), *) => ($(

        /// `[$name]` program input section.
        #[derive(Clone, PartialEq, Eq, Default)]
        pub struct $name {
            input: IndexMap<String, Option<InputValue>>,
        }

        #[allow(clippy::len_without_is_empty)]
        impl $name {
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

            /// Parses main input definitions and stores them in `self`.
            pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
                for definition in definitions {
                    let name = definition.parameter.variable.value;
                    let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

                    self.insert(name, Some(value));
                }

                Ok(())
            }

            /// Returns an `Option` of the main function input at `name`.
            pub fn get(&self, name: &str) -> Option<Option<InputValue>> {
                self.input.get(name).cloned()
            }

            pub fn iter(&self) -> impl Iterator<Item=(&String, &Option<InputValue>)> {
                self.input.iter()
            }
        }
    )*)
}

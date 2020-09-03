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

use crate::ast::Rule;

use pest::error::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("aborting due to syntax error")]
    Error(Error<Rule>),
}

impl From<Error<Rule>> for SyntaxError {
    fn from(mut error: Error<Rule>) -> Self {
        error = error.renamed_rules(|rule| match *rule {
            Rule::LINE_END => "`;`".to_owned(),
            Rule::type_integer => "`u32`".to_owned(),
            Rule::type_field => "`field`".to_owned(),
            Rule::type_group => "`group`".to_owned(),
            Rule::file => "a table or section".to_owned(),
            Rule::identifier => "a variable name".to_owned(),
            Rule::type_ => "a type".to_owned(),
            Rule::header => "[[public]] or [[private]]".to_owned(),

            rule => format!("{:?}", rule),
        });

        SyntaxError::Error(error)
    }
}

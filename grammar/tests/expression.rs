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

use leo_grammar::ast::{LanguageParser, Rule};

use pest::*;

#[test]
fn redundant_parens() {
    parses_to! {
        parser: LanguageParser,
        input:  "(true)",
        rule:   Rule::expression,
        tokens: [
            expression(0, 6, [
                expression_term(0, 6, [expression(1, 5, [expression_term(1, 5, [value(1, 5, [value_boolean(1, 5, [])])])])])
            ])
        ]
    }
}

#[test]
fn multiple_redundant_parens() {
    parses_to! {
        parser: LanguageParser,
        input:  "(((true)))",
        rule:   Rule::expression,
        tokens: [
            expression(0, 10, [
                expression_term(0, 10, [
                    expression(1, 9, [expression_term(1, 9, [
                        expression(2, 8, [expression_term(2, 8, [
                            expression(3, 7, [expression_term(3, 7, [
                                value(3, 7, [value_boolean(3, 7, [])])
                            ])])
                        ])])
                    ])])
                ])
            ])
        ]
    }
}

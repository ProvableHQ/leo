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
fn implicitly_typed() {
    parses_to! {
        parser: LanguageParser,
        input:  "(true, false)",
        rule:   Rule::expression_tuple,
        tokens: [
            expression_tuple(0, 13, [
                expression(1, 5, [expression_term(1, 5, [value(1, 5, [value_boolean(1, 5, [])])])]),
                expression(7, 12, [expression_term(7, 12, [value(7, 12, [value_boolean(7, 12, [])])])])
            ])
        ]
    }
}

#[test]
fn explicitly_typed() {
    parses_to! {
        parser: LanguageParser,
        input:  "let tup: (bool, bool) = (true, false);",
        rule:   Rule::statement_definition,
        tokens: [
            statement_definition(0, 38, [
                declare(0, 4, [let_(0, 4, [])]),
                variables(4, 21, [
                    variable_name(4, 7, [identifier(4, 7, [])]),
                    type_(9, 21, [type_tuple(9, 21, [
                        type_(10, 14, [type_data(10, 14, [type_boolean(10, 14, [])])]),
                        type_(16, 20, [type_data(16, 20, [type_boolean(16, 20, [])])]),
                    ])])
                ]),
                expression(24, 37, [expression_term(24, 37, [expression_tuple(24, 37, [
                    expression(25, 29, [expression_term(25, 29, [value(25, 29, [value_boolean(25, 29, [])])])]),
                    expression(31, 36, [expression_term(31, 36, [value(31, 36, [value_boolean(31, 36, [])])])]),
                ])])]),
                LINE_END(37, 38, [])
            ])
        ]
    }
}

#[test]
fn access() {
    parses_to! {
        parser: LanguageParser,
        input:  "x.0",
        rule:   Rule::expression_postfix,
        tokens: [
            expression_postfix(0, 3, [
                keyword_or_identifier(0, 1, [self_keyword_or_identifier(0, 1, [identifier(0, 1, [])])]),
                access(1, 3, [access_tuple(1, 3, [number_positive(2, 3, [])])])
            ])
        ]
    }
}

#[test]
fn implicit_unit() {
    parses_to! {
        parser: LanguageParser,
        input:  "()",
        rule:   Rule::expression_tuple,
        tokens: [
            expression_tuple(0, 2, [])
        ]
    }
}

#[test]
fn explicit_unit() {
    parses_to! {
        parser: LanguageParser,
        input:  "let x: () = ();",
        rule:   Rule::statement_definition,
        tokens: [
            statement_definition(0, 15, [
                declare(0, 4, [let_(0, 4, [])]),
                variables(4, 9, [
                    variable_name(4, 5, [identifier(4, 5, [])]),
                    type_(7, 9, [type_tuple(7, 9, [])])
                ]),
                expression(12, 14, [expression_term(12, 14, [expression_tuple(12, 14, [])])]),
                LINE_END(14, 15, [])
            ])
        ]
    }
}

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
fn test_deprecated_test_function() {
    parses_to! {
        parser: LanguageParser,
        input:  r#"test function old() {
    return ()
}"#,
        rule:   Rule::deprecated,
        tokens: [
            deprecated(0, 37, [
                test_function(0, 37, [
                    function(5, 37, [
                        identifier(14, 17, []),
                        block(20, 37, [
                            statement(26, 36, [
                                statement_return(26, 36, [expression(33, 36, [expression_term(33, 35, [expression_tuple(33, 35, [])])])])
                            ])
                        ])
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn test_deprecated_context_function() {
    parses_to! {
        parser: LanguageParser,
        input:  "@context(custom)",
        rule:   Rule::annotation,
        tokens: [
            annotation(0, 16, [annotation_symbol(0, 1, []), annotation_name(1, 8, [context(1, 8, [])]), annotation_arguments(8, 16, [annotation_argument(9, 15, [])])])
        ]
    }
}

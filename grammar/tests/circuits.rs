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

use leo_grammar::ast::LanguageParser;
use leo_grammar::ast::Rule;

use pest::*;

#[test]
fn circuit_definition() {
    parses_to! {
        parser: LanguageParser,
        input:  "circuit Foo { a: u32, }",
        rule:   Rule::circuit,
        tokens: [
            circuit(0, 23, [
                identifier(8, 11, []),
                circuit_member(14, 21,
                    [circuit_variable_definition(14, 21, [
                        identifier(14, 15, []),
                        type_(17, 20, [type_data(17, 20, [type_integer(17, 20, [type_integer_unsigned(17, 20, [type_u32(17, 20, [])])])])])
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn circuit_instantiation() {
    parses_to! {
        parser: LanguageParser,
        input:  r#"circuit Foo { a: u32, }
    function main() { let foo = Foo { a, b: 1u32 }; }"#,
        rule:   Rule::file,
        tokens: [
            file(0, 77, [
                definition(0, 23, [
                    circuit(0, 23, [
                        identifier(8, 11, []),
                        circuit_member(14, 21,
                            [circuit_variable_definition(14, 21, [
                                identifier(14, 15, []),
                                type_(17, 20, [type_data(17, 20, [type_integer(17, 20, [type_integer_unsigned(17, 20, [type_u32(17, 20, [])])])])])
                            ])
                        ])
                    ]),
                ]),
                definition(28, 77, [
                    function(28, 77, [
                        identifier(37, 41, []),
                        block(44, 77, [
                            statement(46, 75, [
                                statement_definition(46, 75, [
                                    declare(46, 50, [
                                        let_(46, 50, []),
                                    ]),
                                    variables(50, 54, [
                                        variable_name(50, 53, [
                                            identifier(50, 53, [])
                                        ])
                                    ]),
                                    expression(56, 74, [
                                        expression_term(56, 74, [
                                            expression_circuit_inline(56, 74, [
                                                circuit_name(56, 59, [
                                                    identifier(56, 59, [])
                                                ]),
                                                circuit_implied_variable(62, 63, [
                                                    identifier(62, 63, [])
                                                ]),
                                                circuit_implied_variable(65, 73, [
                                                    circuit_variable(65, 73, [
                                                        identifier(65, 66, []),
                                                        expression(68, 73, [
                                                            expression_term(68, 72, [
                                                                value(68, 72, [
                                                                    value_integer(68, 72, [
                                                                        value_integer_unsigned(68, 72, [
                                                                            number_positive(68, 69, []),
                                                                            type_integer_unsigned(69, 72, [
                                                                                type_u32(69, 72, [])
                                                                            ])
                                                                        ]),
                                                                    ])
                                                                ])
                                                            ])
                                                        ])
                                                    ])
                                                ])
                                            ])
                                        ])
                                    ]),
                                    LINE_END(74, 75, [])
                                ])
                            ])
                        ])
                    ])
                ]),
                EOI(77, 77, [])
            ])
        ]
    }
}

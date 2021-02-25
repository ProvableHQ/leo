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

use leo_grammar::ast::{LanguageParser, Rule};

use pest::*;

#[test]
fn global_const() {
    parses_to! {
        parser: LanguageParser,
        input:  "const basic: u32 = 8;",
        rule:   Rule::global_const,
        tokens: [
            global_const(0, 21, [
                const_(0, 6, []),
                variables(6, 16, [
                    variable_name(6, 11, [
                        identifier(6, 11, [])
                    ]),
                    type_(13, 16, [
                        type_data(13, 16, [
                            type_integer(13, 16, [
                                type_integer_unsigned(13, 16, [
                                    type_u32(13, 16, [])
                                ])
                            ])
                        ])
                    ]),
                ]),
                expression(19, 20, [
                    expression_term(19, 20, [
                        value(19, 20, [
                            value_number(19, 20, [
                                number_positive(19, 20, [])
                            ])
                        ])
                    ])
                ]),
                LINE_END(20, 21, [])
            ])
        ]
    }
}

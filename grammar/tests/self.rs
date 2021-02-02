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
fn self_call() {
    parses_to! {
        parser: LanguageParser,
        input:  "self.hello()",
        rule:   Rule::self_expression_postfix,
        tokens: [
            self_expression_postfix(0, 12, [
                self_keyword(0, 4, []),
                self_access(4, 10, [access_member(4, 10, [identifier(5, 10, [])])]),
                access(10, 12, [access_call(10, 12, [])])
            ])
        ]
    }
}

#[test]
fn self_static() {
    parses_to! {
        parser: LanguageParser,
        input:  "self::hello()",
        rule:   Rule::self_expression_postfix,
        tokens: [
            self_expression_postfix(0, 13, [
                self_keyword(0, 4, []),
                self_access(4, 11, [access_static_member(4, 11, [identifier(6, 11, [])])]),
                access(11, 13, [access_call(11, 13, [])])
            ])
        ]
    }
}

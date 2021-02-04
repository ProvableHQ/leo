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
fn test_annotation_no_context_test() {
    parses_to! {
        parser: LanguageParser,
        input:  "@test",
        rule:   Rule::annotation,
        tokens: [
            annotation(0, 5, [annotation_symbol(0, 1, []), annotation_name(1, 5, [test(1, 5, [])])])
        ]
    }
}

#[test]
fn test_annotation_context_test() {
    parses_to! {
        parser: LanguageParser,
        input:  "@test(custom)",
        rule:   Rule::annotation,
        tokens: [
            annotation(0, 13, [annotation_symbol(0, 1, []), annotation_name(1, 5, [test(1, 5, [])]), annotation_arguments(5, 13, [annotation_argument(6, 12, [])])])
        ]
    }
}

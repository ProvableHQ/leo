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
fn test_import_package_rule() {
    parses_to! {
        parser: LanguageParser,
        input:  "import p.*;",
        rule:   Rule::import,
        tokens: [
            import(0, 11, [
                package_or_packages(7, 10, [
                    package(7, 10, [
                        package_name(7, 8, []),
                        package_access(9, 10, [star(9, 10, [])])
                    ])
                ]),
                LINE_END(10, 11, [])
            ]),
        ]
    }
}

#[test]
fn test_import_packages_rule() {
    parses_to! {
        parser: LanguageParser,
        input:  "import p.(x, y);",
        rule:   Rule::import,
        tokens: [
            import(0, 16, [
                package_or_packages(7, 15, [
                    packages(7, 15, [
                        package_name(7, 8, []),
                        package_access(10, 11, [
                            import_symbol(10, 11, [identifier(10, 11, [])]),
                        ]),
                        package_access(13, 14, [
                            import_symbol(13, 14, [identifier(13, 14, [])]),
                        ]),
                    ])
                ]),
                LINE_END(15, 16, [])
            ])
        ]
    }
}

#[test]
fn test_complex_import_rule() {
    parses_to! {
        parser: LanguageParser,
        input:  "import p.(q.(x, y), z);",
        rule:   Rule::import,
        tokens: [
            import(0, 23, [
                package_or_packages(7, 22, [
                    packages(7, 22, [
                        package_name(7, 8, []),
                        package_access(10, 18, [
                            packages(10, 18, [
                                package_name(10, 11, []),
                                package_access(13, 14, [import_symbol(13, 14, [identifier(13, 14, [])])]),
                                package_access(16, 17, [import_symbol(16, 17, [identifier(16, 17, [])])]),
                            ]),
                        ]),
                        package_access(20, 21, [import_symbol(20, 21, [identifier(20, 21, [])])]),
                    ])
                ]),
                LINE_END(22, 23, [])
            ])
        ]
    }
}

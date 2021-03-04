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

use from_pest::FromPest;
use leo_grammar::ast::LanguageParser;
use leo_grammar::ast::Rule;
use leo_grammar::statements::ConditionalStatement;

use pest::*;

#[test]
fn conditional_statement_display() {
    let input = r#"if (true) {
	
} else {
	
}"#;
    let conditional_statement =
        ConditionalStatement::from_pest(&mut LanguageParser::parse(Rule::statement_conditional, input).unwrap())
            .unwrap();
    let displayed = format!("{}", conditional_statement);

    assert_eq!(input, displayed);
}

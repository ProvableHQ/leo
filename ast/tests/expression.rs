use leo_ast::ast::{LanguageParser, Rule};

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

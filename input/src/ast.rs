//! Abstract syntax tree (ast) representation from leo-input.pest.
use pest::{error::Error, iterators::Pairs, Parser, Span};

#[derive(Parser)]
#[grammar = "leo-input.pest"]
pub struct LanguageParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    LanguageParser::parse(Rule::file, input)
}

pub fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}

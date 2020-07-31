use crate::{
    ast::Rule,
    common::LineEnd,
    macros::{FormattedString, MacroName, MacroSymbol},
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::formatted_macro))]
pub struct FormattedMacro<'ast> {
    pub name: MacroName<'ast>,
    pub symbol: MacroSymbol,
    pub string: Option<FormattedString<'ast>>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for FormattedMacro<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}({}){}",
            self.name,
            self.symbol,
            self.string.as_ref().map(|s| s.to_string()).unwrap_or("".to_string()),
            self.line_end
        )
    }
}

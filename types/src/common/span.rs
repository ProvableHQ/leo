use pest::Span as AstSpan;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Span {
    /// text of input string
    pub text: String,
    /// program line
    pub line: usize,
    /// start column
    pub start: usize,
    /// end column
    pub end: usize,
}

impl<'ast> From<AstSpan<'ast>> for Span {
    fn from(span: AstSpan<'ast>) -> Self {
        let line_col = span.start_pos().line_col();

        Self {
            text: span.as_str().to_string(),
            line: line_col.0,
            start: span.start(),
            end: span.end(),
        }
    }
}

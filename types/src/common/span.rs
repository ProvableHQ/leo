use pest::{Position, Span as AstSpan};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
        let mut text = " ".to_string();
        text.push_str(span.start_pos().line_of().trim_end());
        Self {
            text,
            line: span.start_pos().line_col().0,
            start: find_line_start(&span.start_pos()),
            end: find_line_end(&span.end_pos()),
        }
    }
}

pub fn find_line_start(pos: &Position) -> usize {
    let input = pos.line_of();
    if input.is_empty() {
        return 0;
    };

    // Position's pos is always a UTF-8 border.
    let start = input
        .char_indices()
        .rev()
        .skip_while(|&(i, _)| i >= pos.pos())
        .find(|&(_, c)| c == '\n');
    match start {
        Some((i, _)) => i,
        None => 0,
    }
}

pub fn find_line_end(pos: &Position) -> usize {
    let input = pos.line_of();
    if input.is_empty() {
        0
    } else if pos.pos() == input.len() - 1 {
        input.len()
    } else {
        // Position's pos is always a UTF-8 border.
        let end = input
            .char_indices()
            .skip_while(|&(i, _)| i < pos.pos())
            .find(|&(_, c)| c == '\n');
        match end {
            Some((i, _)) => i,
            None => input.len(),
        }
    }
}

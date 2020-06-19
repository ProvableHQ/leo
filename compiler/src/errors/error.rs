use leo_types::Span;

use std::fmt;

/// Formatted compiler error type
///     --> file.leo 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = undefined value `x`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error {
    /// File path where error occurred
    pub path: Option<String>,
    /// Line number
    pub line: usize,
    /// Starting column
    pub start: usize,
    /// Ending column
    pub end: usize,
    /// Text of errored line
    pub text: String,
    /// Error explanation
    pub message: String,
}

impl Error {
    pub fn new_from_span(message: String, span: Span) -> Self {
        Self {
            path: None,
            line: span.line,
            start: span.start,
            end: span.end,
            text: span.text,
            message,
        }
    }

    pub fn format(&self) -> String {
        let indent = "    ".to_string();
        let path = self.path.as_ref().map(|path| format!("{}:", path)).unwrap_or_default();
        let underline = underline(self.start, self.end);

        format!(
            "{indent     }--> {path} {line}:{start}\n\
             {indent     } |\n\
             {line:width$} | {text}\n\
             {indent     } | {underline}\n\
             {indent     } |\n\
             {indent     } = {message}",
            indent = indent,
            width = indent.len(),
            path = path,
            line = self.line,
            start = self.start,
            text = self.text,
            underline = underline,
            message = self.message,
        )
    }
}

fn underline(start: usize, mut end: usize) -> String {
    if start > end {
        panic!("underline start column is greater than end column")
    }

    let mut underline = String::new();

    for _ in 0..start {
        underline.push(' ');
        end -= 1;
    }

    for _ in 0..end {
        underline.push('^');
    }

    underline
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[test]
fn test_error() {
    let err = Error {
        path: Some("file.leo".to_string()),
        line: 2,
        start: 8,
        end: 9,
        text: "let a = x;".to_string(),
        message: "undefined value `x`".to_string(),
    };

    assert_eq!(
        format!("{}", err),
        vec![
            "    --> file.leo: 2:8",
            "     |",
            "   2 | let a = x;",
            "     |         ^",
            "     |",
            "     = undefined value `x`",
        ]
        .join("\n")
    );
}

#[test]
fn test_from_span() {
    use pest::Span as AstSpan;

    let text = "aaaa";
    let ast_span = AstSpan::new(text, 0, text.len()).unwrap();
    let span = Span::from(ast_span);
    let err = Error::new_from_span("test message".to_string(), span);

    assert_eq!(
        format!("{}", err),
        vec![
            "    -->  1:0",
            "     |",
            "   1 | aaaa",
            "     | ^^^^",
            "     |",
            "     = test message",
        ]
        .join("\n")
    );
}

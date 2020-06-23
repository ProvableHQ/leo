use leo_ast::ParserError;
use leo_types::{Error as FormattedError, ImportSymbol, Span};

use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ParserError(#[from] ParserError),
}

impl ImportError {
    fn new_from_span(message: String, span: Span) -> Self {
        ImportError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn directory_error(error: io::Error, span: Span) -> Self {
        let message = format!("attempt to access current directory failed - {:?}", error);

        Self::new_from_span(message, span)
    }

    pub fn unknown_symbol(symbol: ImportSymbol, file: String, file_path: &PathBuf) -> Self {
        let message = format!("cannot find imported symbol `{}` in imported file `{}`", symbol, file);
        let mut error = FormattedError::new_from_span(message, symbol.span);

        error.path = Some(format!("{:?}", file_path));

        ImportError::Error(error)
    }
}

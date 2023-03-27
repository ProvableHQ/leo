// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::LeoWarning;

use super::LeoError;
use core::{default::Default, fmt};
use std::{cell::RefCell, rc::Rc};

/// Types that are sinks for compiler errors.
pub trait Emitter {
    /// Emit the error `err`.
    fn emit_err(&mut self, err: LeoError);

    /// Tracks last emitted error.
    fn last_emitted_err_code(&self) -> Option<i32>;

    /// Emit the warning.
    fn emit_warning(&mut self, warning: LeoWarning);
}

/// A trivial `Emitter` using the standard error.
pub struct StderrEmitter {
    /// Exit code of the last emitted error.
    last_error_code: Option<i32>,
}

impl Emitter for StderrEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.last_error_code = Some(err.exit_code());
        eprintln!("{err}");
    }

    fn last_emitted_err_code(&self) -> Option<i32> {
        self.last_error_code
    }

    fn emit_warning(&mut self, warning: LeoWarning) {
        eprintln!("{warning}");
    }
}

/// A buffer of `T`s.
#[derive(Debug)]
pub struct Buffer<T>(Vec<T>);

impl<T> Default for Buffer<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Buffer<T> {
    /// Push `x` to the buffer.
    pub fn push(&mut self, x: T) {
        self.0.push(x);
    }

    /// Extract the underlying list of Ts.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    /// Last entry to the buffer.
    pub fn last_entry(&self) -> Option<&T> {
        self.0.last()
    }
}

impl<T: fmt::Display> fmt::Display for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some(x) = iter.next() {
            x.fmt(f)?;
        }
        for x in iter {
            f.write_fmt(format_args!("\n{x}"))?;
        }
        Ok(())
    }
}

/// A buffer of `LeoError`s.
pub type ErrBuffer = Buffer<LeoError>;
/// A buffer of `LeoWarning`s.
pub type WarningBuffer = Buffer<LeoWarning>;

/// An `Emitter` that collects into a list.
#[derive(Default, Clone)]
pub struct BufferEmitter(Rc<RefCell<ErrBuffer>>, Rc<RefCell<WarningBuffer>>);

impl BufferEmitter {
    /// Returns a new buffered emitter.
    pub fn new() -> Self {
        BufferEmitter(<_>::default(), <_>::default())
    }

    /// Extracts all the errors collected in this emitter.
    pub fn extract_errs(&self) -> ErrBuffer {
        self.0.take()
    }

    /// Extracts all the errors collected in this emitter.
    pub fn extract_warnings(&self) -> WarningBuffer {
        self.1.take()
    }
}

impl Emitter for BufferEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.0.borrow_mut().push(err);
    }

    fn last_emitted_err_code(&self) -> Option<i32> {
        let temp = &*self.0.borrow();
        temp.last_entry().map(|entry| entry.exit_code())
    }

    fn emit_warning(&mut self, warning: LeoWarning) {
        self.1.borrow_mut().push(warning);
    }
}

/// Contains the actual data for `Handler`.
/// Modelled this way to afford an API using interior mutability.
struct HandlerInner {
    /// Number of errors emitted thus far.
    err_count: usize,
    /// Number of warnings emitted thus far.
    warn_count: usize,
    /// The sink through which errors will be emitted.
    emitter: Box<dyn Emitter>,
}

impl HandlerInner {
    /// Emit the error `err`.
    fn emit_err(&mut self, err: LeoError) {
        self.err_count = self.err_count.saturating_add(1);
        self.emitter.emit_err(err);
    }

    /// Gets the last emitted error's exit code.
    fn last_emitted_err_code(&self) -> Option<i32> {
        self.emitter.last_emitted_err_code()
    }

    /// Emit the error `err`.
    fn emit_warning(&mut self, warning: LeoWarning) {
        self.warn_count = self.warn_count.saturating_add(1);
        self.emitter.emit_warning(warning);
    }
}

/// A handler deals with errors and other compiler output.
pub struct Handler {
    /// The inner handler.
    /// `RefCell` is used here to avoid `&mut` all over the compiler.
    inner: RefCell<HandlerInner>,
}

impl Default for Handler {
    fn default() -> Self {
        Self::new(Box::new(StderrEmitter { last_error_code: None }))
    }
}

impl Handler {
    /// Construct a `Handler` using the given `emitter`.
    pub fn new(emitter: Box<dyn Emitter>) -> Self {
        let inner = RefCell::new(HandlerInner { err_count: 0, warn_count: 0, emitter });
        Self { inner }
    }

    /// Construct a `Handler` that will append to `buf`.
    pub fn new_with_buf() -> (Self, BufferEmitter) {
        let buf = BufferEmitter::default();
        let handler = Self::new(Box::new(buf.clone()));
        (handler, buf)
    }

    /// Runs `logic` provided a handler that collects all errors into the `String`,
    /// or if there were none, returns some `T`.
    pub fn with<T>(logic: impl for<'a> FnOnce(&'a Handler) -> Result<T, LeoError>) -> Result<T, ErrBuffer> {
        let (handler, buf) = Handler::new_with_buf();
        handler.extend_if_error(logic(&handler)).map_err(|_| buf.extract_errs())
    }

    /// Emit the error `err`.
    pub fn emit_err<E: Into<LeoError>>(&self, err: E) {
        self.inner.borrow_mut().emit_err(err.into());
    }

    /// Emit the error `err`.
    pub fn emit_warning(&self, warning: LeoWarning) {
        self.inner.borrow_mut().emit_warning(warning);
    }

    /// Emits the error `err`.
    /// This will immediately abort compilation.
    pub fn fatal_err(&self, err: LeoError) -> ! {
        let code = err.exit_code();
        self.emit_err(err);
        std::process::exit(code);
    }

    /// The number of errors thus far.
    pub fn err_count(&self) -> usize {
        self.inner.borrow().err_count
    }

    /// The number of warnings thus far.
    pub fn warning_count(&self) -> usize {
        self.inner.borrow().warn_count
    }

    /// Did we have any errors thus far?
    pub fn had_errors(&self) -> bool {
        self.err_count() > 0
    }

    /// Gets the last emitted error's exit code if it exists.
    /// Then exits the program with it if it did exist.
    pub fn last_err(&self) -> Result<(), Box<LeoError>> {
        if let Some(code) = self.inner.borrow().last_emitted_err_code() {
            Err(Box::new(LeoError::LastErrorCode(code)))
        } else {
            Ok(())
        }
    }

    /// Extend handler with `error` given `res = Err(error)`.
    #[allow(clippy::result_unit_err)]
    pub fn extend_if_error<T>(&self, res: Result<T, LeoError>) -> Result<T, ()> {
        match res {
            Ok(_) if self.had_errors() => Err(()),
            Ok(x) => Ok(x),
            Err(e) => {
                self.emit_err(e);
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParserError;
    use leo_span::{symbol::create_session_if_not_set_then, Span};

    #[test]
    fn fresh_no_errors() {
        let handler = Handler::new(Box::new(BufferEmitter::new()));
        assert_eq!(handler.err_count(), 0);
        assert!(!handler.had_errors());
    }

    #[test]
    fn buffer_works() {
        create_session_if_not_set_then(|_| {
            let count_err = |s: String| s.lines().filter(|l| l.contains("Error")).count();

            let res: Result<(), _> = Handler::with(|h| {
                let s = Span::default();
                assert_eq!(h.err_count(), 0);
                h.emit_err(ParserError::invalid_import_list(s));
                assert_eq!(h.err_count(), 1);
                h.emit_err(ParserError::unexpected_eof(s));
                assert_eq!(h.err_count(), 2);
                Err(ParserError::spread_in_array_init(s).into())
            });

            assert_eq!(count_err(res.unwrap_err().to_string()), 3);

            let res: Result<(), _> = Handler::with(|h| {
                let s = Span::default();
                h.emit_err(ParserError::invalid_import_list(s));
                h.emit_err(ParserError::unexpected_eof(s));
                Ok(())
            });
            assert_eq!(count_err(res.unwrap_err().to_string()), 2);

            Handler::with(|_| Ok(())).unwrap();
        })
    }
}

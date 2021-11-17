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

use super::LeoError;
use core::default::Default;
use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

/// Types that are sinks for compiler errors.
pub trait Emitter {
    /// Emit the error `err`.
    fn emit_err(&mut self, err: LeoError);
}

/// A trivial `Emitter` using the standard error.
pub struct StderrEmitter;

impl Emitter for StderrEmitter {
    fn emit_err(&mut self, err: LeoError) {
        eprintln!("{}", err);
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
}

impl<T: fmt::Display> fmt::Display for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some(x) = iter.next() {
            x.fmt(f)?;
        }
        for x in iter {
            f.write_fmt(format_args!("\n{}", x))?;
        }
        Ok(())
    }
}

/// A buffer of `LeoError`s.
pub type ErrBuffer = Buffer<LeoError>;

/// An `Emitter` that collects into a list.
#[derive(Default, Clone)]
pub struct BufferEmitter(Rc<RefCell<ErrBuffer>>);

impl BufferEmitter {
    /// Returns a new buffered emitter.
    pub fn new() -> Self {
        BufferEmitter(<_>::default())
    }

    /// Extracts all the errors collected in this emitter.
    pub fn extract(&self) -> ErrBuffer {
        self.0.take()
    }
}

impl Emitter for BufferEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.0.borrow_mut().push(err);
    }
}

/// Contains the actual data for `Handler`.
/// Modelled this way to afford an API using interior mutability.
struct HandlerInner {
    /// Number of errors emitted thus far.
    count: usize,
    /// The sink through which errors will be emitted.
    emitter: Box<dyn Emitter>,
}

impl HandlerInner {
    /// Emit the error `err`.
    fn emit_err(&mut self, err: LeoError) {
        self.count = self.count.saturating_add(1);
        self.emitter.emit_err(err);
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
        Self::new(Box::new(StderrEmitter))
    }
}

impl Handler {
    /// Construct a `Handler` using the given `emitter`.
    pub fn new(emitter: Box<dyn Emitter>) -> Self {
        let inner = RefCell::new(HandlerInner { count: 0, emitter });
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
        handler.extend_if_error(logic(&handler)).map_err(|_| buf.extract())
    }

    /// Emit the error `err`.
    pub fn emit_err(&self, err: LeoError) {
        self.inner.borrow_mut().emit_err(err);
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
        self.inner.borrow().count
    }

    /// Did we have any errors thus far?
    pub fn had_errors(&self) -> bool {
        self.err_count() > 0
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
    use crate::{ParserError, Span};

    #[test]
    fn fresh_no_errors() {
        let handler = Handler::new(Box::new(BufferEmitter::new()));
        assert_eq!(handler.err_count(), 0);
        assert!(!handler.had_errors());
    }

    #[test]
    fn buffer_works() {
        let count_err = |s: String| s.lines().filter(|l| l.contains("Error")).count();

        let res: Result<(), _> = Handler::with(|h| {
            let s = Span::default();
            assert_eq!(h.err_count(), 0);
            h.emit_err(ParserError::invalid_import_list(&s).into());
            assert_eq!(h.err_count(), 1);
            h.emit_err(ParserError::unexpected_eof(&s).into());
            assert_eq!(h.err_count(), 2);
            Err(ParserError::spread_in_array_init(&s).into())
        });

        assert_eq!(count_err(res.unwrap_err().to_string()), 3);

        let res: Result<(), _> = Handler::with(|h| {
            let s = Span::default();
            h.emit_err(ParserError::invalid_import_list(&s).into());
            h.emit_err(ParserError::unexpected_eof(&s).into());
            Ok(())
        });
        assert_eq!(count_err(res.unwrap_err().to_string()), 2);

        let () = Handler::with(|_| Ok(())).unwrap();
    }
}

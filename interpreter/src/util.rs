// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use leo_errors::{InterpreterHalt, Result};
use leo_span::{Span, Symbol};

use snarkvm::prelude::{Identifier, TestnetV0};

#[macro_export]
macro_rules! tc_fail {
    () => {
        panic!("type checker failure")
    };
}

#[macro_export]
macro_rules! halt_no_span {
    ($($x:tt)*) => {
        return Err(InterpreterHalt::new(format!($($x)*)).into())
    }
}

#[macro_export]
macro_rules! halt {
    ($span: expr) => {
        return Err(InterpreterHalt::new_spanned(String::new(), $span).into())

    };

    ($span: expr, $($x:tt)*) => {
        return Err(InterpreterHalt::new_spanned(format!($($x)*), $span).into())
    };
}

pub trait ExpectTc {
    type T;
    fn expect_tc(self, span: Span) -> Result<Self::T>;
}

impl<T> ExpectTc for Option<T> {
    type T = T;

    fn expect_tc(self, span: Span) -> Result<Self::T> {
        match self {
            Some(t) => Ok(t),
            None => Err(InterpreterHalt::new_spanned("type failure".into(), span).into()),
        }
    }
}

impl<T, U: std::fmt::Debug> ExpectTc for Result<T, U> {
    type T = T;

    fn expect_tc(self, span: Span) -> Result<Self::T> {
        self.map_err(|_e| InterpreterHalt::new_spanned("type failure".into(), span).into())
    }
}

pub fn snarkvm_identifier_to_symbol(id: &Identifier<TestnetV0>) -> Symbol {
    let s = id.to_string();
    Symbol::intern(&s)
}

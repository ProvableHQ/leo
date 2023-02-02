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

//! The test framework to run integration tests with Leo code text.
//!
//! This module contains the [`run_tests()`] method which runs all integration tests in the
//! root [`tests/`] directory.
//!
//! To regenerate the tests after a syntax change or failing test, delete the [`tests/expectations/`]
//! directory and run the [`parser_tests()`] test in [`parser/src/test.rs`].

#![forbid(unsafe_code)]
#![cfg(not(doctest))] // Don't doctest the markdown.
#![doc = include_str!("../README.md")]

pub mod error;

pub mod fetch;

pub mod output;

pub mod runner;

pub mod test;

pub use runner::*;

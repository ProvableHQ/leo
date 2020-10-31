// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use pest::Span;
use serde::Serialize;

// Provide getters for every private field of the remote struct. The getter must
// return either `T` or `&T` where `T` is the type of the field.
#[derive(Serialize)]
#[serde(remote = "Span")]
pub(crate) struct SpanDef<'i> {
    /// # Attention
    ///
    /// This getter only returns a subset of the input.
    /// Namely, it returns `self.input[self.start..self.end]`.
    /// This means you can only accurate serialize (and not deserialize).
    #[serde(getter = "Span::as_str")]
    input: &'i str,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    #[serde(getter = "Span::start")]
    start: usize,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    #[serde(getter = "Span::end")]
    end: usize,
}

// Provide a conversion to construct the remote type.
impl<'i> From<SpanDef<'i>> for Span<'i> {
    fn from(def: SpanDef) -> Span {
        Span::new(def.input, def.start, def.end).unwrap()
    }
}

#[test]
fn test_span_def() {
    // Wrapper serializable JSON struct
    #[derive(Serialize)]
    struct Element<'ast> {
        #[serde(with = "SpanDef")]
        span: Span<'ast>,
    }

    // Starting value
    let span = Span::new("hello world", 1, 5).unwrap();
    let element = Element { span };

    // Attempt to serialize span to string.
    let output = serde_json::to_string(&element).unwrap();

    let expected_output = "{\"span\":{\"input\":\"ello\",\"start\":1,\"end\":5}}";
    assert_eq!(expected_output, output);
}

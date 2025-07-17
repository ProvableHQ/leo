// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::{NetworkName, NodeBuilder};
use leo_errors::{BufferEmitter, Handler, LeoError};
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName, with_session_globals};

use serde::Serialize;
use serial_test::serial;
use std::fmt::Write as _;

fn run_parse_many_test<T: Serialize>(
    test: &str,
    handler: &Handler,
    test_index: usize,
    parse: fn(Handler, &NodeBuilder, &str, u32) -> Result<T, LeoError>,
) -> Result<String, ()> {
    let source_map =
        with_session_globals(|s| s.source_map.new_source(test, FileName::Custom(format!("test_{test_index}"))));
    let result = parse(handler.clone(), &Default::default(), &source_map.src, source_map.absolute_start);
    let serializable = handler.extend_if_error(result)?;
    let value = serde_json::to_value(&serializable).expect("Serialization failure");
    let mut s = serde_json::to_string_pretty(&value).expect("string conversion failure");
    s.push('\n');
    Ok(s)
}

fn runner_parse_many_test<'a, T: Serialize>(
    tests: impl Iterator<Item = &'a str>,
    parse: fn(Handler, &NodeBuilder, &str, u32) -> Result<T, LeoError>,
) -> String {
    create_session_if_not_set_then(|_| {
        let mut output = String::new();
        let buf = BufferEmitter::new();
        let handler = Handler::new(buf.clone());

        for (i, test) in tests.enumerate() {
            match run_parse_many_test(test, &handler, i, parse) {
                Ok(s) => writeln!(output, "{s}").unwrap(),
                Err(()) => write!(output, "{}{}", buf.extract_errs(), buf.extract_warnings()).unwrap(),
            }
        }

        output
    })
}

#[test]
#[serial]
fn parse_module_tests() {
    leo_test_framework::run_tests("parser-module", runner_module_test);
}

// Parse module tests.

fn runner_module_test(test: &str) -> String {
    let test_cases: Vec<String> = test
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|paragraph| paragraph.join("\n"))
        .filter(|s| !s.trim().is_empty())
        .collect();

    runner_parse_many_test(test_cases.iter().map(|s| s.as_str()), |handler, node_builder, source, start_pos| {
        crate::parse_module(
            handler,
            node_builder,
            source,
            start_pos,
            Symbol::intern("module_test"),
            Vec::new(),
            NetworkName::TestnetV0,
        )
    })
}

// Parse expression tests.

fn runner_expression_test(test: &str) -> String {
    let tests = test.lines().map(|line| line.trim()).filter(|line| !line.is_empty());

    runner_parse_many_test(tests, |handler, node_builder, source, start_pos| {
        crate::parse_expression(handler, node_builder, source, start_pos, NetworkName::TestnetV0)
    })
}

#[test]
#[serial]
fn parse_expression_tests() {
    leo_test_framework::run_tests("parser-expression", runner_expression_test);
}

// Parse statement tests.

fn runner_statement_test(test: &str) -> String {
    let tests = test.split("\n\n").map(|text| text.trim()).filter(|text| !text.is_empty());

    runner_parse_many_test(tests, |handler, node_builder, source, start_pos| {
        crate::parse_statement(handler, node_builder, source, start_pos, NetworkName::TestnetV0)
    })
}

#[test]
#[serial]
fn parse_statement_tests() {
    leo_test_framework::run_tests("parser-statement", runner_statement_test);
}

// Full parser tests.

fn run_parser_test(test: &str, handler: &Handler) -> Result<String, ()> {
    let source_file = with_session_globals(|s| s.source_map.new_source(test, FileName::Custom("test".into())));
    let result =
        crate::parse_ast(handler.clone(), &Default::default(), &source_file, &Vec::new(), NetworkName::TestnetV0);
    let ast = handler.extend_if_error(result)?;
    let value = serde_json::to_value(&ast.ast).expect("Serialization failure");
    let mut s = serde_json::to_string_pretty(&value).expect("string conversion failure");
    s.push('\n');
    Ok(s)
}

fn runner_parser_test(test: &str) -> String {
    create_session_if_not_set_then(|_| {
        let buf = BufferEmitter::new();
        let handler = Handler::new(buf.clone());

        match run_parser_test(test, &handler) {
            Ok(x) => x,
            Err(()) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
        }
    })
}

#[test]
#[serial]
fn parser_tests() {
    leo_test_framework::run_tests("parser", runner_parser_test);
}

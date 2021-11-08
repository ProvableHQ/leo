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

use criterion::{criterion_group, criterion_main, Criterion};
use leo_ast::Ast;
use leo_errors::emitter::Handler;
use std::time::Duration;

fn parse_ast(path: &str, input: &str) -> Ast {
    leo_parser::parse_ast(&Handler::default(), path, input).expect("failed to parse benchmark")
}

fn bench_big_if_else(c: &mut Criterion) {
    let ast = parse_ast("./big_if_else.leo", include_str!("./big_if_else.leo"));
    c.bench_function("Ast::big_if_else", |b| b.iter(|| &ast));
}

fn bench_big_ternary(c: &mut Criterion) {
    let ast = parse_ast("./big_ternary.leo", include_str!("./big_ternary.leo"));
    c.bench_function("Ast::big_ternary", |b| b.iter(|| &ast));
}

fn bench_big_circuit(c: &mut Criterion) {
    let ast = parse_ast("./big_circuit.leo", include_str!("./big_circuit.leo"));
    c.bench_function("Ast::big_circuit", |b| b.iter(|| &ast));
}

fn bench_long_expr(c: &mut Criterion) {
    let ast = parse_ast("./long_expr.leo", include_str!("./long_expr.leo"));
    c.bench_function("Ast::long_expr", |b| b.iter(|| &ast));
}

fn bench_long_array(c: &mut Criterion) {
    let ast = parse_ast("./long_array.leo", include_str!("./long_array.leo"));
    c.bench_function("Ast::long_array", |b| b.iter(|| &ast));
}

fn bench_many_foos(c: &mut Criterion) {
    let ast = parse_ast("./many_foos.leo", include_str!("./many_foos.leo"));
    c.bench_function("Ast::many_foos", |b| b.iter(|| &ast));
}

fn bench_many_assigns(c: &mut Criterion) {
    let ast = parse_ast("./many_assigns.leo", include_str!("./many_assigns.leo"));
    c.bench_function("Ast::many_assigns", |b| b.iter(|| &ast));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(200).measurement_time(Duration::from_secs(10)).nresamples(200_000);
    targets = bench_big_circuit,
    bench_long_expr,
    bench_big_if_else,
    bench_big_ternary,
    bench_long_array,
    bench_many_assigns,
    bench_many_foos,
);
criterion_main!(benches);

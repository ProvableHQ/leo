// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::Ast;
use leo_errors::emitter::Handler;
use leo_span::symbol::create_session_if_not_set_then;

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn parse_ast(path: &str, input: &str) -> Ast {
    create_session_if_not_set_then(|_| {
        leo_parser::parse_ast(&Handler::default(), path, input).expect("failed to parse benchmark")
    })
}

fn bench_big_if_else(c: &mut Criterion) {
    let ast = parse_ast("./big_if_else.leo", include_str!("./big_if_else.leo"));
    c.bench_function("Ast::big_if_else", |b| b.iter(|| &ast));
}

// TODO have to figure out why this causes `thread 'main' has overflowed it's stack'
/* fn bench_big_ternary(c: &mut Criterion) {
    let ast = parse_ast("./big_ternary.leo", include_str!("./big_ternary.leo"));
    c.bench_function("Ast::big_ternary", |b| b.iter(|| &ast));
} */

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

fn bench_small_1(c: &mut Criterion) {
    let ast = parse_ast("./small_1.leo", include_str!("./small_1.leo"));
    c.bench_function("Ast::small_1", |b| b.iter(|| &ast));
}

fn bench_small_2(c: &mut Criterion) {
    let ast = parse_ast("./small_2.leo", include_str!("./small_2.leo"));
    c.bench_function("Ast::small_2", |b| b.iter(|| &ast));
}

fn bench_small_3(c: &mut Criterion) {
    let ast = parse_ast("./small_3.leo", include_str!("./small_3.leo"));
    c.bench_function("Ast::small_3", |b| b.iter(|| &ast));
}

fn bench_small_4(c: &mut Criterion) {
    let ast = parse_ast("./small_4.leo", include_str!("./small_4.leo"));
    c.bench_function("Ast::small_4", |b| b.iter(|| &ast));
}

fn bench_small_5(c: &mut Criterion) {
    let ast = parse_ast("./small_5.leo", include_str!("./small_5.leo"));
    c.bench_function("Ast::small_5", |b| b.iter(|| &ast));
}

fn bench_medium_1(c: &mut Criterion) {
    let ast = parse_ast("./medium_1.leo", include_str!("./medium_1.leo"));
    c.bench_function("Ast::medium_1", |b| b.iter(|| &ast));
}

fn bench_medium_2(c: &mut Criterion) {
    let ast = parse_ast("./medium_2.leo", include_str!("./medium_2.leo"));
    c.bench_function("Ast::medium_2", |b| b.iter(|| &ast));
}

fn bench_medium_3(c: &mut Criterion) {
    let ast = parse_ast("./medium_3.leo", include_str!("./medium_3.leo"));
    c.bench_function("Ast::medium_3", |b| b.iter(|| &ast));
}

fn bench_medium_4(c: &mut Criterion) {
    let ast = parse_ast("./medium_4.leo", include_str!("./medium_4.leo"));
    c.bench_function("Ast::medium_4", |b| b.iter(|| &ast));
}

fn bench_medium_5(c: &mut Criterion) {
    let ast = parse_ast("./medium_5.leo", include_str!("./medium_5.leo"));
    c.bench_function("Ast::medium_5", |b| b.iter(|| &ast));
}

fn bench_large_1(c: &mut Criterion) {
    let ast = parse_ast("./large_1.leo", include_str!("./large_1.leo"));
    c.bench_function("Ast::large_1", |b| b.iter(|| &ast));
}

fn bench_large_2(c: &mut Criterion) {
    let ast = parse_ast("./large_2.leo", include_str!("./large_2.leo"));
    c.bench_function("Ast::large_2", |b| b.iter(|| &ast));
}

fn bench_large_3(c: &mut Criterion) {
    let ast = parse_ast("./large_3.leo", include_str!("./large_3.leo"));
    c.bench_function("Ast::large_3", |b| b.iter(|| &ast));
}

fn bench_large_4(c: &mut Criterion) {
    let ast = parse_ast("./large_4.leo", include_str!("./large_4.leo"));
    c.bench_function("Ast::large_4", |b| b.iter(|| &ast));
}

fn bench_large_5(c: &mut Criterion) {
    let ast = parse_ast("./large_5.leo", include_str!("./large_5.leo"));
    c.bench_function("Ast::large_5", |b| b.iter(|| &ast));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(200).measurement_time(Duration::from_secs(10)).nresamples(200_000);
    targets = bench_big_circuit,
    bench_long_expr,
    bench_big_if_else,
    bench_long_array,
    bench_many_assigns,
    bench_many_foos,
    bench_small_1,
    bench_small_2,
    bench_small_3,
    bench_small_4,
    bench_small_5,
    bench_medium_1,
    bench_medium_2,
    bench_medium_3,
    bench_medium_4,
    bench_medium_5,
    bench_large_1,
    bench_large_2,
    bench_large_3,
    bench_large_4,
    bench_large_5,
);
criterion_main!(benches);

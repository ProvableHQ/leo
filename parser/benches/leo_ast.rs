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

macro_rules! bench {
    ($func_name:ident, $file_name:expr) => {
        fn $func_name(c: &mut Criterion) {
            let ast = parse_ast(
                concat!("./", $file_name, ".leo"),
                include_str!(concat!("./", $file_name, ".leo"),),
            );
            c.bench_function(concat!("Ast::", $file_name), |b| b.iter(|| &ast));
        }
    };
}

bench!(bench_big_if_else, "big_if_else");
// TODO have to figure out why this causes `thread 'main' has overflowed it's stack'
// bench!(bench_big_ternary, "big_ternary");
bench!(bench_big_circuit, "big_circuit");
bench!(bench_long_expr, "long_expr");
bench!(bench_long_array, "long_array");
bench!(bench_many_foos, "many_foos");
bench!(bench_many_assigns, "many_assigns");
bench!(bench_small_1, "small_1");
bench!(bench_small_2, "small_2");
bench!(bench_small_3, "small_3");
bench!(bench_small_4, "small_4");
bench!(bench_small_5, "small_5");
bench!(bench_medium_1, "medium_1");
bench!(bench_medium_2, "medium_2");
bench!(bench_medium_3, "medium_3");
bench!(bench_medium_4, "medium_4");
bench!(bench_medium_5, "medium_5");
bench!(bench_large_1, "large_1");
bench!(bench_large_2, "large_2");
bench!(bench_large_3, "large_3");
bench!(bench_large_4, "large_4");
bench!(bench_large_5, "large_5");
bench!(bench_massive, "massive");

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
    bench_massive
);
criterion_main!(benches);

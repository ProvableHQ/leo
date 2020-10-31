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

use leo_grammar::Grammar;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn leo_grammar<'ast>(filepath: &'ast Path, program_string: &'ast str) {
    let result = Grammar::<'ast>::new(filepath, program_string).unwrap();
    black_box(result);
}

fn criterion_benchmark(c: &mut Criterion) {
    let filepath = Path::new("./main.leo").to_path_buf();
    let program_string = include_str!("./main.leo");

    c.bench_function("Grammar::new", |b| b.iter(|| leo_grammar(&filepath, program_string)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

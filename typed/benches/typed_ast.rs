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

use leo_ast::{errors::ParserError, files::File, LeoAst};
use leo_typed::LeoTypedAst;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::{Path, PathBuf};

fn leo_typed_ast<'ast>(ast: &LeoAst<'ast>) {
    let typed_ast = LeoTypedAst::new("leo_typed_tree", &ast);
    black_box(typed_ast);
}

fn criterion_benchmark(c: &mut Criterion) {
    let filepath = Path::new("./main.leo").to_path_buf();
    // let program_string = &LeoAst::load_file(&filepath).unwrap();
    let program_string = include_str!("./main.leo");
    let ast = LeoAst::new(&filepath, program_string).unwrap();

    c.bench_function("LeoTypedAst::new", |b| b.iter(|| leo_typed_ast(black_box(&ast))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

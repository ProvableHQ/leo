use leo_ast::{errors::ParserError, files::File, LeoAst};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::{Path, PathBuf};

fn leo_ast<'ast>(filepath: &'ast PathBuf, program_string: &'ast str) {
    let result = LeoAst::<'ast>::new(filepath, program_string).unwrap();
    black_box(result);
}

fn criterion_benchmark(c: &mut Criterion) {
    let filepath = Path::new("./main.leo").to_path_buf();
    // let program_string = &LeoAst::load_file(&filepath).unwrap();
    let program_string = include_str!("./main.leo");

    c.bench_function("LeoAst::new", |b| {
        b.iter(|| leo_ast(black_box(&filepath), black_box(program_string)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

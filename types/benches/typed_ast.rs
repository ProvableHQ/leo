use leo_ast::{errors::ParserError, files::File, LeoAst};
use leo_types::LeoTypedAst;

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

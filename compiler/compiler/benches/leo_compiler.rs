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

use leo_compiler::Compiler;
use leo_errors::emitter::Handler;
use leo_span::{source_map::FileName, symbol::create_session_if_not_set_then};

use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, path::PathBuf, time::Duration};

fn new_compiler<'a>(handler: &'a Handler, main_file_path: PathBuf) -> Compiler<'a> {
    let output_dir = PathBuf::from("/tmp/output/");
    fs::create_dir_all(output_dir.clone()).expect("Failed to create output dir");

    Compiler::new(handler, main_file_path, output_dir, None)
}

fn compile<'a>(handler: &'a Handler, path: &str, input: &str) -> Compiler<'a> {
    create_session_if_not_set_then(|_| {
        let mut program = new_compiler(handler, PathBuf::from(path));
        program
            .parse_program_from_string(input, FileName::Custom(path.into()))
            .expect("Failed to parse program");
        // TODO add once input files do something
        // program.parse_input(input_path.to_path_buf())?;
        program.compile().expect("Failed to compile program");
        program
    })
}

macro_rules! bench {
    ($func_name:ident, $file_name:expr) => {
        fn $func_name(c: &mut Criterion) {
            let handler = &Handler::default();
            let output = compile(
                handler,
                concat!("./", $file_name, ".leo"),
                include_str!(concat!("./", $file_name, ".leo"),),
            );
            c.bench_function(concat!("Compiler::", $file_name), |b| b.iter(|| &output));
        }
    };
}

bench!(big, "big");
bench!(iteration, "iteration");

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(200).measurement_time(Duration::from_secs(10)).nresamples(200_000);
    targets = big,
    iteration,
);
criterion_main!(benches);

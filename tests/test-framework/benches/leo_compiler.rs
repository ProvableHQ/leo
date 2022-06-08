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
use leo_span::{
    source_map::FileName,
    symbol::{SessionGlobals, SESSION_GLOBALS},
};
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

macro_rules! sample {
    ($name:expr) => {
        Sample {
            name: $name,
            input: include_str!(concat!("./", $name, ".leo")),
            path: concat!("./", $name, ".leo"),
        }
    };
}

#[derive(Clone, Copy)]
struct Sample {
    name: &'static str,
    input: &'static str,
    path: &'static str,
}

fn new_compiler<'a>(handler: &'a Handler, main_file_path: &str) -> Compiler<'a> {
    Compiler::new(
        handler,
        PathBuf::from(main_file_path),
        PathBuf::from("/tmp/output/"),
        None,
    )
}

impl Sample {
    const SAMPLES: &'static [Sample] = &[sample!("big"), sample!("iteration")];

    fn data(&self) -> (&str, FileName) {
        black_box((self.input, FileName::Custom(self.path.into())))
    }

    fn bench_parse(&self, c: &mut Criterion) {
        c.bench_function(&format!("parse {}", self.name), |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::default();
                for _ in 0..iters {
                    SESSION_GLOBALS.set(&SessionGlobals::default(), || {
                        let handler = Handler::default();
                        let mut compiler = new_compiler(&handler, self.path);
                        let (input, name) = self.data();
                        let start = Instant::now();
                        let out = compiler.parse_program_from_string(input, name);
                        time += start.elapsed();
                        out.expect("Failed to parse program")
                    });
                }
                time
            })
        });
    }

    fn bench_symbol_table(&self, c: &mut Criterion) {
        c.bench_function(&format!("symbol table pass {}", self.name), |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::default();
                for _ in 0..iters {
                    SESSION_GLOBALS.set(&SessionGlobals::default(), || {
                        let handler = Handler::default();
                        let mut compiler = new_compiler(&handler, self.path);
                        let (input, name) = self.data();
                        compiler
                            .parse_program_from_string(input, name)
                            .expect("Failed to parse program");
                        let start = Instant::now();
                        let out = compiler.symbol_table_pass();
                        time += start.elapsed();
                        out.expect("failed to generate symbol table");
                    });
                }
                time
            })
        });
    }

    fn bench_type_checker(&self, c: &mut Criterion) {
        c.bench_function(&format!("type checker pass {}", self.name), |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::default();
                for _ in 0..iters {
                    SESSION_GLOBALS.set(&SessionGlobals::default(), || {
                        let handler = Handler::default();
                        let mut compiler = new_compiler(&handler, self.path);
                        let (input, name) = self.data();
                        compiler
                            .parse_program_from_string(input, name)
                            .expect("Failed to parse program");
                        let mut symbol_table = compiler.symbol_table_pass().expect("failed to generate symbol table");
                        let start = Instant::now();
                        let out = compiler.type_checker_pass(&mut symbol_table);
                        time += start.elapsed();
                        out.expect("failed to run type check pass")
                    });
                }
                time
            })
        });
    }

    fn bench_full(&self, c: &mut Criterion) {
        c.bench_function(&format!("full {}", self.name), |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::default();
                for _ in 0..iters {
                    SESSION_GLOBALS.set(&SessionGlobals::default(), || {
                        let handler = Handler::default();
                        let mut compiler = new_compiler(&handler, self.path);
                        let (input, name) = self.data();
                        let start = Instant::now();
                        compiler
                            .parse_program_from_string(input, name)
                            .expect("Failed to parse program");
                        let mut symbol_table = compiler.symbol_table_pass().expect("failed to generate symbol table");
                        compiler
                            .type_checker_pass(&mut symbol_table)
                            .expect("failed to run type check pass");
                        time += start.elapsed();
                    });
                }
                time
            })
        });
    }
}

fn bench_parse(c: &mut Criterion) {
    for sample in Sample::SAMPLES {
        sample.bench_parse(c);
    }
}

fn bench_symbol_table(c: &mut Criterion) {
    for sample in Sample::SAMPLES {
        sample.bench_symbol_table(c);
    }
}

fn bench_type_checker(c: &mut Criterion) {
    for sample in Sample::SAMPLES {
        sample.bench_type_checker(c);
    }
}

fn bench_full(c: &mut Criterion) {
    for sample in Sample::SAMPLES {
        sample.bench_full(c);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(200).measurement_time(Duration::from_secs(30)).nresamples(200_000);
    targets =
        bench_parse,
        bench_symbol_table,
        bench_type_checker,
        bench_full
);
criterion_main!(benches);

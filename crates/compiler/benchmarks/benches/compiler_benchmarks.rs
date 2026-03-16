// Copyright (C) 2019-2026 Provable Inc.
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

use leo_benchmarks::{FixtureData, create_compiler, create_parse_only_compiler, load_fixture, load_source_fixture};
use leo_compiler::Compiler;
use leo_passes::*;
use leo_span::create_session_if_not_set_then;

use std::path::PathBuf;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

// ---------------------------------------------------------------------------
// Fixture loading
// ---------------------------------------------------------------------------

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

/// Loads a committed contrived package. Panics on failure since these fixtures
/// are checked into the repo and must always be loadable.
fn contrived_package(name: &str) -> FixtureData {
    let package_dir = fixtures_root().join("contrived").join(name);
    load_fixture(&package_dir).unwrap_or_else(|err| panic!("failed to load committed fixture contrived/{name}: {err}"))
}

/// Loads a committed single-source fixture. Panics on failure.
fn single_source(name: &str) -> FixtureData {
    let path = fixtures_root().join("single").join(name);
    load_source_fixture(&path).unwrap_or_else(|err| panic!("failed to load committed fixture single/{name}: {err}"))
}

// ---------------------------------------------------------------------------
// Compiler setup helpers
// ---------------------------------------------------------------------------

fn prepare_compiler(fixture: &FixtureData) -> Compiler {
    let module_refs = fixture.module_refs();
    let mut compiler = create_compiler(fixture);
    compiler.parse(&fixture.source, fixture.filename.clone(), &module_refs).expect("parse");
    compiler.add_import_stubs().expect("add_import_stubs");
    compiler
}

/// How far through the frontend pipeline to advance the compiler before the
/// timed routine begins.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Through {
    Parsed,
    NameValidation,
    GlobalVarsCollection,
    PathResolution,
    GlobalItemsCollection,
    CheckInterfaces,
    TypeChecking,
    Disambiguate,
    ProcessingAsync,
}

impl Through {
    const fn rank(self) -> u8 {
        match self {
            Self::Parsed => 0,
            Self::NameValidation => 1,
            Self::GlobalVarsCollection => 2,
            Self::PathResolution => 3,
            Self::GlobalItemsCollection => 4,
            Self::CheckInterfaces => 5,
            Self::TypeChecking => 6,
            Self::Disambiguate => 7,
            Self::ProcessingAsync => 8,
        }
    }

    const fn includes(self, stage: Self) -> bool {
        self.rank() >= stage.rank()
    }
}

/// Builds a fresh compiler advanced through the given stage.
fn prepare_through(fixture: &FixtureData, through: Through) -> Compiler {
    let mut compiler = prepare_compiler(fixture);

    if through.includes(Through::NameValidation) {
        compiler.do_pass::<NameValidation>(()).expect("name_validation");
    }
    if through.includes(Through::GlobalVarsCollection) {
        compiler.do_pass::<GlobalVarsCollection>(()).expect("global_vars_collection");
    }
    if through.includes(Through::PathResolution) {
        compiler.do_pass::<PathResolution>(()).expect("path_resolution");
    }
    if through.includes(Through::GlobalItemsCollection) {
        compiler.do_pass::<GlobalItemsCollection>(()).expect("global_items_collection");
    }
    if through.includes(Through::CheckInterfaces) {
        compiler.do_pass::<CheckInterfaces>(()).expect("check_interfaces");
    }
    if through.includes(Through::TypeChecking) {
        compiler.do_pass::<TypeChecking>(TypeCheckingInput::new(compiler.network())).expect("type_checking");
    }
    if through.includes(Through::Disambiguate) {
        compiler.do_pass::<Disambiguate>(()).expect("disambiguate");
    }
    if through.includes(Through::ProcessingAsync) {
        compiler.do_pass::<ProcessingAsync>(TypeCheckingInput::new(compiler.network())).expect("processing_async");
    }

    compiler
}

// ---------------------------------------------------------------------------
// Shared benchmark patterns
// ---------------------------------------------------------------------------

fn bench_frontend_case(c: &mut Criterion, bench_name: &str, fixture: &FixtureData) {
    let filename = fixture.filename.clone();
    let module_refs = fixture.module_refs();

    c.bench_function(bench_name, |b| {
        b.iter(|| {
            let mut compiler = create_compiler(fixture);
            compiler.parse(&fixture.source, filename.clone(), &module_refs).expect("parse");
            compiler.add_import_stubs().expect("add_import_stubs");
            compiler.frontend_passes().expect("frontend_passes");
        });
    });
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

fn bench_frontend_with_deps(c: &mut Criterion) {
    create_session_if_not_set_then(|_| {
        let settlement = contrived_package("settlement");

        bench_frontend_case(c, "frontend_with_deps/contrived_settlement", &settlement);
    });
}

fn bench_dependency_chain(c: &mut Criterion) {
    create_session_if_not_set_then(|_| {
        let chain: &[(u8, &str)] =
            &[(1, "primitives"), (2, "registry"), (3, "policy"), (4, "router"), (5, "settlement")];

        let mut group = c.benchmark_group("dependency_chain");

        for &(depth, package_name) in chain {
            let fixture = contrived_package(package_name);
            let filename = fixture.filename.clone();
            let module_refs = fixture.module_refs();

            group.bench_function(BenchmarkId::from_parameter(format!("{depth:02}_{package_name}")), |b| {
                b.iter(|| {
                    let mut compiler = create_compiler(&fixture);
                    compiler.parse(&fixture.source, filename.clone(), &module_refs).expect("parse");
                    compiler.add_import_stubs().expect("add_import_stubs");
                    compiler.frontend_passes().expect("frontend_passes");
                });
            });
        }

        group.finish();
    });
}

fn bench_frontend_single_file(c: &mut Criterion) {
    create_session_if_not_set_then(|_| {
        let control_flow_matrix = single_source("control_flow_matrix.leo");

        bench_frontend_case(c, "frontend_single_file/control_flow_matrix", &control_flow_matrix);
    });
}

fn bench_individual_passes(c: &mut Criterion) {
    create_session_if_not_set_then(|_| {
        let fixture = contrived_package("settlement");
        let target = "contrived_settlement";

        let mut group = c.benchmark_group("individual_passes");

        group.bench_function(BenchmarkId::new("01_parsing", target), |b| {
            let filename = fixture.filename.clone();
            let module_refs = fixture.module_refs();
            b.iter(|| {
                let mut compiler = create_parse_only_compiler(&fixture);
                compiler.parse(&fixture.source, filename.clone(), &module_refs).expect("parse");
            });
        });

        group.bench_function(BenchmarkId::new("02_name_validation", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::Parsed),
                |mut c| c.do_pass::<NameValidation>(()).expect("name_validation"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("03_global_vars_collection", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::NameValidation),
                |mut c| c.do_pass::<GlobalVarsCollection>(()).expect("global_vars_collection"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("04_path_resolution", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::GlobalVarsCollection),
                |mut c| c.do_pass::<PathResolution>(()).expect("path_resolution"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("05_global_items_collection", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::PathResolution),
                |mut c| c.do_pass::<GlobalItemsCollection>(()).expect("global_items_collection"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("06_check_interfaces", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::GlobalItemsCollection),
                |mut c| c.do_pass::<CheckInterfaces>(()).expect("check_interfaces"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("07_type_checking", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::CheckInterfaces),
                |mut c| {
                    let network = c.network();
                    c.do_pass::<TypeChecking>(TypeCheckingInput::new(network)).expect("type_checking")
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("08_disambiguate", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::TypeChecking),
                |mut c| c.do_pass::<Disambiguate>(()).expect("disambiguate"),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("09_processing_async", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::Disambiguate),
                |mut c| {
                    let network = c.network();
                    c.do_pass::<ProcessingAsync>(TypeCheckingInput::new(network)).expect("processing_async")
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("10_static_analyzing", target), |b| {
            b.iter_batched(
                || prepare_through(&fixture, Through::ProcessingAsync),
                |mut c| c.do_pass::<StaticAnalyzing>(()).expect("static_analyzing"),
                BatchSize::SmallInput,
            );
        });

        group.finish();
    });
}

criterion_group!(
    benches,
    bench_frontend_with_deps,
    bench_dependency_chain,
    bench_frontend_single_file,
    bench_individual_passes
);
criterion_main!(benches);

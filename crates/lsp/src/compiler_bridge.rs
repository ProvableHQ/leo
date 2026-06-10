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

#![allow(clippy::mutable_key_type)]

//! Compiler-backed semantic analysis for `leo-lsp`.
//!
//! The worker always produces a syntax-derived token stream so highlighting can
//! stay responsive for malformed files. When a snapshot belongs to a resolvable
//! Leo package, or when it is a standalone `.leo` program buffer, this module
//! reruns the compiler frontend against the current in-memory text and upgrades
//! those syntax tokens with stable symbol identities and more accurate token
//! kinds.

use crate::{
    document_store::{AnalysisBucket, DocumentSnapshot, DocumentViewSnapshot, OpenFileOverlay},
    features::{diagnostics::CompilerDiagnostic, lsp_range::hash_text, semantic_tokens::encode_tokens},
    project_model::ProjectContext,
    semantics::{
        CachedDocumentView,
        CachedPackageAnalysis,
        FileRange,
        OccurrenceRole,
        SemanticIndex,
        SemanticKind,
        SemanticSnapshot,
        SemanticSource,
        SemanticTokenOccurrence,
        SourceFingerprint,
        SymbolIdentity,
        SymbolOccurrence,
        merge_occurrences,
        sort_token_occurrences,
    },
    syntax_semantics,
};
use anyhow::anyhow;
use indexmap::IndexMap;
use leo_ast::{
    Ast,
    AstVisitor,
    CallExpression,
    Composite,
    CompositeExpression,
    CompositeFieldInitializer,
    CompositeType,
    ConstDeclaration,
    DefinitionPlace,
    DefinitionStatement,
    DynamicOpExpression,
    DynamicOpKind,
    Function,
    FunctionPrototype,
    Identifier,
    Interface,
    IterationStatement,
    Location,
    Mapping,
    MappingPrototype,
    MemberAccess,
    Module,
    Node,
    Path,
    Program,
    ProgramId,
    ProgramScope,
    RecordPrototype,
    StorageVariable,
    StorageVariablePrototype,
    Stub,
    Type,
    UnitVisitor,
};
use leo_compiler::{Compiler, FrontendAnalysis, load_import_stubs_for_package_with_file_source};
use leo_errors::Handler;
use leo_passes::{SymbolTable, TypeTable, VariableType};
use leo_span::{
    Symbol,
    create_session_if_not_set_then,
    file_source::{DiskFileSource, FileSource},
    source_map::FileName,
    with_session_globals,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fs::Metadata,
    hash::{DefaultHasher, Hash, Hasher},
    io,
    path::{Path as StdPath, PathBuf},
    rc::Rc,
    sync::{Arc, atomic::Ordering},
    time::UNIX_EPOCH,
};

/// Maximum worker-local dependency-stub packages retained at once.
const MAX_PACKAGE_ANALYSIS_CACHE_ENTRIES: usize = 8;

/// Worker result for a package-analysis job.
#[derive(Debug, Clone)]
pub struct PackageWorkerAnalysis {
    /// Shared package analysis cached by package key on the routing thread.
    pub package: Arc<CachedPackageAnalysis>,
    /// Encoded semantic-token view for the document that triggered the job.
    pub document_view: CachedDocumentView,
}

/// Worker-local cache of package dependency stubs.
#[derive(Debug)]
pub struct PackageAnalysisCache {
    entries: HashMap<PathBuf, PackageAnalysisCacheEntry>,
    order: VecDeque<PathBuf>,
    network_sentinel: Arc<PathBuf>,
    library_sentinel: Arc<PathBuf>,
}

impl Default for PackageAnalysisCache {
    /// Create an empty worker-local cache with stable virtual roots for stubs.
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            network_sentinel: Arc::new(PathBuf::from("leo-vfs:network")),
            library_sentinel: Arc::new(PathBuf::from("leo-vfs:library")),
        }
    }
}

/// One cached package entry, including both the imported stubs and the
/// filesystem state used to decide whether those stubs are still current.
#[derive(Debug, Clone)]
struct PackageAnalysisCacheEntry {
    /// Package-wide import stubs reused across snapshots from the same root.
    import_stubs: Arc<IndexMap<Symbol, leo_ast::Stub>>,
    /// Compact source fingerprints recorded while parsing dependency stubs.
    ///
    /// The cache stores these as a sorted slice instead of a `HashMap` because
    /// they live as long as the package entry. We only need map lookup while
    /// lowering one worker result, so `fingerprints_with` builds that transient
    /// map at the boundary where it is actually useful.
    fingerprints: Arc<[(PathBuf, SourceFingerprint)]>,
    /// Program roots keyed by both manifest import names and leaf identifiers.
    dependency_roots: Arc<HashMap<Symbol, Arc<PathBuf>>>,
    /// Filesystem inputs whose metadata changes invalidate `import_stubs`.
    watch_paths: Arc<[PathBuf]>,
    /// Per-path revision memo reused to avoid rehashing unchanged watched inputs.
    watch_state: HashMap<PathBuf, CachedWatchedPathRevision>,
    /// Aggregate watched-path revision for the cached stub set.
    ///
    /// Each path revision hashes the actual file bytes or recursive directory
    /// listing, while `watch_state` lets unchanged metadata stamps reuse those
    /// full revisions between checks.
    revision: u64,
}

/// Cached import stubs plus the dependency source fingerprints captured when
/// those stubs were parsed.
#[derive(Debug, Clone)]
struct CachedImportStubs {
    import_stubs: Arc<IndexMap<Symbol, leo_ast::Stub>>,
    /// Sorted slice instead of a cached hash table to keep package caches lean.
    fingerprints: Arc<[(PathBuf, SourceFingerprint)]>,
    /// Resolved roots used to make program namespace identities collision-free.
    dependency_roots: Arc<HashMap<Symbol, Arc<PathBuf>>>,
}

/// Memoized revision for one watched path.
///
/// The cache keeps the expensive content-or-directory-listing hash around and
/// only recomputes it when lightweight metadata says the path changed.
#[derive(Debug, Clone)]
struct CachedWatchedPathRevision {
    /// Lightweight metadata stamp used as the steady-state cache key.
    stamp: Option<WatchedPathStamp>,
    /// Full content-or-listing revision reused while `stamp` stays unchanged.
    revision: u64,
}

/// Lightweight metadata snapshot used to skip rehashing unchanged watched inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WatchedPathStamp {
    /// The path no longer exists.
    Missing,
    /// A regular file keyed by size and last-modified time.
    File { len: u64, modified_nanos: u128 },
    /// A directory keyed by its last-modified time.
    Directory { modified_nanos: u128 },
}

/// Compiler frontend outputs lowered into LSP-friendly shapes.
///
/// `occurrences` is `None` when the frontend bailed before walking the AST;
/// the caller falls back to syntax-only highlighting but still publishes the
/// buffered diagnostics. Span resolution happens here, inside the live Leo
/// session, because the source map is gone once the session ends.
#[derive(Debug)]
struct CompilerOutput {
    occurrences: Option<Vec<SymbolOccurrence>>,
    fingerprints: HashMap<PathBuf, SourceFingerprint>,
    diagnostic_entries: Vec<crate::features::diagnostics::DiagnosticEntry>,
}

/// Program roots available to one compiler semantic collection pass.
#[derive(Debug, Clone)]
struct ProgramRoots {
    /// Package root for the snapshot's owning package, absent for loose files.
    current: Option<Arc<PathBuf>>,
    /// Source dependency roots keyed by both full import and leaf symbols.
    source_dependencies: Arc<HashMap<Symbol, Arc<PathBuf>>>,
    /// Shared virtual root for bytecode/network-only programs.
    network_sentinel: Arc<PathBuf>,
    /// Shared virtual root for standard-library identities.
    library_sentinel: Arc<PathBuf>,
}

impl ProgramRoots {
    /// Build roots for the current package plus every imported stub identity.
    fn for_package(
        current: Arc<PathBuf>,
        dependency_roots: Arc<HashMap<Symbol, Arc<PathBuf>>>,
        import_stubs: &IndexMap<Symbol, leo_ast::Stub>,
        network_sentinel: Arc<PathBuf>,
        library_sentinel: Arc<PathBuf>,
    ) -> Self {
        let mut source_dependencies = dependency_roots.as_ref().clone();
        for (import_name, stub) in import_stubs {
            let root = match stub {
                Stub::FromLeo { .. } => source_dependencies.get(import_name).cloned(),
                Stub::FromAleo { .. } => Some(Arc::clone(&network_sentinel)),
                Stub::FromLibrary { .. } => Some(Arc::clone(&library_sentinel)),
            };
            if let Some(root) = root {
                insert_stub_root_aliases(&mut source_dependencies, *import_name, stub, root);
            }
        }

        Self {
            current: Some(current),
            source_dependencies: Arc::new(source_dependencies),
            network_sentinel,
            library_sentinel,
        }
    }

    /// Return the root that makes a program identity stable.
    fn root_for_symbol(&self, symbol: Symbol) -> Option<Arc<PathBuf>> {
        self.source_dependencies.get(&symbol).cloned()
    }

    /// Return a virtual root for a compiler stub with no source package.
    fn root_for_stub(&self, stub: &Stub) -> Option<Arc<PathBuf>> {
        match stub {
            Stub::FromLeo { .. } => None,
            Stub::FromAleo { .. } => Some(Arc::clone(&self.network_sentinel)),
            Stub::FromLibrary { .. } => Some(Arc::clone(&self.library_sentinel)),
        }
    }
}

/// Build the latest package analysis plus the trigger document's token view.
///
/// Syntax analysis always runs first so the server can return a best-effort
/// token stream even when package discovery, dependency loading, or compiler
/// analysis fail. Compiler-backed occurrences then replace matching syntax
/// ranges when available, and any buffered diagnostics from the same compiler
/// pass are threaded through to the cached package result.
pub fn analyze_package_snapshot(
    snapshot: &DocumentSnapshot,
    package_cache: &mut PackageAnalysisCache,
) -> PackageWorkerAnalysis {
    let syntax = syntax_semantics::collect(snapshot);
    if snapshot_is_cancelled(snapshot) {
        return package_analysis(
            snapshot,
            syntax.occurrences,
            syntax.tokens,
            SemanticSource::SyntaxOnly,
            HashMap::new(),
            Vec::new(),
        );
    }

    let compiler_occurrences = compiler_occurrences(snapshot, package_cache);
    if snapshot_is_cancelled(snapshot) {
        return package_analysis(
            snapshot,
            syntax.occurrences,
            syntax.tokens,
            SemanticSource::SyntaxOnly,
            HashMap::new(),
            Vec::new(),
        );
    }

    let (occurrences, lexical_tokens, source, fingerprints, diagnostic_entries) = match compiler_occurrences {
        Some(CompilerOutput { occurrences: Some(occurrences), fingerprints, diagnostic_entries }) => {
            let mut merged_fingerprints = syntax.fingerprints;
            merged_fingerprints.extend(fingerprints);
            (
                merge_occurrences(syntax.occurrences, occurrences),
                syntax.tokens,
                SemanticSource::CompilerEnhanced,
                merged_fingerprints,
                diagnostic_entries,
            )
        }
        Some(CompilerOutput { occurrences: None, diagnostic_entries, .. }) => {
            // The compiler buffered diagnostics but did not produce usable
            // AST occurrences (typical when the parser bailed). Use the
            // package-fallback syntax tokens — they cover the full module
            // listing — and still publish the buffered diagnostics so users
            // see the parse error.
            let syntax = syntax_semantics::collect_package_fallback(snapshot);
            (syntax.occurrences, syntax.tokens, SemanticSource::SyntaxOnly, syntax.fingerprints, diagnostic_entries)
        }
        None => {
            // Compiler analysis declined to run at all (no usable file
            // source). Fall back to syntax tokens with no diagnostics;
            // "compiler unavailable" is not a user-facing condition.
            let syntax = syntax_semantics::collect_package_fallback(snapshot);
            (syntax.occurrences, syntax.tokens, SemanticSource::SyntaxOnly, syntax.fingerprints, Vec::new())
        }
    };

    package_analysis(snapshot, occurrences, lexical_tokens, source, fingerprints, diagnostic_entries)
}

/// Compatibility helper retained for PR 2 tests and callers.
#[allow(dead_code)]
pub fn analyze_snapshot(snapshot: &DocumentSnapshot, package_cache: &mut PackageAnalysisCache) -> SemanticSnapshot {
    let analysis = analyze_package_snapshot(snapshot, package_cache);
    SemanticSnapshot {
        encoded_tokens: analysis.document_view.encoded_tokens,
        index: Arc::clone(&analysis.package.index),
        source: analysis.package.source,
    }
}

/// Build a document token view from a cached package analysis.
pub fn build_document_view(snapshot: &DocumentViewSnapshot, package: Arc<CachedPackageAnalysis>) -> CachedDocumentView {
    let syntax = syntax_semantics::collect_view(snapshot);
    let package_tokens = snapshot
        .file_path
        .as_deref()
        .map(|path| package.index.token_occurrences_for_file(path.as_ref()))
        .unwrap_or_default();
    let tokens = semantic_token_occurrences(&package_tokens, syntax.occurrences, syntax.tokens);
    let encoded_tokens = encode_tokens(&tokens, snapshot.file_path.as_deref(), snapshot.line_index.as_ref());
    CachedDocumentView { key: snapshot.key.clone(), encoded_tokens }
}

/// Lower merged occurrences and lowered diagnostic entries into a shared
/// package index, document view, and immutable diagnostic set.
fn package_analysis(
    snapshot: &DocumentSnapshot,
    mut occurrences: Vec<SymbolOccurrence>,
    lexical_tokens: Vec<SemanticTokenOccurrence>,
    source: SemanticSource,
    recorded_fingerprints: HashMap<PathBuf, SourceFingerprint>,
    diagnostic_entries: Vec<crate::features::diagnostics::DiagnosticEntry>,
) -> PackageWorkerAnalysis {
    let package_source_files = PackageSourceFiles::from_snapshot(snapshot);
    retain_in_scope_occurrences(&mut occurrences, &package_source_files);

    let (index, analyzed_files) = SemanticIndex::build(
        &occurrences,
        |path| {
            recorded_fingerprints
                .get(path)
                .cloned()
                .or_else(|| open_buffer_fingerprint(snapshot.open_overlays.as_ref(), path))
                .unwrap_or(SourceFingerprint::Volatile)
        },
        |path| open_line_index(snapshot.open_overlays.as_ref(), path),
        |path| package_source_files.contains(path),
    );
    let index = Arc::new(index);
    let diagnostics = Arc::new(crate::features::diagnostics::DiagnosticSet {
        key: snapshot.package_key.clone(),
        entries: Arc::from(crate::features::diagnostics::finalize_entries(diagnostic_entries)),
    });
    let package = Arc::new(CachedPackageAnalysis {
        key: snapshot.package_key.clone(),
        index: Arc::clone(&index),
        analyzed_files: Arc::new(analyzed_files),
        source,
        diagnostics,
    });

    let package_tokens =
        snapshot.file_path.as_deref().map(|path| index.token_occurrences_for_file(path.as_ref())).unwrap_or_default();
    let semantic_tokens = semantic_token_occurrences(&package_tokens, Vec::new(), lexical_tokens);
    let encoded_tokens = encode_tokens(&semantic_tokens, snapshot.file_path.as_deref(), snapshot.line_index.as_ref());
    let document_view = CachedDocumentView { key: snapshot.view_key.clone(), encoded_tokens };

    PackageWorkerAnalysis { package, document_view }
}

/// Canonical package source membership used to trim dependency-stub noise.
#[derive(Debug, Clone)]
pub(crate) struct PackageSourceFiles {
    source_directory: Option<Arc<PathBuf>>,
    paths: Box<[Arc<PathBuf>]>,
}

impl PackageSourceFiles {
    /// Capture the source paths that belong to the current package snapshot.
    fn from_snapshot(snapshot: &DocumentSnapshot) -> Self {
        let source_directory = snapshot.project.as_ref().map(|project| Arc::clone(&project.source_directory));
        let mut paths = Vec::<Arc<PathBuf>>::new();
        if let Some(file_path) = snapshot.file_path.as_ref() {
            paths.push(Arc::clone(file_path));
        }
        for overlay in snapshot.open_overlays.iter() {
            paths.push(Arc::clone(&overlay.path));
        }
        paths.sort_by(|left, right| left.as_path().cmp(right.as_path()));
        paths.dedup_by(|left, right| left.as_path() == right.as_path());
        Self { source_directory, paths: paths.into_boxed_slice() }
    }

    /// Return whether a path belongs to the package being actively edited.
    pub(crate) fn contains(&self, path: &StdPath) -> bool {
        if self.source_directory.as_ref().is_some_and(|source_directory| path.starts_with(source_directory.as_ref())) {
            return true;
        }
        self.paths.binary_search_by(|candidate| candidate.as_path().cmp(path)).is_ok()
    }
}

/// Drop dependency-stub noise while retaining declarations and keyed references.
pub(crate) fn retain_in_scope_occurrences(
    occurrences: &mut Vec<SymbolOccurrence>,
    package_source_files: &PackageSourceFiles,
) {
    occurrences.retain(|occurrence| {
        package_source_files.contains(occurrence.range.path.as_ref())
            || occurrence.role == OccurrenceRole::Declaration
            || occurrence.identity.key().is_some()
    });
}

/// Merge symbol occurrences with highlighting-only lexical tokens for encoding.
fn semantic_token_occurrences(
    package_tokens: &[SemanticTokenOccurrence],
    syntax_occurrences: Vec<SymbolOccurrence>,
    mut lexical_tokens: Vec<SemanticTokenOccurrence>,
) -> Vec<SemanticTokenOccurrence> {
    let mut tokens = Vec::with_capacity(package_tokens.len() + syntax_occurrences.len() + lexical_tokens.len());
    tokens.extend(package_tokens.iter().cloned());
    tokens.extend(syntax_occurrences.iter().map(SemanticTokenOccurrence::from_symbol));
    tokens.append(&mut lexical_tokens);
    sort_token_occurrences(&mut tokens);

    // Package tokens are inserted before syntax and lexical tokens, so exact
    // range ties keep navigation-grade compiler classifications.
    tokens.dedup_by(|left, right| left.range == right.range);
    tokens
}

/// Try to refine syntax occurrences with compiler frontend analysis.
///
/// Dependency-stub failures and handler-buffered errors both flow back as
/// `CompilerOutput` so the caller can publish them as diagnostics even when
/// no AST was produced.
fn compiler_occurrences(
    snapshot: &DocumentSnapshot,
    package_cache: &mut PackageAnalysisCache,
) -> Option<CompilerOutput> {
    if snapshot_is_cancelled(snapshot) {
        return None;
    }

    let input = compiler_input(snapshot)?;

    let trigger_path = snapshot.file_path.clone();
    let result = create_session_if_not_set_then(|_| {
        // Dependency resolution and parsing intern symbols, so the worker must
        // enter a Leo session before it asks the compiler for frontend state.
        let file_source = RecordingFileSource::new(Arc::clone(&snapshot.open_overlays));
        match input {
            CompilerInput::ManagedPackage { project } => {
                let import_stubs = match package_cache.import_stubs_for(project.as_ref(), &file_source) {
                    Ok(import_stubs) => import_stubs,
                    Err(error) => {
                        tracing::debug!(
                            package = project.package_root.display().to_string(),
                            error = error.as_str(),
                            "dependency stub loading failed"
                        );
                        // Surface stub-loading errors as a synthetic
                        // package-level diagnostic. Falling back to syntax-only
                        // analysis is still useful, but the user needs to see
                        // why their import graph cannot be resolved.
                        let synthetic = vec![dependency_load_error(&error)];
                        let diagnostic_entries = lower_compiler_diagnostics(&synthetic, &[], trigger_path.as_ref());
                        return Ok::<_, String>(CompilerOutput {
                            occurrences: None,
                            fingerprints: file_source.fingerprints_with(&[]),
                            diagnostic_entries,
                        });
                    }
                };
                let program_roots = ProgramRoots::for_package(
                    Arc::clone(&project.package_root),
                    Arc::clone(&import_stubs.dependency_roots),
                    import_stubs.import_stubs.as_ref(),
                    Arc::clone(&package_cache.network_sentinel),
                    Arc::clone(&package_cache.library_sentinel),
                );

                // Run the compiler against every open same-package editor
                // buffer while recording fingerprints for disk files at the
                // exact read boundary.
                let outcome = run_compiler_analysis(
                    Some(project.program_name.to_string()),
                    project.entry_file.as_ref(),
                    project.source_directory.as_ref(),
                    &file_source,
                    import_stubs.import_stubs.as_ref().clone(),
                    program_roots,
                    || check_snapshot_current(snapshot),
                );

                let diagnostic_entries =
                    lower_compiler_diagnostics(&outcome.errors, &outcome.warnings, trigger_path.as_ref());
                check_snapshot_current(snapshot).map_err(|error| error.to_string())?;
                Ok::<_, String>(CompilerOutput {
                    occurrences: outcome.occurrences,
                    fingerprints: file_source.fingerprints_with(import_stubs.fingerprints.as_ref()),
                    diagnostic_entries,
                })
            }
            CompilerInput::StandaloneProgram { file_path, source_directory } => {
                let single_file_source = SingleFileSource::new(&file_source);
                let program_roots = ProgramRoots {
                    current: None,
                    source_dependencies: Arc::new(HashMap::new()),
                    network_sentinel: Arc::clone(&package_cache.network_sentinel),
                    library_sentinel: Arc::clone(&package_cache.library_sentinel),
                };
                // Loose editor buffers should be analyzed as exactly one file.
                // Scanning the parent directory would accidentally treat
                // formatter fixtures or unrelated scratch files as Leo modules.
                let outcome = run_compiler_analysis(
                    None,
                    file_path.as_ref(),
                    source_directory.as_ref(),
                    &single_file_source,
                    IndexMap::new(),
                    program_roots,
                    || check_snapshot_current(snapshot),
                );

                let diagnostic_entries =
                    lower_compiler_diagnostics(&outcome.errors, &outcome.warnings, trigger_path.as_ref());
                check_snapshot_current(snapshot).map_err(|error| error.to_string())?;
                Ok::<_, String>(CompilerOutput {
                    occurrences: outcome.occurrences,
                    fingerprints: file_source.fingerprints_with(&[]),
                    diagnostic_entries,
                })
            }
        }
    });

    match result {
        Ok(output) => Some(output),
        Err(error) => {
            tracing::debug!(uri = snapshot.uri.as_str(), error, "compiler semantic analysis unavailable; falling back");
            None
        }
    }
}

/// Wrap a dependency-stub load failure as a `Backtraced` error so it lowers
/// into a synthetic diagnostic on the saved trigger document.
fn dependency_load_error(message: &str) -> leo_errors::LeoError {
    leo_errors::Backtraced::new_from_backtrace(
        format!("Leo dependency analysis failed: {message}"),
        None,
        0,
        "LSP".to_owned(),
        true,
        leo_errors::Backtrace::new(),
    )
    .into()
}

/// Lower buffered compiler errors and warnings into LSP-ready entries.
///
/// Must run inside `create_session_if_not_set_then` because span resolution
/// reads the source map through `with_session_globals`.
fn lower_compiler_diagnostics(
    errors: &[leo_errors::LeoError],
    warnings: &[leo_errors::LeoWarning],
    trigger_path: Option<&Arc<PathBuf>>,
) -> Vec<crate::features::diagnostics::DiagnosticEntry> {
    let mut diagnostics: Vec<CompilerDiagnostic<'_>> = Vec::with_capacity(errors.len() + warnings.len());
    diagnostics.extend(errors.iter().map(CompilerDiagnostic::Error));
    diagnostics.extend(warnings.iter().map(CompilerDiagnostic::Warning));
    crate::features::diagnostics::lower_diagnostic_entries(trigger_path, diagnostics)
}

/// Compiler frontend input shape selected for a document snapshot.
enum CompilerInput {
    /// A document inside a discovered Leo package source tree.
    ManagedPackage { project: Arc<ProjectContext> },
    /// A loose `.leo` program buffer analyzed without sibling module discovery.
    StandaloneProgram { file_path: Arc<PathBuf>, source_directory: PathBuf },
}

/// Return the compiler analysis mode supported by this snapshot.
fn compiler_input(snapshot: &DocumentSnapshot) -> Option<CompilerInput> {
    let file_path = snapshot.file_path.as_ref()?;
    if let Some(project) = snapshot.project.as_ref()
        && file_path.starts_with(project.source_directory.as_ref())
    {
        return Some(CompilerInput::ManagedPackage { project: Arc::clone(project) });
    }

    Some(CompilerInput::StandaloneProgram {
        file_path: Arc::clone(file_path),
        source_directory: file_path.parent()?.to_path_buf(),
    })
}

/// Result of running the compiler frontend for one LSP analysis pass.
///
/// `occurrences` is `None` when analysis failed before the AST walker ran;
/// `errors` and `warnings` may still be populated in that case.
struct CompilerAnalysisOutcome {
    occurrences: Option<Vec<SymbolOccurrence>>,
    errors: Vec<leo_errors::LeoError>,
    warnings: Vec<leo_errors::LeoWarning>,
}

/// Run frontend analysis with a buffered handler and capture both semantic
/// occurrences and diagnostics.
///
/// After the frontend runs we drain both buffers and merge in any
/// final `LeoError` that the analysis call returned, taking care to drop the
/// `LastErrorCode` sentinel because the real diagnostic has already been
/// emitted through the handler.
fn run_compiler_analysis(
    expected_unit_name: Option<String>,
    entry_file: &StdPath,
    source_directory: &StdPath,
    file_source: &impl FileSource,
    import_stubs: IndexMap<Symbol, leo_ast::Stub>,
    program_roots: ProgramRoots,
    mut should_continue: impl FnMut() -> leo_errors::Result<()>,
) -> CompilerAnalysisOutcome {
    let (handler, emitter) = Handler::new_with_buf();
    let mut compiler = Compiler::new(
        expected_unit_name,
        false,
        handler,
        Rc::new(leo_ast::NodeBuilder::default()),
        PathBuf::default(),
        Some(leo_compiler::CompilerOptions::default()),
        import_stubs,
        leo_ast::NetworkName::TestnetV0,
    );

    let frontend_result = compiler.analyze_frontend_from_directory_with_file_source_and_check(
        entry_file,
        source_directory,
        file_source,
        &mut should_continue,
    );

    // Collect AST-derived occurrences when the frontend ran to completion;
    // diagnostics flow through the handler regardless.
    let mut returned_error: Option<leo_errors::LeoError> = None;
    let occurrences = match frontend_result {
        Ok(FrontendAnalysis { ast, symbol_table, type_table }) => {
            Some(CompilerSemanticCollector::new(symbol_table, type_table, program_roots).collect(ast))
        }
        Err(error) => {
            returned_error = Some(error);
            None
        }
    };

    let mut errors = emitter.extract_errs().into_inner();
    let warnings = emitter.extract_warnings().into_inner();
    if let Some(returned) = returned_error
        && !returned.is_last_error_code()
    {
        // `LastErrorCode` is the handler's way of saying "I already buffered
        // the real diagnostic," so we drop it here to avoid double-publishing
        // the same error. Any other `LeoError` represents a failure path the
        // frontend never had a chance to push through the handler — keep it.
        errors.push(returned);
    }

    CompilerAnalysisOutcome { occurrences, errors, warnings }
}

/// File source that serves all open same-package buffers and records read fingerprints.
///
/// The compiler's [`FileSource`] trait takes `&self`, but go-to-definition must
/// know the exact bytes the compiler consumed for every indexed file. This
/// recorder therefore uses interior mutability to append fingerprints during
/// `read_file`. It is created per worker job and never shared across threads, so
/// `RefCell<HashMap<...>>` is the smallest honest tool here: a `Mutex` would add
/// unnecessary atomic locking/poisoning machinery, and `DashMap` would imply
/// concurrent writers that cannot exist for this job-local file source.
struct RecordingFileSource {
    overlays: Arc<[OpenFileOverlay]>,
    fingerprints: RefCell<HashMap<PathBuf, SourceFingerprint>>,
}

impl RecordingFileSource {
    /// Create a recording file source over the open buffers captured by a snapshot.
    fn new(overlays: Arc<[OpenFileOverlay]>) -> Self {
        Self { overlays, fingerprints: RefCell::new(HashMap::new()) }
    }

    /// Return a compact, deterministic copy of dependency fingerprints for cache storage.
    fn dependency_fingerprints(&self) -> Arc<[(PathBuf, SourceFingerprint)]> {
        let mut fingerprints = self
            .fingerprints
            .borrow()
            .iter()
            .map(|(path, fingerprint)| (path.clone(), fingerprint.clone()))
            .collect::<Vec<_>>();
        // Keep cached dependency fingerprints deterministic and compact. The
        // semantic-index builder can expand this slice into a temporary map
        // later, but the long-lived package cache does not pay for hash buckets.
        fingerprints.sort_by(|(left, _), (right, _)| left.cmp(right));
        Arc::from(fingerprints)
    }

    /// Merge cached dependency fingerprints with reads performed by this job.
    fn fingerprints_with(&self, cached: &[(PathBuf, SourceFingerprint)]) -> HashMap<PathBuf, SourceFingerprint> {
        let recorded = self.fingerprints.borrow();
        let mut fingerprints = HashMap::with_capacity(cached.len() + recorded.len());
        fingerprints.extend(cached.iter().cloned());
        fingerprints.extend(recorded.iter().map(|(path, fingerprint)| (path.clone(), fingerprint.clone())));
        fingerprints
    }

    /// Record the fingerprint for bytes returned through this file source.
    fn record(&self, path: &StdPath, fingerprint: SourceFingerprint) {
        self.fingerprints.borrow_mut().insert(path.to_path_buf(), fingerprint);
    }
}

impl FileSource for RecordingFileSource {
    /// Read a source file from an open overlay or disk and capture its fingerprint.
    fn read_file(&self, path: &StdPath) -> io::Result<String> {
        if let Some(overlay) = self.overlays.iter().find(|overlay| overlay.path.as_ref() == path) {
            let text = overlay.text.to_string();
            self.record(path, SourceFingerprint::OpenBuffer);
            return Ok(text);
        }

        let before = std::fs::metadata(path).ok().and_then(|metadata| disk_stamp(&metadata));
        let contents = std::fs::read_to_string(path)?;
        let after = std::fs::metadata(path).ok().and_then(|metadata| disk_stamp(&metadata));
        let fingerprint = match (before, after) {
            // Only disk bytes bracketed by identical metadata are safe to
            // re-open later for LSP range conversion. If a write races the
            // read, we mark the file volatile and suppress cross-file targets.
            (Some(before), Some(after)) if before == after => SourceFingerprint::Disk {
                modified_nanos: Some(after.modified_nanos),
                len: after.len,
                content_hash: content_hash(contents.as_str()),
            },
            _ => SourceFingerprint::Volatile,
        };
        self.record(path, fingerprint);
        Ok(contents)
    }

    /// List package source modules, including unsaved open overlays.
    fn list_leo_files(&self, dir: &StdPath, exclude: &StdPath) -> io::Result<Vec<PathBuf>> {
        let mut files = DiskFileSource.list_leo_files(dir, exclude)?;
        for overlay in self.overlays.iter() {
            // Include unsaved same-package module buffers in compiler analysis
            // even if the file has not been flushed to disk yet.
            if overlay.path.starts_with(dir)
                && overlay.path.extension().is_some_and(|extension| extension == "leo")
                && overlay.path.as_ref() != exclude
                && !files.iter().any(|path| path == overlay.path.as_ref())
            {
                files.push(overlay.path.as_ref().clone());
            }
        }
        files.sort();
        Ok(files)
    }
}

/// File source adapter for standalone buffers that must not discover modules.
struct SingleFileSource<'a> {
    inner: &'a RecordingFileSource,
}

impl<'a> SingleFileSource<'a> {
    /// Create a single-file view over a recording source.
    fn new(inner: &'a RecordingFileSource) -> Self {
        Self { inner }
    }
}

impl FileSource for SingleFileSource<'_> {
    /// Read through the recording source so fingerprints stay accurate.
    fn read_file(&self, path: &StdPath) -> io::Result<String> {
        self.inner.read_file(path)
    }

    /// Suppress sibling module discovery for loose, unmanaged files.
    fn list_leo_files(&self, _dir: &StdPath, _exclude: &StdPath) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

/// Cheap filesystem stamp used to prove a disk read was stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiskStamp {
    /// File length observed in metadata.
    len: u64,
    /// Last-modified timestamp converted to nanoseconds since the Unix epoch.
    modified_nanos: u128,
}

/// Build a comparable stamp from filesystem metadata.
fn disk_stamp(metadata: &Metadata) -> Option<DiskStamp> {
    Some(DiskStamp { len: metadata.len(), modified_nanos: metadata_modified_nanos(metadata)? })
}

/// Return the current open-buffer fingerprint for an analyzed path.
fn open_buffer_fingerprint(overlays: &[OpenFileOverlay], path: &StdPath) -> Option<SourceFingerprint> {
    overlays.iter().any(|overlay| overlay.path.as_ref() == path).then_some(SourceFingerprint::OpenBuffer)
}

/// Return the line index for an open buffer referenced by compact ranges.
fn open_line_index(overlays: &[OpenFileOverlay], path: &StdPath) -> Option<Arc<line_index::LineIndex>> {
    overlays.iter().find(|overlay| overlay.path.as_ref() == path).map(|overlay| Arc::clone(&overlay.line_index))
}

/// Hash source text for stale-target detection without retaining full text.
fn content_hash(contents: &str) -> u64 {
    hash_text(contents)
}

impl PackageAnalysisCache {
    /// Drop worker-local package stub entries that no longer have open documents.
    pub fn retain_open_buckets(&mut self, open_buckets: &HashSet<AnalysisBucket>) {
        self.entries.retain(|package_root, _| {
            open_buckets
                .iter()
                .any(|bucket| matches!(bucket, AnalysisBucket::ManagedPackage { package_root: root } if root.as_ref() == package_root))
        });
        self.order.retain(|package_root| self.entries.contains_key(package_root));
    }

    /// Return cached import stubs for the project, reloading them whenever the
    /// watched manifest or source metadata changes.
    fn import_stubs_for(
        &mut self,
        project: &ProjectContext,
        file_source: &RecordingFileSource,
    ) -> Result<CachedImportStubs, String> {
        if let Some(entry) = self.entries.get_mut(project.package_root.as_ref())
            && entry.revision == watched_paths_revision_cached(entry.watch_paths.as_ref(), &mut entry.watch_state)
        {
            let import_stubs = Arc::clone(&entry.import_stubs);
            let fingerprints = Arc::clone(&entry.fingerprints);
            let dependency_roots = Arc::clone(&entry.dependency_roots);
            self.touch_entry(project.package_root.as_ref());
            return Ok(CachedImportStubs { import_stubs, fingerprints, dependency_roots });
        }

        // Import stubs are package-wide, but they depend on manifest/source
        // metadata. Rebuild the entry when any watched input changes.
        let loaded = load_import_stubs_for_package_with_file_source(
            project.package_root.as_ref(),
            leo_ast::NetworkName::TestnetV0,
            file_source,
        )
        .map_err(|error| error.to_string())?;
        let watch_paths = Arc::<[PathBuf]>::from(loaded.watch_paths);
        let mut watch_state = HashMap::new();
        let revision = watched_paths_revision_cached(watch_paths.as_ref(), &mut watch_state);
        let import_stubs = Arc::new(loaded.stubs);
        let dependency_roots = dependency_roots_from_stubs(import_stubs.as_ref());
        // Capturing dependency fingerprints here is what lets source-dependency
        // navigation return real disk locations on cache hits. Without carrying
        // these fingerprints alongside the stubs, dependency definition and
        // reference targets would later look volatile and be filtered out.
        let fingerprints = file_source.dependency_fingerprints();
        let package_root = project.package_root.as_ref().clone();
        self.entries.insert(package_root.clone(), PackageAnalysisCacheEntry {
            import_stubs: Arc::clone(&import_stubs),
            fingerprints: Arc::clone(&fingerprints),
            dependency_roots: Arc::clone(&dependency_roots),
            watch_paths,
            watch_state,
            revision,
        });
        self.touch_entry(&package_root);
        self.evict_old_entries(&package_root);
        Ok(CachedImportStubs { import_stubs, fingerprints, dependency_roots })
    }

    /// Mark a package-root cache entry as most recently used.
    fn touch_entry(&mut self, package_root: &StdPath) {
        self.order.retain(|candidate| candidate.as_path() != package_root);
        self.order.push_back(package_root.to_path_buf());
    }

    /// Evict old package stub entries while preserving the entry just loaded.
    fn evict_old_entries(&mut self, protected: &StdPath) {
        self.order.retain(|package_root| self.entries.contains_key(package_root));
        let mut attempts = self.order.len();
        while self.entries.len() > MAX_PACKAGE_ANALYSIS_CACHE_ENTRIES && attempts > 0 {
            attempts -= 1;
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            if oldest.as_path() == protected {
                self.order.push_back(oldest);
            } else {
                self.entries.remove(&oldest);
            }
        }
    }
}

/// Recover source dependency roots from stub declaration spans without widening compiler APIs.
fn dependency_roots_from_stubs(stubs: &IndexMap<Symbol, Stub>) -> Arc<HashMap<Symbol, Arc<PathBuf>>> {
    let mut roots = HashMap::new();
    for (import_name, stub) in stubs {
        match stub {
            Stub::FromLeo { program, .. } => {
                let Some(root) = stub_program_declaration_range(stub)
                    .and_then(|range| package_root_for_source(range.path.as_ref()))
                    .map(Arc::new)
                else {
                    continue;
                };
                insert_program_root_aliases(&mut roots, *import_name, Arc::clone(&root));
                for scope in program.program_scopes.values() {
                    insert_program_root_aliases(&mut roots, scope.program_id.as_symbol(), Arc::clone(&root));
                    insert_program_root_aliases(&mut roots, scope.program_id.name.name, Arc::clone(&root));
                }
            }
            Stub::FromAleo { .. } | Stub::FromLibrary { .. } => {}
        }
    }
    Arc::new(roots)
}

/// Insert both full and leaf symbols when they differ.
fn insert_program_root_aliases(roots: &mut HashMap<Symbol, Arc<PathBuf>>, symbol: Symbol, root: Arc<PathBuf>) {
    roots.entry(symbol).or_insert(root);
}

/// Insert every symbol spelling that can name an imported stub.
fn insert_stub_root_aliases(
    roots: &mut HashMap<Symbol, Arc<PathBuf>>,
    import_name: Symbol,
    stub: &Stub,
    root: Arc<PathBuf>,
) {
    insert_program_root_aliases(roots, import_name, Arc::clone(&root));
    match stub {
        Stub::FromLeo { program, .. } => {
            for scope in program.program_scopes.values() {
                insert_program_root_aliases(roots, scope.program_id.as_symbol(), Arc::clone(&root));
                insert_program_root_aliases(roots, scope.program_id.name.name, Arc::clone(&root));
            }
        }
        Stub::FromAleo { program, .. } => {
            insert_program_root_aliases(roots, program.stub_id.as_symbol(), Arc::clone(&root));
            insert_program_root_aliases(roots, program.stub_id.name.name, root);
        }
        Stub::FromLibrary { library, .. } => {
            insert_program_root_aliases(roots, library.name, root);
        }
    }
}

/// Find the nearest manifest root owning a source file.
fn package_root_for_source(path: &StdPath) -> Option<PathBuf> {
    for ancestor in path.parent()?.ancestors() {
        let manifest = ancestor.join(leo_package::MANIFEST_FILENAME);
        if manifest.is_file() {
            return Some(ancestor.canonicalize().unwrap_or_else(|_| ancestor.to_path_buf()));
        }
    }
    None
}

/// Lexically scoped binding metadata reused when later paths resolve locally.
#[derive(Debug, Clone)]
struct LocalBinding {
    /// Declaration range reused as the stable local identity for later references.
    declaration: FileRange,
    /// Token kind reused by all references to the local binding.
    token_kind: SemanticKind,
    /// Whether the binding should also carry the readonly modifier.
    readonly: bool,
}

/// Walks compiler frontend state and turns declarations and references into a
/// reusable semantic index.
///
/// The collector tracks enough lexical scope and ownership information to keep
/// local identities stable while still attaching global items and members to
/// compiler `Location`s for upcoming navigation features.
struct CompilerSemanticCollector<'a> {
    symbol_table: &'a SymbolTable,
    #[allow(dead_code)]
    type_table: &'a TypeTable,
    occurrences: Vec<SymbolOccurrence>,
    program_roots: ProgramRoots,
    current_program: Symbol,
    current_program_root: Option<Arc<PathBuf>>,
    stub_depth: usize,
    current_module: Vec<Symbol>,
    local_scopes: Vec<HashMap<Symbol, LocalBinding>>,
    owner_stack: Vec<Option<Location>>,
}

impl<'a> CompilerSemanticCollector<'a> {
    /// Create a fresh collector bound to one compiler frontend snapshot.
    fn new(symbol_table: &'a SymbolTable, type_table: &'a TypeTable, program_roots: ProgramRoots) -> Self {
        Self {
            symbol_table,
            type_table,
            occurrences: Vec::new(),
            current_program_root: program_roots.current.clone(),
            program_roots,
            current_program: Symbol::intern(""),
            stub_depth: 0,
            current_module: Vec::new(),
            local_scopes: Vec::new(),
            owner_stack: Vec::new(),
        }
    }

    /// Walk the AST and return every semantic occurrence discovered in it.
    fn collect(mut self, ast: &'a Ast) -> Vec<SymbolOccurrence> {
        match ast {
            Ast::Program(program) => self.visit_program(program),
            Ast::Library(library) => self.visit_library(library),
        }
        self.occurrences
    }

    /// Push a new lexical scope for local bindings.
    fn push_scope(&mut self) {
        self.local_scopes.push(HashMap::new());
    }

    /// Pop the current lexical scope.
    fn pop_scope(&mut self) {
        self.local_scopes.pop();
    }

    /// Record a local declaration and make later local-path references resolve to it.
    fn bind_local(&mut self, identifier: &Identifier, token_kind: SemanticKind, readonly: bool) {
        if let Some(range) = span_to_file_range(identifier.span) {
            self.occurrences.push(SymbolOccurrence {
                range: range.clone(),
                identity: SymbolIdentity::Local { declaration: range.clone() },
                role: OccurrenceRole::Declaration,
                token_kind,
                readonly,
            });
            if let Some(scope) = self.local_scopes.last_mut() {
                scope.insert(identifier.name, LocalBinding { declaration: range, token_kind, readonly });
            }
        }
    }

    /// Return the nearest enclosing owner used for member identities.
    fn current_owner(&self) -> Option<Location> {
        self.owner_stack.iter().rev().find_map(Clone::clone)
    }

    /// Build the compiler location for a declaration in the current program/module context.
    fn current_item_location(&self, name: Symbol) -> Location {
        let mut path = self.current_module.clone();
        path.push(name);
        Location::new(self.current_program, path)
    }

    /// Build a nested declaration location under the current semantic owner.
    fn owned_item_location(&self, name: Symbol) -> Location {
        if let Some(owner) = self.current_owner() {
            let mut path = owner.path;
            path.push(name);
            Location::new(owner.program, path)
        } else {
            self.current_item_location(name)
        }
    }

    /// Record a program or imported-namespace occurrence.
    fn add_namespace_occurrence(&mut self, identifier: &Identifier, role: OccurrenceRole) {
        self.add_namespace_occurrence_with_declaration(
            identifier,
            role,
            matches!(role, OccurrenceRole::Declaration).then(|| span_to_file_range(identifier.span)).flatten(),
            self.namespace_root(identifier.name, role),
        );
    }

    /// Record a program or imported-namespace occurrence with an explicit target.
    fn add_namespace_occurrence_with_declaration(
        &mut self,
        identifier: &Identifier,
        role: OccurrenceRole,
        declaration: Option<FileRange>,
        package_root: Option<Arc<PathBuf>>,
    ) {
        if let Some(range) = span_to_file_range(identifier.span) {
            self.occurrences.push(SymbolOccurrence {
                range,
                identity: SymbolIdentity::Program { program: identifier.name, package_root, declaration },
                role,
                token_kind: SemanticKind::Namespace,
                readonly: false,
            });
        }
    }

    /// Resolve a namespace identifier to a package root when the compiler made it stable.
    fn namespace_root(&self, symbol: Symbol, role: OccurrenceRole) -> Option<Arc<PathBuf>> {
        if role == OccurrenceRole::Declaration {
            return self.current_program_root.clone();
        }
        self.program_roots
            .root_for_symbol(symbol)
            .or_else(|| (symbol == self.current_program).then(|| self.current_program_root.clone()).flatten())
    }

    /// Record a globally addressable declaration or reference.
    fn add_global_occurrence(
        &mut self,
        range: FileRange,
        location: Location,
        declaration: Option<FileRange>,
        role: OccurrenceRole,
        token_kind: SemanticKind,
        readonly: bool,
    ) {
        self.occurrences.push(SymbolOccurrence {
            range,
            identity: SymbolIdentity::GlobalItem { location, declaration },
            role,
            token_kind,
            readonly,
        });
    }

    /// Record a member declaration or reference for an explicitly chosen owner.
    fn add_member_occurrence(
        &mut self,
        owner: Option<Location>,
        identifier: &Identifier,
        role: OccurrenceRole,
        readonly: bool,
        declaration: Option<FileRange>,
    ) {
        if let Some(range) = span_to_file_range(identifier.span) {
            self.occurrences.push(SymbolOccurrence {
                range,
                identity: SymbolIdentity::Member { owner, name: identifier.name, declaration },
                role,
                token_kind: SemanticKind::Property,
                readonly,
            });
        }
    }

    /// Record a member declaration or reference for the current owner stack.
    fn add_current_member_occurrence(
        &mut self,
        identifier: &Identifier,
        role: OccurrenceRole,
        readonly: bool,
        declaration: Option<FileRange>,
    ) {
        self.add_member_occurrence(self.current_owner(), identifier, role, readonly, declaration);
    }

    /// Record an unresolved occurrence that is still useful for highlighting.
    fn add_unknown_occurrence(
        &mut self,
        identifier: &Identifier,
        role: OccurrenceRole,
        token_kind: SemanticKind,
        readonly: bool,
    ) {
        if let Some(range) = span_to_file_range(identifier.span) {
            self.occurrences.push(SymbolOccurrence {
                range,
                identity: SymbolIdentity::Unknown,
                role,
                token_kind,
                readonly,
            });
        }
    }

    /// Recover the owning composite location for a member-capable type.
    fn member_owner_from_type(&self, type_: &Type) -> Option<Location> {
        match type_ {
            Type::Composite(composite) => composite.path.try_global_location().cloned(),
            Type::Optional(optional) => self.member_owner_from_type(optional.inner.as_ref()),
            _ => None,
        }
    }

    /// Recover the owning composite location for `receiver.member`.
    fn member_owner_from_expression(&self, expression: &leo_ast::Expression) -> Option<Location> {
        self.type_table.get(&expression.id()).and_then(|type_| self.member_owner_from_type(&type_))
    }

    /// Recover the owning composite location for `Type { field: ... }`.
    fn member_owner_from_composite_init(&self, input: &CompositeExpression) -> Option<Location> {
        input
            .path
            .try_global_location()
            .cloned()
            .or_else(|| self.type_table.get(&input.id).and_then(|type_| self.member_owner_from_type(&type_)))
    }

    /// Record a source import and point it at the imported program when available.
    fn add_import_occurrence(&mut self, import: &ProgramId, program: &Program) {
        let declaration = program
            .stubs
            .get(&import.as_symbol())
            .or_else(|| program.stubs.get(&import.name.name))
            .and_then(stub_program_declaration_range);
        let package_root = self
            .program_roots
            .root_for_symbol(import.as_symbol())
            .or_else(|| self.program_roots.root_for_symbol(import.name.name))
            .or_else(|| {
                program
                    .stubs
                    .get(&import.as_symbol())
                    .or_else(|| program.stubs.get(&import.name.name))
                    .and_then(|stub| self.program_roots.root_for_stub(stub))
            });
        let import_name = import.name.name.to_string();
        let range = identifier_range_from_span(import.span(), import_name.as_str())
            .or_else(|| span_to_file_range(import.name.span));
        if let Some(range) = range {
            self.occurrences.push(SymbolOccurrence {
                range,
                identity: SymbolIdentity::Program { program: import.name.name, package_root, declaration },
                role: OccurrenceRole::Reference,
                token_kind: SemanticKind::Namespace,
                readonly: false,
            });
        }
    }

    /// Resolve a global path through the compiler symbol tables and emit the
    /// most accurate semantic kind available for its target declaration.
    fn visit_global_path(&mut self, path: &Path) {
        if let Some(user_program) = path.user_program() {
            let package_root = self
                .program_roots
                .root_for_symbol(user_program.as_symbol())
                .or_else(|| self.program_roots.root_for_symbol(user_program.name.name));
            self.add_namespace_occurrence_with_declaration(
                &user_program.name,
                OccurrenceRole::Reference,
                None,
                package_root,
            );
        } else if let Some(first) = path.qualifier().first()
            && (self.symbol_table.is_library(first.name) || first.name == self.current_program)
        {
            let package_root = self.namespace_root(first.name, OccurrenceRole::Reference).or_else(|| {
                self.symbol_table.is_library(first.name).then(|| Arc::clone(&self.program_roots.library_sentinel))
            });
            self.add_namespace_occurrence_with_declaration(first, OccurrenceRole::Reference, None, package_root);
        }

        let location =
            path.try_global_location().cloned().unwrap_or_else(|| self.current_item_location(path.identifier().name));
        let Some(range) = span_to_file_range(path.identifier().span) else {
            return;
        };

        // Resolve the location through the compiler tables in declaration-kind
        // order so the emitted token kind reflects the declaration we actually
        // found, not just the surface syntax of the path.
        if let Some(function) = self.symbol_table.lookup_function(self.current_program, &location) {
            self.add_global_occurrence(
                range,
                location.clone(),
                span_to_file_range(function.function.identifier.span),
                OccurrenceRole::Reference,
                SemanticKind::Function,
                false,
            );
            return;
        }

        if let Some(interface) = self.symbol_table.lookup_interface(self.current_program, &location) {
            self.add_global_occurrence(
                range,
                location.clone(),
                span_to_file_range(interface.identifier.span),
                OccurrenceRole::Reference,
                SemanticKind::Interface,
                false,
            );
            return;
        }

        if let Some(composite) = self
            .symbol_table
            .lookup_struct(self.current_program, &location)
            .or_else(|| self.symbol_table.lookup_record(self.current_program, &location))
        {
            self.add_global_occurrence(
                range,
                location.clone(),
                span_to_file_range(composite.identifier.span),
                OccurrenceRole::Reference,
                SemanticKind::Type,
                false,
            );
            return;
        }

        if let Some(variable) = self.symbol_table.lookup_global(self.current_program, &location) {
            let (token_kind, readonly) = variable_symbol_semantics(variable.declaration);
            self.add_global_occurrence(
                range,
                location,
                span_to_file_range(variable.span),
                OccurrenceRole::Reference,
                token_kind,
                readonly,
            );
        }
    }
}

impl<'a> AstVisitor for CompilerSemanticCollector<'a> {
    /// Compiler semantic collection does not need caller-supplied visitor state.
    type AdditionalInput = ();
    /// Visitor methods record occurrences through side effects.
    type Output = ();

    /// Visit a function call and classify the callee as a function reference.
    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Regular calls highlight the callee path as a function and then visit
        // all compile-time and runtime arguments as expressions.
        self.visit_path(&input.function, &());
        input.const_arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
        input.arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
    }

    /// Visit a composite literal, recording field names as member references.
    fn visit_composite_init(
        &mut self,
        input: &CompositeExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        // Struct literals can mention members both explicitly and via shorthand,
        // so treat every initializer key as a property reference.
        let owner = self.member_owner_from_composite_init(input);
        self.visit_path(&input.path, &());
        input.const_arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
        for CompositeFieldInitializer { identifier, expression, .. } in &input.members {
            self.add_member_occurrence(owner.clone(), identifier, OccurrenceRole::Reference, false, None);
            if let Some(expression) = expression {
                self.visit_expression(expression, &());
            }
        }
    }

    /// Visit a composite type path and any const-generic arguments.
    fn visit_composite_type(&mut self, input: &CompositeType) {
        // Composite types reuse the same path identity logic as value-level
        // references, plus any const-generic arguments they carry.
        self.visit_path(&input.path, &());
        input.const_arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
    }

    /// Visit dynamic operations whose target identity cannot be fully resolved.
    fn visit_dynamic_op(&mut self, input: &DynamicOpExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_type(&input.interface);
        self.visit_expression(&input.target_program, &());
        if let Some(network) = &input.network {
            self.visit_expression(network, &());
        }

        // Dynamic operations do not resolve to a concrete compiler location
        // during frontend analysis, so keep their operation identifiers
        // syntax-shaped while still preserving useful semantic token kinds.
        match &input.kind {
            DynamicOpKind::Call { function, arguments } => {
                self.add_unknown_occurrence(function, OccurrenceRole::Reference, SemanticKind::Function, false);
                arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
            }
            DynamicOpKind::Read { storage } => {
                self.add_unknown_occurrence(storage, OccurrenceRole::Reference, SemanticKind::Property, false);
            }
            DynamicOpKind::Op { member, op, arguments } => {
                self.add_unknown_occurrence(member, OccurrenceRole::Reference, SemanticKind::Property, false);
                self.add_unknown_occurrence(op, OccurrenceRole::Reference, SemanticKind::Function, false);
                arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
            }
        }
    }

    /// Visit `receiver.member` expressions and preserve the receiver-derived owner.
    fn visit_member_access(&mut self, input: &MemberAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        // Member access records the receiver first so nested expressions still
        // contribute their own occurrences before the property reference.
        let owner = self.member_owner_from_expression(&input.inner);
        self.visit_expression(&input.inner, &());
        self.add_member_occurrence(owner, &input.name, OccurrenceRole::Reference, false, None);
    }

    /// Visit a path, preferring lexical bindings before falling back to globals.
    fn visit_path(&mut self, input: &Path, _additional: &Self::AdditionalInput) -> Self::Output {
        if input.is_global() {
            self.visit_global_path(input);
            return;
        }

        // Non-global paths only participate in semantic indexing if they can be
        // matched back to a currently bound lexical declaration.
        let Some(symbol) = input.try_local_symbol() else {
            self.visit_global_path(input);
            return;
        };
        let Some(binding) = self.local_scopes.iter().rev().find_map(|scope| scope.get(&symbol)).cloned() else {
            self.visit_global_path(input);
            return;
        };
        let Some(range) = span_to_file_range(input.identifier().span) else {
            return;
        };
        self.occurrences.push(SymbolOccurrence {
            range,
            identity: SymbolIdentity::Local { declaration: binding.declaration },
            role: OccurrenceRole::Reference,
            token_kind: binding.token_kind,
            readonly: binding.readonly,
        });
    }

    /// Visit a block inside a new lexical scope.
    fn visit_block(&mut self, input: &leo_ast::Block) {
        // Blocks introduce lexical scope for definitions created inside them.
        self.push_scope();
        input.statements.iter().for_each(|statement| self.visit_statement(statement));
        self.pop_scope();
    }

    /// Visit a constant declaration as either global or scoped readonly state.
    fn visit_const(&mut self, input: &ConstDeclaration) {
        // Constants behave like readonly variable declarations for semantic
        // token purposes, even when they appear at top level.
        self.visit_type(&input.type_);
        self.visit_expression(&input.value, &());
        if self.local_scopes.is_empty() {
            // Top-level consts participate in global path resolution, so they
            // must use a global identity instead of the local-binding fallback.
            if let Some(range) = span_to_file_range(input.place.span) {
                self.add_global_occurrence(
                    range.clone(),
                    self.current_item_location(input.place.name),
                    Some(range),
                    OccurrenceRole::Declaration,
                    SemanticKind::Variable,
                    true,
                );
            }
        } else {
            self.bind_local(&input.place, SemanticKind::Variable, true);
        }
    }

    /// Visit a `let` definition and bind every declared local name.
    fn visit_definition(&mut self, input: &DefinitionStatement) {
        if let Some(type_) = &input.type_ {
            self.visit_type(type_);
        }
        self.visit_expression(&input.value, &());
        // Definitions can destructure multiple identifiers, but each bound name
        // still becomes its own local semantic declaration.
        match &input.place {
            DefinitionPlace::Single(identifier) => self.bind_local(identifier, SemanticKind::Variable, false),
            DefinitionPlace::Multiple(identifiers) => {
                identifiers.iter().for_each(|identifier| self.bind_local(identifier, SemanticKind::Variable, false));
            }
        }
    }

    /// Visit a loop while limiting the loop variable to the body scope.
    fn visit_iteration(&mut self, input: &IterationStatement) {
        if let Some(type_) = &input.type_ {
            self.visit_type(type_);
        }
        self.visit_expression(&input.start, &());
        self.visit_expression(&input.stop, &());
        // Loop variables are rebound for the duration of the loop body only.
        self.push_scope();
        self.bind_local(&input.variable, SemanticKind::Variable, true);
        self.visit_block(&input.block);
        self.pop_scope();
    }
}

impl<'a> UnitVisitor for CompilerSemanticCollector<'a> {
    /// Visit an analyzed program, including imported stubs.
    fn visit_program(&mut self, input: &Program) {
        // Visit both owned source and imported stub graphs so semantic
        // identities remain available across dependency boundaries.
        input.imports.values().for_each(|import| self.add_import_occurrence(import, input));
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
        input.modules.values().for_each(|module| self.visit_module(module));
        input.stubs.values().for_each(|stub| self.visit_stub(stub));
    }

    /// Visit dependency stubs with the active program root set to the stub's identity.
    fn visit_stub(&mut self, input: &Stub) {
        let previous_root = self.current_program_root.clone();
        self.stub_depth += 1;
        match input {
            Stub::FromLeo { program, .. } => self.visit_program(program),
            Stub::FromAleo { program, .. } => {
                self.current_program_root = Some(Arc::clone(&self.program_roots.network_sentinel));
                self.visit_aleo_program(program);
            }
            Stub::FromLibrary { library, .. } => {
                self.current_program_root = Some(Arc::clone(&self.program_roots.library_sentinel));
                self.visit_library(library);
            }
        }
        self.stub_depth -= 1;
        self.current_program_root = previous_root;
    }

    /// Visit a library root while preserving the surrounding module context.
    fn visit_library(&mut self, input: &leo_ast::Library) {
        // Libraries reuse the same collector machinery as programs, but their
        // top-level items live directly under the library name.
        let previous_program = self.current_program;
        let previous_module = std::mem::take(&mut self.current_module);

        self.current_program = input.name;
        if self.stub_depth > 0 {
            self.current_program_root = Some(Arc::clone(&self.program_roots.library_sentinel));
        }
        input.consts.iter().for_each(|(_, declaration)| self.visit_const(declaration));
        input.structs.iter().for_each(|(_, composite)| self.visit_composite(composite));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
        input.modules.values().for_each(|module| self.visit_module(module));

        self.current_program = previous_program;
        self.current_module = previous_module;
    }

    /// Visit one program scope and reset module qualifiers for its items.
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Reset the module path at each program scope so top-level locations do
        // not accidentally inherit a nested module qualifier.
        let previous_program = self.current_program;
        let previous_root = self.current_program_root.clone();
        let previous_module = std::mem::take(&mut self.current_module);

        self.current_program = input.program_id.as_symbol();
        self.current_program_root = if self.stub_depth == 0 {
            self.program_roots.current.clone()
        } else {
            self.program_roots
                .root_for_symbol(input.program_id.as_symbol())
                .or_else(|| self.program_roots.root_for_symbol(input.program_id.name.name))
                .or(previous_root.clone())
        };
        self.add_namespace_occurrence(&input.program_id.name, OccurrenceRole::Declaration);
        input.parents.iter().for_each(|(_, parent)| self.visit_type(parent));
        input.consts.iter().for_each(|(_, declaration)| self.visit_const(declaration));
        input.composites.iter().for_each(|(_, composite)| self.visit_composite(composite));
        input.interfaces.iter().for_each(|(_, interface)| self.visit_interface(interface));
        input.mappings.iter().for_each(|(_, mapping)| self.visit_mapping(mapping));
        input.storage_variables.iter().for_each(|(_, storage)| self.visit_storage_variable(storage));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
        if let Some(constructor) = input.constructor.as_ref() {
            self.visit_constructor(constructor);
        }

        self.current_program = previous_program;
        self.current_program_root = previous_root;
        self.current_module = previous_module;
    }

    /// Visit a module with its fully qualified module path active.
    fn visit_module(&mut self, input: &Module) {
        // Modules replace the current module path wholesale because the AST
        // stores the full module path for each module node.
        let previous_program = self.current_program;
        let previous_module = std::mem::replace(&mut self.current_module, input.path.clone());

        self.current_program = input.unit_name;
        input.consts.iter().for_each(|(_, declaration)| self.visit_const(declaration));
        input.composites.iter().for_each(|(_, composite)| self.visit_composite(composite));
        input.interfaces.iter().for_each(|(_, interface)| self.visit_interface(interface));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));

        self.current_program = previous_program;
        self.current_module = previous_module;
    }

    /// Visit a struct or record and anchor member declarations to the composite.
    fn visit_composite(&mut self, input: &Composite) {
        // Member identities are anchored to the enclosing composite location, so
        // push that owner before walking members.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            let location = self.current_item_location(input.identifier.name);
            self.add_global_occurrence(
                range.clone(),
                location.clone(),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Type,
                false,
            );
            self.owner_stack.push(Some(location));
        } else {
            self.owner_stack.push(None);
        }

        self.push_scope();
        input.const_parameters.iter().for_each(|parameter| {
            self.visit_type(&parameter.type_);
            self.bind_local(&parameter.identifier, SemanticKind::Parameter, true);
        });
        input.members.iter().for_each(|member| {
            self.add_current_member_occurrence(
                &member.identifier,
                OccurrenceRole::Declaration,
                member.mode == leo_ast::Mode::Constant,
                span_to_file_range(member.identifier.span),
            );
            self.visit_type(&member.type_);
        });
        self.pop_scope();
        self.owner_stack.pop();
    }

    /// Visit a concrete mapping declaration.
    fn visit_mapping(&mut self, input: &Mapping) {
        // Mappings surface to the editor like property-like global declarations.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            self.add_global_occurrence(
                range.clone(),
                self.current_item_location(input.identifier.name),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Property,
                false,
            );
        }
        self.visit_type(&input.key_type);
        self.visit_type(&input.value_type);
    }

    /// Visit a concrete storage declaration.
    fn visit_storage_variable(&mut self, input: &StorageVariable) {
        // Storage declarations are highlighted the same way as other
        // property-shaped global state.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            self.add_global_occurrence(
                range.clone(),
                self.current_item_location(input.identifier.name),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Property,
                false,
            );
        }
        self.visit_type(&input.type_);
    }

    /// Visit an interface mapping prototype under the active owner.
    fn visit_mapping_prototype(&mut self, input: &MappingPrototype) {
        // Interface mapping prototypes mirror concrete mapping declarations for
        // semantic-token purposes, but without executable bodies.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            self.add_global_occurrence(
                range.clone(),
                self.owned_item_location(input.identifier.name),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Property,
                false,
            );
        }
        self.visit_type(&input.key_type);
        self.visit_type(&input.value_type);
    }

    /// Visit an interface storage prototype under the active owner.
    fn visit_storage_variable_prototype(&mut self, input: &StorageVariablePrototype) {
        // Interface storage prototypes still contribute property declarations
        // even though no backing storage exists in this source file.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            self.add_global_occurrence(
                range.clone(),
                self.owned_item_location(input.identifier.name),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Property,
                false,
            );
        }
        self.visit_type(&input.type_);
    }

    /// Visit a concrete function and bind its parameter/body scopes.
    fn visit_function(&mut self, input: &Function) {
        // Function parameters introduce the outermost lexical scope for the
        // function body, before nested blocks add their own scopes.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            self.add_global_occurrence(
                range.clone(),
                self.current_item_location(input.identifier.name),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Function,
                false,
            );
        }

        self.push_scope();
        input.const_parameters.iter().for_each(|parameter| {
            self.visit_type(&parameter.type_);
            self.bind_local(&parameter.identifier, SemanticKind::Parameter, true);
        });
        input.input.iter().for_each(|parameter| {
            self.visit_type(&parameter.type_);
            self.bind_local(&parameter.identifier, SemanticKind::Parameter, false);
        });
        input.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&input.output_type);
        self.visit_block(&input.block);
        self.pop_scope();
    }

    /// Visit an interface and use it as the owner for all prototypes.
    fn visit_interface(&mut self, input: &Interface) {
        // Interface members share the interface as their semantic owner even
        // though they are prototype declarations rather than full definitions.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            let location = self.current_item_location(input.identifier.name);
            self.add_global_occurrence(
                range.clone(),
                location.clone(),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Interface,
                false,
            );
            self.owner_stack.push(Some(location));
        } else {
            self.owner_stack.push(None);
        }

        input.parents.iter().for_each(|(_, parent)| self.visit_type(parent));
        input.functions.iter().for_each(|(_, function)| self.visit_function_prototype(function));
        input.records.iter().for_each(|(_, record)| self.visit_record_prototype(record));
        input.mappings.iter().for_each(|mapping| self.visit_mapping_prototype(mapping));
        input.storages.iter().for_each(|storage| self.visit_storage_variable_prototype(storage));
        self.owner_stack.pop();
    }

    /// Visit an interface function prototype without a body.
    fn visit_function_prototype(&mut self, input: &FunctionPrototype) {
        // Prototype parameters still participate in local binding/highlighting
        // even though there is no executable body to visit.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            let location = self.owned_item_location(input.identifier.name);
            self.add_global_occurrence(
                range.clone(),
                location,
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Function,
                false,
            );
        }

        self.push_scope();
        input.const_parameters.iter().for_each(|parameter| {
            self.visit_type(&parameter.type_);
            self.bind_local(&parameter.identifier, SemanticKind::Parameter, true);
        });
        input.input.iter().for_each(|parameter| {
            self.visit_type(&parameter.type_);
            self.bind_local(&parameter.identifier, SemanticKind::Parameter, false);
        });
        input.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&input.output_type);
        self.pop_scope();
    }

    /// Visit an interface record prototype and its owned members.
    fn visit_record_prototype(&mut self, input: &RecordPrototype) {
        // Record members inherit the record prototype as their semantic owner.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            let location = self.owned_item_location(input.identifier.name);
            self.add_global_occurrence(
                range.clone(),
                location.clone(),
                Some(range),
                OccurrenceRole::Declaration,
                SemanticKind::Type,
                false,
            );
            self.owner_stack.push(Some(location));
        } else {
            self.owner_stack.push(None);
        }

        input.members.iter().for_each(|member| {
            self.add_current_member_occurrence(
                &member.identifier,
                OccurrenceRole::Declaration,
                member.mode == leo_ast::Mode::Constant,
                span_to_file_range(member.identifier.span),
            );
            self.visit_type(&member.type_);
        });
        self.owner_stack.pop();
    }
}

/// Map compiler variable categories onto LSP-facing token kind and modifiers.
fn variable_symbol_semantics(declaration: VariableType) -> (SemanticKind, bool) {
    match declaration {
        VariableType::Const => (SemanticKind::Variable, true),
        VariableType::ConstParameter => (SemanticKind::Parameter, true),
        VariableType::Input(leo_ast::Mode::Constant) => (SemanticKind::Parameter, true),
        VariableType::Input(_) => (SemanticKind::Parameter, false),
        VariableType::Mut => (SemanticKind::Variable, false),
        VariableType::Storage => (SemanticKind::Property, false),
    }
}

/// Return the source declaration range for an imported dependency stub.
fn stub_program_declaration_range(stub: &Stub) -> Option<FileRange> {
    match stub {
        Stub::FromLeo { program, .. } => program.program_scopes.values().find_map(|scope| {
            span_to_file_range(scope.program_id.name.span)
                .or_else(|| program_name_range_from_span(scope.span, scope.program_id.name.name.to_string().as_str()))
        }),
        Stub::FromAleo { program, .. } => span_to_file_range(program.stub_id.name.span)
            .or_else(|| program_name_range_from_span(program.span, program.stub_id.name.name.to_string().as_str())),
        Stub::FromLibrary { .. } => None,
    }
}

/// Recover a program-name token from a larger program/stub span.
fn program_name_range_from_span(span: leo_span::Span, name: &str) -> Option<FileRange> {
    if span.is_dummy() {
        return None;
    }

    with_session_globals(|session| {
        let source_file = session.source_map.find_source_file(span.lo)?;
        if span.hi > source_file.absolute_end {
            return None;
        }
        let FileName::Real(path) = &source_file.name else {
            return None;
        };

        let span_text = source_file.contents_of_span(span);
        let name_offset = find_program_name_in_text(span_text, name)?;
        let start = source_file.relative_offset(span.lo).checked_add(u32::try_from(name_offset).ok()?)?;
        let end = start.checked_add(u32::try_from(name.len()).ok()?)?;
        FileRange::new(Arc::new(path.clone()), start, end)
    })
}

/// Recover an identifier token from a larger source span.
fn identifier_range_from_span(span: leo_span::Span, name: &str) -> Option<FileRange> {
    if span.is_dummy() {
        return None;
    }

    with_session_globals(|session| {
        let source_file = session.source_map.find_source_file(span.lo)?;
        if span.hi > source_file.absolute_end {
            return None;
        }
        let FileName::Real(path) = &source_file.name else {
            return None;
        };

        let span_text = source_file.contents_of_span(span);
        let name_offset = find_identifier_in_text(span_text, name)?;
        let start = source_file.relative_offset(span.lo).checked_add(u32::try_from(name_offset).ok()?)?;
        let end = start.checked_add(u32::try_from(name.len()).ok()?)?;
        FileRange::new(Arc::new(path.clone()), start, end)
    })
}

/// Find the identifier token in a `program name.aleo` source slice.
fn find_program_name_in_text(text: &str, name: &str) -> Option<usize> {
    let program_end = text.find("program")?.checked_add("program".len())?;
    find_identifier_in_text(&text[program_end..], name).map(|relative| program_end + relative)
}

/// Find an identifier token by spelling while respecting identifier boundaries.
fn find_identifier_in_text(text: &str, name: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    text.match_indices(name).find_map(|(start, _)| {
        let end = start + name.len();
        let left_boundary = start == 0 || !is_identifier_byte(bytes[start - 1]);
        let right_boundary = end == bytes.len() || !is_identifier_byte(bytes[end]);
        (left_boundary && right_boundary).then_some(start)
    })
}

/// Return whether a byte can appear inside a Leo identifier.
fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Convert a compiler span into a real file-relative range for semantic indexing.
fn span_to_file_range(span: leo_span::Span) -> Option<FileRange> {
    if span.is_dummy() {
        return None;
    }

    with_session_globals(|session| {
        let source_file = session.source_map.find_source_file(span.lo)?;
        let FileName::Real(path) = &source_file.name else {
            // Only real package files can round-trip back to an editor buffer.
            return None;
        };
        FileRange::new(
            Arc::new(path.clone()),
            source_file.relative_offset(span.lo),
            source_file.relative_offset(span.hi),
        )
    })
}

/// Fail fast when the document generation that triggered this work is stale.
fn check_snapshot_current(snapshot: &DocumentSnapshot) -> leo_errors::Result<()> {
    if snapshot_is_cancelled(snapshot) { Err(anyhow!("semantic analysis cancelled").into()) } else { Ok(()) }
}

/// Return whether a newer committed generation has superseded this snapshot.
fn snapshot_is_cancelled(snapshot: &DocumentSnapshot) -> bool {
    snapshot.cancel_token.load(Ordering::SeqCst) != snapshot.generation
}

/// Hash the content or directory listing of all stub-producing inputs into one cache revision key.
#[cfg(test)]
fn watched_paths_revision(paths: &[PathBuf]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for path in paths {
        watched_path_revision(path).hash(&mut hasher);
    }
    hasher.finish()
}

/// Hash watched inputs while reusing unchanged per-path revisions across checks.
fn watched_paths_revision_cached(
    paths: &[PathBuf],
    cached_revisions: &mut HashMap<PathBuf, CachedWatchedPathRevision>,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    for path in paths {
        watched_path_revision_cached(path, cached_revisions).hash(&mut hasher);
    }
    hasher.finish()
}

/// Reuse the last full revision for a watched path while its metadata stamp is unchanged.
fn watched_path_revision_cached(
    path: &StdPath,
    cached_revisions: &mut HashMap<PathBuf, CachedWatchedPathRevision>,
) -> u64 {
    let stamp = watched_path_stamp(path);
    if let Some(stamp) = stamp.as_ref()
        && let Some(cached) = cached_revisions.get(path)
        && cached.stamp.as_ref() == Some(stamp)
    {
        return cached.revision;
    }

    let revision = watched_path_revision(path);
    cached_revisions.insert(path.to_path_buf(), CachedWatchedPathRevision { stamp, revision });
    revision
}

/// Hash one watched path using the bytes that actually affect stub loading.
fn watched_path_revision(path: &StdPath) -> u64 {
    let mut hasher = DefaultHasher::new();
    hash_watched_path(path, &mut hasher);
    hasher.finish()
}

/// Hash one watched path using the bytes that actually affect stub loading.
fn hash_watched_path(path: &StdPath, hasher: &mut DefaultHasher) {
    path.hash(hasher);
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => {
            true.hash(hasher);
            hash_leo_directory_listing(path, hasher);
        }
        Ok(_) => {
            true.hash(hasher);
            match std::fs::read(path) {
                Ok(contents) => contents.hash(hasher),
                Err(_) => false.hash(hasher),
            }
        }
        Err(_) => {
            false.hash(hasher);
        }
    }
}

/// Build the cheap metadata stamp used to skip rehashing unchanged watched inputs.
fn watched_path_stamp(path: &StdPath) -> Option<WatchedPathStamp> {
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => {
            Some(WatchedPathStamp::Directory { modified_nanos: metadata_modified_nanos(&metadata)? })
        }
        Ok(metadata) => {
            Some(WatchedPathStamp::File { len: metadata.len(), modified_nanos: metadata_modified_nanos(&metadata)? })
        }
        Err(_) => Some(WatchedPathStamp::Missing),
    }
}

/// Convert filesystem metadata into a comparable nanosecond timestamp.
fn metadata_modified_nanos(metadata: &Metadata) -> Option<u128> {
    metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok().map(|duration| duration.as_nanos())
}

/// Hash the recursive `.leo` file listing for a watched source directory.
fn hash_leo_directory_listing(dir: &StdPath, hasher: &mut DefaultHasher) {
    let mut files = Vec::new();
    if !collect_leo_files(dir, &mut files) {
        false.hash(hasher);
        return;
    }
    files.sort();
    files.hash(hasher);
}

/// Collect `.leo` files recursively and report whether the walk succeeded.
fn collect_leo_files(dir: &StdPath, files: &mut Vec<PathBuf>) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if !collect_leo_files(&path, files) {
                return false;
            }
        } else if path.extension().is_some_and(|extension| extension == "leo") {
            files.push(path);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_PACKAGE_ANALYSIS_CACHE_ENTRIES,
        PackageAnalysisCache,
        PackageAnalysisCacheEntry,
        analyze_snapshot,
    };
    use crate::{
        document_store::{DocumentSnapshot, DocumentStore},
        project_model::ProjectModel,
        semantics::{
            FileRange,
            OccurrenceRole,
            SemanticKind,
            SemanticSource,
            SourceFingerprint,
            SymbolIdentity,
            SymbolOccurrence,
        },
    };
    use leo_ast::NetworkName;
    use leo_compiler::load_import_stubs_for_package;
    use lsp_types::Uri;
    use serde_json::json;
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::Arc,
        thread,
        time::Duration,
    };
    use tempfile::tempdir;

    /// Build a test `file:` URI from a native path.
    fn file_uri(path: &Path) -> Uri {
        #[cfg(target_os = "windows")]
        let path = {
            let display = path.display().to_string();
            let display = display.strip_prefix(r"\\?\").unwrap_or(display.as_str());
            format!("/{}", display).replace('\\', "/")
        };

        #[cfg(not(target_os = "windows"))]
        let path = path.display().to_string();

        format!("file://{path}").parse().expect("file uri")
    }

    /// Build the minimal manifest dependency JSON for a local package.
    fn local_dependency_json(path: &Path) -> String {
        json!([{ "name": "helper", "location": "local", "path": path }]).to_string()
    }

    /// Write a package manifest and source directory for dependency-cache tests.
    fn write_manifest(package_root: &Path, program: &str, dependencies: &str) {
        fs::create_dir_all(package_root.join("src")).expect("create source dir");
        fs::write(
            package_root.join("program.json"),
            format!(
                r#"{{
  "program": "{program}",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "4.0.0",
  "dependencies": {dependencies}
}}
"#,
            ),
        )
        .expect("write manifest");
    }

    /// Build a committed document snapshot for compiler-bridge tests.
    fn snapshot_for(path: &Path, text: &str) -> DocumentSnapshot {
        let uri = file_uri(path);
        let mut projects = ProjectModel::default();
        let (file_path, project) = projects.resolve_document_context(&uri);
        let mut documents = DocumentStore::default();
        documents.commit_open(documents.prepare_open(uri, "leo".to_owned(), 1, text.to_owned(), file_path, project))
    }

    /// Verifies network dependencies are ignored by local stub loading.
    #[test]
    fn import_stub_loader_skips_network_dependencies() {
        let tempdir = tempdir().expect("tempdir");
        let helper_root = tempdir.path().join("helper");
        write_manifest(&helper_root, "helper", "null");
        fs::write(helper_root.join("src").join("lib.leo"), "const VALUE: u32 = 1u32;\n").expect("write helper source");
        let helper_root = helper_root.canonicalize().expect("canonical helper root");

        let root = tempdir.path().join("root");
        write_manifest(
            &root,
            "root.aleo",
            &json!([
                { "name": "helper", "location": "local", "path": helper_root },
                { "name": "credits.aleo", "location": "network" }
            ])
            .to_string(),
        );
        fs::write(root.join("src").join("main.leo"), "program root.aleo {}\n").expect("write root source");

        let loaded = load_import_stubs_for_package(&root, NetworkName::TestnetV0).expect("load import stubs");
        assert_eq!(loaded.stubs.len(), 1);
    }

    /// Verifies unchanged watched inputs reuse cached import stubs.
    #[test]
    fn package_cache_reuses_import_stubs_when_watched_inputs_are_unchanged() {
        let tempdir = tempdir().expect("tempdir");
        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", "null");
        let main_path = root.join("src").join("main.leo");
        fs::write(&main_path, "program root.aleo {}\n").expect("write root source");

        let mut projects = ProjectModel::default();
        let (_, project) = projects.resolve_document_context(&file_uri(&main_path));
        let project = project.expect("project context");

        let mut cache = PackageAnalysisCache::default();
        let file_source = super::RecordingFileSource::new(Arc::from([]));
        let first = cache.import_stubs_for(project.as_ref(), &file_source).expect("initial cache load");
        let second = cache.import_stubs_for(project.as_ref(), &file_source).expect("cached load");

        assert!(Arc::ptr_eq(&first.import_stubs, &second.import_stubs));
    }

    /// Verifies the worker-local package cache evicts old stub entries.
    #[test]
    fn package_cache_caps_open_stub_entries() {
        let mut cache = PackageAnalysisCache::default();

        for index in 0..(MAX_PACKAGE_ANALYSIS_CACHE_ENTRIES + 3) {
            let package_root = Path::new("/tmp").join(format!("pkg-{index}"));
            cache.entries.insert(package_root.clone(), PackageAnalysisCacheEntry {
                import_stubs: Arc::new(Default::default()),
                fingerprints: Arc::from(Vec::<(PathBuf, SourceFingerprint)>::new()),
                dependency_roots: Arc::new(std::collections::HashMap::new()),
                watch_paths: Arc::from([]),
                watch_state: Default::default(),
                revision: index as u64,
            });
            cache.touch_entry(&package_root);
            cache.evict_old_entries(&package_root);
        }

        assert_eq!(cache.entries.len(), MAX_PACKAGE_ANALYSIS_CACHE_ENTRIES);
        assert!(cache.entries.contains_key(Path::new("/tmp").join("pkg-10").as_path()));
    }

    /// Verifies dependency source rewrites invalidate cached stubs.
    #[test]
    fn package_cache_invalidates_when_dependency_sources_change() {
        let tempdir = tempdir().expect("tempdir");
        let helper_root = tempdir.path().join("helper");
        write_manifest(&helper_root, "helper", "null");
        fs::write(helper_root.join("src").join("lib.leo"), "const VALUE: u32 = 1u32;\n").expect("write helper source");
        let helper_root = helper_root.canonicalize().expect("canonical helper root");
        let helper_source = helper_root.join("src").join("lib.leo");

        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", &local_dependency_json(&helper_root));
        let main_path = root.join("src").join("main.leo");
        fs::write(&main_path, "program root.aleo {}\n").expect("write root source");

        let mut projects = ProjectModel::default();
        let (_, project) = projects.resolve_document_context(&file_uri(&main_path));
        let project = project.expect("project context");

        let mut cache = PackageAnalysisCache::default();
        let file_source = super::RecordingFileSource::new(Arc::from([]));
        let first = cache.import_stubs_for(project.as_ref(), &file_source).expect("initial cache load");

        thread::sleep(Duration::from_millis(20));
        fs::write(&helper_source, "const VALUE: u32 = 2u32;\nconst EXTRA: u32 = VALUE + 1u32;\n")
            .expect("update helper source");

        let second = cache.import_stubs_for(project.as_ref(), &file_source).expect("reloaded cache entry");
        assert!(!Arc::ptr_eq(&first.import_stubs, &second.import_stubs));
    }

    /// Verifies new nested dependency modules invalidate cached stubs.
    #[test]
    fn package_cache_invalidates_when_nested_dependency_module_is_added() {
        let tempdir = tempdir().expect("tempdir");
        let helper_root = tempdir.path().join("helper");
        write_manifest(&helper_root, "helper", "null");
        let nested_dir = helper_root.join("src").join("nested");
        fs::create_dir_all(&nested_dir).expect("create nested module dir");
        fs::write(helper_root.join("src").join("lib.leo"), "const VALUE: u32 = 1u32;\n").expect("write helper source");
        let helper_root = helper_root.canonicalize().expect("canonical helper root");
        let nested_module = nested_dir.join("extra.leo");

        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", &local_dependency_json(&helper_root));
        let main_path = root.join("src").join("main.leo");
        fs::write(&main_path, "program root.aleo {}\n").expect("write root source");

        let mut projects = ProjectModel::default();
        let (_, project) = projects.resolve_document_context(&file_uri(&main_path));
        let project = project.expect("project context");

        let mut cache = PackageAnalysisCache::default();
        let file_source = super::RecordingFileSource::new(Arc::from([]));
        let first = cache.import_stubs_for(project.as_ref(), &file_source).expect("initial cache load");

        thread::sleep(Duration::from_millis(20));
        fs::write(nested_module, "const EXTRA: u32 = 2u32;\n").expect("write nested helper module");

        let second = cache.import_stubs_for(project.as_ref(), &file_source).expect("reloaded cache entry");
        assert!(!Arc::ptr_eq(&first.import_stubs, &second.import_stubs));
    }

    /// Verifies same-size rewrites still change watched-path revisions.
    #[test]
    fn watched_paths_revision_changes_on_same_size_rewrite() {
        let tempdir = tempdir().expect("tempdir");
        let file_path = tempdir.path().join("main.leo");
        fs::write(&file_path, "aaaaaaaa").expect("write file");
        let first = super::watched_paths_revision(std::slice::from_ref(&file_path));

        fs::write(&file_path, "bbbbbbbb").expect("rewrite file");
        let second = super::watched_paths_revision(std::slice::from_ref(&file_path));

        assert_ne!(first, second);
    }

    /// Verifies directory listing changes are part of watched-path revisions.
    #[test]
    fn watched_paths_revision_changes_when_directory_listing_changes() {
        let tempdir = tempdir().expect("tempdir");
        let source_dir = tempdir.path().join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("main.leo"), String::new()).expect("write main");

        let first = super::watched_paths_revision(std::slice::from_ref(&source_dir));

        fs::write(source_dir.join("extra.leo"), String::new()).expect("write extra");
        let second = super::watched_paths_revision(std::slice::from_ref(&source_dir));

        assert_ne!(first, second);
    }

    /// Verifies dependency stubs retain declarations and keyed occurrences but drop unkeyed references.
    #[test]
    fn retain_in_scope_occurrences_drops_unkeyed_stub_references() {
        let local = Arc::new(PathBuf::from("/tmp/root/src/main.leo"));
        let stub = Arc::new(PathBuf::from("/tmp/helper/src/main.leo"));
        let declaration = FileRange::new(Arc::clone(&local), 1, 5).expect("declaration");
        let mut occurrences = vec![
            test_occurrence(&local, 1, 5, SymbolIdentity::Unknown),
            test_occurrence(&stub, 1, 5, SymbolIdentity::Unknown),
            test_occurrence(&stub, 8, 12, SymbolIdentity::Member {
                owner: None,
                name: leo_span::Symbol::new(1),
                declaration: None,
            }),
            test_occurrence(&stub, 15, 19, SymbolIdentity::Local { declaration }),
        ];
        occurrences[1].role = OccurrenceRole::Declaration;
        let source_files =
            super::PackageSourceFiles { source_directory: None, paths: vec![Arc::clone(&local)].into_boxed_slice() };

        super::retain_in_scope_occurrences(&mut occurrences, &source_files);

        assert_eq!(occurrences.len(), 3);
        assert_eq!(occurrences[0].range.path.as_ref(), local.as_ref());
        assert_eq!(occurrences[1].range.path.as_ref(), stub.as_ref());
        assert_eq!(occurrences[1].role, OccurrenceRole::Declaration);
        assert!(occurrences[2].identity.key().is_some());
        assert!(
            !occurrences
                .iter()
                .any(|occurrence| matches!(occurrence.identity, SymbolIdentity::Member { owner: None, .. }))
        );
    }

    /// Build a symbol occurrence for filter tests.
    fn test_occurrence(path: &Arc<PathBuf>, start: u32, end: u32, identity: SymbolIdentity) -> SymbolOccurrence {
        SymbolOccurrence {
            range: FileRange::new(Arc::clone(path), start, end).expect("range"),
            identity,
            role: OccurrenceRole::Reference,
            token_kind: SemanticKind::Variable,
            readonly: false,
        }
    }

    /// Verifies top-level const references share the declaration identity.
    #[test]
    fn top_level_consts_share_global_identity_with_references() {
        let tempdir = tempdir().expect("tempdir");
        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", "null");
        let source = "program root.aleo {\n    const LIMIT: u32 = 1u32;\n\n    fn main() -> u32 {\n        return LIMIT;\n    }\n}\n";
        fs::write(root.join("src").join("main.leo"), source).expect("write root source");
        let main_path = root.join("src").join("main.leo").canonicalize().expect("canonical main path");

        let snapshot = snapshot_for(&main_path, source);
        let semantic_snapshot = analyze_snapshot(&snapshot, &mut PackageAnalysisCache::default());

        assert_eq!(semantic_snapshot.source, SemanticSource::CompilerEnhanced);

        let occurrences = semantic_snapshot
            .index
            .occurrences
            .iter()
            .filter(|occurrence| {
                semantic_snapshot.index.files[occurrence.range.file as usize].as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "LIMIT"
            })
            .collect::<Vec<_>>();
        assert_eq!(occurrences.len(), 2);
        assert!(occurrences.iter().all(|occurrence| occurrence.key_id().is_some()));
        assert!(occurrences.iter().any(|occurrence| occurrence.role() == OccurrenceRole::Declaration));
        assert!(occurrences.iter().any(|occurrence| occurrence.role() == OccurrenceRole::Reference));
        let key = occurrences[0].key_id();
        assert!(occurrences.iter().all(|occurrence| occurrence.key_id() == key));
    }

    /// Verifies import names carry a dependency-source definition target.
    #[test]
    fn import_names_resolve_to_dependency_program_declarations() {
        let tempdir = tempdir().expect("tempdir");
        let helper_root = tempdir.path().join("helper");
        write_manifest(&helper_root, "helper.aleo", "null");
        let helper_source = "program helper.aleo {\n    fn double(x: u32) -> u32 { return x + x; }\n}\n";
        fs::write(helper_root.join("src").join("main.leo"), helper_source).expect("write helper source");
        let helper_main_path = helper_root.join("src").join("main.leo").canonicalize().expect("canonical helper path");
        let helper_root = helper_root.canonicalize().expect("canonical helper root");

        let root = tempdir.path().join("root");
        let dependencies = json!([{ "name": "helper.aleo", "location": "local", "path": helper_root }]).to_string();
        write_manifest(&root, "root.aleo", dependencies.as_str());
        let source = "import helper.aleo;\n\nprogram root.aleo {\n    fn main() -> u32 { return helper.aleo::double(1u32); }\n}\n";
        fs::write(root.join("src").join("main.leo"), source).expect("write root source");
        let main_path = root.join("src").join("main.leo").canonicalize().expect("canonical main path");

        let snapshot = snapshot_for(&main_path, source);
        let semantic_snapshot = analyze_snapshot(&snapshot, &mut PackageAnalysisCache::default());

        assert_eq!(semantic_snapshot.source, SemanticSource::CompilerEnhanced);
        let occurrence = semantic_snapshot
            .index
            .occurrences
            .iter()
            .find(|occurrence| {
                semantic_snapshot.index.files[occurrence.range.file as usize].as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "helper"
            })
            .expect("helper import occurrence");
        let key = occurrence.key_id().expect("navigation-grade import key");
        assert!(
            semantic_snapshot
                .index
                .definitions_for(key)
                .iter()
                .any(|range| { semantic_snapshot.index.files[range.file as usize].as_ref() == &helper_main_path })
        );
    }

    /// Verifies member references resolve through the composite owner.
    #[test]
    fn member_references_reuse_resolved_composite_owner() {
        let tempdir = tempdir().expect("tempdir");
        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", "null");
        let source = concat!(
            "struct Point { x: u32, }\n\n",
            "program root.aleo {\n",
            "    fn main() {\n",
            "        let point: Point = Point { x: 1u32 };\n",
            "        let value = point.x;\n",
            "    }\n",
            "}\n",
        );
        fs::write(root.join("src").join("main.leo"), source).expect("write root source");
        let main_path = root.join("src").join("main.leo").canonicalize().expect("canonical main path");

        let snapshot = snapshot_for(&main_path, source);
        let semantic_snapshot = analyze_snapshot(&snapshot, &mut PackageAnalysisCache::default());

        assert_eq!(semantic_snapshot.source, SemanticSource::CompilerEnhanced);

        let x_occurrences = semantic_snapshot
            .index
            .occurrences
            .iter()
            .filter(|occurrence| {
                semantic_snapshot.index.files[occurrence.range.file as usize].as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "x"
            })
            .collect::<Vec<_>>();
        assert_eq!(x_occurrences.len(), 3);

        assert!(x_occurrences.iter().all(|occurrence| occurrence.key_id().is_some()));
        let key = x_occurrences[0].key_id();
        assert!(x_occurrences.iter().all(|occurrence| occurrence.key_id() == key));
        assert!(x_occurrences.iter().any(|occurrence| occurrence.role() == OccurrenceRole::Declaration));
        assert_eq!(x_occurrences.iter().filter(|occurrence| occurrence.role() == OccurrenceRole::Reference).count(), 2);
    }

    /// Verifies same-named interface prototypes stay owner-qualified.
    #[test]
    fn interface_prototypes_with_same_name_use_owner_qualified_identities() {
        let tempdir = tempdir().expect("tempdir");
        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", "null");
        let source = concat!(
            "interface First {\n",
            "    fn shared() -> u32;\n",
            "}\n\n",
            "interface Second {\n",
            "    fn shared() -> u32;\n",
            "}\n\n",
            "program root.aleo {\n",
            "    fn main() {}\n",
            "}\n",
        );
        fs::write(root.join("src").join("main.leo"), source).expect("write root source");
        let main_path = root.join("src").join("main.leo").canonicalize().expect("canonical main path");

        let snapshot = snapshot_for(&main_path, source);
        let semantic_snapshot = analyze_snapshot(&snapshot, &mut PackageAnalysisCache::default());

        assert_eq!(semantic_snapshot.source, SemanticSource::CompilerEnhanced);

        let shared_occurrences = semantic_snapshot
            .index
            .occurrences
            .iter()
            .filter(|occurrence| {
                semantic_snapshot.index.files[occurrence.range.file as usize].as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "shared"
            })
            .collect::<Vec<_>>();
        assert_eq!(shared_occurrences.len(), 2);
        assert!(shared_occurrences.iter().all(|occurrence| occurrence.role() == OccurrenceRole::Declaration));
        assert!(shared_occurrences.iter().all(|occurrence| occurrence.key_id().is_some()));
        assert_ne!(shared_occurrences[0].key_id(), shared_occurrences[1].key_id());
    }
}

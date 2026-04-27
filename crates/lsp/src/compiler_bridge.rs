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

//! Compiler-backed semantic analysis for `leo-lsp`.
//!
//! The worker always produces a syntax-derived token stream so highlighting can
//! stay responsive for malformed or unmanaged files. When the snapshot belongs
//! to a resolvable Leo package, this module reruns the compiler frontend against
//! the current in-memory text and upgrades those syntax tokens with stable
//! symbol identities and more accurate token kinds.

use crate::{
    document_store::DocumentSnapshot,
    features::semantic_tokens::encode_tokens,
    project_model::ProjectContext,
    semantics::{
        FileRange,
        OccurrenceRole,
        SemanticIndex,
        SemanticKind,
        SemanticSnapshot,
        SemanticSource,
        SemanticTokenOccurrence,
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
    ProgramScope,
    RecordPrototype,
    StorageVariable,
    StorageVariablePrototype,
    Type,
    UnitVisitor,
};
use leo_compiler::{Compiler, FrontendAnalysis, load_import_stubs_for_package};
use leo_errors::Handler;
use leo_passes::{SymbolTable, TypeTable, VariableType};
use leo_span::{
    Symbol,
    create_session_if_not_set_then,
    file_source::{DiskFileSource, OverlayFileSource},
    source_map::FileName,
    with_session_globals,
};
use std::{
    collections::HashMap,
    fs::Metadata,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path as StdPath, PathBuf},
    rc::Rc,
    sync::{Arc, atomic::Ordering},
    time::UNIX_EPOCH,
};

/// Worker-local cache of package dependency stubs.
#[derive(Debug, Default)]
pub struct PackageAnalysisCache {
    entries: HashMap<PathBuf, PackageAnalysisCacheEntry>,
}

/// One cached package entry, including both the imported stubs and the
/// filesystem state used to decide whether those stubs are still current.
#[derive(Debug, Clone)]
struct PackageAnalysisCacheEntry {
    /// Package-wide import stubs reused across snapshots from the same root.
    import_stubs: Arc<IndexMap<Symbol, leo_ast::Stub>>,
    /// Filesystem inputs whose metadata changes invalidate `import_stubs`.
    watch_paths: Arc<[PathBuf]>,
    /// Per-path revision memo reused to avoid rehashing unchanged watched inputs.
    watch_state: HashMap<PathBuf, CachedWatchedPathRevision>,
    /// Hash of the last observed metadata for `watch_paths`.
    revision: u64,
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

/// Build the latest semantic snapshot for one committed document generation.
///
/// Syntax analysis always runs first so the server can return a best-effort
/// token stream even when package discovery, dependency loading, or compiler
/// analysis fail. Compiler-backed occurrences then replace matching syntax
/// ranges when available so later features can reuse a richer semantic index.
pub fn analyze_snapshot(snapshot: &DocumentSnapshot, package_cache: &mut PackageAnalysisCache) -> SemanticSnapshot {
    let syntax = syntax_semantics::collect(snapshot);
    if snapshot_is_cancelled(snapshot) {
        return semantic_snapshot(snapshot, syntax.occurrences, syntax.tokens, SemanticSource::SyntaxOnly);
    }

    let compiler_occurrences = compiler_occurrences(snapshot, package_cache);
    if snapshot_is_cancelled(snapshot) {
        return semantic_snapshot(snapshot, syntax.occurrences, syntax.tokens, SemanticSource::SyntaxOnly);
    }

    let (occurrences, source) = match compiler_occurrences {
        Some(compiler_occurrences) => {
            (merge_occurrences(syntax.occurrences, compiler_occurrences), SemanticSource::CompilerEnhanced)
        }
        None => (syntax.occurrences, SemanticSource::SyntaxOnly),
    };

    semantic_snapshot(snapshot, occurrences, syntax.tokens, source)
}

/// Encode one semantic occurrence set into the cached snapshot returned to the main thread.
fn semantic_snapshot(
    snapshot: &DocumentSnapshot,
    occurrences: Vec<SymbolOccurrence>,
    lexical_tokens: Vec<SemanticTokenOccurrence>,
    source: SemanticSource,
) -> SemanticSnapshot {
    let semantic_tokens = semantic_token_occurrences(&occurrences, lexical_tokens);
    let encoded_tokens = encode_tokens(&semantic_tokens, snapshot.file_path.as_deref(), snapshot.line_index.as_ref());

    SemanticSnapshot { encoded_tokens, index: Arc::new(SemanticIndex { occurrences }), source }
}

/// Merge symbol occurrences with highlighting-only lexical tokens for encoding.
fn semantic_token_occurrences(
    occurrences: &[SymbolOccurrence],
    mut lexical_tokens: Vec<SemanticTokenOccurrence>,
) -> Vec<SemanticTokenOccurrence> {
    let mut tokens = Vec::with_capacity(occurrences.len() + lexical_tokens.len());
    tokens.extend(occurrences.iter().map(SemanticTokenOccurrence::from_symbol));
    tokens.append(&mut lexical_tokens);
    sort_token_occurrences(&mut tokens);

    // Symbol tokens are inserted before lexical tokens, so exact range ties keep
    // the navigation-grade semantic classification.
    tokens.dedup_by(|left, right| left.range == right.range);
    tokens
}

/// Try to refine syntax occurrences with compiler frontend analysis.
///
/// Failures stay local to this helper so callers can fall back to syntax-only
/// highlighting without treating compiler analysis as fatal.
fn compiler_occurrences(
    snapshot: &DocumentSnapshot,
    package_cache: &mut PackageAnalysisCache,
) -> Option<Vec<SymbolOccurrence>> {
    if snapshot_is_cancelled(snapshot) {
        return None;
    }

    let (file_path, project) = compiler_inputs(snapshot)?;
    let overlay_path = file_path.as_ref().clone();
    let overlay_text = snapshot.text.to_string();
    let project = Arc::clone(project);

    let result = create_session_if_not_set_then(|_| {
        // Dependency resolution and parsing intern symbols, so the worker must
        // enter a Leo session before it asks the compiler for frontend state.
        let import_stubs = package_cache.import_stubs_for(project.as_ref()).map_err(|error| {
            tracing::debug!(
                package = project.package_root.display().to_string(),
                error,
                "dependency stub loading failed"
            );
            error
        })?;

        // Run the compiler against the unsaved editor buffer while reading all
        // other package files from disk.
        let overlay_source = OverlayFileSource::new(overlay_path, overlay_text, &DiskFileSource);
        let mut compiler = Compiler::new(
            Some(project.program_name.to_string()),
            false,
            Handler::default(),
            Rc::new(leo_ast::NodeBuilder::default()),
            PathBuf::default(),
            Some(leo_compiler::CompilerOptions::default()),
            import_stubs.as_ref().clone(),
            leo_ast::NetworkName::TestnetV0,
        );

        let frontend = compiler
            .analyze_frontend_from_directory_with_file_source_and_check(
                project.entry_file.as_ref(),
                project.source_directory.as_ref(),
                &overlay_source,
                || check_snapshot_current(snapshot),
            )
            .map_err(|error| error.to_string())?;

        let FrontendAnalysis { ast, symbol_table, type_table } = frontend;
        check_snapshot_current(snapshot).map_err(|error| error.to_string())?;
        Ok::<_, String>(CompilerSemanticCollector::new(symbol_table, type_table).collect(ast))
    });

    match result {
        Ok(occurrences) => Some(occurrences),
        Err(error) => {
            tracing::debug!(uri = snapshot.uri.as_str(), error, "compiler semantic analysis unavailable; falling back");
            None
        }
    }
}

/// Return the file and project context required for compiler-backed analysis.
fn compiler_inputs(snapshot: &DocumentSnapshot) -> Option<(&Arc<PathBuf>, &Arc<ProjectContext>)> {
    let file_path = snapshot.file_path.as_ref()?;
    let project = snapshot.project.as_ref()?;
    // Keep compiler analysis inside the package source tree. Files outside the
    // source root may still receive syntax tokens, but the compiler pipeline
    // expects entry/module relationships rooted at the package source dir.
    file_path.starts_with(project.source_directory.as_ref()).then_some((file_path, project))
}

impl PackageAnalysisCache {
    /// Return cached import stubs for the project, reloading them whenever the
    /// watched manifest or source metadata changes.
    fn import_stubs_for(&mut self, project: &ProjectContext) -> Result<Arc<IndexMap<Symbol, leo_ast::Stub>>, String> {
        if let Some(entry) = self.entries.get_mut(project.package_root.as_ref())
            && entry.revision == watched_paths_revision_cached(entry.watch_paths.as_ref(), &mut entry.watch_state)
        {
            return Ok(Arc::clone(&entry.import_stubs));
        }

        // Import stubs are package-wide, but they depend on manifest/source
        // metadata. Rebuild the entry when any watched input changes.
        let loaded = load_import_stubs_for_package(project.package_root.as_ref(), leo_ast::NetworkName::TestnetV0)
            .map_err(|error| error.to_string())?;
        let watch_paths = Arc::<[PathBuf]>::from(loaded.watch_paths);
        let mut watch_state = HashMap::new();
        let revision = watched_paths_revision_cached(watch_paths.as_ref(), &mut watch_state);
        let import_stubs = Arc::new(loaded.stubs);
        self.entries.insert(project.package_root.as_ref().clone(), PackageAnalysisCacheEntry {
            import_stubs: Arc::clone(&import_stubs),
            watch_paths,
            watch_state,
            revision,
        });
        Ok(import_stubs)
    }
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
    current_program: Symbol,
    current_module: Vec<Symbol>,
    local_scopes: Vec<HashMap<Symbol, LocalBinding>>,
    owner_stack: Vec<Option<Location>>,
}

impl<'a> CompilerSemanticCollector<'a> {
    /// Create a fresh collector bound to one compiler frontend snapshot.
    fn new(symbol_table: &'a SymbolTable, type_table: &'a TypeTable) -> Self {
        Self {
            symbol_table,
            type_table,
            occurrences: Vec::new(),
            current_program: Symbol::intern(""),
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

    /// Record a program or imported-namespace occurrence.
    fn add_namespace_occurrence(&mut self, identifier: &Identifier, role: OccurrenceRole) {
        if let Some(range) = span_to_file_range(identifier.span) {
            self.occurrences.push(SymbolOccurrence {
                range: range.clone(),
                identity: SymbolIdentity::Program {
                    name: identifier.name,
                    declaration: matches!(role, OccurrenceRole::Declaration).then_some(range.clone()),
                },
                role,
                token_kind: SemanticKind::Namespace,
                readonly: false,
            });
        }
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

    /// Resolve a global path through the compiler symbol tables and emit the
    /// most accurate semantic kind available for its target declaration.
    fn visit_global_path(&mut self, path: &Path) {
        if let Some(user_program) = path.user_program() {
            self.add_namespace_occurrence(&user_program.name, OccurrenceRole::Reference);
        } else if let Some(first) = path.qualifier().first()
            && (self.symbol_table.is_library(first.name) || first.name == self.current_program)
        {
            self.add_namespace_occurrence(first, OccurrenceRole::Reference);
        }

        let Some(location) = path.try_global_location().cloned() else {
            return;
        };
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
    type AdditionalInput = ();
    type Output = ();

    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Regular calls highlight the callee path as a function and then visit
        // all compile-time and runtime arguments as expressions.
        self.visit_path(&input.function, &());
        input.const_arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
        input.arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
    }

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

    fn visit_composite_type(&mut self, input: &CompositeType) {
        // Composite types reuse the same path identity logic as value-level
        // references, plus any const-generic arguments they carry.
        self.visit_path(&input.path, &());
        input.const_arguments.iter().for_each(|expr| self.visit_expression(expr, &()));
    }

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

    fn visit_member_access(&mut self, input: &MemberAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        // Member access records the receiver first so nested expressions still
        // contribute their own occurrences before the property reference.
        let owner = self.member_owner_from_expression(&input.inner);
        self.visit_expression(&input.inner, &());
        self.add_member_occurrence(owner, &input.name, OccurrenceRole::Reference, false, None);
    }

    fn visit_path(&mut self, input: &Path, _additional: &Self::AdditionalInput) -> Self::Output {
        if input.is_global() {
            self.visit_global_path(input);
            return;
        }

        // Non-global paths only participate in semantic indexing if they can be
        // matched back to a currently bound lexical declaration.
        let Some(symbol) = input.try_local_symbol() else {
            return;
        };
        let Some(binding) = self.local_scopes.iter().rev().find_map(|scope| scope.get(&symbol)).cloned() else {
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

    fn visit_block(&mut self, input: &leo_ast::Block) {
        // Blocks introduce lexical scope for definitions created inside them.
        self.push_scope();
        input.statements.iter().for_each(|statement| self.visit_statement(statement));
        self.pop_scope();
    }

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
    fn visit_program(&mut self, input: &Program) {
        // Visit both owned source and imported stub graphs so semantic
        // identities remain available across dependency boundaries.
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
        input.modules.values().for_each(|module| self.visit_module(module));
        input.stubs.values().for_each(|stub| self.visit_stub(stub));
    }

    fn visit_library(&mut self, input: &leo_ast::Library) {
        // Libraries reuse the same collector machinery as programs, but their
        // top-level items live directly under the library name.
        let previous_program = self.current_program;
        let previous_module = std::mem::take(&mut self.current_module);

        self.current_program = input.name;
        input.consts.iter().for_each(|(_, declaration)| self.visit_const(declaration));
        input.structs.iter().for_each(|(_, composite)| self.visit_composite(composite));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
        input.modules.values().for_each(|module| self.visit_module(module));

        self.current_program = previous_program;
        self.current_module = previous_module;
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Reset the module path at each program scope so top-level locations do
        // not accidentally inherit a nested module qualifier.
        let previous_program = self.current_program;
        let previous_module = std::mem::take(&mut self.current_module);

        self.current_program = input.program_id.as_symbol();
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
        self.current_module = previous_module;
    }

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

    fn visit_mapping_prototype(&mut self, input: &MappingPrototype) {
        // Interface mapping prototypes mirror concrete mapping declarations for
        // semantic-token purposes, but without executable bodies.
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

    fn visit_storage_variable_prototype(&mut self, input: &StorageVariablePrototype) {
        // Interface storage prototypes still contribute property declarations
        // even though no backing storage exists in this source file.
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

    fn visit_function_prototype(&mut self, input: &FunctionPrototype) {
        // Prototype parameters still participate in local binding/highlighting
        // even though there is no executable body to visit.
        if let Some(range) = span_to_file_range(input.identifier.span) {
            let location = self.current_item_location(input.identifier.name);
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

    fn visit_record_prototype(&mut self, input: &RecordPrototype) {
        // Record members inherit the record prototype as their semantic owner.
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
    use super::{PackageAnalysisCache, analyze_snapshot};
    use crate::{
        document_store::DocumentSnapshot,
        project_model::ProjectModel,
        semantics::{OccurrenceRole, SemanticSource, SymbolIdentity},
    };
    use leo_ast::NetworkName;
    use leo_compiler::load_import_stubs_for_package;
    use line_index::LineIndex;
    use lsp_types::Uri;
    use serde_json::json;
    use std::{
        fs,
        path::Path,
        sync::{Arc, atomic::AtomicU64},
        thread,
        time::Duration,
    };
    use tempfile::tempdir;

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

    fn local_dependency_json(path: &Path) -> String {
        json!([{ "name": "helper", "location": "local", "path": path }]).to_string()
    }

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

    fn snapshot_for(path: &Path, text: &str) -> DocumentSnapshot {
        let uri = file_uri(path);
        let mut projects = ProjectModel::default();
        let (file_path, project) = projects.resolve_document_context(&uri);

        DocumentSnapshot {
            uri,
            text: Arc::from(text),
            line_index: Arc::new(LineIndex::new(text)),
            version: 1,
            generation: 1,
            file_path,
            project,
            cancel_token: Arc::new(AtomicU64::new(1)),
        }
    }

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
        let first = cache.import_stubs_for(project.as_ref()).expect("initial cache load");
        let second = cache.import_stubs_for(project.as_ref()).expect("cached load");

        assert!(Arc::ptr_eq(&first, &second));
    }

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
        let first = cache.import_stubs_for(project.as_ref()).expect("initial cache load");

        thread::sleep(Duration::from_millis(20));
        fs::write(&helper_source, "const VALUE: u32 = 2u32;\nconst EXTRA: u32 = VALUE + 1u32;\n")
            .expect("update helper source");

        let second = cache.import_stubs_for(project.as_ref()).expect("reloaded cache entry");
        assert!(!Arc::ptr_eq(&first, &second));
    }

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
                occurrence.range.path.as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "LIMIT"
            })
            .collect::<Vec<_>>();
        assert_eq!(occurrences.len(), 2);
        assert!(occurrences.iter().any(|occurrence| occurrence.role == OccurrenceRole::Declaration));
        assert!(occurrences.iter().any(|occurrence| occurrence.role == OccurrenceRole::Reference));

        let identities = occurrences.iter().map(|occurrence| &occurrence.identity).collect::<Vec<_>>();
        match (&identities[0], &identities[1]) {
            (SymbolIdentity::GlobalItem { location: left, .. }, SymbolIdentity::GlobalItem { location: right, .. }) => {
                assert_eq!(left, right)
            }
            other => panic!("expected shared global identities, got {other:?}"),
        }
    }

    #[test]
    fn member_references_reuse_resolved_composite_owner() {
        let tempdir = tempdir().expect("tempdir");
        let root = tempdir.path().join("root");
        write_manifest(&root, "root.aleo", "null");
        let source = concat!(
            "program root.aleo {\n",
            "    struct Point { x: u32, }\n\n",
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
                occurrence.range.path.as_ref() == &main_path
                    && &source[occurrence.range.start as usize..occurrence.range.end as usize] == "x"
            })
            .collect::<Vec<_>>();
        assert_eq!(x_occurrences.len(), 3);

        let owners = x_occurrences
            .iter()
            .map(|occurrence| match &occurrence.identity {
                SymbolIdentity::Member { owner, .. } => owner.clone(),
                other => panic!("expected member identity, got {other:?}"),
            })
            .collect::<Vec<_>>();

        assert!(owners.iter().all(|owner| owner.is_some()));
        assert!(owners.windows(2).all(|pair| pair[0] == pair[1]));
        assert!(x_occurrences.iter().any(|occurrence| occurrence.role == OccurrenceRole::Declaration));
        assert_eq!(x_occurrences.iter().filter(|occurrence| occurrence.role == OccurrenceRole::Reference).count(), 2);
    }
}

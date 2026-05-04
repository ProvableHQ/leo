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

//! Rowan-backed syntax semantics for Leo LSP fallback highlighting.
//!
//! This module owns the lightweight, compiler-free layer. It produces
//! best-effort symbol occurrences for the reusable semantic index and separate
//! lexical tokens for full editor highlighting coverage.

use crate::{
    document_store::{DocumentSnapshot, DocumentViewSnapshot, OpenFileOverlay},
    project_model::{ProjectContext, ProjectKind},
    semantics::{
        FileRange,
        OccurrenceRole,
        SemanticKind,
        SemanticTokenOccurrence,
        SourceFingerprint,
        SymbolIdentity,
        SymbolOccurrence,
        sort_occurrences,
    },
};
use leo_package::{CompilationUnit, Dependency, Location, Manifest, ProgramData};
use leo_parser_rowan::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, parse_main, parse_module};
use leo_span::{Symbol, create_session_if_not_set_then};
use std::{
    collections::HashMap,
    fs::{self, Metadata},
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
    time::UNIX_EPOCH,
};

/// Compiler-free semantic data extracted from one Rowan parse.
#[derive(Debug, Default)]
pub(crate) struct SyntaxSemantics {
    /// Symbol-like syntax occurrences retained in the reusable semantic index.
    pub(crate) occurrences: Vec<SymbolOccurrence>,
    /// Highlighting-only lexical tokens that should not affect navigation data.
    pub(crate) tokens: Vec<SemanticTokenOccurrence>,
    /// Verified disk fingerprints for syntax-discovered cross-file targets.
    pub(crate) fingerprints: HashMap<PathBuf, SourceFingerprint>,
}

/// Collect syntax-only symbols and lexical tokens for the snapshot's current text.
pub(crate) fn collect(snapshot: &DocumentSnapshot) -> SyntaxSemantics {
    collect_parts(
        snapshot.text.as_ref(),
        snapshot.file_path.as_ref(),
        snapshot.project.as_ref(),
        snapshot.open_overlays.as_ref(),
        ProgramTargetMode::LocalDependencies,
    )
}

/// Collect syntax fallback symbols for every open file in the package bucket.
pub(crate) fn collect_package_fallback(snapshot: &DocumentSnapshot) -> SyntaxSemantics {
    let mut semantics = collect_parts(
        snapshot.text.as_ref(),
        snapshot.file_path.as_ref(),
        snapshot.project.as_ref(),
        snapshot.open_overlays.as_ref(),
        ProgramTargetMode::LocalDependencies,
    );
    let Some(trigger_path) = snapshot.file_path.as_ref() else {
        return semantics;
    };

    for overlay in snapshot.open_overlays.iter().filter(|overlay| overlay.path.as_ref() != trigger_path.as_ref()) {
        let mut overlay_semantics = collect_parts(
            overlay.text.as_ref(),
            Some(&overlay.path),
            snapshot.project.as_ref(),
            snapshot.open_overlays.as_ref(),
            ProgramTargetMode::LocalDependencies,
        );
        semantics.occurrences.append(&mut overlay_semantics.occurrences);
        semantics.fingerprints.extend(overlay_semantics.fingerprints);
    }

    sort_occurrences(&mut semantics.occurrences);
    semantics
}

/// Collect syntax-only symbols and lexical tokens for a document-view job.
pub(crate) fn collect_view(snapshot: &DocumentViewSnapshot) -> SyntaxSemantics {
    collect_parts(
        snapshot.text.as_ref(),
        snapshot.file_path.as_ref(),
        snapshot.project.as_ref(),
        &[],
        ProgramTargetMode::None,
    )
}

/// Shared collection path for package-analysis and document-view snapshots.
fn collect_parts(
    text: &str,
    file_path: Option<&Arc<PathBuf>>,
    project: Option<&Arc<ProjectContext>>,
    open_overlays: &[OpenFileOverlay],
    target_mode: ProgramTargetMode,
) -> SyntaxSemantics {
    let parse = match choose_syntax_parser(file_path, project) {
        SyntaxParser::Main => parse_main(text),
        SyntaxParser::Module => parse_module(text),
    };

    let document_path = current_document_path(file_path);
    let (program_targets, fingerprints) =
        syntax_program_targets(text, Arc::clone(&document_path), project, open_overlays, target_mode);
    let mut semantics = parse
        .map(|tree| SyntaxSemanticCollector::new(document_path, program_targets).collect(&tree))
        .unwrap_or_default();
    semantics.fingerprints = fingerprints;
    // Symbol occurrences are sorted here because compiler-backed occurrences
    // merge with this vector before final semantic-token encoding.
    sort_occurrences(&mut semantics.occurrences);
    semantics
}

/// Choose the Rowan parser entry point that best matches this snapshot.
fn choose_syntax_parser(file_path: Option<&Arc<PathBuf>>, project: Option<&Arc<ProjectContext>>) -> SyntaxParser {
    if let Some(project) = project {
        // The Rowan parser uses a distinct entry-point grammar for `main.leo`.
        // Reuse the project model's entry-file resolution so syntax fallback
        // mirrors the same program-vs-module distinction as the compiler path.
        return match project.kind {
            ProjectKind::Program if file_path == Some(&project.entry_file) => SyntaxParser::Main,
            ProjectKind::Program | ProjectKind::Library => SyntaxParser::Module,
        };
    }

    // The full-file parser accepts both `program` sections and standalone
    // module items, which makes it the best fit for scratch files and compiler
    // fixtures that do not resolve to a package.
    SyntaxParser::Main
}

/// Return the current document path, or an empty placeholder for unmanaged buffers.
fn current_document_path(file_path: Option<&Arc<PathBuf>>) -> Arc<PathBuf> {
    file_path.cloned().unwrap_or_else(|| Arc::new(PathBuf::new()))
}

/// Syntax parser mode used by the fallback highlighter.
#[derive(Debug, Clone, Copy)]
enum SyntaxParser {
    /// Parse a file that may contain a full `program` declaration.
    Main,
    /// Parse a package module or library source file.
    Module,
}

/// Import target discovery depth for a syntax collection pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProgramTargetMode {
    /// Skip navigation target discovery for document-view highlighting.
    None,
    /// Collect open-buffer targets and local dependency sources.
    LocalDependencies,
}

/// Collect program declaration targets visible to syntax-only import handling.
fn syntax_program_targets(
    text: &str,
    document_path: Arc<PathBuf>,
    project: Option<&Arc<ProjectContext>>,
    open_overlays: &[OpenFileOverlay],
    target_mode: ProgramTargetMode,
) -> (HashMap<String, FileRange>, HashMap<PathBuf, SourceFingerprint>) {
    let mut targets = HashMap::new();
    let mut fingerprints = HashMap::new();
    if target_mode == ProgramTargetMode::None {
        return (targets, fingerprints);
    }

    add_program_targets_from_text(text, &document_path, &mut targets);
    for overlay in open_overlays {
        if overlay.path.as_ref() != document_path.as_ref() {
            add_program_targets_from_text(overlay.text.as_ref(), &overlay.path, &mut targets);
        }
    }
    if target_mode == ProgramTargetMode::LocalDependencies
        && let Some(project) = project
    {
        add_local_dependency_program_targets(project.as_ref(), &mut targets, &mut fingerprints);
    }
    (targets, fingerprints)
}

/// Add `program name.aleo` declarations from one parsed source to the target map.
fn add_program_targets_from_text(text: &str, path: &Arc<PathBuf>, targets: &mut HashMap<String, FileRange>) {
    let Ok(tree) = parse_main(text) else {
        return;
    };
    collect_program_target_tokens(&tree, path, targets);
}

/// Recursively collect program declaration name tokens from a Rowan tree.
fn collect_program_target_tokens(node: &SyntaxNode, path: &Arc<PathBuf>, targets: &mut HashMap<String, FileRange>) {
    for element in node.children_with_tokens() {
        match element {
            SyntaxElement::Node(child) => collect_program_target_tokens(&child, path, targets),
            SyntaxElement::Token(token) if is_program_declaration_token(&token) => {
                if let Some(range) =
                    FileRange::new(Arc::clone(path), token.text_range().start().into(), token.text_range().end().into())
                {
                    targets.entry(token.text().to_owned()).or_insert(range);
                }
            }
            SyntaxElement::Token(_) => {}
        }
    }
}

/// Add local manifest dependency program declarations to syntax-only import targets.
fn add_local_dependency_program_targets(
    project: &ProjectContext,
    targets: &mut HashMap<String, FileRange>,
    fingerprints: &mut HashMap<PathBuf, SourceFingerprint>,
) {
    let Ok(manifest) = Manifest::read_from_file(project.manifest_path.as_ref()) else {
        return;
    };

    for dependency in manifest.dependencies.iter().flatten() {
        let Some(source_path) = dependency_source_path(project.package_root.as_ref(), dependency) else {
            continue;
        };
        let Some((source, fingerprint)) = read_stable_source(source_path.as_path()) else {
            continue;
        };
        let source_path = source_path.canonicalize().unwrap_or(source_path);
        let source_path = Arc::new(source_path);
        add_program_targets_from_text(source.as_str(), &source_path, targets);
        fingerprints.insert(source_path.as_ref().clone(), fingerprint);
    }
}

/// Return the package-resolved source path for a local dependency.
fn dependency_source_path(package_root: &Path, dependency: &Dependency) -> Option<PathBuf> {
    if dependency.location != Location::Local {
        return None;
    }
    let path = dependency.path.as_deref()?;
    let root = if path.is_absolute() { path.to_path_buf() } else { package_root.join(path) };
    let unit =
        create_session_if_not_set_then(|_| CompilationUnit::from_package_path(Symbol::intern(&dependency.name), root));
    let ProgramData::SourcePath { source, .. } = unit.ok()?.data else {
        return None;
    };
    Some(source)
}

/// Read a source file and return a fingerprint for the exact bytes consumed.
fn read_stable_source(path: &Path) -> Option<(String, SourceFingerprint)> {
    let before = fs::metadata(path).ok().and_then(|metadata| disk_stamp(&metadata))?;
    let source = fs::read_to_string(path).ok()?;
    let after = fs::metadata(path).ok().and_then(|metadata| disk_stamp(&metadata))?;
    (before == after).then(|| {
        let content_hash = content_hash(source.as_str());
        (source, SourceFingerprint::Disk { modified_nanos: Some(after.modified_nanos), len: after.len, content_hash })
    })
}

/// Rowan-only fallback collector used when compiler analysis is unavailable.
///
/// This collector intentionally records only local syntax context, which keeps
/// fallback highlighting cheap and lets compiler-backed occurrences replace it
/// later on exact source-range matches.
struct SyntaxSemanticCollector {
    path: Arc<PathBuf>,
    semantics: SyntaxSemantics,
    local_scopes: Vec<HashMap<String, SyntaxLocalBinding>>,
    program_targets: HashMap<String, FileRange>,
}

impl SyntaxSemanticCollector {
    /// Create a new syntax collector for one document path.
    fn new(path: Arc<PathBuf>, program_targets: HashMap<String, FileRange>) -> Self {
        Self { path, semantics: SyntaxSemantics::default(), local_scopes: vec![HashMap::new()], program_targets }
    }

    /// Walk a Rowan syntax tree and emit best-effort semantic data.
    fn collect(mut self, tree: &SyntaxNode) -> SyntaxSemantics {
        self.collect_node(tree);
        self.semantics
    }

    /// Recursively walk syntax in source order while maintaining lightweight local scopes.
    fn collect_node(&mut self, node: &SyntaxNode) {
        let starts_scope = syntax_node_starts_scope(node.kind());
        if starts_scope {
            self.local_scopes.push(HashMap::new());
        }

        for element in node.children_with_tokens() {
            match element {
                SyntaxElement::Node(child) => self.collect_node(&child),
                SyntaxElement::Token(token) => self.collect_token(&token),
            }
        }

        if starts_scope {
            self.local_scopes.pop();
        }
    }

    /// Classify and record one token, binding local declarations as they appear.
    fn collect_token(&mut self, token: &SyntaxToken) {
        if let Some(occurrence) = self.import_occurrence(token) {
            self.semantics.occurrences.push(occurrence);
            return;
        }

        let classification = classify_syntax_token(token).or_else(|| self.classify_local_reference(token));
        if let Some((token_kind, role, readonly)) = classification {
            self.push_symbol_token(token, token_kind, role, readonly);
            if role == OccurrenceRole::Declaration
                && matches!(token_kind, SemanticKind::Parameter | SemanticKind::Variable)
            {
                self.bind_local(token, token_kind, readonly);
            }
        } else if let Some(token_kind) = classify_lexical_token(token.kind()) {
            self.push_lexical_token(token, token_kind);
        }
    }

    /// Record an import program segment as a namespace reference.
    fn import_occurrence(&self, token: &SyntaxToken) -> Option<SymbolOccurrence> {
        if !is_import_program_token(token) {
            return None;
        }
        let range = self.token_range(token)?;
        let name = create_session_if_not_set_then(|_| Symbol::intern(token.text()));
        Some(SymbolOccurrence {
            range,
            identity: SymbolIdentity::Program { name, declaration: self.program_targets.get(token.text()).cloned() },
            role: OccurrenceRole::Reference,
            token_kind: SemanticKind::Namespace,
            readonly: false,
        })
    }

    /// Record a syntax-derived symbol occurrence.
    fn push_symbol_token(
        &mut self,
        token: &SyntaxToken,
        token_kind: SemanticKind,
        role: OccurrenceRole,
        readonly: bool,
    ) {
        let Some(range) = self.token_range(token) else {
            return;
        };
        self.semantics.occurrences.push(SymbolOccurrence {
            range,
            identity: SymbolIdentity::Unknown,
            role,
            token_kind,
            readonly,
        });
    }

    /// Record a lexical token that is useful for highlighting but not navigation.
    fn push_lexical_token(&mut self, token: &SyntaxToken, token_kind: SemanticKind) {
        let Some(range) = self.token_range(token) else {
            return;
        };
        self.semantics.tokens.push(SemanticTokenOccurrence {
            range,
            token_kind,
            role: OccurrenceRole::Reference,
            readonly: false,
        });
    }

    /// Convert a Rowan token range into a file-relative LSP range.
    fn token_range(&self, token: &SyntaxToken) -> Option<FileRange> {
        FileRange::new(Arc::clone(&self.path), token.text_range().start().into(), token.text_range().end().into())
    }

    /// Bind a locally declared name in the innermost syntax scope.
    fn bind_local(&mut self, token: &SyntaxToken, token_kind: SemanticKind, readonly: bool) {
        if let Some(scope) = self.local_scopes.last_mut() {
            scope.insert(token.text().to_owned(), SyntaxLocalBinding { token_kind, readonly });
        }
    }

    /// Reuse earlier syntax-only local declarations for later identifier references.
    fn classify_local_reference(&self, token: &SyntaxToken) -> Option<(SemanticKind, OccurrenceRole, bool)> {
        if token.kind() != SyntaxKind::IDENT {
            return None;
        }
        let binding = self.local_scopes.iter().rev().find_map(|scope| scope.get(token.text()))?;
        Some((binding.token_kind, OccurrenceRole::Reference, binding.readonly))
    }
}

/// Minimal binding metadata for syntax-only local reference highlighting.
#[derive(Debug, Clone, Copy)]
struct SyntaxLocalBinding {
    /// Token kind originally assigned to the local declaration.
    token_kind: SemanticKind,
    /// Whether references should inherit the readonly modifier.
    readonly: bool,
}

/// Return whether a syntax node introduces a local name scope.
fn syntax_node_starts_scope(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::PROGRAM_DECL
            | SyntaxKind::FUNCTION_DEF
            | SyntaxKind::FINAL_FN_DEF
            | SyntaxKind::CONSTRUCTOR_DEF
            | SyntaxKind::BLOCK
            | SyntaxKind::FOR_STMT
            | SyntaxKind::FOR_INCLUSIVE_STMT
    )
}

/// Best-effort token classification from local Rowan syntax context.
fn classify_syntax_token(token: &SyntaxToken) -> Option<(SemanticKind, OccurrenceRole, bool)> {
    if token.kind() != SyntaxKind::IDENT {
        return None;
    }

    // Classify by the nearest declaration/reference-shaped ancestor so a token
    // is highlighted according to the local construct it participates in, not
    // whichever outer syntax node happens to contain it.
    let enclosing = token.parent_ancestors().find(|node| {
        matches!(
            node.kind(),
            SyntaxKind::PROGRAM_DECL
                | SyntaxKind::FUNCTION_DEF
                | SyntaxKind::CONSTRUCTOR_DEF
                | SyntaxKind::STRUCT_DEF
                | SyntaxKind::RECORD_DEF
                | SyntaxKind::INTERFACE_DEF
                | SyntaxKind::MAPPING_DEF
                | SyntaxKind::STORAGE_DEF
                | SyntaxKind::GLOBAL_CONST
                | SyntaxKind::PARAM
                | SyntaxKind::PARAM_PUBLIC
                | SyntaxKind::PARAM_PRIVATE
                | SyntaxKind::PARAM_CONSTANT
                | SyntaxKind::CONST_PARAM
                | SyntaxKind::STRUCT_MEMBER
                | SyntaxKind::STRUCT_MEMBER_PUBLIC
                | SyntaxKind::STRUCT_MEMBER_PRIVATE
                | SyntaxKind::STRUCT_MEMBER_CONSTANT
                | SyntaxKind::LET_STMT
                | SyntaxKind::CONST_STMT
                | SyntaxKind::FOR_STMT
                | SyntaxKind::FOR_INCLUSIVE_STMT
                | SyntaxKind::FIELD_EXPR
                | SyntaxKind::CALL_EXPR
                | SyntaxKind::TYPE_PATH
                | SyntaxKind::STRUCT_EXPR
                | SyntaxKind::STRUCT_LOCATOR_EXPR
                | SyntaxKind::STRUCT_FIELD_INIT
                | SyntaxKind::STRUCT_FIELD_SHORTHAND
        )
    })?;

    match enclosing.kind() {
        SyntaxKind::PROGRAM_DECL if next_non_trivia_token(token).is_some_and(|next| next.kind() == SyntaxKind::DOT) => {
            Some((SemanticKind::Namespace, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::FUNCTION_DEF | SyntaxKind::CONSTRUCTOR_DEF
            if token_after_keyword(token, &[SyntaxKind::KW_FN, SyntaxKind::KW_CONSTRUCTOR]) =>
        {
            Some((SemanticKind::Function, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::STRUCT_DEF | SyntaxKind::RECORD_DEF
            if token_after_keyword(token, &[SyntaxKind::KW_STRUCT, SyntaxKind::KW_RECORD]) =>
        {
            Some((SemanticKind::Type, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::INTERFACE_DEF if token_after_keyword(token, &[SyntaxKind::KW_INTERFACE]) => {
            Some((SemanticKind::Interface, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::MAPPING_DEF if token_after_keyword(token, &[SyntaxKind::KW_MAPPING]) => {
            Some((SemanticKind::Property, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::STORAGE_DEF if token_after_keyword(token, &[SyntaxKind::KW_STORAGE]) => {
            Some((SemanticKind::Property, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::GLOBAL_CONST if token_after_keyword(token, &[SyntaxKind::KW_CONST]) => {
            Some((SemanticKind::Variable, OccurrenceRole::Declaration, true))
        }
        SyntaxKind::PARAM | SyntaxKind::PARAM_PUBLIC | SyntaxKind::PARAM_PRIVATE => {
            Some((SemanticKind::Parameter, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::PARAM_CONSTANT | SyntaxKind::CONST_PARAM => {
            Some((SemanticKind::Parameter, OccurrenceRole::Declaration, true))
        }
        SyntaxKind::STRUCT_MEMBER | SyntaxKind::STRUCT_MEMBER_PUBLIC | SyntaxKind::STRUCT_MEMBER_PRIVATE => {
            Some((SemanticKind::Property, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::STRUCT_MEMBER_CONSTANT => Some((SemanticKind::Property, OccurrenceRole::Declaration, true)),
        SyntaxKind::LET_STMT if token_in_ident_pattern(token) => {
            Some((SemanticKind::Variable, OccurrenceRole::Declaration, false))
        }
        SyntaxKind::CONST_STMT if token_in_ident_pattern(token) => {
            Some((SemanticKind::Variable, OccurrenceRole::Declaration, true))
        }
        SyntaxKind::FOR_STMT | SyntaxKind::FOR_INCLUSIVE_STMT if token_in_ident_pattern(token) => {
            Some((SemanticKind::Variable, OccurrenceRole::Declaration, true))
        }
        SyntaxKind::FIELD_EXPR
            if prev_non_trivia_token(token).is_some_and(|previous| previous.kind() == SyntaxKind::DOT) =>
        {
            Some((SemanticKind::Property, OccurrenceRole::Reference, false))
        }
        SyntaxKind::CALL_EXPR if is_final_path_segment(token) => {
            Some((SemanticKind::Function, OccurrenceRole::Reference, false))
        }
        SyntaxKind::TYPE_PATH | SyntaxKind::STRUCT_EXPR | SyntaxKind::STRUCT_LOCATOR_EXPR
            if is_final_path_segment(token) =>
        {
            Some((SemanticKind::Type, OccurrenceRole::Reference, false))
        }
        SyntaxKind::STRUCT_FIELD_INIT | SyntaxKind::STRUCT_FIELD_SHORTHAND => {
            Some((SemanticKind::Property, OccurrenceRole::Reference, false))
        }
        _ => None,
    }
}

/// Classify non-symbol tokens for full semantic-token highlighting coverage.
fn classify_lexical_token(kind: SyntaxKind) -> Option<SemanticKind> {
    if kind.is_type_keyword() {
        return Some(SemanticKind::Type);
    }
    if matches!(kind, SyntaxKind::COMMENT_LINE | SyntaxKind::COMMENT_BLOCK) {
        return Some(SemanticKind::Comment);
    }
    if matches!(kind, SyntaxKind::STRING | SyntaxKind::IDENT_LIT | SyntaxKind::ADDRESS_LIT) {
        return Some(SemanticKind::String);
    }
    if matches!(kind, SyntaxKind::INTEGER) {
        return Some(SemanticKind::Number);
    }
    if matches!(kind, SyntaxKind::KW_ASSERT | SyntaxKind::KW_ASSERT_EQ | SyntaxKind::KW_ASSERT_NEQ) {
        return Some(SemanticKind::Function);
    }
    if kind.is_keyword() {
        return Some(SemanticKind::Keyword);
    }
    if kind.is_operator() || kind.is_punctuation() {
        return Some(SemanticKind::Operator);
    }
    None
}

/// Check whether the token immediately follows one of the given keywords.
fn token_after_keyword(token: &SyntaxToken, keywords: &[SyntaxKind]) -> bool {
    prev_non_trivia_token(token).is_some_and(|previous| keywords.contains(&previous.kind()))
}

/// Detect whether a token participates in an identifier-binding pattern.
fn token_in_ident_pattern(token: &SyntaxToken) -> bool {
    token.parent_ancestors().any(|node| node.kind() == SyntaxKind::IDENT_PATTERN)
}

/// Return whether this token is the last segment in a qualified path.
fn is_final_path_segment(token: &SyntaxToken) -> bool {
    next_non_trivia_token(token).is_none_or(|next| next.kind() != SyntaxKind::COLON_COLON)
}

/// Find the previous non-trivia token in source order.
fn prev_non_trivia_token(token: &SyntaxToken) -> Option<SyntaxToken> {
    let mut current = token.prev_token();
    while let Some(candidate) = current.clone() {
        if !candidate.kind().is_trivia() {
            return Some(candidate);
        }
        current = candidate.prev_token();
    }
    None
}

/// Find the next non-trivia token in source order.
fn next_non_trivia_token(token: &SyntaxToken) -> Option<SyntaxToken> {
    let mut current = token.next_token();
    while let Some(candidate) = current.clone() {
        if !candidate.kind().is_trivia() {
            return Some(candidate);
        }
        current = candidate.next_token();
    }
    None
}

/// Return whether this token is the program name in `program name.aleo`.
fn is_program_declaration_token(token: &SyntaxToken) -> bool {
    token.kind() == SyntaxKind::IDENT
        && prev_non_trivia_token(token).is_some_and(|previous| previous.kind() == SyntaxKind::KW_PROGRAM)
        && token_is_followed_by_aleo_suffix(token)
}

/// Return whether this token is the imported program name in `import name.aleo`.
fn is_import_program_token(token: &SyntaxToken) -> bool {
    token.kind() == SyntaxKind::IDENT
        && prev_non_trivia_token(token).is_some_and(|previous| previous.kind() == SyntaxKind::KW_IMPORT)
        && token_is_followed_by_aleo_suffix(token)
}

/// Return whether an identifier token is followed by `.aleo`.
fn token_is_followed_by_aleo_suffix(token: &SyntaxToken) -> bool {
    let Some(dot) = next_non_trivia_token(token) else {
        return false;
    };
    dot.kind() == SyntaxKind::DOT
        && next_non_trivia_token(&dot).is_some_and(|network| network.kind() == SyntaxKind::KW_ALEO)
}

/// Cheap filesystem stamp used to prove a syntax-side disk read was stable.
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

/// Hash source text for stale-target detection without retaining full text.
fn content_hash(contents: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    contents.hash(&mut hasher);
    hasher.finish()
}

/// Convert file metadata modification time into a nanosecond stamp.
fn metadata_modified_nanos(metadata: &Metadata) -> Option<u128> {
    metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok().map(|duration| duration.as_nanos())
}

#[cfg(test)]
mod tests {
    use super::collect;
    use crate::{
        compiler_bridge::PackageAnalysisCache,
        document_store::{DocumentSnapshot, DocumentStore},
        project_model::ProjectModel,
        semantics::{OccurrenceRole, SemanticKind, SemanticSource},
    };
    use lsp_types::Uri;
    use std::{fs, path::Path};
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

    /// Build a committed document snapshot for syntax fallback tests.
    fn snapshot_for(path: &Path, text: &str) -> DocumentSnapshot {
        let uri = file_uri(path);
        let mut projects = ProjectModel::default();
        let (file_path, project) = projects.resolve_document_context(&uri);
        let mut documents = DocumentStore::default();
        documents.commit_open(documents.prepare_open(uri, "leo".to_owned(), 1, text.to_owned(), file_path, project))
    }

    /// Verifies unmanaged buffers get syntax symbols and lexical token coverage.
    #[test]
    fn unmanaged_program_buffers_emit_symbols_and_full_lexical_coverage() {
        let tempdir = tempdir().expect("tempdir");
        let source = concat!(
            "struct Foo {\n",
            "    a: u8,\n",
            "}\n\n",
            "program test.aleo {\n",
            "    record Token {\n",
            "        // The token owner.\n",
            "        owner: address,\n",
            "        // The token amount.\n",
            "        amount: u64,\n",
            "    }\n\n",
            "    fn main(a: bool, foo: Foo, token: Token) -> bool {\n",
            "        assert_eq(foo, Foo { a: 0u8 });\n",
            "        assert_neq(foo, Foo { a: 1u8 });\n",
            "        assert(a);\n",
            "        assert_neq(token, Token { owner: aleo1abc123, amount: 0u64 });\n",
            "        return token.amount == 0u64 && a;\n",
            "    }\n",
            "}\n",
        );
        let path = tempdir.path().join("assert.leo");
        fs::write(&path, source).expect("write unmanaged source");

        let snapshot = snapshot_for(&path, source);
        let syntax = collect(&snapshot);
        let semantic_snapshot =
            crate::compiler_bridge::analyze_snapshot(&snapshot, &mut PackageAnalysisCache::default());

        assert_eq!(semantic_snapshot.source, SemanticSource::SyntaxOnly);
        assert!(semantic_snapshot.index.occurrences.len() > 1);
        let has_symbol = |spelling: &str, token_kind: SemanticKind, role: Option<OccurrenceRole>| {
            semantic_snapshot.index.occurrences.iter().any(|occurrence| {
                &source[occurrence.range.start as usize..occurrence.range.end as usize] == spelling
                    && occurrence.token_kind == token_kind
                    && role.is_none_or(|role| occurrence.role == role)
            })
        };
        let has_lexical_token = |spelling: &str, token_kind: SemanticKind| {
            syntax.tokens.iter().any(|token| {
                &source[token.range.start as usize..token.range.end as usize] == spelling
                    && token.token_kind == token_kind
            })
        };

        assert!(has_symbol("main", SemanticKind::Function, Some(OccurrenceRole::Declaration)));
        assert!(has_symbol("Token", SemanticKind::Type, Some(OccurrenceRole::Declaration)));
        assert!(has_symbol("amount", SemanticKind::Property, None));
        for parameter in ["a", "foo", "token"] {
            assert!(has_symbol(parameter, SemanticKind::Parameter, Some(OccurrenceRole::Reference)));
        }

        let lexical_kinds = [
            SemanticKind::Keyword,
            SemanticKind::Comment,
            SemanticKind::String,
            SemanticKind::Number,
            SemanticKind::Operator,
        ];
        assert!(
            !semantic_snapshot
                .index
                .occurrences
                .iter()
                .any(|occurrence| lexical_kinds.contains(&occurrence.token_kind)),
            "lexical tokens should not pollute the symbol index"
        );

        for primitive in ["u8", "address", "u64", "bool"] {
            assert!(has_lexical_token(primitive, SemanticKind::Type), "expected primitive type token for {primitive}");
        }
        for builtin in ["assert_eq", "assert_neq", "assert"] {
            assert!(
                has_lexical_token(builtin, SemanticKind::Function),
                "expected built-in function token for {builtin}"
            );
        }
        assert!(has_lexical_token("struct", SemanticKind::Keyword));
        assert!(has_lexical_token("// The token owner.", SemanticKind::Comment));
        assert!(has_lexical_token("aleo1abc123", SemanticKind::String));
        assert!(has_lexical_token("0u64", SemanticKind::Number));
        assert!(has_lexical_token("==", SemanticKind::Operator));
    }
}

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

//! Shared semantic data model for LSP highlighting and navigation.
//!
//! Compiler and syntax collectors emit rich source occurrences into this
//! module's types. Package analysis then lowers them into compact file IDs,
//! symbol-key IDs, and byte ranges so semantic tokens, go-to-definition,
//! references, and future rename support can reuse one bounded index.

use leo_ast::Location;
use leo_span::Symbol;
use line_index::LineIndex;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

/// Stable file-relative byte range for semantic indexing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileRange {
    /// File path owning this range.
    pub path: Arc<PathBuf>,
    /// Inclusive start byte offset relative to `path`.
    pub start: u32,
    /// Exclusive end byte offset relative to `path`.
    pub end: u32,
}

impl FileRange {
    /// Create a non-empty file-relative range.
    pub fn new(path: Arc<PathBuf>, start: u32, end: u32) -> Option<Self> {
        (start < end).then_some(Self { path, start, end })
    }
}

/// Semantic token source used to build the latest snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticSource {
    /// Only syntax-based token classification succeeded.
    SyntaxOnly,
    /// Compiler frontend analysis refined the syntax fallback.
    CompilerEnhanced,
}

/// Internal token kind independent from the LSP wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticKind {
    Namespace,
    Type,
    Interface,
    Function,
    Parameter,
    Variable,
    Property,
    Keyword,
    Comment,
    String,
    Number,
    Operator,
}

impl SemanticKind {
    /// Pack the fixed semantic-kind legend into the occurrence flag word.
    pub fn to_packed_bits(self) -> u32 {
        match self {
            Self::Namespace => 0,
            Self::Type => 1,
            Self::Interface => 2,
            Self::Function => 3,
            Self::Parameter => 4,
            Self::Variable => 5,
            Self::Property => 6,
            Self::Keyword => 7,
            Self::Comment => 8,
            Self::String => 9,
            Self::Number => 10,
            Self::Operator => 11,
        }
    }

    /// Unpack a semantic kind previously written by [`Self::to_packed_bits`].
    pub fn from_packed_bits(bits: u32) -> Self {
        match bits {
            0 => Self::Namespace,
            1 => Self::Type,
            2 => Self::Interface,
            3 => Self::Function,
            4 => Self::Parameter,
            5 => Self::Variable,
            6 => Self::Property,
            7 => Self::Keyword,
            8 => Self::Comment,
            9 => Self::String,
            10 => Self::Number,
            11 => Self::Operator,
            _ => panic!("invalid packed semantic kind {bits}"),
        }
    }
}

/// Declaration-vs-reference role tracked for later navigation features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccurrenceRole {
    Declaration,
    Reference,
}

/// Stable semantic identity shared across occurrences of the same symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolIdentity {
    /// A lexical binding keyed by its declaration span.
    Local { declaration: FileRange },
    /// A top-level declaration tracked by compiler `Location`.
    GlobalItem { location: Location, declaration: Option<FileRange> },
    /// A member declaration or access owned by the current surrounding item.
    Member { owner: Option<Location>, name: Symbol, declaration: Option<FileRange> },
    /// A program declaration or namespace occurrence scoped by package root.
    Program { program: Symbol, package_root: Option<Arc<PathBuf>>, declaration: Option<FileRange> },
    /// Syntax-only occurrence without enough semantic data to identify the symbol.
    Unknown,
}

impl SymbolIdentity {
    /// Return the transient semantic key used while lowering worker results.
    ///
    /// The key intentionally excludes embedded direct-declaration ranges for
    /// global, member, and program identities. Those ranges are target hints,
    /// not part of the identity: references and declarations for the same item
    /// may learn their declaration range through different compiler paths.
    pub fn key(&self) -> Option<SymbolKey> {
        match self {
            Self::Local { declaration } => Some(SymbolKey::Local { declaration: declaration.clone() }),
            Self::GlobalItem { location, .. } => Some(SymbolKey::GlobalItem { location: location.clone() }),
            Self::Member { owner: Some(owner), name, .. } => {
                Some(SymbolKey::Member { owner: owner.clone(), name: *name })
            }
            Self::Member { owner: None, .. } | Self::Unknown => None,
            Self::Program { program, package_root: Some(package_root), .. } => {
                Some(SymbolKey::Program { program: *program, package_root: Arc::clone(package_root) })
            }
            Self::Program { package_root: None, .. } => None,
        }
    }

    /// Return the source range the compiler attached directly to this identity.
    ///
    /// Direct declarations let references jump even before the definition map
    /// has seen the corresponding declaration occurrence. They are still
    /// validated through the compact analyzed-file table before any LSP
    /// response is emitted.
    pub fn direct_declaration(&self) -> Option<&FileRange> {
        match self {
            Self::Local { declaration } => Some(declaration),
            Self::GlobalItem { declaration, .. }
            | Self::Member { declaration, .. }
            | Self::Program { declaration, .. } => declaration.as_ref(),
            Self::Unknown => None,
        }
    }
}

/// Build-time semantic key shared by related declarations and references.
///
/// `SymbolKey` is deliberately not stored in package caches. Local keys still
/// contain `FileRange`, which duplicates paths and range objects. Worker
/// lowering converts every key into [`CompactSymbolKey`] after file IDs are
/// interned for the package analysis.
#[derive(Debug, Clone, Eq)]
pub enum SymbolKey {
    /// A lexical binding keyed by its declaration token.
    Local { declaration: FileRange },
    /// A globally addressable item keyed by compiler location.
    GlobalItem { location: Location },
    /// A member keyed by concrete owner and member name.
    Member { owner: Location, name: Symbol },
    /// A program or namespace key scoped by resolved package root.
    Program { program: Symbol, package_root: Arc<PathBuf> },
}

impl PartialEq for SymbolKey {
    /// Compare symbol keys while treating equal package-root paths as equal Arcs.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Local { declaration: left }, Self::Local { declaration: right }) => left == right,
            (Self::GlobalItem { location: left }, Self::GlobalItem { location: right }) => left == right,
            (
                Self::Member { owner: left_owner, name: left_name },
                Self::Member { owner: right_owner, name: right_name },
            ) => left_owner == right_owner && left_name == right_name,
            (
                Self::Program { program: left_program, package_root: left_root },
                Self::Program { program: right_program, package_root: right_root },
            ) => {
                left_program == right_program
                    && (Arc::ptr_eq(left_root, right_root) || left_root.as_ref() == right_root.as_ref())
            }
            _ => false,
        }
    }
}

impl Hash for SymbolKey {
    /// Hash symbol keys by semantic identity rather than Arc pointer identity.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Local { declaration } => {
                0_u8.hash(state);
                declaration.hash(state);
            }
            Self::GlobalItem { location } => {
                1_u8.hash(state);
                location.hash(state);
            }
            Self::Member { owner, name } => {
                2_u8.hash(state);
                owner.hash(state);
                name.hash(state);
            }
            Self::Program { program, package_root } => {
                3_u8.hash(state);
                program.hash(state);
                package_root.as_ref().hash(state);
            }
        }
    }
}

/// Compact file identifier used inside package-level semantic caches.
pub type FileId = u32;

/// Compact symbol-key identifier used inside package-level semantic caches.
pub type SymbolKeyId = u32;

/// Sentinel packed into [`CompactOccurrence`] for syntax-only tokens without a key.
pub const NO_SYMBOL_KEY: SymbolKeyId = u32::MAX;

/// File-relative byte range stored without cloning paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompactRange {
    /// Interned analyzed-file ID.
    pub file: FileId,
    /// Inclusive UTF-8 byte offset.
    pub start: u32,
    /// Exclusive UTF-8 byte offset.
    pub end: u32,
}

/// Cached semantic key after local declarations have been lowered to file IDs.
#[derive(Debug, Clone, Eq)]
pub enum CompactSymbolKey {
    /// A lexical binding keyed by its compact declaration token.
    Local { declaration: CompactRange },
    /// A globally addressable item keyed by compiler location.
    GlobalItem { location: Location },
    /// A member keyed by concrete owner and member name.
    Member { owner: Location, name: Symbol },
    /// A program or namespace key scoped by resolved package root.
    Program { program: Symbol, package_root: Arc<PathBuf> },
}

impl PartialEq for CompactSymbolKey {
    /// Compare compact keys while preserving path-value equality for program roots.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Local { declaration: left }, Self::Local { declaration: right }) => left == right,
            (Self::GlobalItem { location: left }, Self::GlobalItem { location: right }) => left == right,
            (
                Self::Member { owner: left_owner, name: left_name },
                Self::Member { owner: right_owner, name: right_name },
            ) => left_owner == right_owner && left_name == right_name,
            (
                Self::Program { program: left_program, package_root: left_root },
                Self::Program { program: right_program, package_root: right_root },
            ) => {
                left_program == right_program
                    && (Arc::ptr_eq(left_root, right_root) || left_root.as_ref() == right_root.as_ref())
            }
            _ => false,
        }
    }
}

impl Hash for CompactSymbolKey {
    /// Hash compact keys by semantic identity rather than Arc pointer identity.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Local { declaration } => {
                0_u8.hash(state);
                declaration.hash(state);
            }
            Self::GlobalItem { location } => {
                1_u8.hash(state);
                location.hash(state);
            }
            Self::Member { owner, name } => {
                2_u8.hash(state);
                owner.hash(state);
                name.hash(state);
            }
            Self::Program { program, package_root } => {
                3_u8.hash(state);
                program.hash(state);
                package_root.as_ref().hash(state);
            }
        }
    }
}

/// Compact occurrence retained by package analysis.
///
/// The semantic key and token metadata intentionally share one trailing word:
/// low 26 bits store the package-local key ID, while the high 6 bits store
/// declaration/reference, readonly, and semantic-kind flags. That keeps the
/// occurrence table at 16 bytes, which is the memory budget that pays for the
/// secondary references index.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactOccurrence {
    /// Source range for the occurrence.
    pub range: CompactRange,
    /// Packed semantic key plus role/readonly/kind flags.
    key_and_flags: u32,
}

const PACKED_KEY_BITS: u32 = 26;
const PACKED_KEY_MASK: u32 = (1 << PACKED_KEY_BITS) - 1;
const PACKED_NO_SYMBOL_KEY: u32 = PACKED_KEY_MASK;
const FLAG_ROLE_DECLARATION: u32 = 1 << PACKED_KEY_BITS;
const FLAG_READONLY: u32 = 1 << (PACKED_KEY_BITS + 1);
const FLAG_KIND_SHIFT: u32 = PACKED_KEY_BITS + 2;
const FLAG_KIND_MASK: u32 = 0b1111 << FLAG_KIND_SHIFT;

impl CompactOccurrence {
    /// Construct a compact occurrence with packed metadata flags.
    pub fn new(
        range: CompactRange,
        key: SymbolKeyId,
        role: OccurrenceRole,
        token_kind: SemanticKind,
        readonly: bool,
    ) -> Self {
        let mut key_and_flags = pack_symbol_key(key) | (token_kind.to_packed_bits() << FLAG_KIND_SHIFT);
        if role == OccurrenceRole::Declaration {
            key_and_flags |= FLAG_ROLE_DECLARATION;
        }
        if readonly {
            key_and_flags |= FLAG_READONLY;
        }
        Self { range, key_and_flags }
    }

    /// Return the optional semantic key for navigation-grade occurrences.
    pub fn key_id(&self) -> Option<SymbolKeyId> {
        let key = self.key_and_flags & PACKED_KEY_MASK;
        (key != PACKED_NO_SYMBOL_KEY).then_some(key)
    }

    /// Return whether this occurrence is a declaration or reference.
    pub fn role(&self) -> OccurrenceRole {
        if self.key_and_flags & FLAG_ROLE_DECLARATION != 0 {
            OccurrenceRole::Declaration
        } else {
            OccurrenceRole::Reference
        }
    }

    /// Return the token kind carried by this occurrence.
    pub fn token_kind(&self) -> SemanticKind {
        SemanticKind::from_packed_bits((self.key_and_flags & FLAG_KIND_MASK) >> FLAG_KIND_SHIFT)
    }

    /// Return whether this occurrence carries the readonly modifier.
    pub fn readonly(&self) -> bool {
        self.key_and_flags & FLAG_READONLY != 0
    }
}

/// Pack an optional symbol key into the reserved low bits of an occurrence.
fn pack_symbol_key(key: SymbolKeyId) -> u32 {
    if key == NO_SYMBOL_KEY {
        return PACKED_NO_SYMBOL_KEY;
    }
    assert!(key < PACKED_NO_SYMBOL_KEY, "too many distinct symbol keys for one LSP package index");
    key
}

/// Borrowed occurrence returned by fast cursor lookup.
#[derive(Debug, Clone, Copy)]
pub struct CompactOccurrenceRef<'a> {
    /// The compact occurrence under the cursor.
    pub occurrence: &'a CompactOccurrence,
}

/// Source fingerprint captured for exactly the bytes used during analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFingerprint {
    /// The analyzed file came from an open editor buffer.
    ///
    /// Open buffers carry their own line index, so navigation can convert
    /// compact byte ranges without touching disk.
    OpenBuffer,
    /// The analyzed file came from disk with stable metadata around the read.
    ///
    /// Disk targets must be re-read and hash-checked before returning an LSP
    /// location, because the server intentionally does not retain file text.
    Disk { modified_nanos: Option<u128>, len: u64, content_hash: u64 },
    /// The file source could not prove that metadata matched the read bytes.
    ///
    /// Volatile files stay in the semantic index for highlighting and local
    /// lookup, but cross-file definition responses are suppressed for them.
    Volatile,
}

/// Compact metadata for one analyzed file.
#[derive(Debug, Clone)]
pub struct AnalyzedFile {
    /// Interned file ID used by compact ranges.
    pub id: FileId,
    /// Canonical or compiler-normalized path for this file.
    pub path: Arc<PathBuf>,
    /// Source fingerprint for the exact bytes that fed semantic analysis.
    pub fingerprint: SourceFingerprint,
    /// Open-buffer line index retained only for unsaved editor content.
    pub open_line_index: Option<Arc<LineIndex>>,
    /// Virtual sentinel paths are semantic identities only, never LSP locations.
    pub is_sentinel: bool,
}

/// Memory-light analyzed-file table owned by one package analysis.
#[derive(Debug, Clone, Default)]
pub struct AnalyzedFileSet {
    files: Arc<[AnalyzedFile]>,
}

impl AnalyzedFileSet {
    /// Build a new analyzed-file table from file metadata already assigned IDs.
    pub fn new(files: Vec<AnalyzedFile>) -> Self {
        Self { files: Arc::from(files) }
    }

    /// Return the analyzed file for a compact ID.
    pub fn get(&self, id: FileId) -> Option<&AnalyzedFile> {
        self.files.get(id as usize).filter(|file| file.id == id)
    }

    /// Return all analyzed files in ID order.
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[AnalyzedFile] {
        self.files.as_ref()
    }
}

/// One symbol occurrence in a source file.
///
/// These occurrences feed both semantic highlighting and the reusable semantic
/// index that later navigation features build on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolOccurrence {
    /// File-relative source range for the occurrence.
    pub range: FileRange,
    /// Stable identity shared with related declarations or references.
    pub identity: SymbolIdentity,
    /// Whether this occurrence is a declaration or reference.
    pub role: OccurrenceRole,
    /// Internal token classification before LSP encoding.
    pub token_kind: SemanticKind,
    /// Whether the occurrence should also carry the readonly modifier.
    pub readonly: bool,
}

/// One highlighting-only token in a source file.
///
/// Lexical tokens such as comments and operators should improve editor
/// coverage without becoming part of the symbol index used by navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticTokenOccurrence {
    /// File-relative source range for the token.
    pub range: FileRange,
    /// Internal token classification before LSP encoding.
    pub token_kind: SemanticKind,
    /// Whether this token is a declaration or reference.
    pub role: OccurrenceRole,
    /// Whether the token should also carry the readonly modifier.
    pub readonly: bool,
}

impl SemanticTokenOccurrence {
    /// Convert a symbol occurrence into its highlighting representation.
    pub fn from_symbol(occurrence: &SymbolOccurrence) -> Self {
        Self {
            range: occurrence.range.clone(),
            token_kind: occurrence.token_kind,
            role: occurrence.role,
            readonly: occurrence.readonly,
        }
    }
}

/// Compact semantic index cached once per package-analysis generation.
///
/// The index stores all paths and semantic keys once, then represents
/// occurrences and definition targets as dense integer IDs plus byte ranges.
/// This is the core memory guardrail for PR 3: opening ten files in one package
/// must not clone ten package-sized occurrence graphs.
#[derive(Debug, Clone, Default)]
pub struct SemanticIndex {
    /// Interned analyzed file paths.
    pub files: Arc<[Arc<PathBuf>]>,
    /// Path-to-file lookup table sorted for binary search.
    pub file_lookup: Arc<[(Arc<PathBuf>, FileId)]>,
    /// All compact occurrences in file/source order.
    pub occurrences: Arc<[CompactOccurrence]>,
    /// Per-file occurrence slices into `occurrences`.
    pub file_occurrence_ranges: Arc<[(FileId, std::ops::Range<u32>)]>,
    /// Sorted distinct keys with at least one definition target.
    pub definition_keys: Arc<[SymbolKeyId]>,
    /// Cumulative end offsets into `definition_ranges`.
    pub definition_ends: Arc<[u32]>,
    /// Deduplicated compact definition targets.
    pub definition_ranges: Arc<[CompactRange]>,
    /// Sorted distinct keys with at least one occurrence.
    pub key_occurrence_keys: Arc<[SymbolKeyId]>,
    /// Cumulative end offsets into `key_occurrence_indices`.
    pub key_occurrence_ends: Arc<[u32]>,
    /// Occurrence indices grouped by key and ordered by file/range.
    pub key_occurrence_indices: Arc<[u32]>,
}

impl SemanticIndex {
    /// Lower rich worker occurrences into a compact package index.
    ///
    /// The returned `AnalyzedFileSet` mirrors `files`, assigning the same
    /// `FileId`s so later LSP range conversion can recover paths and
    /// line-index/fingerprint metadata without retaining source text.
    pub fn build(
        occurrences: &[SymbolOccurrence],
        mut fingerprint_for_path: impl FnMut(&Path) -> SourceFingerprint,
        mut open_line_index_for_path: impl FnMut(&Path) -> Option<Arc<LineIndex>>,
    ) -> (Self, AnalyzedFileSet) {
        let mut path_ids = HashMap::<Arc<PathBuf>, FileId>::new();
        let mut files = Vec::<Arc<PathBuf>>::new();

        for occurrence in occurrences {
            intern_file(&occurrence.range.path, &mut path_ids, &mut files);
            if let Some(declaration) = occurrence.identity.direct_declaration() {
                intern_file(&declaration.path, &mut path_ids, &mut files);
            }
        }

        let mut key_ids = HashMap::<CompactSymbolKey, SymbolKeyId>::new();
        let mut compact_occurrences = Vec::<CompactOccurrence>::with_capacity(occurrences.len());
        let mut definition_pairs = Vec::<(SymbolKeyId, CompactRange)>::new();

        for occurrence in occurrences {
            let range = compact_range(&occurrence.range, &path_ids).expect("occurrence file interned");
            let key = occurrence.identity.key().and_then(|key| {
                let compact = compact_symbol_key(key, &path_ids)?;
                Some(intern_symbol_key(compact, &mut key_ids))
            });

            compact_occurrences.push(CompactOccurrence::new(
                range,
                key.unwrap_or(NO_SYMBOL_KEY),
                occurrence.role,
                occurrence.token_kind,
                occurrence.readonly,
            ));

            if let Some(key) = key {
                if occurrence.role == OccurrenceRole::Declaration {
                    definition_pairs.push((key, range));
                }
                // Some compiler paths attach the declaration range directly to
                // references before the declaration occurrence is visited. Keep
                // both sources and deduplicate after sorting so go-to-definition
                // is robust to AST traversal order.
                if let Some(declaration) = occurrence.identity.direct_declaration()
                    && let Some(declaration_range) = compact_range(declaration, &path_ids)
                {
                    definition_pairs.push((key, declaration_range));
                }
            }
        }

        compact_occurrences.sort_by(|left, right| {
            left.range
                .file
                .cmp(&right.range.file)
                .then_with(|| left.range.start.cmp(&right.range.start))
                .then_with(|| left.range.end.cmp(&right.range.end))
        });

        let file_occurrence_ranges = file_occurrence_ranges(&compact_occurrences);
        let (definition_keys, definition_ends, definition_ranges) = definition_slices(definition_pairs);
        let (key_occurrence_keys, key_occurrence_ends, key_occurrence_indices) =
            key_occurrence_slices(&compact_occurrences);
        let mut file_lookup =
            files.iter().enumerate().map(|(id, path)| (Arc::clone(path), id as FileId)).collect::<Vec<_>>();
        file_lookup.sort_by(|(left, _), (right, _)| left.cmp(right));

        let analyzed_files = files
            .iter()
            .enumerate()
            .map(|(id, path)| AnalyzedFile {
                id: id as FileId,
                path: Arc::clone(path),
                fingerprint: fingerprint_for_path(path.as_ref()),
                open_line_index: open_line_index_for_path(path.as_ref()),
                is_sentinel: is_sentinel_path(path.as_ref()),
            })
            .collect();

        (
            Self {
                files: Arc::from(files),
                file_lookup: Arc::from(file_lookup),
                occurrences: Arc::from(compact_occurrences),
                file_occurrence_ranges: Arc::from(file_occurrence_ranges),
                definition_keys: Arc::from(definition_keys),
                definition_ends: Arc::from(definition_ends),
                definition_ranges: Arc::from(definition_ranges),
                key_occurrence_keys: Arc::from(key_occurrence_keys),
                key_occurrence_ends: Arc::from(key_occurrence_ends),
                key_occurrence_indices: Arc::from(key_occurrence_indices),
            },
            AnalyzedFileSet::new(analyzed_files),
        )
    }

    /// Return the navigation-grade occurrence under a cursor byte offset.
    ///
    /// The lookup searches only the selected file slice. It accepts a cursor
    /// immediately after an identifier because editors often report that
    /// position when the caret visually sits at the end of a token.
    pub fn occurrence_at(&self, path: &Path, offset: u32) -> Option<CompactOccurrenceRef<'_>> {
        let file = self.file_id(path)?;
        let range = self.file_occurrence_range(file)?;

        let mut best = None::<&CompactOccurrence>;
        for occurrence in &self.occurrences[range.start as usize..range.end as usize] {
            if occurrence.key_id().is_none() {
                continue;
            }
            let contains = occurrence.range.start <= offset && offset < occurrence.range.end;
            let at_end = occurrence.range.start < offset && offset == occurrence.range.end;
            if !(contains || at_end) {
                continue;
            }

            best = match best {
                Some(current) if range_len(current.range) <= range_len(occurrence.range) => Some(current),
                _ => Some(occurrence),
            };
        }

        best.map(|occurrence| CompactOccurrenceRef { occurrence })
    }

    /// Return all deduplicated definition targets for a compact key.
    pub fn definitions_for(&self, key: SymbolKeyId) -> &[CompactRange] {
        let Ok(index) = self.definition_keys.binary_search(&key) else {
            return &[];
        };
        let start = if index == 0 { 0 } else { self.definition_ends[index - 1] };
        let end = self.definition_ends[index];
        &self.definition_ranges[start as usize..end as usize]
    }

    /// Return all occurrences for a compact key in deterministic file/range order.
    pub fn occurrences_for(&self, key: SymbolKeyId) -> impl Iterator<Item = &CompactOccurrence> + '_ {
        let range = self.key_occurrence_index_range(key).unwrap_or(0..0);
        self.key_occurrence_indices[range.start as usize..range.end as usize]
            .iter()
            .map(|index| &self.occurrences[*index as usize])
    }

    /// Return semantic-token occurrences for one file.
    pub fn token_occurrences_for_file(&self, path: &Path) -> Vec<SemanticTokenOccurrence> {
        let Some(file) = self.file_id(path) else {
            return Vec::new();
        };
        let Some(range) = self.file_occurrence_range(file) else {
            return Vec::new();
        };

        self.occurrences[range.start as usize..range.end as usize]
            .iter()
            .map(|occurrence| SemanticTokenOccurrence {
                range: FileRange {
                    path: Arc::clone(&self.files[occurrence.range.file as usize]),
                    start: occurrence.range.start,
                    end: occurrence.range.end,
                },
                token_kind: occurrence.token_kind(),
                role: occurrence.role(),
                readonly: occurrence.readonly(),
            })
            .collect()
    }

    /// Return the compact ID for a path interned in this index.
    pub fn file_id(&self, path: &Path) -> Option<FileId> {
        let index = self.file_lookup.binary_search_by(|(candidate, _)| candidate.as_ref().as_path().cmp(path)).ok()?;
        Some(self.file_lookup[index].1)
    }

    /// Return the contiguous occurrence slice for one interned file.
    fn file_occurrence_range(&self, file: FileId) -> Option<std::ops::Range<u32>> {
        let index = self.file_occurrence_ranges.binary_search_by_key(&file, |(candidate, _)| *candidate).ok()?;
        Some(self.file_occurrence_ranges[index].1.clone())
    }

    /// Return the compact index slice for one symbol key.
    fn key_occurrence_index_range(&self, key: SymbolKeyId) -> Option<std::ops::Range<u32>> {
        let index = self.key_occurrence_keys.binary_search(&key).ok()?;
        let start = if index == 0 { 0 } else { self.key_occurrence_ends[index - 1] };
        Some(start..self.key_occurrence_ends[index])
    }
}

/// Package-level semantic analysis shared by navigation and semantic tokens.
#[derive(Debug, Clone)]
pub struct CachedPackageAnalysis {
    /// Freshness key for this package analysis.
    pub key: crate::document_store::PackageAnalysisKey,
    /// Compact semantic index for the package generation.
    pub index: Arc<SemanticIndex>,
    /// Metadata for every file referenced by compact ranges.
    pub analyzed_files: Arc<AnalyzedFileSet>,
    /// Whether compiler analysis refined the syntax fallback.
    #[allow(dead_code)]
    pub source: SemanticSource,
}

/// Small per-document semantic-token view built from a package analysis.
#[derive(Debug, Clone)]
pub struct CachedDocumentView {
    /// Freshness key for this encoded token payload.
    pub key: crate::document_store::DocumentViewKey,
    /// LSP wire-format token data ready to return to the client.
    pub encoded_tokens: Arc<[u32]>,
}

/// Cached semantic-token payload and reusable semantic index for one document generation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SemanticSnapshot {
    /// LSP wire-format token data ready to return to the client.
    pub encoded_tokens: Arc<[u32]>,
    /// Rich occurrence data kept for future semantic navigation features.
    #[allow(dead_code)]
    pub index: Arc<SemanticIndex>,
    /// Whether the snapshot is syntax-only or compiler-enhanced.
    #[allow(dead_code)]
    pub source: SemanticSource,
}

/// Intern a path and return the compact file ID used by ranges.
fn intern_file(
    path: &Arc<PathBuf>,
    path_ids: &mut HashMap<Arc<PathBuf>, FileId>,
    files: &mut Vec<Arc<PathBuf>>,
) -> FileId {
    if let Some(id) = path_ids.get(path) {
        return *id;
    }
    let id = files.len() as FileId;
    files.push(Arc::clone(path));
    path_ids.insert(Arc::clone(path), id);
    id
}

/// Convert a path-bearing range into an ID-bearing compact range.
fn compact_range(range: &FileRange, path_ids: &HashMap<Arc<PathBuf>, FileId>) -> Option<CompactRange> {
    Some(CompactRange { file: *path_ids.get(&range.path)?, start: range.start, end: range.end })
}

/// Lower a build-time symbol key into the compact cached representation.
fn compact_symbol_key(key: SymbolKey, path_ids: &HashMap<Arc<PathBuf>, FileId>) -> Option<CompactSymbolKey> {
    match key {
        SymbolKey::Local { declaration } => {
            Some(CompactSymbolKey::Local { declaration: compact_range(&declaration, path_ids)? })
        }
        SymbolKey::GlobalItem { location } => Some(CompactSymbolKey::GlobalItem { location }),
        SymbolKey::Member { owner, name } => Some(CompactSymbolKey::Member { owner, name }),
        SymbolKey::Program { program, package_root } => Some(CompactSymbolKey::Program { program, package_root }),
    }
}

/// Intern a compact symbol key and return its stable package-local ID.
fn intern_symbol_key(key: CompactSymbolKey, key_ids: &mut HashMap<CompactSymbolKey, SymbolKeyId>) -> SymbolKeyId {
    if let Some(id) = key_ids.get(&key) {
        return *id;
    }
    let id = key_ids.len() as SymbolKeyId;
    key_ids.insert(key, id);
    id
}

/// Build per-file slices after occurrences have been sorted by file and range.
fn file_occurrence_ranges(occurrences: &[CompactOccurrence]) -> Vec<(FileId, std::ops::Range<u32>)> {
    let mut ranges = Vec::new();
    let mut start = 0_usize;
    while start < occurrences.len() {
        let file = occurrences[start].range.file;
        let mut end = start + 1;
        while end < occurrences.len() && occurrences[end].range.file == file {
            end += 1;
        }
        ranges.push((file, start as u32..end as u32));
        start = end;
    }
    ranges
}

/// Build compact definition lookup tables from `(key, target)` pairs.
fn definition_slices(mut pairs: Vec<(SymbolKeyId, CompactRange)>) -> (Vec<SymbolKeyId>, Vec<u32>, Vec<CompactRange>) {
    pairs.sort_by(|(left_key, left_range), (right_key, right_range)| {
        left_key.cmp(right_key).then_with(|| left_range.cmp(right_range))
    });
    pairs.dedup();

    // CSR keeps the key lookup table compact while preserving binary-search
    // lookup for definition and references requests.
    let mut keys = Vec::new();
    let mut ends = Vec::new();
    let mut ranges = Vec::new();
    let mut start = 0_usize;
    while start < pairs.len() {
        let key = pairs[start].0;
        let mut end = start;
        while end < pairs.len() && pairs[end].0 == key {
            ranges.push(pairs[end].1);
            end += 1;
        }
        keys.push(key);
        ends.push(ranges.len() as u32);
        start = end;
    }

    (keys, ends, ranges)
}

/// Build the secondary occurrence index in key/file/range order.
fn key_occurrence_slices(occurrences: &[CompactOccurrence]) -> (Vec<SymbolKeyId>, Vec<u32>, Vec<u32>) {
    let mut pairs = Vec::<(SymbolKeyId, u32)>::new();
    for (index, occurrence) in occurrences.iter().enumerate() {
        if let Some(key) = occurrence.key_id() {
            pairs.push((key, index as u32));
        }
    }

    pairs.sort_unstable_by(|left, right| {
        left.0.cmp(&right.0).then_with(|| {
            let left_range = occurrences[left.1 as usize].range;
            let right_range = occurrences[right.1 as usize].range;
            left_range.cmp(&right_range)
        })
    });

    let mut keys = Vec::<SymbolKeyId>::new();
    let mut ends = Vec::<u32>::new();
    let mut indices = Vec::<u32>::with_capacity(pairs.len());
    let mut start = 0_usize;
    while start < pairs.len() {
        let key = pairs[start].0;
        let mut end = start;
        while end < pairs.len() && pairs[end].0 == key {
            indices.push(pairs[end].1);
            end += 1;
        }
        keys.push(key);
        ends.push(indices.len() as u32);
        start = end;
    }

    (keys, ends, indices)
}

/// Return a compact range length, saturating defensively for malformed inputs.
fn range_len(range: CompactRange) -> u32 {
    range.end.saturating_sub(range.start)
}

/// Return whether an interned path is a compiler-bridge virtual sentinel.
pub(crate) fn is_sentinel_path(path: &Path) -> bool {
    path.as_os_str().to_string_lossy().starts_with("leo-vfs:")
}

/// Sort occurrences into stable file-relative source order.
///
/// The sort stays stable so callers can order preferred duplicates ahead of
/// fallback ones before deduplicating equal ranges.
pub(crate) fn sort_occurrences(occurrences: &mut [SymbolOccurrence]) {
    occurrences.sort_by(|left, right| {
        left.range
            .path
            .cmp(&right.range.path)
            .then_with(|| left.range.start.cmp(&right.range.start))
            .then_with(|| left.range.end.cmp(&right.range.end))
    });
}

/// Sort semantic-token occurrences into stable file-relative source order.
pub(crate) fn sort_token_occurrences(tokens: &mut [SemanticTokenOccurrence]) {
    tokens.sort_by(|left, right| {
        left.range
            .path
            .cmp(&right.range.path)
            .then_with(|| left.range.start.cmp(&right.range.start))
            .then_with(|| left.range.end.cmp(&right.range.end))
    });
}

/// Merge syntax-only and compiler-backed occurrences, preferring compiler
/// truth on exact range conflicts.
///
/// The merge consumes both vectors because the caller already owns them and the
/// result must be owned anyway. Borrowing slices here would force the merge to
/// clone every surviving occurrence into the returned buffer.
pub fn merge_occurrences(
    mut syntax_occurrences: Vec<SymbolOccurrence>,
    mut compiler_occurrences: Vec<SymbolOccurrence>,
) -> Vec<SymbolOccurrence> {
    if syntax_occurrences.is_empty() {
        sort_occurrences(&mut compiler_occurrences);
        return compiler_occurrences;
    }

    if compiler_occurrences.is_empty() {
        sort_occurrences(&mut syntax_occurrences);
        return syntax_occurrences;
    }

    compiler_occurrences.reserve(syntax_occurrences.len());
    compiler_occurrences.append(&mut syntax_occurrences);
    sort_occurrences(&mut compiler_occurrences);

    // Stable sorting keeps compiler occurrences ahead of syntax occurrences for
    // exact range ties, so deduplication preserves compiler-backed semantics.
    compiler_occurrences.dedup_by(|left, right| left.range == right.range);
    compiler_occurrences
}

#[cfg(test)]
mod tests {
    use super::{
        CompactOccurrence,
        CompactRange,
        FileRange,
        OccurrenceRole,
        SemanticIndex,
        SemanticKind,
        SourceFingerprint,
        SymbolIdentity,
        SymbolKeyId,
        SymbolOccurrence,
        merge_occurrences,
    };
    use std::{path::PathBuf, sync::Arc};

    /// Build a minimal occurrence for merge-order unit tests.
    fn occurrence(path: &Arc<PathBuf>, start: u32, end: u32, token_kind: SemanticKind) -> SymbolOccurrence {
        SymbolOccurrence {
            range: FileRange::new(Arc::clone(path), start, end).expect("non-empty range"),
            identity: SymbolIdentity::Unknown,
            role: OccurrenceRole::Reference,
            token_kind,
            readonly: false,
        }
    }

    /// Verifies every semantic kind survives compact occurrence packing.
    #[test]
    fn compact_occurrence_flags_round_trip() {
        let range = CompactRange { file: 0, start: 1, end: 4 };
        let kinds = [
            SemanticKind::Namespace,
            SemanticKind::Type,
            SemanticKind::Interface,
            SemanticKind::Function,
            SemanticKind::Parameter,
            SemanticKind::Variable,
            SemanticKind::Property,
            SemanticKind::Keyword,
            SemanticKind::Comment,
            SemanticKind::String,
            SemanticKind::Number,
            SemanticKind::Operator,
        ];

        for kind in kinds {
            let occurrence = CompactOccurrence::new(range, 7, OccurrenceRole::Declaration, kind, true);
            assert_eq!(occurrence.key_id(), Some(7));
            assert_eq!(occurrence.role(), OccurrenceRole::Declaration);
            assert_eq!(occurrence.token_kind(), kind);
            assert!(occurrence.readonly());
        }
        assert_eq!(std::mem::size_of::<CompactOccurrence>(), 16);
    }

    /// Verifies the secondary occurrence index returns source-ordered matches.
    #[test]
    fn occurrences_for_returns_keyed_source_order() {
        let path = Arc::new(PathBuf::from("main.leo"));
        let declaration = FileRange::new(Arc::clone(&path), 4, 9).expect("declaration range");
        let occurrences = vec![
            SymbolOccurrence {
                range: FileRange::new(Arc::clone(&path), 20, 25).expect("reference range"),
                identity: SymbolIdentity::Local { declaration: declaration.clone() },
                role: OccurrenceRole::Reference,
                token_kind: SemanticKind::Variable,
                readonly: false,
            },
            SymbolOccurrence {
                range: declaration.clone(),
                identity: SymbolIdentity::Local { declaration },
                role: OccurrenceRole::Declaration,
                token_kind: SemanticKind::Variable,
                readonly: false,
            },
        ];

        let (index, _) = SemanticIndex::build(&occurrences, |_| SourceFingerprint::Volatile, |_| None);
        let key = index.occurrences[0].key_id().expect("keyed declaration");
        let starts = index.occurrences_for(key).map(|occurrence| occurrence.range.start).collect::<Vec<_>>();

        assert_eq!(starts, vec![4, 20]);
        assert!(index.occurrences_for(SymbolKeyId::MAX - 1).next().is_none());
    }

    /// Verifies compiler occurrences win exact range conflicts with syntax fallback.
    #[test]
    fn merge_occurrences_prefers_compiler_occurrences_on_exact_range_conflicts() {
        let path = Arc::new(PathBuf::from("main.leo"));
        let syntax_occurrences = vec![occurrence(&path, 4, 9, SemanticKind::Variable)];
        let mut compiler_occurrence = occurrence(&path, 4, 9, SemanticKind::Function);
        compiler_occurrence.role = OccurrenceRole::Declaration;
        compiler_occurrence.readonly = true;

        let merged = merge_occurrences(syntax_occurrences, vec![compiler_occurrence]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].token_kind, SemanticKind::Function);
        assert_eq!(merged[0].role, OccurrenceRole::Declaration);
        assert!(merged[0].readonly);
    }

    /// Verifies merged occurrences are returned in stable source order.
    #[test]
    fn merge_occurrences_returns_source_ordered_output() {
        let a_path = Arc::new(PathBuf::from("a.leo"));
        let b_path = Arc::new(PathBuf::from("b.leo"));
        let merged = merge_occurrences(
            vec![
                occurrence(&b_path, 8, 10, SemanticKind::Variable),
                occurrence(&a_path, 9, 11, SemanticKind::Variable),
            ],
            vec![occurrence(&b_path, 1, 3, SemanticKind::Function), occurrence(&a_path, 2, 4, SemanticKind::Function)],
        );

        let starts = merged
            .iter()
            .map(|occurrence| (occurrence.range.path.as_ref().clone(), occurrence.range.start))
            .collect::<Vec<_>>();
        assert_eq!(starts, vec![
            (PathBuf::from("a.leo"), 2),
            (PathBuf::from("a.leo"), 9),
            (PathBuf::from("b.leo"), 1),
            (PathBuf::from("b.leo"), 8),
        ]);
    }
}

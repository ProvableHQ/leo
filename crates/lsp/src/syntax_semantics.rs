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
    document_store::DocumentSnapshot,
    project_model::ProjectKind,
    semantics::{
        FileRange,
        OccurrenceRole,
        SemanticKind,
        SemanticTokenOccurrence,
        SymbolIdentity,
        SymbolOccurrence,
        sort_occurrences,
    },
};
use leo_parser_rowan::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, parse_main, parse_module};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

/// Compiler-free semantic data extracted from one Rowan parse.
#[derive(Debug, Default)]
pub(crate) struct SyntaxSemantics {
    /// Symbol-like syntax occurrences retained in the reusable semantic index.
    pub(crate) occurrences: Vec<SymbolOccurrence>,
    /// Highlighting-only lexical tokens that should not affect navigation data.
    pub(crate) tokens: Vec<SemanticTokenOccurrence>,
}

/// Collect syntax-only symbols and lexical tokens for the snapshot's current text.
pub(crate) fn collect(snapshot: &DocumentSnapshot) -> SyntaxSemantics {
    let parse = match choose_syntax_parser(snapshot) {
        SyntaxParser::Main => parse_main(snapshot.text.as_ref()),
        SyntaxParser::Module => parse_module(snapshot.text.as_ref()),
    };

    let mut semantics = parse
        .map(|tree| SyntaxSemanticCollector::new(current_document_path(snapshot)).collect(&tree))
        .unwrap_or_default();
    // Symbol occurrences are sorted here because compiler-backed occurrences
    // merge with this vector before final semantic-token encoding.
    sort_occurrences(&mut semantics.occurrences);
    semantics
}

/// Choose the Rowan parser entry point that best matches this snapshot.
fn choose_syntax_parser(snapshot: &DocumentSnapshot) -> SyntaxParser {
    if let Some(project) = snapshot.project.as_ref() {
        // The Rowan parser uses a distinct entry-point grammar for `main.leo`.
        // Reuse the project model's entry-file resolution so syntax fallback
        // mirrors the same program-vs-module distinction as the compiler path.
        return match project.kind {
            ProjectKind::Program if snapshot.file_path.as_ref() == Some(&project.entry_file) => SyntaxParser::Main,
            ProjectKind::Program | ProjectKind::Library => SyntaxParser::Module,
        };
    }

    // The full-file parser accepts both `program` sections and standalone
    // module items, which makes it the best fit for scratch files and compiler
    // fixtures that do not resolve to a package.
    SyntaxParser::Main
}

/// Return the current document path, or an empty placeholder for unmanaged buffers.
fn current_document_path(snapshot: &DocumentSnapshot) -> Arc<PathBuf> {
    snapshot.file_path.clone().unwrap_or_else(|| Arc::new(PathBuf::new()))
}

/// Syntax parser mode used by the fallback highlighter.
#[derive(Debug, Clone, Copy)]
enum SyntaxParser {
    Main,
    Module,
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
}

impl SyntaxSemanticCollector {
    /// Create a new syntax collector for one document path.
    fn new(path: Arc<PathBuf>) -> Self {
        Self { path, semantics: SyntaxSemantics::default(), local_scopes: vec![HashMap::new()] }
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
    token_kind: SemanticKind,
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

#[cfg(test)]
mod tests {
    use super::collect;
    use crate::{
        compiler_bridge::PackageAnalysisCache,
        document_store::DocumentSnapshot,
        project_model::ProjectModel,
        semantics::{OccurrenceRole, SemanticKind, SemanticSource},
    };
    use line_index::LineIndex;
    use lsp_types::Uri;
    use std::{
        fs,
        path::Path,
        sync::{Arc, atomic::AtomicU64},
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

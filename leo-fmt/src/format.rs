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

//! Formatting logic for Leo source code.

use crate::{LINE_WIDTH, output::Output};
use leo_parser_rowan::{
    SyntaxElement,
    SyntaxKind::{self, *},
    SyntaxNode,
};

/// Format any syntax node.
pub fn format_node(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        // Top-level
        ROOT => format_root(node, out),
        PROGRAM_DECL => format_program(node, out),

        // Declarations
        FUNCTION_DEF | CONSTRUCTOR_DEF => format_function(node, out),
        STRUCT_DEF | RECORD_DEF => format_composite(node, out),
        IMPORT => format_import(node, out),
        MAPPING_DEF => format_mapping(node, out),
        STORAGE_DEF => format_storage(node, out),
        GLOBAL_CONST => format_global_const(node, out),
        ANNOTATION => format_annotation(node, out),
        PARAM_LIST => format_parameter_list(node, out),
        PARAM => format_parameter(node, out),
        RETURN_TYPE => format_return_type(node, out),
        CONST_PARAM => format_const_parameter(node, out),
        CONST_PARAM_LIST => format_const_parameter_list(node, out),
        CONST_ARG_LIST => format_const_argument_list(node, out),
        STRUCT_MEMBER => format_struct_member(node, out),

        // Statements
        BLOCK => format_block(node, out),
        RETURN_STMT => format_return(node, out),
        LET_STMT => format_definition(node, out),
        CONST_STMT => format_const_stmt(node, out),
        ASSIGN_STMT => format_assign(node, out),
        IF_STMT => format_conditional(node, out),
        FOR_STMT => format_iteration(node, out),
        ASSERT_STMT => format_assert(node, out),
        ASSERT_EQ_STMT => format_assert_pair(node, out, "assert_eq"),
        ASSERT_NEQ_STMT => format_assert_pair(node, out, "assert_neq"),
        EXPR_STMT => format_expr_stmt(node, out),

        // Expressions
        LITERAL => format_literal(node, out),
        CALL_EXPR => format_call(node, out),
        BINARY_EXPR => format_binary(node, out),
        PATH_EXPR => format_path(node, out),
        UNARY_EXPR => format_unary(node, out),
        TERNARY_EXPR => format_ternary(node, out),
        FIELD_EXPR => format_field_expr(node, out),
        INDEX_EXPR => format_index_expr(node, out),
        CAST_EXPR => format_cast(node, out),
        ARRAY_EXPR => format_array_expr(node, out),
        TUPLE_EXPR => format_tuple_expr(node, out),
        PAREN_EXPR => format_parenthesized(node, out),
        STRUCT_EXPR => format_struct_expr(node, out),
        STRUCT_FIELD_INIT => format_struct_field_init(node, out),
        ASYNC_EXPR => format_async_expr(node, out),
        LOCATOR_EXPR => format_locator_expr(node, out),

        // Patterns
        IDENT_PATTERN => format_ident_pattern(node, out),
        TUPLE_PATTERN => format_tuple_pattern(node, out),
        WILDCARD_PATTERN => out.write("_"),

        // Types
        k if is_type_node(k) => format_type(node, out),

        // Error nodes: emit the original text verbatim to avoid corrupting broken code.
        ERROR => {
            out.write(node.text().to_string().trim());
        }

        _other => {
            #[cfg(debug_assertions)]
            eprintln!("[leo-fmt] unhandled node kind: {_other:?}");
        }
    }
}

// =============================================================================
// Top-level
// =============================================================================

fn format_root(node: &SyntaxNode, out: &mut Output) {
    let mut prev_was_import = false;
    let mut had_output = false;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                COMMENT_LINE => {
                    if had_output {
                        out.ensure_newline();
                    }
                    out.write(tok.text().trim_end());
                    out.newline();
                    had_output = true;
                }
                COMMENT_BLOCK => {
                    if had_output {
                        out.ensure_newline();
                    }
                    out.write(tok.text());
                    had_output = true;
                }
                _ => {} // skip WHITESPACE, LINEBREAK, etc.
            },
            SyntaxElement::Node(n) => {
                let kind = n.kind();
                if kind == IMPORT {
                    format_import(&n, out);
                    prev_was_import = true;
                    had_output = true;
                } else if kind == PROGRAM_DECL {
                    if prev_was_import {
                        out.newline();
                    }
                    format_program(&n, out);
                    prev_was_import = false;
                    had_output = true;
                } else if kind == ANNOTATION {
                    if prev_was_import {
                        out.newline();
                    }
                    format_annotation(&n, out);
                    prev_was_import = false;
                    had_output = true;
                } else if is_program_item(kind) {
                    if prev_was_import {
                        out.newline();
                    }
                    format_node(&n, out);
                    prev_was_import = false;
                    had_output = true;
                } else if kind == ERROR {
                    let text = n.text().to_string();
                    let text = text.trim();
                    if !text.is_empty() {
                        if had_output {
                            out.ensure_newline();
                        }
                        out.write(text);
                        out.newline();
                        prev_was_import = false;
                        had_output = true;
                    }
                }
            }
        }
    }
}

fn format_program(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);

    // Count "item groups" — an annotation followed by a declaration counts as one group
    // with the declaration. Standalone annotations also count.
    let children: Vec<_> = node.children().collect();
    let mut item_group_count = 0;
    {
        let mut ci = 0;
        while ci < children.len() {
            let k = children[ci].kind();
            if k == ANNOTATION {
                // Skip contiguous annotations
                while ci < children.len() && children[ci].kind() == ANNOTATION {
                    ci += 1;
                }
                // The following item is part of the same group
                if ci < children.len() && is_program_item_non_annotation(children[ci].kind()) {
                    ci += 1;
                }
                item_group_count += 1;
            } else if is_program_item_non_annotation(k) || k == ERROR {
                item_group_count += 1;
                ci += 1;
            } else {
                ci += 1;
            }
        }
    }

    // Write "program name.aleo {"
    out.write("program");
    out.space();

    // Write the program ID tokens (name.aleo) between KW_PROGRAM and L_BRACE
    let prog_idx = find_token_index(&elems, KW_PROGRAM).unwrap_or(0);
    let lbrace_idx = find_token_index(&elems, L_BRACE).unwrap_or(elems.len());
    for elem in elems[prog_idx + 1..lbrace_idx].iter() {
        if let SyntaxElement::Token(tok) = elem {
            let k = tok.kind();
            if k != WHITESPACE && k != LINEBREAK {
                out.write(tok.text());
            }
        }
    }

    out.space();
    out.write("{");
    if item_group_count > 0 {
        out.newline();
    }

    // Iterate children_with_tokens to handle items and comments in order
    let mut item_group_idx = 0;
    let mut after_lbrace = false;
    let mut saw_linebreak = false;

    for elem in &elems {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                L_BRACE => {
                    after_lbrace = true;
                }
                R_BRACE => {}
                LINEBREAK => {
                    saw_linebreak = true;
                }
                WHITESPACE => {}
                COMMENT_LINE if after_lbrace => {
                    out.indented(|out| {
                        if saw_linebreak {
                            out.ensure_newline();
                        } else {
                            out.space();
                        }
                        out.write(tok.text().trim_end());
                        out.newline();
                    });
                    saw_linebreak = false;
                }
                COMMENT_BLOCK if after_lbrace => {
                    out.indented(|out| {
                        if saw_linebreak {
                            out.ensure_newline();
                        } else {
                            out.space();
                        }
                        out.write(tok.text());
                    });
                    saw_linebreak = false;
                }
                _ => {}
            },
            SyntaxElement::Node(n) if after_lbrace => {
                let kind = n.kind();
                if kind == ANNOTATION {
                    out.indented(|out| {
                        format_annotation(n, out);
                    });
                    saw_linebreak = false;
                } else if is_program_item_non_annotation(kind) {
                    out.indented(|out| {
                        format_node(n, out);
                        item_group_idx += 1;
                        if item_group_idx < item_group_count {
                            out.insert_newline_at_mark();
                        }
                    });
                    saw_linebreak = false;
                } else if kind == ERROR {
                    let text = n.text().to_string();
                    let text = text.trim();
                    if !text.is_empty() {
                        out.indented(|out| {
                            out.write(text);
                            out.newline();
                            out.set_mark();
                            item_group_idx += 1;
                            if item_group_idx < item_group_count {
                                out.insert_newline_at_mark();
                            }
                        });
                    }
                    saw_linebreak = false;
                }
            }
            _ => {}
        }
    }

    out.write("}");
    out.newline();
}

fn is_program_item_non_annotation(kind: SyntaxKind) -> bool {
    matches!(kind, FUNCTION_DEF | CONSTRUCTOR_DEF | STRUCT_DEF | RECORD_DEF | MAPPING_DEF | STORAGE_DEF | GLOBAL_CONST)
}

// =============================================================================
// Declarations
// =============================================================================

fn format_function(node: &SyntaxNode, out: &mut Output) {
    // Emit leading comments (trivia that appears before the first keyword)
    emit_leading_comments(node, out);

    // Emit keywords: async, function/transition/inline/script/constructor, name
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_ASYNC | KW_FUNCTION | KW_TRANSITION | KW_INLINE | KW_SCRIPT => {
                        out.write(tok.text());
                        out.space();
                    }
                    KW_CONSTRUCTOR => {
                        out.write(tok.text());
                    }
                    IDENT => {
                        out.write(tok.text());
                    }
                    COLON_COLON => {
                        out.write("::");
                    }
                    ARROW => {
                        out.space();
                        out.write("->");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                match k {
                    PARAM_LIST => format_parameter_list(&n, out),
                    CONST_PARAM_LIST => format_const_parameter_list(&n, out),
                    RETURN_TYPE => format_return_type(&n, out),
                    BLOCK => {
                        out.space();
                        format_block(&n, out);
                    }
                    k if is_type_node(k) => {
                        // Single return type (not wrapped in RETURN_TYPE)
                        format_type(&n, out);
                    }
                    _ => {}
                }
            }
        }
    }
    // Check next sibling for stolen trailing comments
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
    out.set_mark();
    out.ensure_newline();
}

fn format_annotation(node: &SyntaxNode, out: &mut Output) {
    let has_paren = has_token(node, L_PAREN);

    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    AT => out.write("@"),
                    L_PAREN => out.write("("),
                    R_PAREN => out.write(")"),
                    COMMA => {
                        out.write(",");
                        out.space();
                    }
                    EQ => {
                        out.space();
                        out.write("=");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => {
                        if has_paren || k == IDENT || k.is_keyword() {
                            out.write(tok.text());
                        }
                    }
                }
            }
            SyntaxElement::Node(_) => {}
        }
    }
    out.newline();
}

fn format_composite(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);
    let rbrace_idx = find_last_token_index(&elems, R_BRACE);

    // Emit leading comments
    emit_leading_comments(node, out);

    // Write keyword, name, and body
    let mut after_lbrace = false;
    for elem in &elems {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_STRUCT | KW_RECORD => {
                        out.write(tok.text());
                        out.space();
                    }
                    IDENT if !after_lbrace => {
                        out.write(tok.text());
                        out.space();
                    }
                    COLON_COLON => {
                        out.write("::");
                    }
                    L_BRACE => {
                        out.write("{");
                        out.newline();
                        after_lbrace = true;
                    }
                    R_BRACE => {}
                    COMMA => {}
                    // Emit comments that are direct children of the struct body
                    // (e.g. block comments after the last member's comma).
                    COMMENT_LINE if after_lbrace => {
                        out.indented(|out| {
                            out.write(tok.text().trim_end());
                        });
                        out.newline();
                    }
                    COMMENT_BLOCK if after_lbrace => {
                        out.indented(|out| {
                            out.write(tok.text());
                        });
                        out.newline();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                match k {
                    CONST_PARAM_LIST => format_const_parameter_list(n, out),
                    STRUCT_MEMBER => {
                        out.indented(|out| {
                            format_struct_member(n, out);
                            out.write(",");
                            // Emit stolen trailing comments from next member
                            if let Some(next) = n.next_sibling() {
                                emit_stolen_trailing_comments(&next, out);
                            }
                        });
                        out.newline();
                    }
                    _ => {}
                }
            }
        }
    }
    out.write("}");
    // Emit comments after closing brace
    if let Some(idx) = rbrace_idx {
        emit_comments_after(&elems, idx, out);
    }
    // Check next sibling for stolen trailing comments
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
    out.set_mark();
    out.ensure_newline();
}

fn format_struct_member(node: &SyntaxNode, out: &mut Output) {
    // Emit leading comments (the parser attaches comments as leading trivia
    // of the next token, so STRUCT_MEMBER may start with comments).
    emit_leading_comments(node, out);

    // Skip leading trivia (already handled above), then format the member.
    let mut past_leading_trivia = false;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !past_leading_trivia {
                    match k {
                        WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => continue,
                        _ => past_leading_trivia = true,
                    }
                }
                match k {
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    COMMA => {} // handled by parent
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT => {
                        out.write(tok.text());
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => {
                past_leading_trivia = true;
                format_type(&n, out);
            }
            _ => {}
        }
    }
}

fn format_import(node: &SyntaxNode, out: &mut Output) {
    out.write("import");
    out.space();

    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_IMPORT | WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(_) => {}
        }
    }
    out.ensure_newline();
}

fn format_mapping(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_MAPPING => {
                        out.write("mapping");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    FAT_ARROW => {
                        out.space();
                        out.write("=>");
                        out.space();
                    }
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_storage(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_STORAGE => {
                        out.write("storage");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_global_const(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_CONST => {
                        out.write("const");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    EQ => {
                        out.space();
                        out.write("=");
                        out.space();
                    }
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_type_node(k) {
                    format_type(&n, out);
                } else if is_expression(k) {
                    format_node(&n, out);
                }
            }
        }
    }
    out.ensure_newline();
}

// =============================================================================
// Parameters and return types
// =============================================================================

fn format_parameter_list(node: &SyntaxNode, out: &mut Output) {
    let params: Vec<_> = node.children().filter(|c| c.kind() == PARAM).collect();

    let param_strings: Vec<String> = params.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    if fits_on_one_line(col, "(", ")", &param_strings) {
        // Single-line: (param1, param2)
        out.write("(");
        for (i, param) in params.iter().enumerate() {
            format_parameter(param, out);
            if i < params.len() - 1 {
                out.write(",");
                emit_inline_comments_after_param(node, param, out);
                out.space();
            }
        }
        out.write(")");
    } else {
        // Multi-line: wrap each param on its own line
        out.write("(");
        out.newline();
        out.indented(|out| {
            for (i, param) in params.iter().enumerate() {
                format_parameter(param, out);
                out.write(",");
                if i < params.len() - 1 {
                    emit_inline_comments_after_param(node, param, out);
                }
                out.newline();
            }
        });
        out.write(")");
    }
}

fn format_parameter(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT => {
                        out.write(tok.text());
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_return_type(node: &SyntaxNode, out: &mut Output) {
    // First try single-line
    let single_line = format_return_type_to_string(node);
    let col = out.current_column();

    if col + single_line.len() <= LINE_WIDTH {
        out.write(&single_line);
    } else {
        // Multi-line: wrap tuple return type entries
        // Only tuple return types (with L_PAREN) get wrapped
        if has_token(node, L_PAREN) {
            // Collect return type entries: each is a sequence of visibility + type tokens/nodes
            // between commas. We format them individually.
            let entries = collect_return_type_entries(node);

            out.write("(");
            out.newline();
            out.indented(|out| {
                for entry in &entries {
                    out.write(entry);
                    out.write(",");
                    out.newline();
                }
            });
            out.write(")");
        } else {
            // Non-tuple — just emit inline
            out.write(&single_line);
        }
    }
}

fn format_return_type_to_string(node: &SyntaxNode) -> String {
    let mut out = Output::new();
    format_return_type_inline(node, &mut out);
    out.into_raw()
}

fn format_return_type_inline(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    L_PAREN => out.write("("),
                    R_PAREN => out.write(")"),
                    COMMA => {
                        out.write(",");
                        out.space();
                    }
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT => {
                        out.write(tok.text());
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

/// Collect the entries of a tuple return type as formatted strings.
/// Each entry is e.g. "public u64" or "field".
fn collect_return_type_entries(node: &SyntaxNode) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut in_tuple = false;

    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    L_PAREN => {
                        in_tuple = true;
                    }
                    R_PAREN => {
                        let trimmed = current.trim().to_string();
                        if !trimmed.is_empty() {
                            entries.push(trimmed);
                        }
                        current.clear();
                    }
                    COMMA if in_tuple => {
                        let trimmed = current.trim().to_string();
                        if !trimmed.is_empty() {
                            entries.push(trimmed);
                        }
                        current.clear();
                    }
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT if in_tuple => {
                        current.push_str(tok.text());
                        current.push(' ');
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ if in_tuple => {
                        current.push_str(tok.text());
                    }
                    _ => {}
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) && in_tuple => {
                let mut tmp = Output::new();
                format_type(&n, &mut tmp);
                current.push_str(&tmp.into_raw());
            }
            _ => {}
        }
    }

    entries
}

fn format_const_parameter(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_const_parameter_list(node: &SyntaxNode, out: &mut Output) {
    let params: Vec<_> = node.children().filter(|c| c.kind() == CONST_PARAM).collect();

    out.write("[");
    for (i, param) in params.iter().enumerate() {
        format_const_parameter(param, out);
        if i < params.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write("]");
}

fn format_const_argument_list(node: &SyntaxNode, out: &mut Output) {
    let args: Vec<_> = node.children().filter(|c| is_type_node(c.kind()) || is_expression(c.kind())).collect();

    out.write("[");
    for (i, arg) in args.iter().enumerate() {
        format_node(arg, out);
        if i < args.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write("]");
}

// =============================================================================
// Types
// =============================================================================

fn format_type(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        TYPE_PATH => format_type_path(node, out),
        TYPE_ARRAY => format_type_array(node, out),
        TYPE_TUPLE => format_type_tuple(node, out),
        TYPE_FUTURE => format_type_future(node, out),
        TYPE_MAPPING => format_type_mapping(node, out),
        TYPE_OPTIONAL => format_type_optional(node, out),
        _ => {}
    }
}

fn format_type_path(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => {
                if n.kind() == CONST_ARG_LIST {
                    format_const_argument_list(&n, out);
                } else {
                    format_node(&n, out);
                }
            }
        }
    }
}

fn format_type_array(node: &SyntaxNode, out: &mut Output) {
    out.write("[");
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    L_BRACKET | R_BRACKET => {}
                    SEMICOLON => {
                        out.write(";");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => {
                if is_type_node(n.kind()) {
                    format_type(&n, out);
                } else if is_expression(n.kind()) {
                    format_node(&n, out);
                }
            }
        }
    }
    out.write("]");
}

fn format_type_tuple(node: &SyntaxNode, out: &mut Output) {
    let types: Vec<_> = node.children().filter(|c| is_type_node(c.kind())).collect();

    out.write("(");
    for (i, ty) in types.iter().enumerate() {
        format_type(ty, out);
        if i < types.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write(")");
}

fn format_type_future(node: &SyntaxNode, out: &mut Output) {
    // Future or Future<Fn(...) -> ...>
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_type_mapping(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_type_optional(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_type_node(n.kind()) => format_type(&n, out),
            _ => {}
        }
    }
}

// =============================================================================
// Statements
// =============================================================================

fn format_block(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);

    // Check if block has any statements or comments (content worth indenting)
    let has_content = elems.iter().any(|e| match e {
        SyntaxElement::Node(n) => is_statement(n.kind()) || n.kind() == ERROR,
        SyntaxElement::Token(t) => matches!(t.kind(), COMMENT_LINE | COMMENT_BLOCK),
    });

    out.write("{");
    if has_content {
        out.newline();
        out.indented(|out| {
            // Iterate all children_with_tokens to emit statements and comments.
            // Comments appear as sibling tokens in rowan. We use LINEBREAK to
            // determine if a comment is trailing (same line as previous stmt)
            // or standalone (own line).
            let mut after_lbrace = false;
            let mut saw_linebreak = false;

            for elem in &elems {
                match elem {
                    SyntaxElement::Token(tok) => match tok.kind() {
                        L_BRACE => {
                            after_lbrace = true;
                        }
                        R_BRACE => {}
                        LINEBREAK => {
                            saw_linebreak = true;
                        }
                        WHITESPACE => {}
                        COMMENT_LINE if after_lbrace => {
                            if saw_linebreak {
                                out.ensure_newline();
                            } else {
                                out.space();
                            }
                            out.write(tok.text().trim_end());
                            out.newline();
                            saw_linebreak = false;
                        }
                        COMMENT_BLOCK if after_lbrace => {
                            if saw_linebreak {
                                out.ensure_newline();
                            } else {
                                out.space();
                            }
                            out.write(tok.text());
                            saw_linebreak = false;
                        }
                        _ => {}
                    },
                    SyntaxElement::Node(n) if after_lbrace && is_statement(n.kind()) => {
                        out.ensure_newline();
                        format_node(n, out);
                        saw_linebreak = false;
                    }
                    SyntaxElement::Node(n) if after_lbrace && n.kind() == ERROR => {
                        let text = n.text().to_string();
                        let text = text.trim();
                        if !text.is_empty() {
                            out.ensure_newline();
                            out.write(text);
                        }
                        saw_linebreak = false;
                    }
                    _ => {}
                }
            }
            out.ensure_newline();
        });
    }
    out.write("}");
    out.set_mark();
    // Emit comments after closing brace (at parent level)
    if let Some(idx) = find_last_token_index(&elems, R_BRACE) {
        emit_comments_after(&elems, idx, out);
    }
}

fn format_return(node: &SyntaxNode, out: &mut Output) {
    out.write("return");

    for child in node.children() {
        if is_expression(child.kind()) {
            out.space();
            format_node(&child, out);
        }
    }

    write_semicolon_with_comments(node, out);
}

fn format_definition(node: &SyntaxNode, out: &mut Output) {
    out.write("let");
    out.space();

    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_LET | WHITESPACE | LINEBREAK => {}
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    EQ => {
                        out.space();
                        out.write("=");
                        out.space();
                    }
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_type_node(k) {
                    format_type(&n, out);
                } else if is_expression(k) || matches!(k, IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN) {
                    format_node(&n, out);
                }
            }
        }
    }
}

fn format_const_stmt(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_CONST => {
                        out.write("const");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    EQ => {
                        out.space();
                        out.write("=");
                        out.space();
                    }
                    SEMICOLON => {
                        write_semicolon_with_comments(node, out);
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_type_node(k) {
                    format_type(&n, out);
                } else if is_expression(k) {
                    format_node(&n, out);
                }
            }
        }
    }
}

fn format_assign(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k == WHITESPACE || k == LINEBREAK {
                    // skip
                } else if is_assignment_op(k) {
                    out.space();
                    out.write(tok.text());
                    out.space();
                } else if k == SEMICOLON {
                    write_semicolon_with_comments(node, out);
                } else {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_expression(n.kind()) => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_conditional(node: &SyntaxNode, out: &mut Output) {
    let mut first_block = true;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_IF => {
                        out.write("if");
                        out.space();
                    }
                    KW_ELSE => {
                        out.space();
                        out.write("else");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_expression(k) {
                    format_node(&n, out);
                } else if k == BLOCK {
                    if first_block {
                        out.space();
                        first_block = false;
                    }
                    format_block(&n, out);
                } else if k == IF_STMT {
                    format_conditional(&n, out);
                }
            }
        }
    }
}

fn format_iteration(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_FOR => {
                        out.write("for");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    KW_IN => {
                        out.space();
                        out.write("in");
                        out.space();
                    }
                    DOT_DOT => {
                        out.write("..");
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_type_node(k) {
                    format_type(&n, out);
                } else if is_expression(k) {
                    format_node(&n, out);
                } else if k == BLOCK {
                    out.space();
                    format_block(&n, out);
                }
            }
        }
    }
}

fn format_assert(node: &SyntaxNode, out: &mut Output) {
    out.write("assert(");

    for child in node.children() {
        if is_expression(child.kind()) {
            format_node(&child, out);
        }
    }

    out.write(")");
    write_semicolon_with_comments(node, out);
}

fn format_assert_pair(node: &SyntaxNode, out: &mut Output, keyword: &str) {
    let exprs: Vec<_> = node.children().filter(|c| is_expression(c.kind())).collect();
    let expr_strings: Vec<String> = exprs.iter().map(format_node_to_string).collect();

    out.write(keyword);
    out.write("(");
    let col = out.current_column();

    if fits_on_one_line(col, "", ");", &expr_strings) {
        for (i, expr) in exprs.iter().enumerate() {
            format_node(expr, out);
            if i < exprs.len() - 1 {
                out.write(",");
                out.space();
            }
        }
    } else {
        // Multi-line (no trailing comma — fixed arity)
        out.newline();
        out.indented(|out| {
            for (i, expr) in exprs.iter().enumerate() {
                format_node(expr, out);
                if i < exprs.len() - 1 {
                    out.write(",");
                }
                out.newline();
            }
        });
    }
    out.write(")");
    write_semicolon_with_comments(node, out);
}

fn format_expr_stmt(node: &SyntaxNode, out: &mut Output) {
    for child in node.children() {
        if is_expression(child.kind()) {
            format_node(&child, out);
        }
    }

    write_semicolon_with_comments(node, out);
}

// =============================================================================
// Expressions
// =============================================================================

fn format_literal(node: &SyntaxNode, out: &mut Output) {
    // LITERAL wraps a single token (INTEGER, STRING, ADDRESS_LIT, KW_TRUE, etc.)
    // In rowan, the suffix is already included in the token text (e.g. "42u64")
    for elem in node.children_with_tokens() {
        if let SyntaxElement::Token(tok) = elem {
            let k = tok.kind();
            if k != WHITESPACE && k != LINEBREAK {
                out.write(tok.text());
            }
        }
    }
}

fn format_call(node: &SyntaxNode, out: &mut Output) {
    // CALL_EXPR: callee_node, L_PAREN, [args separated by COMMA], R_PAREN
    let col = out.current_column();
    let single_line = format_call_inline(node);

    if col + single_line.len() <= LINE_WIDTH {
        // Fits on one line — write the pre-formatted string directly
        out.write(&single_line);
    } else {
        // Wrap: write callee, then wrap args
        let elems = elements(node);
        let lparen_idx = find_token_index(&elems, L_PAREN).unwrap_or(elems.len());

        // Write callee (everything before L_PAREN)
        for elem in elems[..lparen_idx].iter() {
            match elem {
                SyntaxElement::Token(tok) => {
                    let k = tok.kind();
                    if k != WHITESPACE && k != LINEBREAK {
                        out.write(tok.text());
                    }
                }
                SyntaxElement::Node(n) => format_node(n, out),
            }
        }

        // Collect args (expression nodes after L_PAREN)
        let args: Vec<_> = elems[lparen_idx..]
            .iter()
            .filter_map(|e| {
                if let SyntaxElement::Node(n) = e {
                    if is_expression(n.kind()) { Some(n.clone()) } else { None }
                } else {
                    None
                }
            })
            .collect();

        out.write("(");
        out.newline();
        out.indented(|out| {
            for arg in &args {
                format_node(arg, out);
                out.write(",");
                out.newline();
            }
        });
        out.write(")");
    }
}

/// Format a call expression as a single-line string (no wrapping).
/// Used for measurement to avoid infinite recursion with format_node_to_string.
fn format_call_inline(node: &SyntaxNode) -> String {
    let mut out = Output::new();
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    L_PAREN => out.write("("),
                    R_PAREN => out.write(")"),
                    COMMA => {
                        out.write(",");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => format_node(&n, &mut out),
        }
    }
    out.into_raw()
}

fn format_binary(node: &SyntaxNode, out: &mut Output) {
    // BINARY_EXPR: lhs, operator_token, rhs (with trivia interleaved)
    let children: Vec<_> = node.children().collect();
    if children.len() == 2 {
        let op_token =
            node.children_with_tokens().find(|e| matches!(e, SyntaxElement::Token(t) if is_binary_op(t.kind())));

        format_node(&children[0], out);
        out.space();
        if let Some(SyntaxElement::Token(tok)) = op_token {
            out.write(tok.text());
        }
        out.space();
        format_node(&children[1], out);
    } else {
        // Malformed binary expression — emit children verbatim
        for child in &children {
            format_node(child, out);
        }
    }
}

fn format_path(node: &SyntaxNode, out: &mut Output) {
    // PATH_EXPR: IDENT, [COLON_COLON, IDENT]*, [COLON_COLON, CONST_ARG_LIST]
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => {
                if n.kind() == CONST_ARG_LIST {
                    format_const_argument_list(&n, out);
                } else {
                    format_node(&n, out);
                }
            }
        }
    }
}

fn format_unary(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_expression(n.kind()) => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_ternary(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children().filter(|c| is_expression(c.kind())).collect();
    if exprs.len() >= 3 {
        format_node(&exprs[0], out);
        out.space();
        out.write("?");
        out.space();
        format_node(&exprs[1], out);
        out.space();
        out.write(":");
        out.space();
        format_node(&exprs[2], out);
    }
}

fn format_field_expr(node: &SyntaxNode, out: &mut Output) {
    // FIELD_EXPR: base_node, DOT, IDENT|INTEGER|keyword
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => format_node(&n, out),
        }
    }
}

fn format_index_expr(node: &SyntaxNode, out: &mut Output) {
    // INDEX_EXPR: base_node, L_BRACKET, index_expr, R_BRACKET
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_expression(n.kind()) => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_cast(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_AS => {
                        out.space();
                        out.write("as");
                        out.space();
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if is_expression(k) {
                    format_node(&n, out);
                } else if is_type_node(k) {
                    format_type(&n, out);
                }
            }
        }
    }
}

fn format_array_expr(node: &SyntaxNode, out: &mut Output) {
    // ARRAY_EXPR handles both [a, b, c] and [x; n] (repeat)
    let has_semi = has_token(node, SEMICOLON);

    if has_semi {
        // Repeat expression: [value; count] — never wraps
        let exprs: Vec<_> = node.children().filter(|c| is_expression(c.kind())).collect();
        out.write("[");
        if exprs.len() >= 2 {
            format_node(&exprs[0], out);
            out.write(";");
            out.space();
            format_node(&exprs[1], out);
        }
        out.write("]");
    } else {
        // Array literal: [a, b, c]
        let exprs: Vec<_> = node.children().filter(|c| is_expression(c.kind())).collect();
        format_wrapping_list(out, "[", "]", &exprs, true);
    }
}

fn format_tuple_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children().filter(|c| is_expression(c.kind())).collect();

    if exprs.len() == 1 {
        // Single-element tuples need a trailing comma to distinguish from PAREN_EXPR.
        let item_string = format_node_to_string(&exprs[0]);
        let col = out.current_column();
        if fits_on_one_line(col, "(", ",)", &[item_string]) {
            out.write("(");
            format_node(&exprs[0], out);
            out.write(",)");
        } else {
            out.write("(");
            out.newline();
            out.indented(|out| {
                format_node(&exprs[0], out);
                out.write(",");
                out.newline();
            });
            out.write(")");
        }
    } else {
        format_wrapping_list(out, "(", ")", &exprs, true);
    }
}

fn format_parenthesized(node: &SyntaxNode, out: &mut Output) {
    out.write("(");
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK && k != L_PAREN && k != R_PAREN {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if is_expression(n.kind()) => format_node(&n, out),
            _ => {}
        }
    }
    out.write(")");
}

fn format_struct_expr(node: &SyntaxNode, out: &mut Output) {
    let inits: Vec<_> = node.children().filter(|c| c.kind() == STRUCT_FIELD_INIT).collect();
    let elems = elements(node);

    // Write everything before L_BRACE (struct path/name)
    let lbrace_idx = find_token_index(&elems, L_BRACE).unwrap_or(elems.len());
    for elem in elems[..lbrace_idx].iter() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => {
                if n.kind() == CONST_ARG_LIST {
                    format_const_argument_list(n, out);
                }
            }
        }
    }

    if inits.is_empty() {
        out.space();
        out.write("{}");
        return;
    }

    format_wrapping_list(out, " { ", " }", &inits, true);
}

fn format_struct_field_init(node: &SyntaxNode, out: &mut Output) {
    let has_colon = has_token(node, COLON);
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    COLON => {
                        out.write(":");
                        out.space();
                    }
                    IDENT => out.write(tok.text()),
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if has_colon && is_expression(n.kind()) => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_async_expr(node: &SyntaxNode, out: &mut Output) {
    out.write("async");
    out.space();
    for child in node.children() {
        if child.kind() == BLOCK {
            format_block(&child, out);
        }
    }
}

fn format_locator_expr(node: &SyntaxNode, out: &mut Output) {
    // LOCATOR_EXPR: IDENT.aleo/IDENT[::CONST_ARG_LIST]
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if k != WHITESPACE && k != LINEBREAK {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => {
                if n.kind() == CONST_ARG_LIST {
                    format_const_argument_list(&n, out);
                }
            }
        }
    }
}

// =============================================================================
// Patterns
// =============================================================================

fn format_ident_pattern(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        if let SyntaxElement::Token(tok) = elem {
            let k = tok.kind();
            if k != WHITESPACE && k != LINEBREAK {
                out.write(tok.text());
            }
        }
    }
}

fn format_tuple_pattern(node: &SyntaxNode, out: &mut Output) {
    let patterns: Vec<_> =
        node.children().filter(|c| matches!(c.kind(), IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN)).collect();

    out.write("(");
    for (i, pat) in patterns.iter().enumerate() {
        format_node(pat, out);
        if i < patterns.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write(")");
}

// =============================================================================
// Comment handling
// =============================================================================

/// Emit comments found among children_with_tokens, starting after `start_idx`.
///
/// Uses linebreaks to distinguish trailing comments (same line) from standalone
/// comments (own line). WHITESPACE and LINEBREAK tokens are skipped since the
/// formatter controls all spacing.
fn emit_comments_after(elems: &[SyntaxElement], start_idx: usize, out: &mut Output) {
    let mut saw_linebreak = false;
    for elem in elems.iter().skip(start_idx + 1) {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    saw_linebreak = true;
                }
                COMMENT_LINE => {
                    if saw_linebreak {
                        out.ensure_newline();
                    } else {
                        out.space();
                    }
                    out.write(tok.text().trim_end());
                    out.newline();
                    saw_linebreak = false;
                }
                COMMENT_BLOCK => {
                    if saw_linebreak {
                        out.ensure_newline();
                    } else {
                        out.space();
                    }
                    out.write(tok.text());
                    saw_linebreak = false;
                }
                WHITESPACE => {}
                _ => break,
            },
            SyntaxElement::Node(_) => break,
        }
    }
}

/// Emit leading comment tokens that appear before the first non-trivia token in a node.
/// In rowan, `skip_trivia()` in the parser causes leading trivia (including comments)
/// to be consumed into item nodes. We need to emit these before the structural content.
///
/// Comments that appear before any LINEBREAK in the leading trivia are "stolen"
/// trailing comments from the previous item (on the same line as the previous
/// item's closing token). These are skipped here and should be handled by the
/// previous item's formatter via `emit_stolen_trailing_comments`.
fn emit_leading_comments(node: &SyntaxNode, out: &mut Output) {
    let mut saw_linebreak = false;
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    saw_linebreak = true;
                }
                COMMENT_LINE if saw_linebreak => {
                    out.write(tok.text().trim_end());
                    out.newline();
                }
                COMMENT_BLOCK if saw_linebreak => {
                    out.write(tok.text());
                    out.newline();
                }
                // Inline comment before any linebreak — stolen trailing comment
                // from the previous item. Skip it here.
                COMMENT_LINE | COMMENT_BLOCK => {}
                WHITESPACE => {}
                _ => break, // Stop at first structural token
            },
            SyntaxElement::Node(_) => break, // Stop at first child node
        }
    }
}

/// Emit "stolen" trailing comments from a node's leading trivia.
///
/// The parser consumes trailing comments (e.g. `// foo` after a semicolon)
/// into the next item's node as leading trivia. This function detects such
/// comments (those appearing before any LINEBREAK) and emits them inline
/// so they stay on the previous item's line.
fn emit_stolen_trailing_comments(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => break,
                COMMENT_LINE => {
                    out.space();
                    out.write(tok.text().trim_end());
                }
                COMMENT_BLOCK => {
                    out.space();
                    out.write(tok.text());
                }
                WHITESPACE => {}
                _ => break,
            },
            SyntaxElement::Node(_) => break,
        }
    }
}

/// Emit inline comments that appear after a child node within its parent.
/// Scans sibling elements after `child` (past the COMMA) and emits any comments
/// found before the next structural element.
fn emit_inline_comments_after_param(parent: &SyntaxNode, child: &SyntaxNode, out: &mut Output) {
    let mut past_child = false;
    let mut past_comma = false;
    for elem in parent.children_with_tokens() {
        if !past_child {
            if let SyntaxElement::Node(n) = &elem
                && n.text_range() == child.text_range()
            {
                past_child = true;
            }
            continue;
        }
        if !past_comma {
            if matches!(&elem, SyntaxElement::Token(t) if t.kind() == COMMA) {
                past_comma = true;
            }
            continue;
        }
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                COMMENT_LINE => {
                    out.space();
                    out.write(tok.text().trim_end());
                    out.newline();
                }
                COMMENT_BLOCK => {
                    out.space();
                    out.write(tok.text());
                }
                WHITESPACE | LINEBREAK => {}
                _ => break,
            },
            SyntaxElement::Node(_) => break,
        }
    }
}

/// Write a semicolon and emit any trailing comments that follow it.
///
/// Also checks the next sibling node for "stolen" trailing comments that
/// the parser consumed into the following item's leading trivia.
fn write_semicolon_with_comments(node: &SyntaxNode, out: &mut Output) {
    out.write(";");
    let elems = elements(node);
    if let Some(idx) = find_token_index(&elems, SEMICOLON) {
        emit_comments_after(&elems, idx, out);
    }
    // Check next sibling for stolen trailing comments
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
    out.set_mark();
}

// =============================================================================
// Line-width measurement helpers
// =============================================================================

/// Format a comma-separated list of nodes, wrapping to multiple lines if needed.
///
/// Single-line: `{open}item1, item2{close}`
/// Multi-line (trailing spaces stripped from `open`, leading from `close`):
/// ```text
/// {open}
///     item1,
///     item2,
/// {close}
/// ```
fn format_wrapping_list(out: &mut Output, open: &str, close: &str, items: &[SyntaxNode], multi_trailing_comma: bool) {
    let item_strings: Vec<String> = items.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    if fits_on_one_line(col, open, close, &item_strings) {
        out.write(open);
        for (i, item) in items.iter().enumerate() {
            format_node(item, out);
            if i < items.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(close);
    } else {
        out.write(open.trim_end());
        out.newline();
        out.indented(|out| {
            for (i, item) in items.iter().enumerate() {
                format_node(item, out);
                if multi_trailing_comma || i < items.len() - 1 {
                    out.write(",");
                }
                out.newline();
            }
        });
        out.write(close.trim_start());
    }
}

/// Format a node into a temporary buffer and return the result as a string.
fn format_node_to_string(node: &SyntaxNode) -> String {
    let mut out = Output::new();
    format_node(node, &mut out);
    out.into_raw()
}

/// Check if a delimited list fits on one line.
///
/// Computes: `col + prefix.len() + joined_items(", ") + suffix.len() <= LINE_WIDTH`
fn fits_on_one_line(col: usize, prefix: &str, suffix: &str, items: &[String]) -> bool {
    let items_len: usize = if items.is_empty() {
        0
    } else {
        items.iter().map(|s| s.len()).sum::<usize>() + (items.len() - 1) * 2 // ", " between items
    };
    col + prefix.len() + items_len + suffix.len() <= LINE_WIDTH
}

// =============================================================================
// Helpers
// =============================================================================

fn is_statement(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        BLOCK
            | RETURN_STMT
            | LET_STMT
            | CONST_STMT
            | ASSIGN_STMT
            | IF_STMT
            | FOR_STMT
            | ASSERT_STMT
            | ASSERT_EQ_STMT
            | ASSERT_NEQ_STMT
            | EXPR_STMT
    )
}

fn is_program_item(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        FUNCTION_DEF
            | CONSTRUCTOR_DEF
            | STRUCT_DEF
            | RECORD_DEF
            | MAPPING_DEF
            | STORAGE_DEF
            | GLOBAL_CONST
            | ANNOTATION
    )
}

fn is_type_node(kind: SyntaxKind) -> bool {
    matches!(kind, TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_OPTIONAL | TYPE_FUTURE | TYPE_MAPPING)
}

fn is_expression(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        LITERAL
            | CALL_EXPR
            | BINARY_EXPR
            | PATH_EXPR
            | UNARY_EXPR
            | TERNARY_EXPR
            | FIELD_EXPR
            | INDEX_EXPR
            | CAST_EXPR
            | ARRAY_EXPR
            | TUPLE_EXPR
            | STRUCT_EXPR
            | PAREN_EXPR
            | ASYNC_EXPR
            | LOCATOR_EXPR
    )
}

fn is_assignment_op(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        EQ | PLUS_EQ
            | MINUS_EQ
            | STAR_EQ
            | SLASH_EQ
            | PERCENT_EQ
            | STAR2_EQ
            | AMP2_EQ
            | PIPE2_EQ
            | AMP_EQ
            | PIPE_EQ
            | CARET_EQ
            | SHL_EQ
            | SHR_EQ
    )
}

fn is_binary_op(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        PLUS | MINUS
            | STAR
            | SLASH
            | PERCENT
            | STAR2
            | EQ2
            | BANG_EQ
            | LT
            | LT_EQ
            | GT
            | GT_EQ
            | AMP2
            | PIPE2
            | AMP
            | PIPE
            | CARET
            | SHL
            | SHR
    )
}

/// Collect all children_with_tokens() as a Vec for indexed access.
fn elements(node: &SyntaxNode) -> Vec<SyntaxElement> {
    node.children_with_tokens().collect()
}

/// Check if a node has a child token of a given kind.
fn has_token(node: &SyntaxNode, kind: SyntaxKind) -> bool {
    node.children_with_tokens().any(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == kind))
}

/// Find the index of a token with a given kind.
fn find_token_index(elems: &[SyntaxElement], kind: SyntaxKind) -> Option<usize> {
    elems.iter().position(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == kind))
}

/// Find the last index of a token with a given kind.
fn find_last_token_index(elems: &[SyntaxElement], kind: SyntaxKind) -> Option<usize> {
    elems.iter().rposition(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == kind))
}

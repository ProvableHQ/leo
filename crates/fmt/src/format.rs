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
    SyntaxToken,
};

const MAX_INLINE_TUPLE_ITEMS: usize = 12;

struct ReturnTypeEntry {
    visibility: Vec<String>,
    ty: Option<SyntaxNode>,
}

/// Format any syntax node.
pub fn format_node(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        // Top-level
        ROOT => format_root(node, out),
        PROGRAM_DECL => format_program(node, out),

        // Declarations
        FUNCTION_DEF | FINAL_FN_DEF | CONSTRUCTOR_DEF => format_function(node, out),
        STRUCT_DEF | RECORD_DEF => format_composite(node, out),
        INTERFACE_DEF => format_interface(node, out),
        FN_PROTOTYPE_DEF => format_fn_prototype(node, out),
        RECORD_PROTOTYPE_DEF => format_record_prototype(node, out),
        IMPORT => format_import(node, out),
        MAPPING_DEF => format_mapping(node, out),
        STORAGE_DEF => format_storage(node, out),
        GLOBAL_CONST => format_global_const(node, out),
        ANNOTATION => format_annotation(node, out),
        PARAM_LIST => format_parameter_list(node, out),
        PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT => format_parameter(node, out),
        RETURN_TYPE => format_return_type(node, out),
        CONST_PARAM => format_const_parameter(node, out),
        CONST_PARAM_LIST => format_const_parameter_list(node, out),
        CONST_ARG_LIST => format_const_argument_list(node, out),
        STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT => {
            format_struct_member(node, out)
        }

        // Statements
        BLOCK => format_block(node, out),
        RETURN_STMT => format_return(node, out),
        LET_STMT => format_definition(node, out),
        CONST_STMT => format_const_stmt(node, out),
        ASSIGN_STMT | COMPOUND_ASSIGN_STMT => format_assign(node, out),
        IF_STMT => format_conditional(node, out),
        FOR_STMT | FOR_INCLUSIVE_STMT => format_iteration(node, out),
        ASSERT_STMT => format_assert(node, out),
        ASSERT_EQ_STMT => format_assert_pair(node, out, "assert_eq"),
        ASSERT_NEQ_STMT => format_assert_pair(node, out, "assert_neq"),
        EXPR_STMT => format_expr_stmt(node, out),

        // Expressions
        k if k.is_literal_node() => format_literal(node, out),
        CALL_EXPR => format_call(node, out),
        DYNAMIC_CALL_EXPR => format_dynamic_call(node, out),
        METHOD_CALL_EXPR => format_method_call(node, out),
        BINARY_EXPR => format_binary(node, out),
        PATH_EXPR | PATH_LOCATOR_EXPR | PROGRAM_REF_EXPR => format_path(node, out),
        SELF_EXPR => out.write("self"),
        BLOCK_KW_EXPR => out.write("block"),
        NETWORK_KW_EXPR => out.write("network"),
        UNARY_EXPR => format_unary(node, out),
        TERNARY_EXPR => format_ternary(node, out),
        FIELD_EXPR => format_field_expr(node, out),
        TUPLE_ACCESS_EXPR => format_tuple_access(node, out),
        INDEX_EXPR => format_index_expr(node, out),
        CAST_EXPR => format_cast(node, out),
        ARRAY_EXPR => format_array_expr(node, out),
        REPEAT_EXPR => format_repeat_expr(node, out),
        TUPLE_EXPR => format_tuple_expr(node, out),
        PAREN_EXPR => format_parenthesized(node, out),
        STRUCT_EXPR | STRUCT_LOCATOR_EXPR => format_struct_expr(node, out),
        STRUCT_FIELD_INIT => format_struct_field_init(node, out),
        STRUCT_FIELD_SHORTHAND => format_struct_field_shorthand(node, out),
        FINAL_EXPR => format_final_expr(node, out),

        // Patterns
        IDENT_PATTERN => format_ident_pattern(node, out),
        TUPLE_PATTERN => format_tuple_pattern(node, out),
        WILDCARD_PATTERN => out.write("_"),

        // Types
        k if k.is_type() => format_type(node, out),

        // Error nodes: emit the original text verbatim to avoid corrupting broken code.
        ERROR => {
            out.write(node.text().to_string().trim());
        }

        _other => {
            #[cfg(debug_assertions)]
            eprintln!("[leo-fmt] unhandled node kind: {_other:?}");
            // Emit verbatim to avoid silent content loss on newly introduced nodes.
            out.write(&node.text().to_string());
        }
    }
}

// =============================================================================
// Top-level
// =============================================================================

fn format_root(node: &SyntaxNode, out: &mut Output) {
    if node.children_with_tokens().any(|elem| {
        matches!(
            elem,
            SyntaxElement::Token(tok)
                if !matches!(tok.kind(), WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK)
        )
    }) {
        write_node_verbatim(node, out);
        return;
    }

    if node.children().any(|child| child.kind() == ERROR)
        && node.children().all(|child| matches!(child.kind(), ERROR | GLOBAL_CONST))
    {
        write_node_verbatim(node, out);
        return;
    }

    if node.children().any(|child| {
        !matches!(child.kind(), IMPORT | PROGRAM_DECL | ANNOTATION | ERROR) && !is_program_item(child.kind())
    }) {
        write_node_verbatim(node, out);
        return;
    }

    let mut prev_was_import = false;
    let mut prev_was_block_item = false;
    let mut prev_was_comment = false;
    let mut had_output = false;
    let mut linebreak_count: usize = 0;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    linebreak_count += 1;
                }
                COMMENT_LINE | COMMENT_BLOCK => {
                    let inline_trailing = had_output
                        && linebreak_count == 0
                        && !prev_was_import
                        && !prev_was_block_item
                        && !prev_was_comment;
                    if inline_trailing {
                        out.space();
                    } else {
                        if had_output {
                            out.ensure_newline();
                        }
                        // Blank line before comment if after imports, after a block item,
                        // or if the source had a blank line.
                        if prev_was_import || prev_was_block_item || (had_output && linebreak_count >= 2) {
                            out.newline();
                        }
                    }
                    let text = if tok.kind() == COMMENT_LINE { tok.text().trim_end() } else { tok.text() };
                    out.write(text);
                    out.newline();
                    had_output = true;
                    prev_was_import = false;
                    prev_was_block_item = false;
                    prev_was_comment = true;
                    linebreak_count = 0;
                }
                _ => {} // skip WHITESPACE, etc.
            },
            SyntaxElement::Node(n) => {
                let kind = n.kind();
                if kind == IMPORT {
                    format_import(&n, out);
                    prev_was_import = true;
                    prev_was_block_item = false;
                    prev_was_comment = false;
                    had_output = true;
                    linebreak_count = 0;
                } else if kind == PROGRAM_DECL {
                    // Always blank line before program if there was a previous item.
                    if prev_was_import || prev_was_block_item || (had_output && linebreak_count >= 2) {
                        out.newline();
                    }
                    format_program(&n, out);
                    prev_was_import = false;
                    prev_was_block_item = true;
                    prev_was_comment = false;
                    had_output = true;
                    linebreak_count = 0;
                } else if kind == ANNOTATION {
                    if prev_was_import || prev_was_block_item || (had_output && linebreak_count >= 2) {
                        out.newline();
                    }
                    format_annotation(&n, out);
                    prev_was_import = false;
                    // Don't set prev_was_block_item here; the annotated item will set it.
                    prev_was_comment = false;
                    had_output = true;
                    linebreak_count = 0;
                } else if is_program_item(kind) {
                    let is_block = is_block_item(kind);
                    let wants_blank = had_output
                        && (linebreak_count >= 2
                            || has_blank_line_in_leading_trivia(&n)
                            || (!prev_was_comment && (prev_was_import || is_block || prev_was_block_item)));
                    if wants_blank {
                        out.newline();
                    }
                    format_node(&n, out);
                    if let Some(next) = n.next_sibling()
                        && emit_stolen_trailing_comments(&next, out)
                    {
                        out.newline();
                    }
                    prev_was_import = false;
                    prev_was_block_item = is_block;
                    prev_was_comment = false;
                    had_output = true;
                    linebreak_count = 0;
                } else if kind == ERROR {
                    if should_inline_adjacent_error(&n) {
                        continue;
                    }
                    let text = n.text().to_string();
                    let text = text.trim();
                    if !text.is_empty() {
                        if had_output {
                            out.ensure_newline();
                        }
                        out.write(text);
                        out.newline();
                        prev_was_import = false;
                        prev_was_block_item = false;
                        prev_was_comment = false;
                        had_output = true;
                        linebreak_count = 0;
                    }
                }
            }
        }
    }
}

fn format_program(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);

    // Check if there are any items in the program body.
    let has_items = node.children().any(|c| is_program_item_non_annotation(c.kind()) || c.kind() == ERROR);

    // Write "program name.aleo {"
    out.write("program");
    out.space();

    // Write the program ID and optional parent list between KW_PROGRAM and L_BRACE.
    let prog_idx = find_token_index(&elems, KW_PROGRAM).unwrap_or(0);
    let lbrace_idx = find_token_index(&elems, L_BRACE).unwrap_or(elems.len());
    for elem in elems[prog_idx + 1..lbrace_idx].iter() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                COLON => {
                    out.write(":");
                    out.space();
                }
                WHITESPACE | LINEBREAK => {}
                _ => out.write(tok.text()),
            },
            SyntaxElement::Node(n) if n.kind() == PARENT_LIST => {
                format_parent_list(n, out);
            }
            _ => {}
        }
    }

    out.space();
    out.write("{");
    if has_items {
        if let Some(first_item) = node.children().find(|child| is_program_item(child.kind()) || child.kind() == ERROR) {
            emit_stolen_trailing_comments(&first_item, out);
        }
        out.newline();
    }

    // Iterate children_with_tokens to handle items and comments in order.
    // Block items (structs, records, functions, interfaces, constructors) always
    // get a blank line separating them from adjacent items. Inline items (consts,
    // mappings) preserve source spacing — only get a blank line if the source had one.
    let mut after_lbrace = false;
    let mut saw_linebreak = false;
    let mut linebreak_count: usize = 0;
    let mut had_item = false;
    let mut prev_was_block_item = false;
    let mut prev_was_comment = false;

    for elem in &elems {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                L_BRACE => {
                    after_lbrace = true;
                }
                R_BRACE => {}
                LINEBREAK => {
                    saw_linebreak = true;
                    linebreak_count += 1;
                }
                WHITESPACE => {}
                COMMENT_LINE | COMMENT_BLOCK if after_lbrace => {
                    if had_item && linebreak_count >= 2 {
                        out.insert_newline_at_mark();
                    }
                    let text = if tok.kind() == COMMENT_LINE { tok.text().trim_end() } else { tok.text() };
                    out.indented(|out| {
                        if saw_linebreak {
                            out.ensure_newline();
                        } else {
                            out.space();
                        }
                        out.write(text);
                    });
                    out.set_mark();
                    out.newline();
                    had_item = true;
                    prev_was_block_item = false;
                    prev_was_comment = true;
                    saw_linebreak = false;
                    linebreak_count = 0;
                }
                _ => {}
            },
            SyntaxElement::Node(n) if after_lbrace => {
                let kind = n.kind();
                let is_content = kind == ERROR || is_program_item_non_annotation(kind);
                if is_content {
                    let is_block = is_block_item(kind);
                    let wants_blank = had_item
                        && (linebreak_count >= 2
                            || has_blank_line_in_leading_trivia(n)
                            || (!prev_was_comment && (is_block || prev_was_block_item)));
                    if kind == ERROR {
                        if should_inline_adjacent_error(n) {
                            continue;
                        }
                        let text = n.text().to_string();
                        let text = text.trim();
                        if !text.is_empty() {
                            if wants_blank {
                                out.insert_newline_at_mark();
                            }
                            out.indented(|out| {
                                out.write(text);
                                out.newline();
                                out.set_mark();
                            });
                            had_item = true;
                            prev_was_block_item = false;
                            prev_was_comment = false;
                        }
                    } else {
                        if wants_blank {
                            out.insert_newline_at_mark();
                        }
                        out.indented(|out| format_node(n, out));
                        had_item = true;
                        prev_was_block_item = is_block;
                        prev_was_comment = false;
                    }
                    saw_linebreak = false;
                    linebreak_count = 0;
                }
            }
            _ => {}
        }
    }

    if has_token(node, R_BRACE) {
        out.write("}");
        out.newline();
    }
}

fn is_program_item_non_annotation(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        FUNCTION_DEF
            | FINAL_FN_DEF
            | CONSTRUCTOR_DEF
            | STRUCT_DEF
            | RECORD_DEF
            | MAPPING_DEF
            | STORAGE_DEF
            | GLOBAL_CONST
            | INTERFACE_DEF
    )
}

/// Block items have braces and span multiple lines — always separated by blank lines.
fn is_block_item(kind: SyntaxKind) -> bool {
    matches!(kind, FUNCTION_DEF | FINAL_FN_DEF | CONSTRUCTOR_DEF | STRUCT_DEF | RECORD_DEF | INTERFACE_DEF)
}

// =============================================================================
// Declarations
// =============================================================================

fn format_function(node: &SyntaxNode, out: &mut Output) {
    if write_node_with_inline_error_verbatim(node, out) {
        out.set_mark();
        out.ensure_newline();
        return;
    }

    // Emit leading comments (trivia that appears before the first keyword)
    emit_leading_comments(node, out);

    // Emit keywords: final, fn/script/constructor, name
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_FINAL | KW_FN | KW_SCRIPT => {
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
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT => {
                        out.write(tok.text());
                        out.space();
                    }
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                match k {
                    ANNOTATION => format_annotation(&n, out),
                    PARAM_LIST => {
                        // Compute the trailing suffix width (return type + " {")
                        // so parameter wrapping accounts for the full signature line.
                        let suffix_width = compute_fn_suffix_width(node);
                        format_parameter_list_with_suffix(&n, out, suffix_width);
                    }
                    CONST_PARAM_LIST => format_const_parameter_list(&n, out),
                    RETURN_TYPE => format_return_type(&n, out),
                    BLOCK => {
                        out.space();
                        format_block(&n, out);
                    }
                    k if k.is_type() => {
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
    emit_inline_error_suffix(node, out);
    out.set_mark();
    out.ensure_newline();
}

/// Compute the width of everything after the parameter list closing paren
/// in a function signature: " -> ReturnType" plus " {".
fn compute_fn_suffix_width(fn_node: &SyntaxNode) -> usize {
    let mut width = 0;
    // Check for return type (the ARROW token is separate from the type node)
    for child in fn_node.children() {
        if child.kind() == RETURN_TYPE {
            let ret_str = format_return_type_to_string(&child);
            // " -> " (arrow with spaces) + formatted return type
            width += " -> ".len() + ret_str.len();
            break;
        }
        if child.kind().is_type() {
            // Single return type without RETURN_TYPE wrapper
            let type_str = format_node_to_string(&child);
            width += " -> ".len() + type_str.len();
            break;
        }
    }
    // Account for " {" after the return type
    width += " {".len();
    width
}

fn format_annotation(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    let has_paren = has_token(node, L_PAREN);
    // A well-formed annotation has an IDENT or keyword name token after the `@`.
    // If the name is missing (error-recovery node), omit the trailing newline so
    // that a following `@` remains directly adjacent.  This preserves the rowan
    // parse-tree structure: a newline between two `@` signs causes the parser to
    // take the trivia-recovery path, emitting a different error than when `@@`
    // appear on the same line.
    let has_name = node
        .children_with_tokens()
        .any(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == IDENT || t.kind().is_keyword()));

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
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => {
                        if has_paren || k == IDENT || k.is_keyword() {
                            out.write(tok.text());
                        }
                    }
                }
            }
            SyntaxElement::Node(n) if n.kind() == ANNOTATION_PAIR => {
                // Write annotation pair tokens: key = "value"
                for pair_elem in n.children_with_tokens() {
                    if let SyntaxElement::Token(t) = pair_elem {
                        let k = t.kind();
                        match k {
                            EQ => {
                                out.space();
                                out.write("=");
                                out.space();
                            }
                            WHITESPACE | LINEBREAK => {}
                            _ => out.write(t.text()),
                        }
                    }
                }
            }
            SyntaxElement::Node(_) => {}
        }
    }
    if has_name {
        out.newline();
    }
}

fn format_composite(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);
    let rbrace_idx = find_last_token_index(&elems, R_BRACE);

    // Emit leading comments
    emit_leading_comments(node, out);

    // Write keyword, name, and body
    let mut after_lbrace = false;
    let mut had_member = false;
    let mut idx = 0;
    while idx < elems.len() {
        match &elems[idx] {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_STRUCT | KW_RECORD => {
                        out.write(tok.text());
                        out.space();
                    }
                    IDENT if !after_lbrace => {
                        out.write(tok.text());
                        if !next_significant_is_colon_colon(&elems, idx) {
                            out.space();
                        }
                    }
                    COLON_COLON => {
                        out.write("::");
                    }
                    L_BRACE => {
                        if previous_significant_is_const_params(&elems, idx) {
                            out.space();
                        }
                        out.write("{");
                        idx += 1;
                        while idx < elems.len() {
                            match &elems[idx] {
                                SyntaxElement::Token(tok) => match tok.kind() {
                                    WHITESPACE => {
                                        idx += 1;
                                    }
                                    COMMENT_LINE => {
                                        out.space();
                                        out.write(tok.text().trim_end());
                                        idx += 1;
                                    }
                                    COMMENT_BLOCK => {
                                        out.space();
                                        out.write(tok.text());
                                        idx += 1;
                                    }
                                    LINEBREAK => {
                                        idx += 1;
                                        break;
                                    }
                                    _ => break,
                                },
                                _ => break,
                            }
                        }
                        if let Some(first_member) = node.children().find(|child| {
                            matches!(
                                child.kind(),
                                STRUCT_MEMBER
                                    | STRUCT_MEMBER_PUBLIC
                                    | STRUCT_MEMBER_PRIVATE
                                    | STRUCT_MEMBER_CONSTANT
                                    | ERROR
                            )
                        }) {
                            emit_stolen_trailing_comments(&first_member, out);
                        }
                        out.newline();
                        after_lbrace = true;
                        continue;
                    }
                    R_BRACE => {}
                    COMMA => {}
                    COMMENT_LINE if after_lbrace && !had_member => {
                        out.indented(|out| {
                            out.write(tok.text().trim_end());
                        });
                        out.newline();
                    }
                    COMMENT_BLOCK if after_lbrace && !had_member => {
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
                    STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT => {
                        had_member = true;
                        out.indented(|out| {
                            format_struct_member(n, out);
                            out.write(",");
                            let trailing_comment_on_newline = emit_node_trailing_comments(n, out);
                            emit_trailing_list_comments(node, n, out);
                            if let Some(next) = n.next_sibling() {
                                emit_stolen_trailing_comments(&next, out);
                            }
                            if !trailing_comment_on_newline {
                                out.ensure_newline();
                            }
                        });
                    }
                    ERROR => {
                        had_member = true;
                        let text = n.text().to_string();
                        let text = text.trim();
                        if !text.is_empty() {
                            out.indented(|out| {
                                out.write(text);
                                out.newline();
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        idx += 1;
    }
    if has_token(node, R_BRACE) {
        out.write("}");
        // Emit comments after closing brace
        if let Some(idx) = rbrace_idx {
            emit_comments_after(&elems, idx, out);
        }
    }
    // Check next sibling for stolen trailing comments
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
    emit_inline_error_suffix(node, out);
    out.set_mark();
    out.ensure_newline();
}

fn format_interface(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);
    let rbrace_idx = find_last_token_index(&elems, R_BRACE);

    emit_leading_comments(node, out);

    let mut after_lbrace = false;
    for elem in &elems {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_INTERFACE => {
                        out.write("interface");
                        out.space();
                    }
                    IDENT if !after_lbrace => {
                        out.write(tok.text());
                    }
                    COLON if !after_lbrace => {
                        out.write(":");
                        out.space();
                    }
                    L_BRACE => {
                        out.space();
                        out.write("{");
                        if let Some(first_item) = node.children().find(|child| {
                            matches!(
                                child.kind(),
                                FN_PROTOTYPE_DEF | RECORD_PROTOTYPE_DEF | MAPPING_DEF | STORAGE_DEF | ERROR
                            )
                        }) {
                            emit_stolen_trailing_comments(&first_item, out);
                        }
                        out.newline();
                        after_lbrace = true;
                    }
                    R_BRACE => {}
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
                    _ if !after_lbrace => out.write(tok.text()),
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => match n.kind() {
                PARENT_LIST => format_parent_list(n, out),
                FN_PROTOTYPE_DEF | RECORD_PROTOTYPE_DEF => {
                    out.indented(|out| format_node(n, out));
                    out.newline();
                }
                MAPPING_DEF | STORAGE_DEF => {
                    out.indented(|out| format_node(n, out));
                    if n.next_sibling().is_some() {
                        out.newline();
                    }
                }
                ERROR => {
                    let text = n.text().to_string();
                    let text = text.trim();
                    if !text.is_empty() {
                        out.indented(|out| {
                            out.write(text);
                            out.newline();
                        });
                    }
                }
                _ => {}
            },
        }
    }

    if has_token(node, R_BRACE) {
        out.write("}");
        if let Some(idx) = rbrace_idx {
            emit_comments_after(&elems, idx, out);
        }
    }
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
    out.set_mark();
    out.ensure_newline();
}

fn format_fn_prototype(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                KW_FN => {
                    out.write("fn");
                    out.space();
                }
                IDENT => out.write(tok.text()),
                COLON_COLON => out.write("::"),
                ARROW => {
                    out.space();
                    out.write("->");
                    out.space();
                }
                KW_PUBLIC | KW_PRIVATE | KW_CONSTANT => {
                    out.write(tok.text());
                    out.space();
                }
                SEMICOLON => out.write(";"),
                WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                _ => out.write(tok.text()),
            },
            SyntaxElement::Node(n) => match n.kind() {
                CONST_PARAM_LIST => format_const_parameter_list(&n, out),
                PARAM_LIST => format_parameter_list(&n, out),
                RETURN_TYPE => format_return_type(&n, out),
                k if k.is_type() => format_type(&n, out),
                _ => {}
            },
        }
    }
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
}

fn format_record_prototype(node: &SyntaxNode, out: &mut Output) {
    if !has_token(node, SEMICOLON) {
        let elems = elements(node);
        let rbrace_idx = find_last_token_index(&elems, R_BRACE);

        emit_leading_comments(node, out);

        let mut after_lbrace = false;
        let mut had_member = false;
        let mut idx = 0;
        while idx < elems.len() {
            match &elems[idx] {
                SyntaxElement::Token(tok) => {
                    let k = tok.kind();
                    match k {
                        KW_RECORD => {
                            out.write("record");
                            out.space();
                        }
                        IDENT if !after_lbrace => {
                            out.write(tok.text());
                            out.space();
                        }
                        L_BRACE => {
                            out.write("{");
                            out.newline();
                            after_lbrace = true;
                        }
                        DOT_DOT if after_lbrace => {
                            out.indented(|out| {
                                out.write("..");
                                out.newline();
                            });
                        }
                        R_BRACE => {}
                        COMMENT_LINE if after_lbrace && !had_member => {
                            out.indented(|out| {
                                out.write(tok.text().trim_end());
                            });
                            out.newline();
                        }
                        COMMENT_BLOCK if after_lbrace && !had_member => {
                            out.indented(|out| {
                                out.write(tok.text());
                            });
                            out.newline();
                        }
                        WHITESPACE | LINEBREAK | COMMA => {}
                        _ => out.write(tok.text()),
                    }
                }
                SyntaxElement::Node(n) => match n.kind() {
                    STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT => {
                        had_member = true;
                        out.indented(|out| {
                            format_struct_member(n, out);
                            out.write(",");
                            let trailing_comment_on_newline = emit_node_trailing_comments(n, out);
                            emit_trailing_list_comments(node, n, out);
                            if let Some(next) = n.next_sibling() {
                                emit_stolen_trailing_comments(&next, out);
                            }
                            if !trailing_comment_on_newline {
                                out.ensure_newline();
                            }
                        });
                    }
                    ERROR => {
                        let text = n.text().to_string();
                        let text = text.trim();
                        if !text.is_empty() {
                            out.indented(|out| {
                                out.write(text);
                                out.newline();
                            });
                        }
                    }
                    _ => {}
                },
            }
            idx += 1;
        }

        if has_token(node, R_BRACE) {
            out.write("}");
            if let Some(idx) = rbrace_idx {
                emit_comments_after(&elems, idx, out);
            }
        }
        if let Some(next) = node.next_sibling() {
            emit_stolen_trailing_comments(&next, out);
        }
        return;
    }

    emit_leading_comments(node, out);
    for elem in node.children_with_tokens() {
        if let SyntaxElement::Token(tok) = elem {
            match tok.kind() {
                KW_RECORD => {
                    out.write("record");
                    out.space();
                }
                IDENT => out.write(tok.text()),
                SEMICOLON => out.write(";"),
                WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                _ => out.write(tok.text()),
            }
        }
    }
    if let Some(next) = node.next_sibling() {
        emit_stolen_trailing_comments(&next, out);
    }
}

fn format_parent_list(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                PLUS => {
                    out.space();
                    out.write("+");
                    out.space();
                }
                WHITESPACE | LINEBREAK => {}
                _ => out.write(tok.text()),
            },
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_struct_member(node: &SyntaxNode, out: &mut Output) {
    let ident_count = node
        .children_with_tokens()
        .filter(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == IDENT))
        .count();
    let type_count = node.children().filter(|child| child.kind().is_type()).count();

    if has_error_descendant(node) || ident_count != 1 || type_count != 1 {
        write_node_verbatim(node, out);
        return;
    }

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
            SyntaxElement::Node(n) if n.kind().is_type() => {
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
    if write_node_with_inline_error_verbatim(node, out) {
        out.ensure_newline();
        return;
    }

    let ident_count = node
        .children_with_tokens()
        .filter(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == IDENT))
        .count();
    let type_count = node.children().filter(|child| child.kind().is_type()).count();
    let colon_count = node
        .children_with_tokens()
        .filter(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == COLON))
        .count();
    let fat_arrow_count = node
        .children_with_tokens()
        .filter(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == FAT_ARROW))
        .count();
    let has_nested_mapping_syntax = node.children().filter(|child| child.kind().is_type()).any(|child| {
        let text = child.text().to_string();
        text.contains("=>") || text.contains("->")
    });
    let has_unsupported_tokens = node.children_with_tokens().any(|elem| match elem {
        SyntaxElement::Token(tok) => !matches!(
            tok.kind(),
            KW_MAPPING | IDENT | COLON | FAT_ARROW | SEMICOLON | WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK
        ),
        SyntaxElement::Node(n) => !n.kind().is_type() && n.kind() != ERROR,
    });

    if has_error_descendant(node)
        || ident_count != 1
        || type_count != 2
        || colon_count != 1
        || fat_arrow_count != 1
        || has_nested_mapping_syntax
        || has_unsupported_tokens
    {
        write_node_verbatim(node, out);
        out.ensure_newline();
        return;
    }

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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
    if !has_same_line_parent_comment_after_node(node) {
        out.ensure_newline();
    }
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
    if !has_same_line_parent_comment_after_node(node) {
        out.ensure_newline();
    }
}

fn format_global_const(node: &SyntaxNode, out: &mut Output) {
    if write_node_with_inline_error_verbatim(node, out) {
        out.ensure_newline();
        return;
    }

    if has_error_descendant(node) {
        write_node_verbatim(node, out);
        out.ensure_newline();
        return;
    }

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
                if k.is_type() {
                    format_type(&n, out);
                } else if k.is_expression() || matches!(k, IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN) {
                    format_node(&n, out);
                } else if k == ERROR {
                    write_node_verbatim(&n, out);
                }
            }
        }
    }
    if !has_same_line_parent_comment_after_node(node) {
        out.ensure_newline();
    }
}

// =============================================================================
// Parameters and return types
// =============================================================================

fn format_parameter_list(node: &SyntaxNode, out: &mut Output) {
    format_parameter_list_with_suffix(node, out, 0);
}

fn format_parameter_list_with_suffix(node: &SyntaxNode, out: &mut Output, suffix_width: usize) {
    let has_direct_non_list_tokens = node.children_with_tokens().any(|elem| match elem {
        SyntaxElement::Token(tok) => {
            !matches!(tok.kind(), L_PAREN | R_PAREN | COMMA | WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK)
        }
        SyntaxElement::Node(n) => !matches!(n.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT | ERROR),
    });

    if has_error_descendant(node) || has_direct_non_list_tokens {
        write_node_verbatim(node, out);
        return;
    }

    let params: Vec<_> = node
        .children()
        .filter(|c| matches!(c.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT | ERROR))
        .collect();

    if params.is_empty() {
        out.write("()");
        return;
    }

    let param_strings: Vec<String> = params.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    // Force multi-line when comments exist anywhere in parameter list trivia.
    // This avoids line-comment swallowing in single-line mode and keeps
    // comments attached to the intended parameter in multi-line mode.
    let has_comments = node
        .children_with_tokens()
        .any(|elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)))
        || params.iter().any(has_comment_token);

    if !has_comments && fits_on_one_line_with_suffix(col, "(", ")", &param_strings, suffix_width) {
        // Single-line: (param1, param2)
        out.write("(");
        for (i, param) in params.iter().enumerate() {
            if matches!(param.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT) {
                format_parameter(param, out);
            } else {
                format_node(param, out);
            }
            if i < params.len() - 1 {
                out.write(",");
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
                emit_leading_comments_inner(param, out, i == 0);
                if matches!(param.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT) {
                    format_parameter(param, out);
                } else {
                    format_node(param, out);
                }
                out.write(",");
                emit_node_trailing_comments(param, out);
                if i < params.len() - 1 {
                    emit_comments_after_list_item(node, param, out);
                    emit_stolen_trailing_comments(&params[i + 1], out);
                    if out.current_column() != out.current_indent_width() {
                        out.ensure_newline();
                    }
                }
            }
            emit_trailing_list_comments(node, params.last().unwrap(), out);
            if out.current_column() != out.current_indent_width() {
                out.ensure_newline();
            }
        });
        out.write(")");
    }
}

fn format_parameter(node: &SyntaxNode, out: &mut Output) {
    let ident_count = node
        .children_with_tokens()
        .filter(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == IDENT))
        .count();
    let has_mut_ident = node
        .children_with_tokens()
        .any(|elem| matches!(elem, SyntaxElement::Token(tok) if tok.kind() == IDENT && tok.text() == "mut"));

    if has_error_descendant(node) || ident_count != 1 || has_mut_ident {
        write_node_verbatim(node, out);
        return;
    }

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
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            SyntaxElement::Node(n) if n.kind() == ERROR => write_node_verbatim(&n, out),
            _ => {}
        }
    }
}

fn format_return_type(node: &SyntaxNode, out: &mut Output) {
    if has_deep_comment(node) {
        write_node_verbatim(node, out);
        return;
    }

    if has_token(node, L_PAREN) {
        let entries = collect_return_type_entries(node);
        let entry_strings: Vec<_> = entries.iter().map(format_return_type_entry_to_string).collect();
        let should_wrap = entries.len() > MAX_INLINE_TUPLE_ITEMS
            || entry_strings.iter().any(|entry| entry.contains('\n'))
            || !fits_on_one_line(out.current_column(), "(", ")", &entry_strings);

        if !should_wrap {
            out.write("(");
            for (i, entry) in entries.iter().enumerate() {
                format_return_type_entry(entry, out);
                if i < entries.len() - 1 {
                    out.write(",");
                    out.space();
                }
            }
            out.write(")");
            return;
        }

        format_wrapping_string_list_with_dense_simple_items(out, "(", ")", &entry_strings, true);
        return;
    }

    let formatted = format_return_type_to_string(node);
    if !formatted.contains('\n') && out.current_column() + formatted.len() <= LINE_WIDTH {
        out.write(&formatted);
        return;
    }

    if let Some(ty) = node.children().find(|child| child.kind().is_type()) {
        format_type(&ty, out);
    } else {
        out.write(&formatted);
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
}

/// Collect the entries of a tuple return type.
fn collect_return_type_entries(node: &SyntaxNode) -> Vec<ReturnTypeEntry> {
    let mut entries = Vec::new();
    let mut current_visibility = Vec::new();
    let mut current_type = None;
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
                        if current_type.is_some() || !current_visibility.is_empty() {
                            entries.push(ReturnTypeEntry { visibility: current_visibility, ty: current_type });
                        }
                        break;
                    }
                    COMMA if in_tuple => {
                        if current_type.is_some() || !current_visibility.is_empty() {
                            entries.push(ReturnTypeEntry { visibility: current_visibility, ty: current_type });
                            current_visibility = Vec::new();
                            current_type = None;
                        }
                    }
                    KW_PUBLIC | KW_PRIVATE | KW_CONSTANT if in_tuple => {
                        current_visibility.push(tok.text().to_string());
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) if n.kind().is_type() && in_tuple => current_type = Some(n),
            _ => {}
        }
    }

    entries
}

fn format_return_type_entry(entry: &ReturnTypeEntry, out: &mut Output) {
    for visibility in &entry.visibility {
        out.write(visibility);
        out.space();
    }

    if let Some(ty) = &entry.ty {
        format_type(ty, out);
    }
}

fn format_return_type_entry_to_string(entry: &ReturnTypeEntry) -> String {
    let mut out = Output::new();
    format_return_type_entry(entry, &mut out);
    out.into_raw()
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_const_parameter_list(node: &SyntaxNode, out: &mut Output) {
    let params: Vec<_> = node.children().filter(|c| c.kind() == CONST_PARAM).collect();
    format_wrapping_list(node, out, "[", "]", &params, false, false);
}

fn format_const_argument_list(node: &SyntaxNode, out: &mut Output) {
    if has_token(node, LT) {
        let args: Vec<_> = node
            .children_with_tokens()
            .filter_map(|elem| match elem {
                SyntaxElement::Token(tok)
                    if !matches!(
                        tok.kind(),
                        LT | GT | COMMA | WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK
                    ) =>
                {
                    Some(tok.text().to_string())
                }
                SyntaxElement::Node(n)
                    if n.kind().is_type() || n.kind().is_expression() || n.kind() == DYNAMIC_CALL_RETURN_TYPE =>
                {
                    Some(format_node_to_string(&n))
                }
                _ => None,
            })
            .collect();

        out.write("<");
        for (i, arg) in args.iter().enumerate() {
            out.write(arg);
            if i < args.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(">");
        return;
    }

    let args: Vec<_> = node
        .children()
        .filter(|c| c.kind().is_type() || c.kind().is_expression() || c.kind() == DYNAMIC_CALL_RETURN_TYPE)
        .collect();
    format_wrapping_list(node, out, "[", "]", &args, false, false);
}

// =============================================================================
// Types
// =============================================================================

fn format_type(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        TYPE_PRIMITIVE | TYPE_DYN_RECORD => format_type_primitive(node, out),
        TYPE_LOCATOR => format_type_locator(node, out),
        TYPE_PATH => format_type_path(node, out),
        TYPE_ARRAY => format_type_array(node, out),
        TYPE_VECTOR => format_type_vector(node, out),
        TYPE_TUPLE => format_type_tuple(node, out),
        TYPE_FINAL => format_type_final(node, out),
        TYPE_MAPPING => format_type_mapping(node, out),
        TYPE_OPTIONAL => format_type_optional(node, out),
        DYNAMIC_CALL_RETURN_TYPE => format_dynamic_call_return_type(node, out),
        _ => {}
    }
}

/// Formats a `DYNAMIC_CALL_RETURN_TYPE` node: visibility keyword + type (e.g. `public u64`).
fn format_dynamic_call_return_type(node: &SyntaxNode, out: &mut Output) {
    let mut first = true;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) if !tok.kind().is_trivia() => {
                if !first {
                    out.space();
                }
                out.write(tok.text());
                first = false;
            }
            SyntaxElement::Node(n) => {
                if !first {
                    out.space();
                }
                format_node(&n, out);
                first = false;
            }
            _ => {}
        }
    }
}

fn format_type_primitive(node: &SyntaxNode, out: &mut Output) {
    let mut first = true;
    for tok in node.children_with_tokens().filter_map(|e| e.into_token()).filter(|t| !t.kind().is_trivia()) {
        if !first {
            out.space();
        }
        out.write(tok.text());
        first = false;
    }
}

fn format_type_locator(node: &SyntaxNode, out: &mut Output) {
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
    if has_error_descendant(node) || has_token(node, COMMA) {
        write_node_verbatim(node, out);
        return;
    }

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
                if n.kind().is_type() {
                    format_type(&n, out);
                } else if n.kind() == ARRAY_LENGTH {
                    for child in n.children_with_tokens() {
                        match child {
                            SyntaxElement::Node(inner) if inner.kind().is_expression() => format_node(&inner, out),
                            SyntaxElement::Token(tok) if !tok.kind().is_trivia() => out.write(tok.text()),
                            _ => {}
                        }
                    }
                } else if n.kind().is_expression() {
                    format_node(&n, out);
                }
            }
        }
    }
    out.write("]");
}

fn format_type_vector(node: &SyntaxNode, out: &mut Output) {
    out.write("[");
    if let Some(elem_type) = node.children().find(|c| c.kind().is_type()) {
        format_type(&elem_type, out);
    }
    out.write("]");
}

fn format_type_tuple(node: &SyntaxNode, out: &mut Output) {
    let types: Vec<_> = node.children().filter(|c| c.kind().is_type()).collect();
    let type_strings: Vec<_> = types.iter().map(format_node_to_string).collect();

    if types.is_empty() {
        out.write("()");
        return;
    }

    let should_wrap = types.len() > MAX_INLINE_TUPLE_ITEMS
        || type_strings.iter().any(|ty| ty.contains('\n'))
        || !fits_on_one_line(out.current_column(), "(", ")", &type_strings);

    if !should_wrap {
        out.write("(");
        for (i, ty) in types.iter().enumerate() {
            format_type(ty, out);
            if i < types.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(")");
        return;
    }

    format_wrapping_string_list_with_dense_simple_items(out, "(", ")", &type_strings, true);
}

fn format_type_final(node: &SyntaxNode, out: &mut Output) {
    if !has_token(node, LT) {
        out.write("Final");
        return;
    }

    let (params, return_type) = collect_final_signature(node);
    let param_strings: Vec<_> = params.iter().map(format_node_to_string).collect();
    let return_string = return_type.as_ref().map(format_node_to_string);
    let inline_param_len = if param_strings.is_empty() {
        0
    } else {
        param_strings.iter().map(String::len).sum::<usize>() + (param_strings.len() - 1) * 2
    };
    let inline_return_len = return_string.as_ref().map_or(0, |ret| " -> ".len() + ret.len());
    let inline_len = "Final<Fn(".len() + inline_param_len + ")".len() + inline_return_len + ">".len();
    let should_wrap = params.len() > MAX_INLINE_TUPLE_ITEMS
        || param_strings.iter().any(|param| param.contains('\n'))
        || return_string.as_ref().is_some_and(|ret| ret.contains('\n'))
        || out.current_column() + inline_len > LINE_WIDTH;

    if !should_wrap {
        out.write("Final<Fn(");
        for (i, param) in params.iter().enumerate() {
            format_type(param, out);
            if i < params.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(")");
        if let Some(ret) = &return_type {
            out.space();
            out.write("->");
            out.space();
            format_type(ret, out);
        }
        out.write(">");
        return;
    }

    out.write("Final<Fn(");
    if params.is_empty() {
        out.write(")");
    } else {
        out.newline();
        out.indented(|out| {
            for param in &params {
                format_type(param, out);
                out.write(",");
                out.newline();
            }
        });
        out.write(")");
    }

    if let Some(ret) = &return_type {
        let return_fits_inline = return_string
            .as_ref()
            .is_some_and(|ret| !ret.contains('\n') && out.current_column() + " -> ".len() + ret.len() <= LINE_WIDTH);
        if return_fits_inline {
            out.space();
            out.write("->");
            out.space();
            format_type(ret, out);
        } else {
            out.space();
            out.write("->");
            out.newline();
            out.indented(|out| format_type(ret, out));
        }
    }

    out.write(">");
}

fn collect_final_signature(node: &SyntaxNode) -> (Vec<SyntaxNode>, Option<SyntaxNode>) {
    let mut params = Vec::new();
    let mut return_type = None;
    let mut in_params = false;
    let mut saw_arrow = false;

    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                L_PAREN => in_params = true,
                R_PAREN => in_params = false,
                ARROW => saw_arrow = true,
                _ => {}
            },
            SyntaxElement::Node(n) if n.kind().is_type() => {
                if in_params {
                    params.push(n);
                } else if saw_arrow {
                    return_type = Some(n);
                    saw_arrow = false;
                }
            }
            _ => {}
        }
    }

    (params, return_type)
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
}

fn node_is_canonically_formatted(node: &SyntaxNode) -> bool {
    node.text().to_string().trim() == format_node_to_string(node)
}

// =============================================================================
// Statements
// =============================================================================

fn format_block(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);
    let lbrace_idx = find_token_index(&elems, L_BRACE).unwrap_or(0);

    // Check if block has any statements or comments (content worth indenting)
    let has_content = elems.iter().any(|e| match e {
        SyntaxElement::Node(n) => n.kind().is_statement() || n.kind() == ERROR,
        SyntaxElement::Token(t) => matches!(t.kind(), COMMENT_LINE | COMMENT_BLOCK),
    });

    out.write("{");
    if has_content {
        let mut start_idx = lbrace_idx + 1;
        while start_idx < elems.len() {
            match &elems[start_idx] {
                SyntaxElement::Token(tok) => match tok.kind() {
                    WHITESPACE => {
                        start_idx += 1;
                    }
                    COMMENT_LINE => {
                        out.space();
                        out.write(tok.text().trim_end());
                        start_idx += 1;
                    }
                    COMMENT_BLOCK => {
                        out.space();
                        out.write(tok.text());
                        start_idx += 1;
                    }
                    LINEBREAK => {
                        start_idx += 1;
                        break;
                    }
                    _ => break,
                },
                _ => break,
            }
        }

        if let Some(first_item) = elems[start_idx..].iter().find_map(|elem| match elem {
            SyntaxElement::Node(n) if n.kind().is_statement() || n.kind() == ERROR => Some(n.clone()),
            _ => None,
        }) {
            emit_stolen_trailing_comments(&first_item, out);
        }

        out.newline();
        out.indented(|out| {
            // Iterate all children_with_tokens to emit statements and comments.
            // Comments appear as sibling tokens in rowan. We use LINEBREAK to
            // determine if a comment is trailing (same line as previous stmt)
            // or standalone (own line).
            let mut after_lbrace = true;
            let mut had_entry = false;
            let mut previous_statement_canonical = false;
            let mut linebreak_count: usize = 0;

            for elem in &elems[start_idx..] {
                match elem {
                    SyntaxElement::Token(tok) => match tok.kind() {
                        L_BRACE => {
                            after_lbrace = true;
                        }
                        R_BRACE => {}
                        LINEBREAK => {
                            linebreak_count += 1;
                        }
                        WHITESPACE => {}
                        COMMENT_LINE if after_lbrace => {
                            if had_entry && linebreak_count >= 2 && previous_statement_canonical {
                                out.ensure_newline();
                                out.newline();
                            } else if linebreak_count > 0 {
                                out.ensure_newline();
                            } else {
                                out.space();
                            }
                            out.write(tok.text().trim_end());
                            out.newline();
                            had_entry = true;
                            previous_statement_canonical = false;
                            linebreak_count = 0;
                        }
                        COMMENT_BLOCK if after_lbrace => {
                            if had_entry && linebreak_count >= 2 && previous_statement_canonical {
                                out.ensure_newline();
                                out.newline();
                            } else if linebreak_count > 0 {
                                out.ensure_newline();
                            } else {
                                out.space();
                            }
                            out.write(tok.text());
                            had_entry = true;
                            previous_statement_canonical = false;
                            linebreak_count = 0;
                        }
                        _ => {}
                    },
                    SyntaxElement::Node(n) if after_lbrace && n.kind().is_statement() => {
                        let current_statement_canonical = node_is_canonically_formatted(n);
                        if had_entry {
                            out.ensure_newline();
                            if linebreak_count >= 2 && previous_statement_canonical && current_statement_canonical {
                                out.newline();
                            }
                        }
                        format_node(n, out);
                        emit_node_trailing_comments(n, out);
                        if let Some(next) = n.next_sibling() {
                            emit_stolen_trailing_comments(&next, out);
                        }
                        had_entry = true;
                        previous_statement_canonical = current_statement_canonical;
                        linebreak_count = 0;
                    }
                    SyntaxElement::Node(n) if after_lbrace && n.kind() == ERROR => {
                        if should_inline_adjacent_error(n) {
                            continue;
                        }
                        let text = n.text().to_string();
                        let text = text.trim();
                        if !text.is_empty() {
                            if had_entry {
                                out.ensure_newline();
                            }
                            out.write(text);
                        }
                        had_entry = true;
                        previous_statement_canonical = false;
                        linebreak_count = 0;
                    }
                    _ => {}
                }
            }
            out.ensure_newline();
        });
    }
    if has_token(node, R_BRACE) {
        out.write("}");
        out.set_mark();
        // Emit comments after closing brace (at parent level)
        if let Some(idx) = find_last_token_index(&elems, R_BRACE) {
            emit_comments_after(&elems, idx, out);
        }
    }
}

fn format_return(node: &SyntaxNode, out: &mut Output) {
    emit_leading_comments(node, out);
    out.write("return");

    for child in node.children() {
        if child.kind().is_expression() {
            out.space();
            format_node(&child, out);
        }
    }

    write_semicolon_with_comments(node, out);
}

fn format_definition(node: &SyntaxNode, out: &mut Output) {
    if has_error_descendant(node)
        || node.children_with_tokens().any(|elem| match elem {
            SyntaxElement::Token(tok) => {
                !matches!(tok.kind(), KW_LET | WHITESPACE | LINEBREAK | COLON | EQ | SEMICOLON)
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                !(k.is_type() || k.is_expression() || matches!(k, IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN))
            }
        })
    {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
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
                if k.is_type() {
                    format_type(&n, out);
                } else if k.is_expression() || matches!(k, IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN) {
                    format_node(&n, out);
                }
            }
        }
    }
}

fn format_const_stmt(node: &SyntaxNode, out: &mut Output) {
    if has_error_descendant(node) {
        write_node_verbatim(node, out);
        return;
    }

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
                    WHITESPACE | LINEBREAK => {}
                    _ => {}
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if k.is_type() {
                    format_type(&n, out);
                } else if k.is_expression() || matches!(k, IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN) {
                    format_node(&n, out);
                } else if k == ERROR {
                    write_node_verbatim(&n, out);
                }
            }
        }
    }
}

fn format_assign(node: &SyntaxNode, out: &mut Output) {
    if has_error_descendant(node) {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
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
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_conditional(node: &SyntaxNode, out: &mut Output) {
    if has_error_descendant(node) {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
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
                if k.is_expression() {
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
    if has_error_descendant(node) {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
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
                    DOT_DOT_EQ => {
                        out.write("..=");
                    }
                    WHITESPACE | LINEBREAK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => {
                let k = n.kind();
                if k.is_type() {
                    format_type(&n, out);
                } else if k.is_expression() {
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
    if has_error_descendant(node) || node.children().filter(|child| child.kind().is_expression()).count() != 1 {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
    out.write("assert(");

    for child in node.children() {
        if child.kind().is_expression() {
            format_node(&child, out);
        }
    }

    out.write(")");
    write_semicolon_with_comments(node, out);
}

fn format_assert_pair(node: &SyntaxNode, out: &mut Output, keyword: &str) {
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();

    if has_error_descendant(node) || exprs.len() != 2 {
        write_node_verbatim(node, out);
        return;
    }
    let expr_strings: Vec<String> = exprs.iter().map(format_node_to_string).collect();

    emit_leading_comments(node, out);
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
    if has_error_descendant(node) || node.children().filter(|child| child.kind().is_expression()).count() != 1 {
        write_node_verbatim(node, out);
        return;
    }

    emit_leading_comments(node, out);
    for child in node.children() {
        if child.kind().is_expression() {
            format_node(&child, out);
        }
    }

    write_semicolon_with_comments(node, out);
}

// =============================================================================
// Expressions
// =============================================================================

fn format_literal(node: &SyntaxNode, out: &mut Output) {
    // Literal nodes wrap a single token (INTEGER, STRING, ADDRESS_LIT, KW_TRUE, etc.)
    // In rowan, the suffix is already included in the token text (e.g. "42u64")
    for elem in node.children_with_tokens() {
        if let SyntaxElement::Token(tok) = elem {
            let k = tok.kind();
            if !is_trivia(k) {
                out.write(tok.text());
            }
        }
    }
}

fn format_call(node: &SyntaxNode, out: &mut Output) {
    // CALL_EXPR: callee_node, L_PAREN, [args separated by COMMA], R_PAREN
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
                if n.kind().is_expression() { Some(n.clone()) } else { None }
            } else {
                None
            }
        })
        .collect();

    if args.is_empty() {
        out.write("()");
        return;
    }

    // Force multi-line whenever comments are present among call argument trivia.
    // Single-line rendering cannot safely preserve `//` comments.
    let has_comments = elems[lparen_idx..]
        .iter()
        .any(|elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)))
        || args.iter().any(has_deep_comment);
    let arg_strings: Vec<String> = args.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    if !has_comments && fits_on_one_line(col, "(", ")", &arg_strings) {
        out.write("(");
        for (i, arg) in args.iter().enumerate() {
            format_node(arg, out);
            if i < args.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(")");
        return;
    }

    out.write("(");
    out.newline();
    out.indented(|out| {
        for (i, arg) in args.iter().enumerate() {
            emit_leading_comments_inner(arg, out, i == 0);
            format_node(arg, out);
            out.write(",");
            emit_node_trailing_comments(arg, out);
            if i < args.len() - 1 {
                emit_comments_after_list_item(node, arg, out);
                emit_stolen_trailing_comments(&args[i + 1], out);
                if out.current_column() != out.current_indent_width() {
                    out.ensure_newline();
                }
            }
        }
        emit_trailing_list_comments(node, args.last().unwrap(), out);
        if out.current_column() != out.current_indent_width() {
            out.ensure_newline();
        }
    });
    out.write(")");
}

fn format_method_call(node: &SyntaxNode, out: &mut Output) {
    if try_format_postfix_chain(node, out) {
        return;
    }

    // METHOD_CALL_EXPR: receiver, DOT, method_name, L_PAREN, [args], R_PAREN
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
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) => format_node(&n, out),
        }
    }
}

fn format_dynamic_call(node: &SyntaxNode, out: &mut Output) {
    if has_deep_comment(node) {
        write_node_verbatim(node, out);
        return;
    }

    let Some(interface) = node.children().find(|child| child.kind().is_type()) else {
        write_node_verbatim(node, out);
        return;
    };

    // Find the last :: token — this is the separator between @(target) and function name.
    // (Earlier :: tokens may be part of the interface type path, e.g. `foo.aleo::Interface`.)
    let Some(sep_offset) = node
        .children_with_tokens()
        .filter_map(|elem| match elem {
            SyntaxElement::Token(tok) if tok.kind() == COLON_COLON => Some(tok.text_range().start()),
            _ => None,
        })
        .last()
    else {
        write_node_verbatim(node, out);
        return;
    };

    let exprs: Vec<_> = node.children().filter(|child| child.kind().is_expression()).collect();
    let (pre_sep, args): (Vec<_>, Vec<_>) =
        exprs.into_iter().partition(|child| child.text_range().start() < sep_offset);

    let Some(target) = pre_sep.first() else {
        write_node_verbatim(node, out);
        return;
    };

    let mut saw_sep = false;
    let Some(function_name) = node.children_with_tokens().find_map(|elem| match elem {
        SyntaxElement::Token(tok) if tok.kind() == COLON_COLON => {
            saw_sep = true;
            None
        }
        SyntaxElement::Token(tok) if saw_sep && tok.kind() == IDENT => Some(tok.text().to_string()),
        _ => None,
    }) else {
        write_node_verbatim(node, out);
        return;
    };

    format_node(&interface, out);
    out.write("@(");
    format_node(target, out);
    if let Some(network) = pre_sep.get(1) {
        out.write(", ");
        format_node(network, out);
    }
    out.write(")::");
    out.write(&function_name);

    if args.is_empty() {
        out.write("()");
        return;
    }

    let arg_strings: Vec<String> = args.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    if fits_on_one_line(col, "(", ")", &arg_strings) {
        out.write("(");
        for (i, arg) in args.iter().enumerate() {
            format_node(arg, out);
            if i < args.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(")");
        return;
    }

    out.write("(");
    out.newline();
    out.indented(|out| {
        for (i, arg) in args.iter().enumerate() {
            emit_leading_comments_inner(arg, out, i == 0);
            format_node(arg, out);
            out.write(",");
            emit_node_trailing_comments(arg, out);
            if i < args.len() - 1 {
                emit_comments_after_list_item(node, arg, out);
                emit_stolen_trailing_comments(&args[i + 1], out);
                if out.current_column() != out.current_indent_width() {
                    out.ensure_newline();
                }
            }
        }
        emit_trailing_list_comments(node, args.last().unwrap(), out);
        if out.current_column() != out.current_indent_width() {
            out.ensure_newline();
        }
    });
    out.write(")");
}

fn format_binary(node: &SyntaxNode, out: &mut Output) {
    // BINARY_EXPR: lhs, operator_token, rhs (with trivia interleaved)
    if has_internal_binary_comment(node) {
        // Flattening removes clause-attached comments, so keep the original
        // layout for comment-bearing binary expressions.
        write_node_verbatim(node, out);
        return;
    }

    let children: Vec<_> = node.children().collect();
    if children.len() != 2 {
        // Malformed binary expression — emit children verbatim.
        for child in &children {
            format_node(child, out);
        }
        return;
    }

    // Flatten the left-associative chain into operands and operators.
    let (operands, operators) = collect_binary_chain(node);
    let operand_strings: Vec<String> = operands.iter().map(format_node_to_string).collect();

    // Measure: "op1 + op2 ** op3 + ..."  — each operator contributes " op " (len + 2 spaces).
    let total: usize = operand_strings.iter().map(|s| s.len()).sum::<usize>()
        + operators.iter().map(|op| op.text().len() + 2).sum::<usize>();

    if out.current_column() + total <= LINE_WIDTH {
        // Fits on one line.
        out.write(&operand_strings[0]);
        for (op, s) in operators.iter().zip(&operand_strings[1..]) {
            out.space();
            out.write(op.text());
            out.space();
            out.write(s);
        }
    } else {
        // Wrap: first operand on current line, rest indented with leading operator.
        format_node(&operands[0], out);
        out.indented(|out| {
            for (op, operand) in operators.iter().zip(&operands[1..]) {
                out.newline();
                out.write(op.text());
                out.space();
                format_node(operand, out);
            }
        });
    }
}

/// Flatten a left-associative binary expression chain into its operands and operators.
fn collect_binary_chain(node: &SyntaxNode) -> (Vec<SyntaxNode>, Vec<SyntaxToken>) {
    let mut operands = Vec::new();
    let mut operators = Vec::new();
    let mut current = node.clone();

    loop {
        let children: Vec<_> = current.children().collect();
        if children.len() != 2 {
            operands.push(current);
            break;
        }

        if let Some(SyntaxElement::Token(tok)) =
            current.children_with_tokens().find(|e| matches!(e, SyntaxElement::Token(t) if is_binary_op(t.kind())))
        {
            operators.push(tok);
        }

        operands.push(children[1].clone());

        if children[0].kind() == BINARY_EXPR {
            current = children[0].clone();
        } else {
            operands.push(children[0].clone());
            break;
        }
    }

    operands.reverse();
    operators.reverse();
    (operands, operators)
}

fn format_path(node: &SyntaxNode, out: &mut Output) {
    // PATH_EXPR: IDENT, [COLON_COLON, IDENT]*, [COLON_COLON, CONST_ARG_LIST]
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !is_trivia(k) {
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
                if !is_trivia(k) {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_ternary(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();
    if exprs.len() >= 3 {
        if has_deep_comment(node) {
            format_node(&exprs[0], out);
            out.newline();
            out.indented(|out| {
                out.write("?");
                out.space();
                format_node(&exprs[1], out);
                emit_node_trailing_comments(&exprs[1], out);
                emit_comments_after_child_until_token(node, &exprs[1], COLON, out);
                if out.current_column() != out.current_indent_width() {
                    out.newline();
                }
                out.write(":");
                out.space();
                format_node(&exprs[2], out);
                emit_node_trailing_comments(&exprs[2], out);
            });
        } else {
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
}

fn format_field_expr(node: &SyntaxNode, out: &mut Output) {
    if try_format_postfix_chain(node, out) {
        return;
    }

    // FIELD_EXPR: base_node, DOT, IDENT|keyword
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !is_trivia(k) {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => format_node(&n, out),
        }
    }
}

fn format_tuple_access(node: &SyntaxNode, out: &mut Output) {
    if try_format_postfix_chain(node, out) {
        return;
    }

    // TUPLE_ACCESS_EXPR: base_node, DOT, INTEGER
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !is_trivia(k) {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) => format_node(&n, out),
        }
    }
}

fn format_index_expr(node: &SyntaxNode, out: &mut Output) {
    if try_format_postfix_chain(node, out) {
        return;
    }

    // INDEX_EXPR: base_node, L_BRACKET, index_expr, R_BRACKET
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !is_trivia(k) {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
            _ => {}
        }
    }
}

fn try_format_postfix_chain(node: &SyntaxNode, out: &mut Output) -> bool {
    if !is_postfix_chain_root(node) || has_deep_comment(node) {
        return false;
    }

    let Some((base, segments)) = collect_postfix_chain(node) else {
        return false;
    };
    if segments.is_empty() {
        return false;
    }

    let base_string = format_node_to_string(&base);
    if base_string.contains('\n') || segments.iter().any(|segment| segment.contains('\n')) {
        return false;
    }

    let total_len = base_string.len() + segments.iter().map(String::len).sum::<usize>();
    if out.current_column() + total_len <= LINE_WIDTH {
        return false;
    }

    out.write(&base_string);
    out.indented(|out| {
        let mut on_continuation_line = false;
        for segment in segments {
            if segment.starts_with('.') {
                out.newline();
                out.write(&segment);
                on_continuation_line = true;
            } else if !on_continuation_line && out.current_column() + segment.len() <= LINE_WIDTH {
                out.write(&segment);
                on_continuation_line = true;
            } else if on_continuation_line && out.current_column() + segment.len() <= LINE_WIDTH {
                out.write(&segment);
            } else {
                out.newline();
                out.write(&segment);
                on_continuation_line = true;
            }
        }
    });

    true
}

fn is_postfix_chain_root(node: &SyntaxNode) -> bool {
    match node.kind() {
        METHOD_CALL_EXPR | FIELD_EXPR | TUPLE_ACCESS_EXPR | INDEX_EXPR => node.parent().is_none_or(|parent| {
            !matches!(parent.kind(), METHOD_CALL_EXPR | FIELD_EXPR | TUPLE_ACCESS_EXPR | INDEX_EXPR)
        }),
        _ => false,
    }
}

fn collect_postfix_chain(node: &SyntaxNode) -> Option<(SyntaxNode, Vec<String>)> {
    match node.kind() {
        METHOD_CALL_EXPR | FIELD_EXPR | TUPLE_ACCESS_EXPR | INDEX_EXPR => {
            let receiver = node.children().find(|child| child.kind().is_expression())?;
            let (base, mut segments) =
                if matches!(receiver.kind(), METHOD_CALL_EXPR | FIELD_EXPR | TUPLE_ACCESS_EXPR | INDEX_EXPR) {
                    collect_postfix_chain(&receiver)?
                } else {
                    (receiver.clone(), Vec::new())
                };
            segments.push(format_postfix_suffix(node, &receiver)?);
            Some((base, segments))
        }
        _ => Some((node.clone(), Vec::new())),
    }
}

fn format_postfix_suffix(node: &SyntaxNode, receiver: &SyntaxNode) -> Option<String> {
    let mut suffix = String::new();
    let mut past_receiver = false;

    for elem in node.children_with_tokens() {
        if !past_receiver {
            if let SyntaxElement::Node(n) = &elem
                && n.text_range() == receiver.text_range()
            {
                past_receiver = true;
            }
            continue;
        }

        match elem {
            SyntaxElement::Token(tok) => {
                if !is_trivia(tok.kind()) {
                    suffix.push_str(tok.text());
                }
            }
            SyntaxElement::Node(n) if n.kind().is_expression() => {
                let formatted = format_node_to_string(&n);
                if formatted.contains('\n') {
                    return None;
                }
                suffix.push_str(&formatted);
            }
            SyntaxElement::Node(_) => {}
        }
    }

    (!suffix.is_empty()).then_some(suffix)
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
                if k.is_expression() {
                    format_node(&n, out);
                } else if k.is_type() {
                    format_type(&n, out);
                }
            }
        }
    }
}

fn format_array_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();
    format_wrapping_list(node, out, "[", "]", &exprs, true, false);
}

fn format_repeat_expr(node: &SyntaxNode, out: &mut Output) {
    // REPEAT_EXPR: [value; count]
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();
    out.write("[");
    if exprs.len() >= 2 {
        format_node(&exprs[0], out);
        out.write(";");
        out.space();
        format_node(&exprs[1], out);
    }
    out.write("]");
}

fn format_tuple_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();

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
        let expr_strings: Vec<_> = exprs.iter().map(format_node_to_string).collect();
        let has_multiline_item = expr_strings.iter().any(|expr| expr.contains('\n'));
        let has_comment = exprs.iter().any(has_deep_comment)
            || node.children_with_tokens().any(
                |elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)),
            );

        if !has_comment && !has_multiline_item {
            let col = out.current_column();
            if fits_on_one_line(col, "(", ")", &expr_strings) {
                out.write("(");
                for (i, expr) in exprs.iter().enumerate() {
                    format_node(expr, out);
                    if i < exprs.len() - 1 {
                        out.write(",");
                        out.space();
                    }
                }
                out.write(")");
                return;
            }
        }

        if !has_comment && (has_multiline_item || exprs.len() > MAX_INLINE_TUPLE_ITEMS) {
            format_wrapping_list_with_dense_simple_items(out, "(", ")", &exprs, &expr_strings, true);
        } else {
            format_wrapping_list(node, out, "(", ")", &exprs, true, true);
        }
    }
}

fn format_parenthesized(node: &SyntaxNode, out: &mut Output) {
    out.write("(");
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                if !is_trivia(k) && k != L_PAREN && k != R_PAREN {
                    out.write(tok.text());
                }
            }
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
            _ => {}
        }
    }
    out.write(")");
}

fn format_struct_expr(node: &SyntaxNode, out: &mut Output) {
    let inits: Vec<_> =
        node.children().filter(|c| matches!(c.kind(), STRUCT_FIELD_INIT | STRUCT_FIELD_SHORTHAND)).collect();
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

    if inits.iter().any(struct_field_init_contains_nested_struct_expr) {
        format_wrapping_list_multiline(node, out, " { ", " }", &inits, true);
        return;
    }

    format_wrapping_list(node, out, " { ", " }", &inits, true, false);
}

fn format_struct_field_init(node: &SyntaxNode, out: &mut Output) {
    // Leading comments are handled by `format_wrapping_list`, not here.
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
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
            _ => {}
        }
    }
}

fn format_struct_field_shorthand(node: &SyntaxNode, out: &mut Output) {
    if let Some(tok) = node.children_with_tokens().find_map(|e| e.into_token().filter(|t| t.kind() == IDENT)) {
        out.write(tok.text());
    }
}

fn format_final_expr(node: &SyntaxNode, out: &mut Output) {
    out.write("final");
    out.space();
    for child in node.children() {
        if child.kind() == BLOCK {
            if final_expr_supports_compact_layout(node)
                && let Some(compact) = compact_final_block_string(&child, out.current_column())
            {
                out.write(&compact);
            } else {
                format_block(&child, out);
            }
        }
    }
}

fn final_expr_supports_compact_layout(node: &SyntaxNode) -> bool {
    let mut parent = node.parent();
    while let Some(current) = parent {
        match current.kind() {
            PAREN_EXPR => parent = current.parent(),
            RETURN_STMT | LET_STMT | CONST_STMT | ASSIGN_STMT | COMPOUND_ASSIGN_STMT | EXPR_STMT => return true,
            _ => return false,
        }
    }

    false
}

fn compact_final_block_string(block: &SyntaxNode, col: usize) -> Option<String> {
    if has_error_descendant(block) || has_deep_comment(block) {
        return None;
    }

    let statements: Vec<_> = block.children().filter(|child| child.kind().is_statement()).collect();
    if statements.len() != 1 {
        return None;
    }

    let stmt = format_node_to_string(&statements[0]);
    if stmt.contains('\n') {
        return None;
    }

    let compact = format!("{{ {stmt} }}");
    if col + compact.len() <= LINE_WIDTH { Some(compact) } else { None }
}

// =============================================================================
// Patterns
// =============================================================================

fn format_ident_pattern(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        if let SyntaxElement::Token(tok) = elem {
            let k = tok.kind();
            if !is_trivia(k) {
                out.write(tok.text());
            }
        }
    }
}

fn write_node_verbatim(node: &SyntaxNode, out: &mut Output) {
    let text = node.text().to_string();
    let text = text.trim();
    let base_indent = out.current_indent_width();

    for (idx, line) in text.split('\n').enumerate() {
        if idx > 0 {
            out.newline();
        }
        let line = line.trim_end();
        if idx == 0 {
            out.write(line);
            continue;
        }

        let mut stripped = line;
        let mut to_trim = base_indent;
        while to_trim > 0 && stripped.starts_with(' ') {
            stripped = &stripped[1..];
            to_trim -= 1;
        }
        out.write(stripped);
    }
}

fn write_node_with_inline_error_verbatim(node: &SyntaxNode, out: &mut Output) -> bool {
    if let Some(next) = node.next_sibling()
        && should_inline_adjacent_error(&next)
    {
        out.write(format!("{}{}", node.text(), next.text()).trim());
        return true;
    }
    false
}

fn emit_inline_error_suffix(node: &SyntaxNode, out: &mut Output) {
    if let Some(next) = node.next_sibling()
        && should_inline_adjacent_error(&next)
    {
        out.write(next.text().to_string().trim_end());
    }
}

fn should_inline_adjacent_error(node: &SyntaxNode) -> bool {
    let text = node.text().to_string();
    let Some(prev) = node.prev_sibling() else {
        return false;
    };

    node.kind() == ERROR
        && text.chars().next().is_some_and(|ch| !ch.is_whitespace())
        && matches!(
            prev.kind(),
            FUNCTION_DEF | FINAL_FN_DEF | CONSTRUCTOR_DEF | STRUCT_DEF | RECORD_DEF | MAPPING_DEF | GLOBAL_CONST
        )
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
    emit_leading_comments_inner(node, out, false);
}

/// Like `emit_leading_comments`, but uses the token stream to distinguish
/// genuine leading comments from same-line comments that belong to the
/// previous item.
fn emit_leading_comments_inner(node: &SyntaxNode, out: &mut Output, _is_first: bool) {
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                COMMENT_LINE if comment_starts_own_line(tok) => {
                    out.write(tok.text().trim_end());
                    out.newline();
                }
                COMMENT_BLOCK if comment_starts_own_line(tok) => {
                    out.write(tok.text());
                    out.newline();
                }
                COMMENT_LINE | COMMENT_BLOCK | WHITESPACE | LINEBREAK => {}
                _ => break, // Stop at first structural token
            },
            SyntaxElement::Node(_) => break, // Stop at first child node
        }
    }
}

/// Returns true when the comment token starts on its own source line.
///
/// We inspect the real token stream rather than node boundaries because rowan
/// often attaches leading trivia to the following item node.
fn comment_starts_own_line(token: &SyntaxToken) -> bool {
    let Some(parent) = token.parent() else {
        return true;
    };
    let root = parent.ancestors().last().unwrap_or(parent);
    let text = root.text().to_string();
    let start: usize = u32::from(token.text_range().start()) as usize;
    let line_start = text[..start].rfind('\n').map_or(0, |idx| idx + 1);
    text[line_start..start].chars().all(char::is_whitespace)
}

/// Emit "stolen" trailing comments from a node's leading trivia.
///
/// The parser consumes trailing comments (e.g. `// foo` after a semicolon)
/// into the next item's node as leading trivia. This function detects such
/// comments (those appearing before any LINEBREAK) and emits them inline
/// so they stay on the previous item's line.
fn emit_stolen_trailing_comments(node: &SyntaxNode, out: &mut Output) -> bool {
    let mut emitted = false;
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                COMMENT_LINE if !comment_starts_own_line(tok) => {
                    out.space();
                    out.write(tok.text().trim_end());
                    emitted = true;
                }
                COMMENT_BLOCK if !comment_starts_own_line(tok) => {
                    out.space();
                    out.write(tok.text());
                    emitted = true;
                }
                LINEBREAK | WHITESPACE => {}
                COMMENT_LINE | COMMENT_BLOCK => break,
                _ => break,
            },
            SyntaxElement::Node(_) => break,
        }
    }
    emitted
}

/// Emit comment tokens between a list `child` and the next list item in `parent`.
///
/// This captures comments represented as direct parent-level tokens after the
/// separating comma, before the next structural element.
fn emit_comments_after_list_item(parent: &SyntaxNode, child: &SyntaxNode, out: &mut Output) {
    let mut past_child = false;
    let mut past_comma = false;
    let mut saw_linebreak = false;

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
            match &elem {
                SyntaxElement::Token(tok) => match tok.kind() {
                    COMMA => past_comma = true,
                    WHITESPACE | LINEBREAK => {}
                    _ => break,
                },
                SyntaxElement::Node(_) => break,
            }
            continue;
        }

        match &elem {
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

fn emit_comments_after_child_until_token(
    parent: &SyntaxNode,
    child: &SyntaxNode,
    stop_token: SyntaxKind,
    out: &mut Output,
) {
    let mut past_child = false;
    let mut saw_linebreak = false;

    for elem in parent.children_with_tokens() {
        if !past_child {
            if let SyntaxElement::Node(n) = &elem
                && n.text_range() == child.text_range()
            {
                past_child = true;
            }
            continue;
        }

        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                k if k == stop_token => break,
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
                _ => {}
            },
            SyntaxElement::Node(_) => break,
        }
    }
}

/// Emit comments between the last list item and the closing delimiter.
///
/// Walks parent-level tokens after `last_item`, skipping commas and whitespace,
/// and emits any comments found before the next structural token or node.
fn emit_trailing_list_comments(parent: &SyntaxNode, last_item: &SyntaxNode, out: &mut Output) {
    let mut past_item = false;
    let mut saw_linebreak = false;
    for elem in parent.children_with_tokens() {
        if !past_item {
            if let SyntaxElement::Node(n) = &elem
                && n.text_range() == last_item.text_range()
            {
                past_item = true;
            }
            continue;
        }
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    saw_linebreak = true;
                }
                COMMENT_LINE => {
                    let at_line_start = out.current_column() == out.current_indent_width();
                    if saw_linebreak && !at_line_start {
                        out.newline();
                    } else if !saw_linebreak {
                        out.space();
                    }
                    out.write(tok.text().trim_end());
                    out.newline();
                    saw_linebreak = false;
                }
                COMMENT_BLOCK => {
                    let at_line_start = out.current_column() == out.current_indent_width();
                    if saw_linebreak && !at_line_start {
                        out.newline();
                    } else if !saw_linebreak {
                        out.space();
                    }
                    out.write(tok.text());
                    out.newline();
                    saw_linebreak = false;
                }
                WHITESPACE | COMMA => {}
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
fn format_wrapping_list(
    parent: &SyntaxNode,
    out: &mut Output,
    open: &str,
    close: &str,
    items: &[SyntaxNode],
    multi_trailing_comma: bool,
    dense_simple_items: bool,
) {
    let item_strings: Vec<String> = items.iter().map(format_node_to_string).collect();
    let col = out.current_column();
    let has_multiline_item = item_strings.iter().any(|s| s.contains('\n'));

    // Force multi-line when any comment exists: either deeply nested inside an
    // item's subtree, or as a direct child of the parent node (between items or
    // after the last item). Line comments cannot be kept inline in a single-line
    // list without swallowing subsequent items.
    let has_comment = items.iter().any(has_deep_comment)
        || parent.children_with_tokens().any(
            |elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)),
        );

    if !has_comment && !has_multiline_item && fits_on_one_line(col, open, close, &item_strings) {
        out.write(open);
        for (i, item) in items.iter().enumerate() {
            format_node(item, out);
            if i < items.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(close);
    } else if !has_comment && dense_simple_items && has_multiline_item {
        format_wrapping_list_with_dense_simple_items(out, open, close, items, &item_strings, multi_trailing_comma);
    } else {
        format_wrapping_list_multiline(parent, out, open, close, items, multi_trailing_comma);
    }
}

fn format_wrapping_list_multiline(
    parent: &SyntaxNode,
    out: &mut Output,
    open: &str,
    close: &str,
    items: &[SyntaxNode],
    multi_trailing_comma: bool,
) {
    out.write(open.trim_end());
    out.newline();
    out.indented(|out| {
        for (i, item) in items.iter().enumerate() {
            emit_leading_comments_inner(item, out, i == 0);
            format_node(item, out);
            if multi_trailing_comma || i < items.len() - 1 {
                out.write(",");
            }
            emit_node_trailing_comments(item, out);
            if i < items.len() - 1 {
                emit_comments_after_list_item(parent, item, out);
                emit_stolen_trailing_comments(&items[i + 1], out);
                if out.current_column() != out.current_indent_width() {
                    out.newline();
                }
            }
        }
        emit_trailing_list_comments(parent, items.last().unwrap(), out);
        if out.current_column() != out.current_indent_width() {
            out.newline();
        }
    });
    out.write(close.trim_start());
}

fn format_wrapping_list_with_dense_simple_items(
    out: &mut Output,
    open: &str,
    close: &str,
    items: &[SyntaxNode],
    item_strings: &[String],
    multi_trailing_comma: bool,
) {
    out.write(open.trim_end());
    out.newline();
    out.indented(|out| {
        let mut line_has_simple_items = false;

        for (i, item) in items.iter().enumerate() {
            let item_string = &item_strings[i];
            let needs_comma = multi_trailing_comma || i < items.len() - 1;

            if item_string.contains('\n') {
                if line_has_simple_items {
                    out.newline();
                    line_has_simple_items = false;
                }
                format_node(item, out);
                if needs_comma {
                    out.write(",");
                }
                out.newline();
                continue;
            }

            let line_start = out.current_column() == out.current_indent_width();
            let fragment_len = item_string.len() + usize::from(needs_comma);

            if !line_start {
                if out.current_column() + 1 + fragment_len > LINE_WIDTH {
                    out.newline();
                } else {
                    out.space();
                }
            }

            out.write(item_string);
            if needs_comma {
                out.write(",");
            }
            line_has_simple_items = true;
        }

        if line_has_simple_items {
            out.newline();
        }
    });
    out.write(close.trim_start());
}

fn format_wrapping_string_list_with_dense_simple_items(
    out: &mut Output,
    open: &str,
    close: &str,
    item_strings: &[String],
    multi_trailing_comma: bool,
) {
    out.write(open.trim_end());
    out.newline();
    out.indented(|out| {
        let mut line_has_simple_items = false;

        for (i, item_string) in item_strings.iter().enumerate() {
            let needs_comma = multi_trailing_comma || i < item_strings.len() - 1;

            if item_string.contains('\n') {
                if line_has_simple_items {
                    out.newline();
                    line_has_simple_items = false;
                }
                write_formatted_string(out, item_string);
                if needs_comma {
                    out.write(",");
                }
                out.newline();
                continue;
            }

            let line_start = out.current_column() == out.current_indent_width();
            let fragment_len = item_string.len() + usize::from(needs_comma);

            if !line_start {
                if out.current_column() + 1 + fragment_len > LINE_WIDTH {
                    out.newline();
                } else {
                    out.space();
                }
            }

            out.write(item_string);
            if needs_comma {
                out.write(",");
            }
            line_has_simple_items = true;
        }

        if line_has_simple_items {
            out.newline();
        }
    });
    out.write(close.trim_start());
}

fn write_formatted_string(out: &mut Output, s: &str) {
    let mut lines = s.lines().peekable();
    while let Some(line) = lines.next() {
        out.write(line);
        if lines.peek().is_some() {
            out.newline();
        }
    }
}

/// Format a node into a temporary buffer and return the result as a string.
fn format_node_to_string(node: &SyntaxNode) -> String {
    let mut out = Output::new();
    format_node(node, &mut out);
    out.into_raw()
}

fn struct_field_init_contains_nested_struct_expr(node: &SyntaxNode) -> bool {
    node.descendants().skip(1).any(|child| matches!(child.kind(), STRUCT_EXPR | STRUCT_LOCATOR_EXPR))
}

/// Check if a delimited list fits on one line.
///
/// Computes: `col + prefix.len() + joined_items(", ") + suffix.len() <= LINE_WIDTH`
fn fits_on_one_line(col: usize, prefix: &str, suffix: &str, items: &[String]) -> bool {
    fits_on_one_line_with_suffix(col, prefix, suffix, items, 0)
}

/// Like `fits_on_one_line` but also accounts for extra characters that follow
/// after the suffix (e.g. a return type and opening brace after a parameter list).
fn fits_on_one_line_with_suffix(
    col: usize,
    prefix: &str,
    suffix: &str,
    items: &[String],
    extra_suffix_width: usize,
) -> bool {
    let items_len: usize = if items.is_empty() {
        0
    } else {
        items.iter().map(|s| s.len()).sum::<usize>() + (items.len() - 1) * 2 // ", " between items
    };
    col + prefix.len() + items_len + suffix.len() + extra_suffix_width <= LINE_WIDTH
}

// =============================================================================
// Helpers
// =============================================================================

fn is_program_item(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        FUNCTION_DEF
            | FINAL_FN_DEF
            | CONSTRUCTOR_DEF
            | STRUCT_DEF
            | RECORD_DEF
            | MAPPING_DEF
            | STORAGE_DEF
            | GLOBAL_CONST
            | INTERFACE_DEF
            | ANNOTATION
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

fn next_significant_is_colon_colon(elems: &[SyntaxElement], idx: usize) -> bool {
    elems[idx + 1..].iter().find_map(|elem| match elem {
        SyntaxElement::Token(tok) if is_trivia(tok.kind()) => None,
        SyntaxElement::Token(tok) => Some(tok.kind() == COLON_COLON),
        SyntaxElement::Node(_) => Some(false),
    }) == Some(true)
}

fn has_same_line_parent_comment_after_node(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    let mut past_node = false;
    for elem in parent.children_with_tokens() {
        if !past_node {
            if let SyntaxElement::Node(n) = &elem
                && n.text_range() == node.text_range()
            {
                past_node = true;
            }
            continue;
        }

        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                WHITESPACE => {}
                COMMENT_LINE | COMMENT_BLOCK => return true,
                LINEBREAK => return false,
                _ => return false,
            },
            SyntaxElement::Node(_) => return false,
        }
    }

    false
}

fn previous_significant_is_const_params(elems: &[SyntaxElement], idx: usize) -> bool {
    elems[..idx].iter().rev().find_map(|elem| match elem {
        SyntaxElement::Token(tok) if is_trivia(tok.kind()) => None,
        SyntaxElement::Token(_) => Some(false),
        SyntaxElement::Node(n) => Some(n.kind() == CONST_PARAM_LIST),
    }) == Some(true)
}

/// Check if a node's leading trivia contains a blank line (>= 2 LINEBREAK tokens).
fn has_blank_line_in_leading_trivia(node: &SyntaxNode) -> bool {
    let mut linebreak_count = 0;
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    linebreak_count += 1;
                    if linebreak_count >= 2 {
                        return true;
                    }
                }
                WHITESPACE | COMMENT_LINE | COMMENT_BLOCK => {}
                _ => break,
            },
            SyntaxElement::Node(_) => break,
        }
    }
    false
}

/// Check if a token kind is trivia (whitespace, linebreaks, or comments).
fn is_trivia(kind: SyntaxKind) -> bool {
    matches!(kind, WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK)
}

/// A comment found trailing an expression node's subtree.
struct TrailingComment {
    kind: SyntaxKind,
    text: String,
    starts_own_line: bool,
}

/// Recursively extract trailing comments from the rightmost path of a node's subtree.
///
/// In rowan, trailing trivia (comments between the last expression token and
/// the next delimiter) gets attached inside the innermost expression node.
/// This function walks the rightmost children to find and extract them.
fn collect_node_trailing_comments(node: &SyntaxNode) -> Vec<TrailingComment> {
    let mut comments = Vec::new();
    collect_trailing_inner(node, &mut comments);
    comments.reverse();
    comments
}

fn collect_trailing_inner(node: &SyntaxNode, comments: &mut Vec<TrailingComment>) {
    let children: Vec<_> = node.children_with_tokens().collect();
    for elem in children.iter().rev() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                WHITESPACE => {}
                LINEBREAK => {}
                COMMENT_LINE | COMMENT_BLOCK => {
                    comments.push(TrailingComment {
                        kind: tok.kind(),
                        text: tok.text().to_string(),
                        starts_own_line: comment_starts_own_line(tok),
                    });
                }
                _ => return,
            },
            SyntaxElement::Node(n) => {
                collect_trailing_inner(n, comments);
                return;
            }
        }
    }
}

/// Emit trailing comments extracted from a node's subtree.
fn emit_node_trailing_comments(node: &SyntaxNode, out: &mut Output) -> bool {
    let comments = collect_node_trailing_comments(node);
    let mut emitted_own_line_comment = false;
    for tc in &comments {
        if tc.starts_own_line {
            let at_line_start = out.current_column() == out.current_indent_width();
            if !at_line_start {
                out.newline();
            }
            emitted_own_line_comment = true;
        } else {
            out.space();
        }
        if tc.kind == COMMENT_LINE {
            out.write(tc.text.trim_end());
        } else {
            out.write(&tc.text);
        }
        if tc.starts_own_line {
            out.newline();
        }
    }
    emitted_own_line_comment
}

/// Check if a node contains comment tokens among its direct children/tokens.
fn has_comment_token(node: &SyntaxNode) -> bool {
    node.children_with_tokens()
        .any(|elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)))
}

/// Recursively check if a node or any descendant contains a comment token.
fn has_deep_comment(node: &SyntaxNode) -> bool {
    for elem in node.children_with_tokens() {
        match &elem {
            SyntaxElement::Token(tok) => {
                if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK) {
                    return true;
                }
            }
            SyntaxElement::Node(n) => {
                if has_deep_comment(n) {
                    return true;
                }
            }
        }
    }
    false
}

/// Check whether a binary expression contains comments that belong to the
/// chain itself rather than trailing trivia on the final operand.
fn has_internal_binary_comment(node: &SyntaxNode) -> bool {
    let mut expr_nodes_seen = 0usize;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK) => {
                if expr_nodes_seen < 2 {
                    return true;
                }
            }
            SyntaxElement::Node(child) => {
                if expr_nodes_seen == 0 && has_internal_binary_comment(&child) {
                    return true;
                }
                if child.kind().is_expression() {
                    expr_nodes_seen += 1;
                }
            }
            _ => {}
        }
    }

    false
}

fn has_error_descendant(node: &SyntaxNode) -> bool {
    node.kind() == ERROR || node.children().any(|child| has_error_descendant(&child))
}

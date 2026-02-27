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
    let mut prev_was_import = false;
    let mut prev_was_block_item = false;
    let mut had_output = false;
    let mut linebreak_count: usize = 0;
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => match tok.kind() {
                LINEBREAK => {
                    linebreak_count += 1;
                }
                COMMENT_LINE | COMMENT_BLOCK => {
                    if had_output {
                        out.ensure_newline();
                    }
                    // Blank line before comment if after imports, after a block item,
                    // or if the source had a blank line.
                    if prev_was_import || prev_was_block_item || (had_output && linebreak_count >= 2) {
                        out.newline();
                        prev_was_import = false;
                    }
                    let text = if tok.kind() == COMMENT_LINE { tok.text().trim_end() } else { tok.text() };
                    out.write(text);
                    out.newline();
                    had_output = true;
                    prev_was_block_item = false;
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
                    had_output = true;
                    linebreak_count = 0;
                } else if kind == ANNOTATION {
                    if prev_was_import || prev_was_block_item || (had_output && linebreak_count >= 2) {
                        out.newline();
                    }
                    format_annotation(&n, out);
                    prev_was_import = false;
                    // Don't set prev_was_block_item here; the annotated item will set it.
                    had_output = true;
                    linebreak_count = 0;
                } else if is_program_item(kind) {
                    let is_block = is_block_item(kind);
                    let wants_blank = had_output
                        && (prev_was_import
                            || is_block
                            || prev_was_block_item
                            || linebreak_count >= 2
                            || has_blank_line_in_leading_trivia(&n));
                    if wants_blank {
                        out.newline();
                    }
                    format_node(&n, out);
                    prev_was_import = false;
                    prev_was_block_item = is_block;
                    had_output = true;
                    linebreak_count = 0;
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
                        prev_was_block_item = false;
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
                        && (is_block
                            || prev_was_block_item
                            || linebreak_count >= 2
                            || has_blank_line_in_leading_trivia(n));
                    if kind == ERROR {
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
                        }
                    } else {
                        if wants_blank {
                            out.insert_newline_at_mark();
                        }
                        out.indented(|out| format_node(n, out));
                        had_item = true;
                        prev_was_block_item = is_block;
                    }
                    saw_linebreak = false;
                    linebreak_count = 0;
                }
            }
            _ => {}
        }
    }

    out.write("}");
    out.newline();
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
                    STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT => {
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

    out.write("}");
    if let Some(idx) = rbrace_idx {
        emit_comments_after(&elems, idx, out);
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
                if k.is_type() {
                    format_type(&n, out);
                } else if k.is_expression() {
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
    format_parameter_list_with_suffix(node, out, 0);
}

fn format_parameter_list_with_suffix(node: &SyntaxNode, out: &mut Output, suffix_width: usize) {
    let params: Vec<_> =
        node.children().filter(|c| matches!(c.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT)).collect();

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
            format_parameter(param, out);
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
                format_parameter(param, out);
                out.write(",");
                emit_node_trailing_comments(param, out);
                if i < params.len() - 1 {
                    emit_comments_after_list_item(node, param, out);
                    emit_stolen_trailing_comments(&params[i + 1], out);
                }
                out.ensure_newline();
            }
            emit_trailing_list_comments(node, params.last().unwrap(), out);
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
                    WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK => {}
                    _ => out.write(tok.text()),
                }
            }
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
            SyntaxElement::Node(n) if n.kind().is_type() && in_tuple => {
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
    let args: Vec<_> = node.children().filter(|c| c.kind().is_type() || c.kind().is_expression()).collect();

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
        TYPE_PRIMITIVE => format_type_primitive(node, out),
        TYPE_LOCATOR => format_type_locator(node, out),
        TYPE_PATH => format_type_path(node, out),
        TYPE_ARRAY => format_type_array(node, out),
        TYPE_VECTOR => format_type_vector(node, out),
        TYPE_TUPLE => format_type_tuple(node, out),
        TYPE_FINAL => format_type_final(node, out),
        TYPE_MAPPING => format_type_mapping(node, out),
        TYPE_OPTIONAL => format_type_optional(node, out),
        _ => {}
    }
}

fn format_type_primitive(node: &SyntaxNode, out: &mut Output) {
    // Just write the keyword token text.
    if let Some(tok) = node.children_with_tokens().find_map(|e| e.into_token().filter(|t| !t.kind().is_trivia())) {
        out.write(tok.text());
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

fn format_type_final(node: &SyntaxNode, out: &mut Output) {
    // Final or Final<Fn(...) -> ...>
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

// =============================================================================
// Statements
// =============================================================================

fn format_block(node: &SyntaxNode, out: &mut Output) {
    let elems = elements(node);

    // Check if block has any statements or comments (content worth indenting)
    let has_content = elems.iter().any(|e| match e {
        SyntaxElement::Node(n) => n.kind().is_statement() || n.kind() == ERROR,
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
                    SyntaxElement::Node(n) if after_lbrace && n.kind().is_statement() => {
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
        if child.kind().is_expression() {
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
                } else if k.is_expression() {
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
            SyntaxElement::Node(n) if n.kind().is_expression() => format_node(&n, out),
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
            }
            out.ensure_newline();
        }
        emit_trailing_list_comments(node, args.last().unwrap(), out);
    });
    out.write(")");
}

fn format_method_call(node: &SyntaxNode, out: &mut Output) {
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
    format_wrapping_list(node, out, "[", "]", &exprs, true);
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
        format_wrapping_list(node, out, "(", ")", &exprs, true);
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

    format_wrapping_list(node, out, " { ", " }", &inits, true);
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
            format_block(&child, out);
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
            if !is_trivia(k) {
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
    emit_leading_comments_inner(node, out, false);
}

/// Like `emit_leading_comments`, but when `is_first` is true, treats all
/// leading comments as genuine (not stolen) because there is no previous item
/// to steal from. This prevents dropping comments on the first item in a list.
fn emit_leading_comments_inner(node: &SyntaxNode, out: &mut Output, is_first: bool) {
    let mut saw_linebreak = is_first;
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

/// Emit comments between the last list item and the closing delimiter.
///
/// Walks parent-level tokens after `last_item`, skipping commas and whitespace,
/// and emits any comments found before the next structural token or node.
fn emit_trailing_list_comments(parent: &SyntaxNode, last_item: &SyntaxNode, out: &mut Output) {
    let mut past_item = false;
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
                COMMENT_LINE => {
                    out.write(tok.text().trim_end());
                    out.newline();
                }
                COMMENT_BLOCK => {
                    out.write(tok.text());
                    out.newline();
                }
                WHITESPACE | LINEBREAK | COMMA => {}
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
) {
    let item_strings: Vec<String> = items.iter().map(format_node_to_string).collect();
    let col = out.current_column();

    // Force multi-line when any comment exists: either deeply nested inside an
    // item's subtree, or as a direct child of the parent node (between items or
    // after the last item). Line comments cannot be kept inline in a single-line
    // list without swallowing subsequent items.
    let has_comment = items.iter().any(has_deep_comment)
        || parent.children_with_tokens().any(
            |elem| matches!(elem, SyntaxElement::Token(tok) if matches!(tok.kind(), COMMENT_LINE | COMMENT_BLOCK)),
        );

    if !has_comment && fits_on_one_line(col, open, close, &item_strings) {
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
                emit_leading_comments_inner(item, out, i == 0);
                format_node(item, out);
                if multi_trailing_comma || i < items.len() - 1 {
                    out.write(",");
                }
                emit_node_trailing_comments(item, out);
                if i < items.len() - 1 {
                    emit_comments_after_list_item(parent, item, out);
                    emit_stolen_trailing_comments(&items[i + 1], out);
                }
                out.newline();
            }
            emit_trailing_list_comments(parent, items.last().unwrap(), out);
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
                WHITESPACE | LINEBREAK => {}
                COMMENT_LINE | COMMENT_BLOCK => {
                    comments.push(TrailingComment { kind: tok.kind(), text: tok.text().to_string() });
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
fn emit_node_trailing_comments(node: &SyntaxNode, out: &mut Output) {
    let comments = collect_node_trailing_comments(node);
    for tc in &comments {
        out.newline();
        if tc.kind == COMMENT_LINE {
            out.write(tc.text.trim_end());
        } else {
            out.write(&tc.text);
        }
    }
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

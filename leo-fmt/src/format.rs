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

use crate::output::Output;
use leo_parser_rowan::{
    SyntaxElement,
    SyntaxKind::{self, *},
    SyntaxNode,
    SyntaxToken,
};

/// Format any syntax node.
pub fn format_node(node: &SyntaxNode, out: &mut Output) {
    match node.kind() {
        // Top-level
        ROOT => format_root(node, out),
        PROGRAM_DECL => format_program(node, out),

        // Declarations
        FUNCTION_DEF | FINAL_FN_DEF | SCRIPT_DEF | CONSTRUCTOR_DEF => format_function(node, out),
        STRUCT_DEF | RECORD_DEF => format_composite(node, out),
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

    // Count item groups (annotations are nested inside function/constructor nodes).
    let item_group_count =
        node.children().filter(|c| is_program_item_non_annotation(c.kind()) || c.kind() == ERROR).count();

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
                if is_program_item_non_annotation(kind) {
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
    matches!(
        kind,
        FUNCTION_DEF
            | FINAL_FN_DEF
            | SCRIPT_DEF
            | CONSTRUCTOR_DEF
            | STRUCT_DEF
            | RECORD_DEF
            | MAPPING_DEF
            | STORAGE_DEF
            | GLOBAL_CONST
    )
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
                        // Part of const params, handled by CONST_PARAM_LIST
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
                    ANNOTATION => format_annotation(&n, out),
                    PARAM_LIST => format_parameter_list(&n, out),
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

    // Write keyword and name
    for elem in &elems {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
                match k {
                    KW_STRUCT | KW_RECORD => {
                        out.write(tok.text());
                        out.space();
                    }
                    IDENT => {
                        out.write(tok.text());
                        out.space();
                    }
                    COLON_COLON => {
                        // Handled by CONST_PARAM_LIST
                    }
                    L_BRACE => {
                        out.write("{");
                        out.newline();
                    }
                    R_BRACE => {}
                    COMMA => {}
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
                        });
                        out.newline();
                    }
                    _ => {}
                }
            }
        }
    }
    out.write("}");
    out.set_mark();
    // Emit comments after closing brace
    if let Some(idx) = rbrace_idx {
        emit_comments_after(&elems, idx, out);
    }
    out.ensure_newline();
}

fn format_struct_member(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
        match elem {
            SyntaxElement::Token(tok) => {
                let k = tok.kind();
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
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
    let params: Vec<_> =
        node.children().filter(|c| matches!(c.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT)).collect();

    out.write("(");
    for (i, param) in params.iter().enumerate() {
        format_parameter(param, out);
        if i < params.len() - 1 {
            out.write(",");
            // Emit any trailing comments after this comma
            emit_inline_comments_after_param(node, param, out);
            out.space();
        }
    }
    out.write(")");
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
            SyntaxElement::Node(n) if n.kind().is_type() => format_type(&n, out),
            _ => {}
        }
    }
}

fn format_return_type(node: &SyntaxNode, out: &mut Output) {
    // RETURN_TYPE wraps tuple return types: (vis Type, vis Type, ...)
    // Iterate children_with_tokens and emit them preserving structure.
    // We use the COMMA tokens from the tree to know where to place commas.
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

    out.write("::[");
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

    out.write("::[");
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
        TYPE_FINAL => format_type_future(node, out),
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
    out.write(keyword);
    out.write("(");

    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();
    for (i, expr) in exprs.iter().enumerate() {
        format_node(expr, out);
        if i < exprs.len() - 1 {
            out.write(",");
            out.space();
        }
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
            if k != WHITESPACE && k != LINEBREAK {
                out.write(tok.text());
            }
        }
    }
}

fn format_call(node: &SyntaxNode, out: &mut Output) {
    // CALL_EXPR: callee_node, L_PAREN, [args separated by COMMA], R_PAREN
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
            SyntaxElement::Node(n) => format_node(&n, out),
        }
    }
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
                    WHITESPACE | LINEBREAK => {}
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
                if k != WHITESPACE && k != LINEBREAK {
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
    // ARRAY_EXPR: [a, b, c]
    let exprs: Vec<_> = node.children().filter(|c| c.kind().is_expression()).collect();
    out.write("[");
    for (i, expr) in exprs.iter().enumerate() {
        format_node(expr, out);
        if i < exprs.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write("]");
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

    out.write("(");
    for (i, expr) in exprs.iter().enumerate() {
        format_node(expr, out);
        if i < exprs.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    // Single-element tuples need a trailing comma to distinguish from PAREN_EXPR.
    if exprs.len() == 1 {
        out.write(",");
    }
    out.write(")");
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

    out.space();
    out.write("{");
    if !inits.is_empty() {
        out.space();
        for (i, init) in inits.iter().enumerate() {
            format_node(init, out);
            if i < inits.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.space();
    }
    out.write("}");
}

fn format_struct_field_init(node: &SyntaxNode, out: &mut Output) {
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
fn emit_leading_comments(node: &SyntaxNode, out: &mut Output) {
    for elem in node.children_with_tokens() {
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
                WHITESPACE | LINEBREAK => {}
                _ => break, // Stop at first structural token
            },
            SyntaxElement::Node(_) => break, // Stop at first child node
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
fn write_semicolon_with_comments(node: &SyntaxNode, out: &mut Output) {
    out.write(";");
    out.set_mark();
    let elems = elements(node);
    if let Some(idx) = find_token_index(&elems, SEMICOLON) {
        emit_comments_after(&elems, idx, out);
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn is_program_item(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        FUNCTION_DEF
            | FINAL_FN_DEF
            | SCRIPT_DEF
            | CONSTRUCTOR_DEF
            | STRUCT_DEF
            | RECORD_DEF
            | MAPPING_DEF
            | STORAGE_DEF
            | GLOBAL_CONST
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

/// Get the first child token with a given kind.
fn first_token(node: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    node.children_with_tokens().filter_map(|e| e.into_token()).find(|t| t.kind() == kind)
}

/// Check if a node has a child token of a given kind.
fn has_token(node: &SyntaxNode, kind: SyntaxKind) -> bool {
    first_token(node, kind).is_some()
}

/// Find the index of a token with a given kind.
fn find_token_index(elems: &[SyntaxElement], kind: SyntaxKind) -> Option<usize> {
    elems.iter().position(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == kind))
}

/// Find the last index of a token with a given kind.
fn find_last_token_index(elems: &[SyntaxElement], kind: SyntaxKind) -> Option<usize> {
    elems.iter().rposition(|e| matches!(e, SyntaxElement::Token(t) if t.kind() == kind))
}

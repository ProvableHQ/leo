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
use leo_parser_lossless::{ExpressionKind, IntegerLiteralKind, LiteralKind, StatementKind, SyntaxKind, SyntaxNode};

/// Returns true if this operator token requires surrounding spaces.
fn is_spaced_operator(text: &str) -> bool {
    matches!(
        text,
        // Binary arithmetic
        "+" | "-" | "*" | "/" | "%" | "**" |
        // Binary logical
        "&&" | "||" |
        // Binary bitwise
        "&" | "|" | "^" | "<<" | ">>" |
        // Binary comparison
        "<" | ">" | "<=" | ">=" | "==" | "!=" |
        // Assignment (simple and compound)
        "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "**=" |
        "<<=" | ">>=" | "&=" | "|=" | "^=" | "&&=" | "||=" |
        // Range
        ".."
    )
}

/// Format any syntax node.
pub fn format_node(node: &SyntaxNode, out: &mut Output) {
    match &node.kind {
        // Top-level
        SyntaxKind::MainContents => format_main(node, out),
        SyntaxKind::ProgramDeclaration => format_program(node, out),
        SyntaxKind::ModuleContents => format_module_contents(node, out),

        // Declarations
        SyntaxKind::Function | SyntaxKind::Constructor => format_function(node, out),
        SyntaxKind::CompositeDeclaration => format_composite(node, out),
        SyntaxKind::Import => format_import(node, out),
        SyntaxKind::Mapping => format_mapping(node, out),
        SyntaxKind::Storage => format_storage(node, out),
        SyntaxKind::GlobalConst => format_global_const(node, out),
        SyntaxKind::Annotation => format_annotation(node, out),
        SyntaxKind::AnnotationList => format_annotation_list(node, out),
        SyntaxKind::AnnotationMember => format_annotation_member(node, out),
        SyntaxKind::ParameterList => format_parameter_list(node, out),
        SyntaxKind::Parameter => format_parameter(node, out),
        SyntaxKind::FunctionOutputs => format_function_outputs(node, out),
        SyntaxKind::FunctionOutput => format_function_output(node, out),
        SyntaxKind::ConstParameter => format_const_parameter(node, out),
        SyntaxKind::ConstParameterList => format_const_parameter_list(node, out),
        SyntaxKind::ConstArgumentList => format_const_argument_list(node, out),
        SyntaxKind::CompositeMemberDeclarationList => format_composite_member_list(node, out),
        SyntaxKind::CompositeMemberDeclaration => format_composite_member(node, out),

        // Statements
        SyntaxKind::Statement(StatementKind::Block) => format_block(node, out),
        SyntaxKind::Statement(StatementKind::Return) => format_return(node, out),
        SyntaxKind::Statement(StatementKind::Definition) => format_definition(node, out),
        SyntaxKind::Statement(StatementKind::Const) => format_const_stmt(node, out),
        SyntaxKind::Statement(StatementKind::Assign) => format_assign(node, out),
        SyntaxKind::Statement(StatementKind::Conditional) => format_conditional(node, out),
        SyntaxKind::Statement(StatementKind::Iteration) => format_iteration(node, out),
        SyntaxKind::Statement(StatementKind::Assert) => format_assert(node, out, "assert"),
        SyntaxKind::Statement(StatementKind::AssertEq) => format_assert_eq(node, out),
        SyntaxKind::Statement(StatementKind::AssertNeq) => format_assert_neq(node, out),
        SyntaxKind::Statement(StatementKind::Expression) => format_expr_stmt(node, out),

        // Expressions
        SyntaxKind::Expression(ExpressionKind::Literal(_)) => format_literal(node, out),
        SyntaxKind::Expression(ExpressionKind::Call) => format_call(node, out),
        SyntaxKind::Expression(ExpressionKind::Binary) => format_binary(node, out),
        SyntaxKind::Expression(ExpressionKind::Path) => format_path(node, out),
        SyntaxKind::Expression(ExpressionKind::Unary) => format_unary(node, out),
        SyntaxKind::Expression(ExpressionKind::Ternary) => format_ternary(node, out),
        SyntaxKind::Expression(ExpressionKind::MethodCall) => format_method_call(node, out),
        SyntaxKind::Expression(ExpressionKind::AssociatedFunctionCall) => format_assoc_call(node, out),
        SyntaxKind::Expression(ExpressionKind::AssociatedConstant) => format_assoc_const(node, out),
        SyntaxKind::Expression(ExpressionKind::Composite) => format_composite_expr(node, out),
        SyntaxKind::Expression(ExpressionKind::ArrayAccess) => format_array_access(node, out),
        SyntaxKind::Expression(ExpressionKind::TupleAccess) => format_tuple_access(node, out),
        SyntaxKind::Expression(ExpressionKind::MemberAccess) => format_member_access(node, out),
        SyntaxKind::Expression(ExpressionKind::Cast) => format_cast(node, out),
        SyntaxKind::Expression(ExpressionKind::Array) => format_array_expr(node, out),
        SyntaxKind::Expression(ExpressionKind::Tuple) => format_tuple_expr(node, out),
        SyntaxKind::Expression(ExpressionKind::Parenthesized) => format_parenthesized(node, out),
        SyntaxKind::Expression(ExpressionKind::Repeat) => format_repeat_expr(node, out),
        SyntaxKind::Expression(ExpressionKind::Async) => format_async_expr(node, out),
        SyntaxKind::Expression(ExpressionKind::Intrinsic) => format_intrinsic(node, out),
        SyntaxKind::Expression(ExpressionKind::SpecialAccess) => format_special_access(node, out),
        SyntaxKind::Expression(ExpressionKind::Unit) => out.write("()"),
        SyntaxKind::CompositeMemberInitializer => format_composite_member_init(node, out),

        // Types
        SyntaxKind::Type(_) => format_type(node, out),

        // Trivia
        SyntaxKind::Whitespace | SyntaxKind::Linebreak => {}
        SyntaxKind::CommentLine => {
            out.space();
            out.write(node.text.trim_end());
            out.newline();
        }
        SyntaxKind::CommentBlock => {
            out.space();
            out.write(node.text);
        }
        SyntaxKind::Token => {
            out.write(node.text);
            emit_trivia(&node.children, out);
        }
    }
}

/// Emit comments from a token's trailing trivia children.
///
/// Uses linebreaks in the trivia to distinguish trailing comments (same line)
/// from standalone comments (own line). Whitespace trivia is ignored since
/// the formatter controls all spacing.
fn emit_trivia(children: &[SyntaxNode], out: &mut Output) {
    let mut saw_linebreak = false;
    for child in children {
        match child.kind {
            SyntaxKind::Linebreak => {
                saw_linebreak = true;
            }
            SyntaxKind::CommentLine => {
                if saw_linebreak {
                    // Standalone comment — goes on its own line
                    out.ensure_newline();
                } else {
                    // Trailing comment — stays on same line
                    out.space();
                }
                out.write(child.text.trim_end());
                out.newline();
                saw_linebreak = false;
            }
            SyntaxKind::CommentBlock => {
                if saw_linebreak {
                    out.ensure_newline();
                } else {
                    out.space();
                }
                out.write(child.text);
                saw_linebreak = false;
            }
            _ => {}
        }
    }
}

fn format_main(node: &SyntaxNode, out: &mut Output) {
    let mut prev_was_import = false;
    for child in &node.children {
        let is_import = matches!(child.kind, SyntaxKind::Import);
        if prev_was_import && !is_import {
            out.newline();
        }
        format_node(child, out);
        prev_was_import = is_import;
    }
}

fn is_program_item(kind: &SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::Function
            | SyntaxKind::Constructor
            | SyntaxKind::CompositeDeclaration
            | SyntaxKind::Mapping
            | SyntaxKind::Storage
            | SyntaxKind::GlobalConst
    )
}

fn format_program(node: &SyntaxNode, out: &mut Output) {
    let item_count = node.children.iter().filter(|c| is_program_item(&c.kind)).count();
    let mut item_idx = 0;

    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "program" => {
                out.write("program");
                out.space();
            }
            SyntaxKind::Token if child.text == "{" => {
                out.space();
                out.write("{");
                if item_count > 0 {
                    out.newline();
                }
                // Emit comments after opening brace (before first item).
                out.indented(|out| {
                    emit_trivia(&child.children, out);
                });
            }
            SyntaxKind::Token if child.text == "}" => {
                out.write("}");
                out.newline();
            }
            SyntaxKind::Token => out.write(child.text),
            kind if is_program_item(kind) => {
                out.indented(|out| {
                    format_node(child, out);
                    item_idx += 1;
                    if item_idx < item_count {
                        out.insert_newline_at_mark();
                    }
                });
            }
            _ => {}
        }
    }
}

fn format_module_contents(node: &SyntaxNode, out: &mut Output) {
    let item_count = node.children.iter().filter(|c| is_program_item(&c.kind)).count();
    let mut item_idx = 0;

    for child in &node.children {
        match &child.kind {
            kind if is_program_item(kind) => {
                format_node(child, out);
                item_idx += 1;
                if item_idx < item_count {
                    out.insert_newline_at_mark();
                }
            }
            _ => {}
        }
    }
}

fn format_function(node: &SyntaxNode, out: &mut Output) {
    // Annotations first
    for child in &node.children {
        if matches!(child.kind, SyntaxKind::Annotation) {
            format_node(child, out);
        }
    }
    // Then the rest
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Annotation => {}
            SyntaxKind::Token => match child.text {
                "async" | "function" | "transition" | "inline" | "constructor" => {
                    out.write(child.text);
                    out.space();
                }
                "->" => {
                    out.space();
                    out.write("->");
                    out.space();
                }
                _ => out.write(child.text),
            },
            SyntaxKind::ParameterList => format_parameter_list(child, out),
            SyntaxKind::FunctionOutputs => format_function_outputs(child, out),
            SyntaxKind::Statement(StatementKind::Block) => {
                out.space();
                format_block(child, out);
            }
            _ => format_node(child, out),
        }
    }
    out.ensure_newline();
}

fn format_annotation(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::AnnotationList => format_annotation_list(child, out),
            _ => {}
        }
    }
    out.newline();
}

fn format_composite(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "struct" || child.text == "record" => {
                out.write(child.text);
                out.space();
            }
            SyntaxKind::Token => {
                // Identifier
                out.write(child.text);
                out.space();
            }
            SyntaxKind::CompositeMemberDeclarationList => {
                format_composite_member_list(child, out);
            }
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_composite_member_list(node: &SyntaxNode, out: &mut Output) {
    let members: Vec<_> =
        node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::CompositeMemberDeclaration)).collect();

    let close_brace = node.children.iter().find(|c| c.kind == SyntaxKind::Token && c.text == "}");

    out.write("{");
    out.newline();
    for member in &members {
        out.indented(|out| {
            format_composite_member(member, out);
            out.write(",");
        });
        out.newline();
    }
    out.write("}");
    out.set_mark();
    // Emit comments that trail after the closing brace (between items).
    if let Some(brace) = close_brace {
        emit_trivia(&brace.children, out);
    }
}

fn format_composite_member(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "," => {} // handled by parent
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
}

fn format_import(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "import" => {
                out.write("import");
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            _ => {}
        }
    }
    out.newline();
}

fn format_mapping(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "mapping" => {
                out.write("mapping");
                out.space();
            }
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "=>" => {
                out.space();
                out.write("=>");
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                out.set_mark();
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_parameter(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "public" || child.text == "private" || child.text == "constant" => {
                out.write(child.text);
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
}

fn format_function_outputs(node: &SyntaxNode, out: &mut Output) {
    let has_paren = node.children.iter().any(|c| c.kind == SyntaxKind::Token && c.text == "(");
    if has_paren {
        let outputs: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::FunctionOutput)).collect();
        out.write("(");
        for (i, child) in outputs.iter().enumerate() {
            format_function_output(child, out);
            if i < outputs.len() - 1 {
                out.write(",");
                out.space();
            }
        }
        out.write(")");
    } else {
        for child in &node.children {
            if matches!(child.kind, SyntaxKind::FunctionOutput) {
                format_function_output(child, out);
            }
        }
    }
}

fn format_function_output(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "public" || child.text == "private" || child.text == "constant" => {
                out.write(child.text);
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
}

fn format_type(node: &SyntaxNode, out: &mut Output) {
    if !node.text.is_empty() {
        out.write(node.text);
        return;
    }
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                out.space();
            }
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => format_node(child, out),
        }
    }
}

fn format_block(node: &SyntaxNode, out: &mut Output) {
    let open_brace = node.children.iter().find(|c| c.kind == SyntaxKind::Token && c.text == "{");
    let has_stmts = node.children.iter().any(|c| matches!(c.kind, SyntaxKind::Statement(_)));
    let has_open_comments = open_brace
        .map(|b| b.children.iter().any(|c| matches!(c.kind, SyntaxKind::CommentLine | SyntaxKind::CommentBlock)))
        .unwrap_or(false);

    out.write("{");
    if has_stmts || has_open_comments {
        out.newline();
        out.indented(|out| {
            // Emit comments attached as trivia to the opening brace.
            if let Some(brace) = open_brace {
                emit_trivia(&brace.children, out);
            }
            for child in &node.children {
                if matches!(child.kind, SyntaxKind::Statement(_)) {
                    format_node(child, out);
                    out.ensure_newline();
                }
            }
        });
    }
    out.write("}");
    out.set_mark();
    // Emit comments that trail after the closing brace (between items).
    let close_brace = node.children.iter().find(|c| c.kind == SyntaxKind::Token && c.text == "}");
    if let Some(brace) = close_brace {
        emit_trivia(&brace.children, out);
    }
}

fn format_return(node: &SyntaxNode, out: &mut Output) {
    out.write("return");
    let has_expr = node.children.iter().any(|c| matches!(c.kind, SyntaxKind::Expression(_)));
    if has_expr {
        out.space();
    }
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "return" => {}
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_literal(node: &SyntaxNode, out: &mut Output) {
    if !node.text.is_empty() {
        out.write(node.text);
    }
    // Suffix is encoded in the kind, not the text
    if let SyntaxKind::Expression(ExpressionKind::Literal(lit)) = &node.kind {
        let suffix = match lit {
            LiteralKind::Integer(int) => match int {
                IntegerLiteralKind::U8 => "u8",
                IntegerLiteralKind::U16 => "u16",
                IntegerLiteralKind::U32 => "u32",
                IntegerLiteralKind::U64 => "u64",
                IntegerLiteralKind::U128 => "u128",
                IntegerLiteralKind::I8 => "i8",
                IntegerLiteralKind::I16 => "i16",
                IntegerLiteralKind::I32 => "i32",
                IntegerLiteralKind::I64 => "i64",
                IntegerLiteralKind::I128 => "i128",
            },
            LiteralKind::Field => "field",
            LiteralKind::Group => "group",
            LiteralKind::Scalar => "scalar",
            LiteralKind::Address
            | LiteralKind::Boolean
            | LiteralKind::String
            | LiteralKind::None
            | LiteralKind::Unsuffixed => "",
        };
        out.write(suffix);
    }
}

fn format_call(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            _ => format_node(child, out),
        }
    }
}

/// Binary expressions have the structure: [lhs, operator, rhs].
fn format_binary(node: &SyntaxNode, out: &mut Output) {
    let children: Vec<_> = node.children.iter().collect();
    format_node(children[0], out);
    out.space();
    out.write(children[1].text);
    out.space();
    format_node(children[2], out);
}

fn format_path(node: &SyntaxNode, out: &mut Output) {
    if !node.text.is_empty() {
        out.write(node.text);
    } else {
        for child in &node.children {
            format_node(child, out);
        }
    }
}

// --- Storage and Global Const ---

fn format_storage(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "storage" => {
                out.write("storage");
                out.space();
            }
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                out.set_mark();
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_global_const(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "const" => {
                out.write("const");
                out.space();
            }
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "=" => {
                out.space();
                out.write("=");
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                out.set_mark();
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

// --- Annotations ---

fn format_annotation_list(node: &SyntaxNode, out: &mut Output) {
    let members: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::AnnotationMember)).collect();

    out.write("(");
    for (i, child) in members.iter().enumerate() {
        format_annotation_member(child, out);
        if i < members.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write(")");
}

fn format_annotation_member(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "=" => {
                out.space();
                out.write("=");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            _ => {}
        }
    }
}

// --- Const Parameters and Arguments ---

fn format_parameter_list(node: &SyntaxNode, out: &mut Output) {
    let params: Vec<_> =
        node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Parameter | SyntaxKind::FunctionOutput)).collect();

    out.write("(");
    for (i, child) in params.iter().enumerate() {
        format_node(child, out);
        if i < params.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write(")");
}

fn format_const_parameter(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
}

fn format_const_parameter_list(node: &SyntaxNode, out: &mut Output) {
    let params: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::ConstParameter)).collect();

    out.write("::[");
    for (i, child) in params.iter().enumerate() {
        format_const_parameter(child, out);
        if i < params.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write("]");
}

fn format_const_argument_list(node: &SyntaxNode, out: &mut Output) {
    let args: Vec<_> =
        node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Expression(_) | SyntaxKind::Type(_))).collect();

    out.write("::[");
    for (i, child) in args.iter().enumerate() {
        format_node(child, out);
        if i < args.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write("]");
}

// --- Statement Formatters ---

fn format_definition(node: &SyntaxNode, out: &mut Output) {
    out.write("let");
    out.space();

    let has_paren = node.children.iter().any(|c| c.kind == SyntaxKind::Token && c.text == "(");

    if has_paren {
        // Tuple destructuring: let (a, b) = expr;
        let mut in_parens = false;
        for child in &node.children {
            match &child.kind {
                SyntaxKind::Token if child.text == "let" => {}
                SyntaxKind::Token if child.text == "(" => {
                    out.write("(");
                    in_parens = true;
                }
                SyntaxKind::Token if child.text == ")" => {
                    out.write(")");
                    in_parens = false;
                }
                SyntaxKind::Token if child.text == "," && in_parens => {
                    out.write(",");
                    out.space();
                }
                SyntaxKind::Token if child.text == ":" => {
                    out.write(":");
                    out.space();
                }
                SyntaxKind::Token if child.text == "=" => {
                    out.space();
                    out.write("=");
                    out.space();
                }
                SyntaxKind::Token if child.text == ";" => {
                    out.write(";");
                    emit_trivia(&child.children, out);
                }
                SyntaxKind::Token if in_parens => {
                    out.write(child.text);
                }
                SyntaxKind::Token => out.write(child.text),
                SyntaxKind::Type(_) => format_type(child, out),
                SyntaxKind::Expression(_) => format_node(child, out),
                _ => {}
            }
        }
    } else {
        // Simple: let x = expr; or let x: T = expr;
        for child in &node.children {
            match &child.kind {
                SyntaxKind::Token if child.text == "let" => {}
                SyntaxKind::Token if child.text == ":" => {
                    out.write(":");
                    out.space();
                }
                SyntaxKind::Token if child.text == "=" => {
                    out.space();
                    out.write("=");
                    out.space();
                }
                SyntaxKind::Token if child.text == ";" => {
                    out.write(";");
                    emit_trivia(&child.children, out);
                }
                SyntaxKind::Token => out.write(child.text),
                SyntaxKind::Type(_) => format_type(child, out),
                SyntaxKind::Expression(_) => format_node(child, out),
                _ => {}
            }
        }
    }
}

fn format_const_stmt(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "const" => {
                out.write("const");
                out.space();
            }
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "=" => {
                out.space();
                out.write("=");
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_assign(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if is_spaced_operator(child.text) => {
                out.space();
                out.write(child.text);
                out.space();
            }
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_conditional(node: &SyntaxNode, out: &mut Output) {
    let mut first_block = true;
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "if" => {
                out.write("if");
                out.space();
            }
            SyntaxKind::Token if child.text == "else" => {
                out.space();
                out.write("else");
                out.space();
            }
            SyntaxKind::Expression(_) => format_node(child, out),
            SyntaxKind::Statement(StatementKind::Block) => {
                if first_block {
                    out.space();
                    first_block = false;
                }
                format_block(child, out);
            }
            SyntaxKind::Statement(StatementKind::Conditional) => {
                // else if chain
                format_conditional(child, out);
            }
            _ => {}
        }
    }
}

fn format_iteration(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "for" => {
                out.write("for");
                out.space();
            }
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "in" => {
                out.space();
                out.write("in");
                out.space();
            }
            SyntaxKind::Token if child.text == ".." => {
                out.write("..");
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            SyntaxKind::Expression(_) => format_node(child, out),
            SyntaxKind::Statement(StatementKind::Block) => {
                out.space();
                format_block(child, out);
            }
            _ => {}
        }
    }
}

fn format_assert(node: &SyntaxNode, out: &mut Output, keyword: &str) {
    out.write(keyword);
    out.write("(");
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token
                if child.text == keyword || child.text == "(" || child.text == ")" || child.text == ";" => {}
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
    out.write(");");
    if let Some(semi) = node.children.iter().find(|c| c.kind == SyntaxKind::Token && c.text == ";") {
        emit_trivia(&semi.children, out);
    }
}

fn format_assert_eq(node: &SyntaxNode, out: &mut Output) {
    format_assert_pair(node, out, "assert_eq");
}

fn format_assert_neq(node: &SyntaxNode, out: &mut Output) {
    format_assert_pair(node, out, "assert_neq");
}

fn format_assert_pair(node: &SyntaxNode, out: &mut Output, keyword: &str) {
    out.write(keyword);
    out.write("(");
    let mut first = true;
    for child in &node.children {
        if matches!(child.kind, SyntaxKind::Expression(_)) {
            if !first {
                out.write(",");
                out.space();
            }
            format_node(child, out);
            first = false;
        }
    }
    out.write(");");
    if let Some(semi) = node.children.iter().find(|c| c.kind == SyntaxKind::Token && c.text == ";") {
        emit_trivia(&semi.children, out);
    }
}

fn format_expr_stmt(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ";" => {
                out.write(";");
                emit_trivia(&child.children, out);
            }
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

// --- Expression Formatters ---

fn format_unary(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_ternary(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Expression(_))).collect();
    if exprs.len() >= 3 {
        format_node(exprs[0], out);
        out.space();
        out.write("?");
        out.space();
        format_node(exprs[1], out);
        out.space();
        out.write(":");
        out.space();
        format_node(exprs[2], out);
    }
}

fn format_method_call(node: &SyntaxNode, out: &mut Output) {
    let mut after_dot = false;
    let mut in_args = false;

    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "." => {
                out.write(".");
                after_dot = true;
            }
            SyntaxKind::Token if child.text == "(" => {
                out.write("(");
                in_args = true;
            }
            SyntaxKind::Token if child.text == ")" => {
                out.write(")");
                in_args = false;
            }
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.space();
            }
            SyntaxKind::Token if after_dot && !in_args => {
                out.write(child.text);
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) if !in_args => {
                format_node(child, out);
            }
            SyntaxKind::Expression(_) => {
                format_node(child, out);
            }
            _ => {}
        }
    }
}

fn format_assoc_call(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::ConstArgumentList => format_const_argument_list(child, out),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_assoc_const(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        if let SyntaxKind::Token = &child.kind {
            out.write(child.text);
        }
    }
}

fn format_composite_expr(node: &SyntaxNode, out: &mut Output) {
    let inits: Vec<_> =
        node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::CompositeMemberInitializer)).collect();

    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "{" => {
                out.space();
                out.write("{");
                if !inits.is_empty() {
                    out.space();
                    // Format all initializers inline
                    for (i, init) in inits.iter().enumerate() {
                        format_composite_member_init(init, out);
                        if i < inits.len() - 1 {
                            out.write(",");
                            out.space();
                        }
                    }
                    out.space();
                }
            }
            SyntaxKind::Token if child.text == "}" => {
                out.write("}");
            }
            SyntaxKind::Token if child.text == "," => {} // handled above with initializers
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::ConstArgumentList => format_const_argument_list(child, out),
            SyntaxKind::CompositeMemberInitializer => {} // handled above
            _ => {}
        }
    }
}

fn format_composite_member_init(node: &SyntaxNode, out: &mut Output) {
    let has_colon = node.children.iter().any(|c| c.kind == SyntaxKind::Token && c.text == ":");
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) if has_colon => format_node(child, out),
            _ => {}
        }
    }
}

fn format_array_access(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "[" || child.text == "]" => out.write(child.text),
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_tuple_access(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_member_access(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_cast(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "as" => {
                out.space();
                out.write("as");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
}

fn format_array_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Expression(_))).collect();

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

fn format_tuple_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Expression(_))).collect();

    out.write("(");
    for (i, expr) in exprs.iter().enumerate() {
        format_node(expr, out);
        if i < exprs.len() - 1 {
            out.write(",");
            out.space();
        }
    }
    out.write(")");
}

fn format_parenthesized(node: &SyntaxNode, out: &mut Output) {
    out.write("(");
    for child in &node.children {
        if let SyntaxKind::Expression(_) = &child.kind {
            format_node(child, out);
        }
    }
    out.write(")");
}

fn format_repeat_expr(node: &SyntaxNode, out: &mut Output) {
    let exprs: Vec<_> = node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Expression(_))).collect();

    out.write("[");
    if exprs.len() >= 2 {
        format_node(exprs[0], out);
        out.write(";");
        out.space();
        format_node(exprs[1], out);
    }
    out.write("]");
}

fn format_async_expr(node: &SyntaxNode, out: &mut Output) {
    out.write("async");
    out.space();
    for child in &node.children {
        if let SyntaxKind::Statement(StatementKind::Block) = &child.kind {
            format_block(child, out);
        }
    }
}

fn format_intrinsic(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.space();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Expression(_) => format_node(child, out),
            _ => {}
        }
    }
}

fn format_special_access(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        if let SyntaxKind::Token = &child.kind {
            out.write(child.text);
        }
    }
}

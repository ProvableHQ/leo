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

/// Format any syntax node.
pub fn format_node(node: &SyntaxNode, out: &mut Output) {
    match &node.kind {
        SyntaxKind::MainContents => format_main(node, out),
        SyntaxKind::ProgramDeclaration => format_program(node, out),
        SyntaxKind::ModuleContents => format_module_contents(node, out),
        SyntaxKind::Function => format_function(node, out),
        SyntaxKind::Constructor => format_function(node, out),
        SyntaxKind::CompositeDeclaration => format_struct(node, out),
        SyntaxKind::Import => format_import(node, out),
        SyntaxKind::Mapping => format_mapping(node, out),
        SyntaxKind::Annotation => format_annotation(node, out),
        SyntaxKind::ParameterList => format_delimited(node, out, "(", ")", ","),
        SyntaxKind::Parameter => format_parameter(node, out),
        SyntaxKind::FunctionOutputs => format_function_outputs(node, out),
        SyntaxKind::FunctionOutput => format_function_output(node, out),
        SyntaxKind::CompositeMemberDeclarationList => format_composite_member_list(node, out),
        SyntaxKind::CompositeMemberDeclaration => format_composite_member(node, out),
        SyntaxKind::Statement(StatementKind::Block) => format_block(node, out),
        SyntaxKind::Statement(StatementKind::Return) => format_return(node, out),
        SyntaxKind::Expression(ExpressionKind::Literal(_)) => format_literal(node, out),
        SyntaxKind::Expression(ExpressionKind::Call) => format_call(node, out),
        SyntaxKind::Expression(ExpressionKind::Binary) => format_binary(node, out),
        SyntaxKind::Expression(ExpressionKind::Path) => format_path(node, out),
        SyntaxKind::Type(_) => format_type(node, out),
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
            emit_comments(&node.children, out);
        }
        // TODO: Handle remaining statement and expression formatting.
        _ => format_children(node, out),
    }
}

fn format_children(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        format_node(child, out);
    }
}

fn emit_comments(children: &[SyntaxNode], out: &mut Output) {
    for child in children {
        match child.kind {
            SyntaxKind::CommentLine => {
                out.space();
                out.write(child.text.trim_end());
                out.newline();
            }
            SyntaxKind::CommentBlock => {
                out.space();
                out.write(child.text);
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

fn format_program(node: &SyntaxNode, out: &mut Output) {
    // Collect items (functions, structs, mappings)
    let items: Vec<_> = node
        .children
        .iter()
        .filter(|c| {
            matches!(
                c.kind,
                SyntaxKind::Function | SyntaxKind::Constructor | SyntaxKind::CompositeDeclaration | SyntaxKind::Mapping
            )
        })
        .collect();

    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "program" => {
                out.write("program");
                out.space();
            }
            SyntaxKind::Token if child.text == "{" => {
                out.space();
                out.write("{");
                if !items.is_empty() {
                    out.newline();
                }
            }
            SyntaxKind::Token if child.text == "}" => {
                out.write("}");
                out.newline();
            }
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Function | SyntaxKind::Constructor | SyntaxKind::CompositeDeclaration | SyntaxKind::Mapping => {
                out.indented(|out| {
                    format_node(child, out);
                    // Add blank line between items (but not after the last one)
                    let is_last = items.iter().position(|x| std::ptr::eq(*x, child)) == Some(items.len() - 1);
                    if !is_last {
                        out.newline();
                    }
                });
            }
            _ => {}
        }
    }
}

fn format_module_contents(node: &SyntaxNode, out: &mut Output) {
    let items: Vec<_> = node
        .children
        .iter()
        .filter(|c| {
            matches!(
                c.kind,
                SyntaxKind::Function | SyntaxKind::Constructor | SyntaxKind::CompositeDeclaration | SyntaxKind::Mapping
            )
        })
        .collect();

    for child in &node.children {
        match &child.kind {
            SyntaxKind::Function | SyntaxKind::Constructor | SyntaxKind::CompositeDeclaration | SyntaxKind::Mapping => {
                format_node(child, out);
                let is_last = items.iter().position(|x| std::ptr::eq(*x, child)) == Some(items.len() - 1);
                if !is_last {
                    out.newline();
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
            SyntaxKind::ParameterList => format_delimited(child, out, "(", ")", ","),
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
        if let SyntaxKind::Token = &child.kind {
            out.write(child.text);
        }
    }
    out.newline();
}

fn format_struct(node: &SyntaxNode, out: &mut Output) {
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
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == "{" => {
                out.write("{");
                out.newline();
            }
            SyntaxKind::Token if child.text == "}" => {
                out.write("}");
            }
            SyntaxKind::Token if child.text == "," => {
                out.write(",");
                out.newline();
            }
            SyntaxKind::CompositeMemberDeclaration => {
                out.indented(|out| {
                    format_composite_member(child, out);
                });
            }
            _ => {}
        }
    }
}

fn format_composite_member(node: &SyntaxNode, out: &mut Output) {
    for child in &node.children {
        match &child.kind {
            SyntaxKind::Token if child.text == ":" => {
                out.write(":");
                out.space();
            }
            SyntaxKind::Token if child.text == "," => out.write(","),
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
            SyntaxKind::Token if child.text == ";" => out.write(";"),
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
            SyntaxKind::Token if child.text == ";" => out.write(";"),
            SyntaxKind::Token => out.write(child.text),
            SyntaxKind::Type(_) => format_type(child, out),
            _ => {}
        }
    }
    out.ensure_newline();
}

fn format_delimited(node: &SyntaxNode, out: &mut Output, open: &str, close: &str, sep: &str) {
    out.write(open);
    let items: Vec<_> =
        node.children.iter().filter(|c| matches!(c.kind, SyntaxKind::Parameter | SyntaxKind::FunctionOutput)).collect();

    for (i, child) in items.iter().enumerate() {
        format_node(child, out);
        if i < items.len() - 1 {
            out.write(sep);
            out.space();
        }
    }
    out.write(close);
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
        format_delimited(node, out, "(", ")", ",");
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
    out.write("{");
    let has_stmts = node.children.iter().any(|c| matches!(c.kind, SyntaxKind::Statement(_)));
    if has_stmts {
        out.newline();
        out.indented(|out| {
            for child in &node.children {
                if matches!(child.kind, SyntaxKind::Statement(_)) {
                    format_node(child, out);
                    out.ensure_newline();
                }
            }
        });
    }
    out.write("}");
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
            SyntaxKind::Token if child.text == ";" => out.write(";"),
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
            // TODO: Handle expression formatting.
            _ => format_node(child, out),
        }
    }
}

/// Binary expressions have the structure: [lhs, operator, rhs].
fn format_binary(node: &SyntaxNode, out: &mut Output) {
    let children: Vec<_> = node.children.iter().collect();
    if children.len() >= 3 {
        format_node(children[0], out);
        out.space();
        out.write(children[1].text);
        out.space();
        format_node(children[2], out);
    } else {
        format_children(node, out);
    }
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

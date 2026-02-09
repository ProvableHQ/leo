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

//! Rowan-based parser implementation.
//!
//! This module uses `leo-parser-rowan` (rowan) and converts its output
//! to the Leo AST via the `ConversionContext`.

// Implementation notes:
//
// - All user-reachable errors should be emitted via the error handler (mostly
//   `ERROR` nodes).
// - All implementation bugs (e.g. unexpected node structure from
//   `leo_parser_rowan`) should `panic!` in order to catch logic bugs in the
//   compiler.

use itertools::Itertools as _;
use snarkvm::prelude::{Address, TestnetV0};

use leo_ast::{NetworkName, NodeBuilder};
use leo_errors::{Handler, ParserError, Result};
use leo_parser_rowan::{SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken};
use leo_span::{
    Span,
    Symbol,
    source_map::{FileName, SourceFile},
    sym,
};

// =============================================================================
// ConversionContext
// =============================================================================

/// Context for converting rowan syntax nodes to Leo AST nodes.
struct ConversionContext<'a> {
    #[allow(dead_code)]
    handler: &'a Handler,
    #[allow(dead_code)]
    builder: &'a NodeBuilder,
    /// The absolute start position to offset spans by.
    start_pos: u32,
}

impl<'a> ConversionContext<'a> {
    /// Create a new conversion context.
    fn new(handler: &'a Handler, builder: &'a NodeBuilder, start_pos: u32) -> Self {
        Self { handler, builder, start_pos }
    }

    // =========================================================================
    // Utility Methods
    // =========================================================================

    /// Convert a rowan TextRange to a leo_span::Span.
    fn to_span(&self, node: &SyntaxNode) -> Span {
        let range = node.text_range();
        Span::new(
            u32::from(range.start()) + self.start_pos,
            u32::from(range.end()) + self.start_pos,
        )
    }

    /// Convert a token's text range to a Span.
    fn token_span(&self, token: &SyntaxToken) -> Span {
        let range = token.text_range();
        Span::new(
            u32::from(range.start()) + self.start_pos,
            u32::from(range.end()) + self.start_pos,
        )
    }

    /// Convert an IDENT token to a leo_ast::Identifier.
    fn to_identifier(&self, token: &SyntaxToken) -> leo_ast::Identifier {
        debug_assert_eq!(token.kind(), IDENT);
        leo_ast::Identifier {
            name: Symbol::intern(token.text()),
            span: self.token_span(token),
            id: self.builder.next_id(),
        }
    }

    // =========================================================================
    // Type Conversions
    // =========================================================================

    /// Convert a type syntax node to a Leo AST Type.
    fn to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        let ty = match node.kind() {
            TYPE_PATH => self.type_path_to_type(node)?,
            TYPE_ARRAY => self.type_array_to_type(node)?,
            TYPE_TUPLE => self.type_tuple_to_type(node)?,
            TYPE_OPTIONAL => self.type_optional_to_type(node)?,
            TYPE_FUTURE => self.type_future_to_type(node)?,
            TYPE_MAPPING => {
                // Mapping types appear in storage contexts, handled elsewhere
                panic!("TYPE_MAPPING should be handled in storage context")
            }
            ERROR => {
                // Error recovery: return Err type for ERROR nodes
                self.handler.emit_err(ParserError::unexpected_str(
                    "valid type",
                    node.text().to_string(),
                    self.to_span(node),
                ));
                leo_ast::Type::Err
            }
            kind => panic!("unexpected type node kind: {:?}", kind),
        };
        Ok(ty)
    }

    /// Convert a TYPE_PATH node to a Type.
    ///
    /// TYPE_PATH can represent:
    /// - Primitive types (bool, u32, field, etc.) via type keywords
    /// - Named composite types (Foo, Foo::Bar)
    /// - Locators (program.aleo/Type)
    /// - Const generic types (Foo::[N])
    fn type_path_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_PATH);

        // Check for primitive type keywords first
        let first_token = tokens(node).next();
        if let Some(ref token) = first_token {
            if let Some(prim) = keyword_to_primitive_type(token.kind()) {
                return Ok(prim);
            }
        }

        // It's a named/composite type - collect path components
        let text = node.text().to_string();

        // Check for locator: program.aleo/Type
        if let Some((program_str, type_str)) = text.split_once(".aleo/") {
            let span = self.to_span(node);
            let program_span = Span::new(span.lo, span.lo + program_str.len() as u32);
            let type_span = Span::new(span.lo + program_str.len() as u32 + 6, span.hi);

            let program = leo_ast::Identifier {
                name: Symbol::intern(program_str),
                span: program_span,
                id: self.builder.next_id(),
            };
            let type_name =
                leo_ast::Identifier { name: Symbol::intern(type_str), span: type_span, id: self.builder.next_id() };

            let path = leo_ast::Path::new(Some(program), Vec::new(), type_name, span, self.builder.next_id());
            return Ok(leo_ast::CompositeType { path, const_arguments: Vec::new() }.into());
        }

        // Check for program.aleo without /Type
        if text.ends_with(".aleo") {
            let program_str = text.trim_end_matches(".aleo");
            let span = self.to_span(node);

            let program = leo_ast::Identifier { name: Symbol::intern(program_str), span, id: self.builder.next_id() };

            // Just the program name, no type - this is a program ID reference
            let path = leo_ast::Path::new(Some(program), Vec::new(), program.clone(), span, self.builder.next_id());
            return Ok(leo_ast::CompositeType { path, const_arguments: Vec::new() }.into());
        }

        // Regular path: collect identifiers and const generic args
        let mut path_components = Vec::new();
        let mut const_arguments = Vec::new();
        let span = self.to_span(node);

        for token in tokens(node) {
            match token.kind() {
                IDENT => {
                    path_components.push(self.to_identifier(&token));
                }
                INTEGER => {
                    // Const generic argument
                    let expr = self.integer_token_to_expression(&token)?;
                    const_arguments.push(expr);
                }
                // Skip punctuation
                COLON_COLON | L_BRACKET | R_BRACKET | LT | GT | DOT | COMMA => {}
                KW_ALEO => {}
                kind => panic!("unexpected token in TYPE_PATH: {:?}", kind),
            }
        }

        // The last component is the type name, rest are path segments
        let name = path_components.pop().expect("TYPE_PATH should have at least one identifier");
        let path = leo_ast::Path::new(None, path_components, name, span, self.builder.next_id());
        Ok(leo_ast::CompositeType { path, const_arguments }.into())
    }

    /// Convert a TYPE_ARRAY node to an ArrayType or VectorType.
    fn type_array_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_ARRAY);

        let mut type_children = children(node)
            .filter(|n| matches!(n.kind(), TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_OPTIONAL | TYPE_FUTURE));
        let element_node = type_children.next().expect("array type should have element type");
        let element_type = self.to_type(&element_node)?;

        // Check if there's a length (INTEGER or IDENT token after semicolon)
        let has_length = tokens(node).any(|t| t.kind() == SEMICOLON);

        if has_length {
            // Array with explicit length: [T; N]
            // Find the length expression (INTEGER or IDENT after semicolon)
            let length_expr = self.array_length_to_expression(node)?;
            Ok(leo_ast::ArrayType { element_type: Box::new(element_type), length: Box::new(length_expr) }.into())
        } else {
            // Vector: [T]
            Ok(leo_ast::VectorType { element_type: Box::new(element_type) }.into())
        }
    }

    /// Extract the array length expression from a TYPE_ARRAY node.
    fn array_length_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        // Find tokens after the semicolon
        let mut found_semicolon = false;
        for token in tokens(node) {
            if token.kind() == SEMICOLON {
                found_semicolon = true;
                continue;
            }
            if found_semicolon {
                match token.kind() {
                    INTEGER => return self.integer_token_to_expression(&token),
                    IDENT => {
                        // Const generic name - wrap as a Path expression
                        let ident = self.to_identifier(&token);
                        let span = ident.span;
                        let path = leo_ast::Path::new(None, Vec::new(), ident, span, self.builder.next_id());
                        return Ok(leo_ast::Expression::Path(path));
                    }
                    _ => continue,
                }
            }
        }
        // Error recovery: missing length after semicolon
        let span = self.to_span(node);
        self.handler.emit_err(ParserError::unexpected_str("array length", node.text().to_string(), span));
        Ok(leo_ast::ErrExpression { span, id: self.builder.next_id() }.into())
    }

    /// Convert an INTEGER token to an Expression.
    fn integer_token_to_expression(&self, token: &SyntaxToken) -> Result<leo_ast::Expression> {
        debug_assert_eq!(token.kind(), INTEGER);
        let text = token.text();
        let span = self.token_span(token);
        let id = self.builder.next_id();

        // Check for integer type suffix
        let suffixes = [
            ("u128", leo_ast::IntegerType::U128),
            ("u64", leo_ast::IntegerType::U64),
            ("u32", leo_ast::IntegerType::U32),
            ("u16", leo_ast::IntegerType::U16),
            ("u8", leo_ast::IntegerType::U8),
            ("i128", leo_ast::IntegerType::I128),
            ("i64", leo_ast::IntegerType::I64),
            ("i32", leo_ast::IntegerType::I32),
            ("i16", leo_ast::IntegerType::I16),
            ("i8", leo_ast::IntegerType::I8),
        ];

        for (suffix, int_type) in suffixes {
            if text.ends_with(suffix) {
                // Suffixed integer - preserve underscores in value
                let value = text.strip_suffix(suffix).unwrap().to_string();
                return Ok(leo_ast::Literal::integer(int_type, value, span, id).into());
            }
        }

        // No suffix - use Unsuffixed variant (preserving underscores)
        Ok(leo_ast::Literal::unsuffixed(text.to_string(), span, id).into())
    }

    /// Convert a TYPE_TUPLE node to a TupleType or Unit.
    fn type_tuple_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_TUPLE);

        let type_nodes: Vec<_> = children(node)
            .filter(|n| matches!(n.kind(), TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_OPTIONAL | TYPE_FUTURE))
            .collect();

        if type_nodes.is_empty() {
            // Unit type: ()
            return Ok(leo_ast::Type::Unit);
        }

        let elements = type_nodes.iter().map(|n| self.to_type(n)).collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::TupleType::new(elements).into())
    }

    /// Convert a TYPE_OPTIONAL node to an OptionalType.
    fn type_optional_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_OPTIONAL);

        let inner_node = children(node)
            .find(|n| matches!(n.kind(), TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_FUTURE))
            .expect("optional type should have inner type");

        let inner = self.to_type(&inner_node)?;
        Ok(leo_ast::Type::Optional(leo_ast::OptionalType { inner: Box::new(inner) }))
    }

    /// Convert a TYPE_FUTURE node to a FutureType.
    fn type_future_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_FUTURE);

        // Collect any type children (for Future<fn(T) -> R> syntax)
        let type_nodes: Vec<_> = children(node)
            .filter(|n| matches!(n.kind(), TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_OPTIONAL | TYPE_FUTURE))
            .collect();

        if type_nodes.is_empty() {
            // Simple Future with no explicit signature
            return Ok(leo_ast::FutureType::default().into());
        }

        // Future with explicit signature: Future<fn(T1, T2) -> R>
        let types = type_nodes.iter().map(|n| self.to_type(n)).collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::FutureType::new(types, None, true).into())
    }

    // =========================================================================
    // Expression Conversions
    // =========================================================================

    /// Convert a syntax node to an expression.
    fn to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let expr = match node.kind() {
            LITERAL => self.literal_to_expression(node)?,
            BINARY_EXPR => self.binary_expr_to_expression(node)?,
            UNARY_EXPR => self.unary_expr_to_expression(node)?,
            CALL_EXPR => self.call_expr_to_expression(node)?,
            FIELD_EXPR => self.field_expr_to_expression(node)?,
            INDEX_EXPR => self.index_expr_to_expression(node)?,
            CAST_EXPR => self.cast_expr_to_expression(node)?,
            TERNARY_EXPR => self.ternary_expr_to_expression(node)?,
            ARRAY_EXPR => self.array_expr_to_expression(node)?,
            TUPLE_EXPR => self.tuple_expr_to_expression(node)?,
            STRUCT_EXPR => self.struct_expr_to_expression(node)?,
            PATH_EXPR => self.path_expr_to_expression(node)?,
            PAREN_EXPR => {
                // Parenthesized expression - just unwrap
                if let Some(inner) = children(node).find(|n| is_expression_kind(n.kind())) {
                    self.to_expression(&inner)?
                } else {
                    // No inner expression found - likely parse error
                    self.handler.emit_err(ParserError::unexpected_str(
                        "expression in parentheses",
                        node.text().to_string(),
                        span,
                    ));
                    leo_ast::ErrExpression { span, id }.into()
                }
            }
            UNIT_EXPR => leo_ast::UnitExpression { span, id }.into(),
            // Async expression block
            ASYNC_EXPR => self.async_expr_to_expression(node)?,
            // For ROOT nodes that wrap an expression (from parse_expression_entry)
            ROOT => {
                if let Some(inner) = children(node).find(|n| is_expression_kind(n.kind())) {
                    self.to_expression(&inner)?
                } else {
                    // No valid expression found - likely parse error
                    self.handler.emit_err(ParserError::unexpected_str(
                        "valid expression",
                        node.text().to_string(),
                        span,
                    ));
                    leo_ast::ErrExpression { span, id }.into()
                }
            }
            // Error recovery: return ErrExpression for ERROR nodes
            ERROR => {
                self.handler.emit_err(ParserError::unexpected_str(
                    "valid expression",
                    node.text().to_string(),
                    span,
                ));
                leo_ast::ErrExpression { span, id }.into()
            }
            kind => panic!("unexpected expression kind: {:?}", kind),
        };

        Ok(expr)
    }

    /// Convert a LITERAL node to an expression.
    fn literal_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), LITERAL);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get the token inside the LITERAL node
        let token = tokens(node).next().expect("LITERAL should have a token");
        let text = token.text();

        let expr = match token.kind() {
            INTEGER => {
                // Check for special suffixes: field, group, scalar
                if let Some(value) = text.strip_suffix("field") {
                    // Hex, octal, and binary literals are not allowed for field type
                    if text.starts_with("0x") || text.starts_with("0o") || text.starts_with("0b") {
                        // Error span only covers the numeric prefix, not the suffix
                        let prefix_span = Span::new(span.lo, span.hi - "field".len() as u32);
                        self.handler.emit_err(ParserError::hexbin_literal_nonintegers(prefix_span));
                    }
                    leo_ast::Literal::field(value.to_string(), span, id).into()
                } else if let Some(value) = text.strip_suffix("group") {
                    // Hex, octal, and binary literals are not allowed for group type
                    if text.starts_with("0x") || text.starts_with("0o") || text.starts_with("0b") {
                        let prefix_span = Span::new(span.lo, span.hi - "group".len() as u32);
                        self.handler.emit_err(ParserError::hexbin_literal_nonintegers(prefix_span));
                    }
                    leo_ast::Literal::group(value.to_string(), span, id).into()
                } else if let Some(value) = text.strip_suffix("scalar") {
                    // Hex, octal, and binary literals are not allowed for scalar type
                    if text.starts_with("0x") || text.starts_with("0o") || text.starts_with("0b") {
                        let prefix_span = Span::new(span.lo, span.hi - "scalar".len() as u32);
                        self.handler.emit_err(ParserError::hexbin_literal_nonintegers(prefix_span));
                    }
                    leo_ast::Literal::scalar(value.to_string(), span, id).into()
                } else {
                    // Regular integer
                    self.integer_token_to_expression(&token)?
                }
            }
            STRING => leo_ast::Literal::string(text.to_string(), span, id).into(),
            ADDRESS_LIT => {
                    // Validate address literal (skip program addresses like "program.aleo")
                    if !text.contains(".aleo") && text.parse::<Address<TestnetV0>>().is_err() {
                        self.handler.emit_err(ParserError::invalid_address_lit(text, span));
                    }
                    leo_ast::Literal::address(text.to_string(), span, id).into()
                }
            KW_TRUE => leo_ast::Literal::boolean(true, span, id).into(),
            KW_FALSE => leo_ast::Literal::boolean(false, span, id).into(),
            KW_NONE => leo_ast::Literal::none(span, id).into(),
            kind => panic!("unexpected literal token kind: {:?}", kind),
        };

        Ok(expr)
    }

    /// Convert a BINARY_EXPR node to a BinaryExpression.
    fn binary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), BINARY_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Collect expression children (left and right operands)
        let expr_children: Vec<_> =
            children(node).filter(|n| is_expression_kind(n.kind()) || is_type_kind(n.kind())).collect();

        // Find the operator token
        let op_token = match tokens(node).find(|t| t.kind().is_operator() || t.kind() == KW_AS) {
            Some(token) => token,
            None => {
                self.handler.emit_err(ParserError::unexpected_str(
                    "operator in binary expression",
                    node.text().to_string(),
                    span,
                ));
                return Ok(leo_ast::ErrExpression { span, id }.into());
            }
        };

        let op = token_to_binary_op(op_token.kind());

        // Get left operand
        let left = if let Some(left_node) = expr_children.first() {
            self.to_expression(left_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str(
                "left operand in binary expression",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        // Get right operand
        let right = if op_token.kind() == KW_AS {
            // Cast expression - should be handled as CAST_EXPR, emit error for unexpected case
            self.handler.emit_err(ParserError::unexpected_str(
                "cast expression",
                "binary AS expression",
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        } else if let Some(right_node) = expr_children.get(1) {
            self.to_expression(right_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str(
                "right operand in binary expression",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        Ok(leo_ast::BinaryExpression { left, right, op, span, id }.into())
    }

    /// Convert a UNARY_EXPR node to a UnaryExpression.
    fn unary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), UNARY_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get the operator
        let Some(op_token) = tokens(node).find(|t| matches!(t.kind(), BANG | MINUS)) else {
            self.handler.emit_err(ParserError::unexpected_str(
                "operator in unary expression",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        let op = match op_token.kind() {
            BANG => leo_ast::UnaryOperation::Not,
            MINUS => leo_ast::UnaryOperation::Negate,
            _ => {
                self.handler.emit_err(ParserError::unexpected_str(
                    "! or -",
                    node.text().to_string(),
                    span,
                ));
                return Ok(leo_ast::ErrExpression { span, id }.into());
            }
        };

        // Get the operand
        let Some(operand) = children(node).find(|n| is_expression_kind(n.kind())) else {
            self.handler.emit_err(ParserError::unexpected_str(
                "operand in unary expression",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        let mut receiver = self.to_expression(&operand)?;

        // Fold negation into numeric literals
        if op == leo_ast::UnaryOperation::Negate {
            use leo_ast::LiteralVariant::*;
            if let leo_ast::Expression::Literal(leo_ast::Literal {
                variant:
                    Integer(_, ref mut string) | Field(ref mut string) | Group(ref mut string) | Scalar(ref mut string),
                span: ref mut lit_span,
                ..
            }) = receiver
            {
                if !string.starts_with('-') {
                    string.insert(0, '-');
                    *lit_span = span;
                    return Ok(receiver);
                }
            }
        }

        Ok(leo_ast::UnaryExpression { receiver, op, span, id }.into())
    }

    /// Convert a CALL_EXPR node to a CallExpression.
    fn call_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), CALL_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // The first child should be the function being called (PATH_EXPR or FIELD_EXPR)
        let mut child_iter = children(node);
        let callee_node = child_iter.next().expect("call expr should have callee");

        // If callee is FIELD_EXPR, this is a method call
        if callee_node.kind() == FIELD_EXPR {
            return self.method_call_to_expression(node, &callee_node);
        }

        // Regular function call
        let function = self.path_expr_to_path(&callee_node)?;

        // Collect arguments (remaining expression children)
        let arguments = children(node)
            .skip(1)  // Skip the callee
            .filter(|n| is_expression_kind(n.kind()))
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::CallExpression { function, const_arguments: Vec::new(), arguments, span, id }.into())
    }

    /// Convert a method call (CALL_EXPR with FIELD_EXPR callee) to the appropriate expression.
    fn method_call_to_expression(
        &self,
        call_node: &SyntaxNode,
        field_node: &SyntaxNode,
    ) -> Result<leo_ast::Expression> {
        let span = self.to_span(call_node);
        let id = self.builder.next_id();

        // Get receiver and method name from FIELD_EXPR
        let mut field_children = children(field_node);
        let receiver = match field_children.next() {
            Some(receiver_node) => self.to_expression(&receiver_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("receiver in method call", field_node.text().to_string(), span));
                return Ok(leo_ast::ErrExpression { span, id }.into());
            }
        };

        // Get the method name (token after DOT)
        let method_name = match tokens(field_node).find(|t| t.kind() == IDENT) {
            Some(method_token) => self.to_identifier(&method_token),
            None => {
                self.handler.emit_err(ParserError::unexpected_str("method name in method call", field_node.text().to_string(), span));
                return Ok(leo_ast::ErrExpression { span, id }.into());
            }
        };

        // Collect arguments
        let mut args: Vec<_> = children(call_node)
            .skip(1)  // Skip the callee (FIELD_EXPR)
            .filter(|n| is_expression_kind(n.kind()))
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        // Check for known methods that map to unary/binary operations or intrinsics
        if args.is_empty() {
            if let Some(op) = leo_ast::UnaryOperation::from_symbol(method_name.name) {
                return Ok(leo_ast::UnaryExpression { span, op, receiver, id }.into());
            }
        } else if args.len() == 1 {
            if let Some(op) = leo_ast::BinaryOperation::from_symbol(method_name.name) {
                return Ok(
                    leo_ast::BinaryExpression { span, op, left: receiver, right: args.pop().unwrap(), id }.into()
                );
            }
        }

        // Check for mapping/vector intrinsics
        if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(sym::Mapping, method_name.name) {
            return Ok(leo_ast::IntrinsicExpression {
                name: intrinsic_name,
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(sym::Vector, method_name.name) {
            return Ok(leo_ast::IntrinsicExpression {
                name: intrinsic_name,
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(sym::Future, method_name.name) {
            return Ok(leo_ast::IntrinsicExpression {
                name: intrinsic_name,
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(sym::Optional, method_name.name) {
            return Ok(leo_ast::IntrinsicExpression {
                name: intrinsic_name,
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(sym::signature, method_name.name) {
            return Ok(leo_ast::IntrinsicExpression {
                name: intrinsic_name,
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        // Handle unresolved get/set for mappings
        if method_name.name == sym::get && args.len() == 1 {
            return Ok(leo_ast::IntrinsicExpression {
                name: Symbol::intern("__unresolved_get"),
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        if method_name.name == sym::set && args.len() == 2 {
            return Ok(leo_ast::IntrinsicExpression {
                name: Symbol::intern("__unresolved_set"),
                type_parameters: Vec::new(),
                arguments: std::iter::once(receiver).chain(args).collect(),
                span,
                id,
            }
            .into());
        }

        // Unknown method call - emit error
        self.handler.emit_err(ParserError::invalid_method_call(receiver, method_name, args.len(), span));
        Ok(leo_ast::ErrExpression { span, id }.into())
    }

    /// Convert a FIELD_EXPR node to a MemberAccess or TupleAccess expression.
    fn field_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), FIELD_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get the inner expression
        let inner = if let Some(inner_node) = children(node).find(|n| is_expression_kind(n.kind())) {
            self.to_expression(&inner_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str(
                "expression in field access",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        // Get the field name or tuple index (token after DOT)
        let field_token = match tokens(node).filter(|t| matches!(t.kind(), IDENT | INTEGER)).last() {
            Some(token) => token,
            None => {
                self.handler.emit_err(ParserError::unexpected_str(
                    "field name in field access",
                    node.text().to_string(),
                    span,
                ));
                return Ok(leo_ast::ErrExpression { span, id }.into());
            }
        };

        if field_token.kind() == INTEGER {
            // Tuple access: x.0
            let index_text = field_token.text().replace('_', "");
            let index: usize = match index_text.parse() {
                Ok(idx) => idx,
                Err(_) => {
                    self.handler.emit_err(ParserError::unexpected_str("valid tuple index", index_text, span));
                    return Ok(leo_ast::ErrExpression { span, id }.into());
                }
            };
            Ok(leo_ast::TupleAccess { tuple: inner, index: index.into(), span, id }.into())
        } else {
            // Member access: x.field
            // Check for special accesses: self.caller, block.height, etc.
            if let leo_ast::Expression::Path(ref path) = inner {
                let receiver_name = path.identifier().name;
                let field_name = Symbol::intern(field_token.text());

                let special = match (receiver_name, field_name) {
                    (sym::SelfLower, sym::address) => Some(sym::_self_address),
                    (sym::SelfLower, sym::caller) => Some(sym::_self_caller),
                    (sym::SelfLower, sym::checksum) => Some(sym::_self_checksum),
                    (sym::SelfLower, sym::edition) => Some(sym::_self_edition),
                    (sym::SelfLower, sym::id) => Some(sym::_self_id),
                    (sym::SelfLower, sym::program_owner) => Some(sym::_self_program_owner),
                    (sym::SelfLower, sym::signer) => Some(sym::_self_signer),
                    (sym::block, sym::height) => Some(sym::_block_height),
                    (sym::block, sym::timestamp) => Some(sym::_block_timestamp),
                    (sym::network, sym::id) => Some(sym::_network_id),
                    _ => None,
                };

                if let Some(intrinsic_name) = special {
                    return Ok(leo_ast::IntrinsicExpression {
                        name: intrinsic_name,
                        type_parameters: Vec::new(),
                        arguments: Vec::new(),
                        span,
                        id,
                    }
                    .into());
                }
            }

            let name = self.to_identifier(&field_token);
            Ok(leo_ast::MemberAccess { inner, name, span, id }.into())
        }
    }

    /// Convert an INDEX_EXPR node to an ArrayAccess expression.
    fn index_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), INDEX_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let expr_children: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();

        let array = if let Some(array_node) = expr_children.first() {
            self.to_expression(array_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("array in index expression", node.text().to_string(), span));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        let index = if let Some(index_node) = expr_children.get(1) {
            self.to_expression(index_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("index in index expression", node.text().to_string(), span));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        Ok(leo_ast::ArrayAccess { array, index, span, id }.into())
    }

    /// Convert a CAST_EXPR node to a CastExpression.
    fn cast_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), CAST_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get the expression being cast
        let Some(expr_node) = children(node).find(|n| is_expression_kind(n.kind())) else {
            self.handler.emit_err(ParserError::unexpected_str(
                "expression in cast",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };
        let expression = self.to_expression(&expr_node)?;

        // Get the target type
        let Some(type_node) = children(node).find(|n| is_type_kind(n.kind())) else {
            self.handler.emit_err(ParserError::unexpected_str(
                "type in cast expression",
                node.text().to_string(),
                span,
            ));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };
        let type_ = self.to_type(&type_node)?;

        Ok(leo_ast::CastExpression { expression, type_, span, id }.into())
    }

    /// Convert a TERNARY_EXPR node to a TernaryExpression.
    fn ternary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), TERNARY_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let expr_children: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();

        let condition = if let Some(node) = expr_children.first() {
            self.to_expression(node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("condition in ternary expression", node.text().to_string(), span));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        let if_true = if let Some(node) = expr_children.get(1) {
            self.to_expression(node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("true branch in ternary expression", node.text().to_string(), span));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        let if_false = if let Some(node) = expr_children.get(2) {
            self.to_expression(node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("false branch in ternary expression", node.text().to_string(), span));
            return Ok(leo_ast::ErrExpression { span, id }.into());
        };

        Ok(leo_ast::TernaryExpression { condition, if_true, if_false, span, id }.into())
    }

    /// Convert an ARRAY_EXPR node to an ArrayExpression or RepeatExpression.
    fn array_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), ARRAY_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let expr_children: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();

        // Check for repeat syntax: [x; n] (has exactly 2 children with semicolon)
        let has_semicolon = tokens(node).any(|t| t.kind() == SEMICOLON);
        if has_semicolon && expr_children.len() == 2 {
            let expr = self.to_expression(&expr_children[0])?;
            let count = self.to_expression(&expr_children[1])?;
            return Ok(leo_ast::RepeatExpression { expr, count, span, id }.into());
        }

        // Regular array expression: [a, b, c]
        let elements = expr_children.iter().map(|n| self.to_expression(n)).collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::ArrayExpression { elements, span, id }.into())
    }

    /// Convert a TUPLE_EXPR node to a TupleExpression or UnitExpression.
    fn tuple_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), TUPLE_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let elements: Vec<_> = children(node)
            .filter(|n| is_expression_kind(n.kind()))
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        if elements.is_empty() {
            Ok(leo_ast::UnitExpression { span, id }.into())
        } else {
            Ok(leo_ast::TupleExpression { elements, span, id }.into())
        }
    }

    /// Convert a STRUCT_EXPR node to a CompositeExpression.
    fn struct_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), STRUCT_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get the struct path from tokens
        let path = self.struct_expr_to_path(node)?;

        // Collect field initializers
        let members = children(node)
            .filter(|n| n.kind() == STRUCT_FIELD_INIT)
            .map(|n| self.struct_field_init_to_member(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::CompositeExpression { path, const_arguments: Vec::new(), members, span, id }.into())
    }

    /// Extract a Path from a STRUCT_EXPR node's name tokens.
    fn struct_expr_to_path(&self, node: &SyntaxNode) -> Result<leo_ast::Path> {
        let span = self.to_span(node);
        let text = node.text().to_string();

        // Check for locator: program.aleo/Type
        if let Some((program_str, rest)) = text.split_once(".aleo/") {
            let type_str = rest.split(|c| c == ' ' || c == '{').next().unwrap_or(rest);
            let program = leo_ast::Identifier {
                name: Symbol::intern(program_str),
                span: Span::new(span.lo, span.lo + program_str.len() as u32),
                id: self.builder.next_id(),
            };
            let type_name = leo_ast::Identifier {
                name: Symbol::intern(type_str),
                span: Span::new(
                    span.lo + program_str.len() as u32 + 6,
                    span.lo + program_str.len() as u32 + 6 + type_str.len() as u32,
                ),
                id: self.builder.next_id(),
            };
            return Ok(leo_ast::Path::new(Some(program), Vec::new(), type_name, span, self.builder.next_id()));
        }

        // Regular path from IDENT tokens (before L_BRACE)
        let mut path_components = Vec::new();
        for token in tokens(node) {
            if token.kind() == L_BRACE {
                break;
            }
            if token.kind() == IDENT {
                path_components.push(self.to_identifier(&token));
            }
        }

        let name = path_components.pop().expect("struct expr should have type name");
        Ok(leo_ast::Path::new(None, path_components, name, span, self.builder.next_id()))
    }

    /// Convert a STRUCT_FIELD_INIT node to a CompositeFieldInitializer.
    fn struct_field_init_to_member(&self, node: &SyntaxNode) -> Result<leo_ast::CompositeFieldInitializer> {
        debug_assert_eq!(node.kind(), STRUCT_FIELD_INIT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let Some(ident_token) = tokens(node).find(|t| t.kind() == IDENT) else {
            // No identifier found - create a placeholder with error message
            self.handler.emit_err(ParserError::unexpected_str(
                "identifier in struct field",
                node.text().to_string(),
                span,
            ));
            let identifier = leo_ast::Identifier {
                name: Symbol::intern("__error__"),
                span,
                id: self.builder.next_id(),
            };
            return Ok(leo_ast::CompositeFieldInitializer { identifier, expression: None, span, id });
        };
        let identifier = self.to_identifier(&ident_token);

        // Check if there's an expression (has COLON followed by expression)
        let has_colon = tokens(node).any(|t| t.kind() == COLON);
        let expression = if has_colon {
            children(node)
                .find(|n| is_expression_kind(n.kind()))
                .map(|n| self.to_expression(&n))
                .transpose()?
        } else {
            None
        };

        Ok(leo_ast::CompositeFieldInitializer { identifier, expression, span, id })
    }

    /// Convert a PATH_EXPR node to an Expression.
    fn path_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), PATH_EXPR);
        let path = self.path_expr_to_path(node)?;
        Ok(leo_ast::Expression::Path(path))
    }

    /// Convert an ASYNC_EXPR node to an Expression.
    fn async_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), ASYNC_EXPR);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Find the block inside the async expression
        if let Some(block_node) = children(node).find(|n| n.kind() == BLOCK) {
            let block = self.to_block(&block_node)?;
            Ok(leo_ast::AsyncExpression { block, span, id }.into())
        } else {
            // No block found - emit error
            self.handler.emit_err(ParserError::unexpected_str(
                "block in async expression",
                node.text().to_string(),
                span,
            ));
            Ok(leo_ast::ErrExpression { span, id }.into())
        }
    }

    /// Convert a PATH_EXPR node to a Path.
    fn path_expr_to_path(&self, node: &SyntaxNode) -> Result<leo_ast::Path> {
        let span = self.to_span(node);
        let text = node.text().to_string();

        // Check for locator: program.aleo/name
        if let Some((program_str, name_str)) = text.split_once(".aleo/") {
            let program = leo_ast::Identifier {
                name: Symbol::intern(program_str),
                span: Span::new(span.lo, span.lo + program_str.len() as u32),
                id: self.builder.next_id(),
            };
            let name = leo_ast::Identifier {
                name: Symbol::intern(name_str),
                span: Span::new(span.lo + program_str.len() as u32 + 6, span.hi),
                id: self.builder.next_id(),
            };
            return Ok(leo_ast::Path::new(Some(program), Vec::new(), name, span, self.builder.next_id()));
        }

        // Regular path: collect identifiers
        let mut path_components = Vec::new();
        for token in tokens(node) {
            if token.kind() == IDENT {
                path_components.push(self.to_identifier(&token));
            }
            // Also handle self keyword
            if token.kind() == KW_SELF {
                path_components.push(leo_ast::Identifier {
                    name: sym::SelfLower,
                    span: self.token_span(&token),
                    id: self.builder.next_id(),
                });
            }
            if token.kind() == KW_BLOCK {
                path_components.push(leo_ast::Identifier {
                    name: sym::block,
                    span: self.token_span(&token),
                    id: self.builder.next_id(),
                });
            }
            if token.kind() == KW_NETWORK {
                path_components.push(leo_ast::Identifier {
                    name: sym::network,
                    span: self.token_span(&token),
                    id: self.builder.next_id(),
                });
            }
        }

        let name = path_components.pop().unwrap_or_else(|| {
            // If no components found, check for keywords that may have been parsed differently
            panic!("PATH_EXPR should have at least one identifier: {:?}", node.text())
        });
        Ok(leo_ast::Path::new(None, path_components, name, span, self.builder.next_id()))
    }

    // =========================================================================
    // Statement Conversions
    // =========================================================================

    /// Convert a syntax node to a statement.
    fn to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let stmt = match node.kind() {
            LET_STMT => self.let_stmt_to_statement(node)?,
            CONST_STMT => self.const_stmt_to_statement(node)?,
            RETURN_STMT => self.return_stmt_to_statement(node)?,
            EXPR_STMT => self.expr_stmt_to_statement(node)?,
            ASSIGN_STMT => self.assign_stmt_to_statement(node)?,
            IF_STMT => self.if_stmt_to_statement(node)?,
            FOR_STMT => self.for_stmt_to_statement(node)?,
            BLOCK => self.to_block(node)?.into(),
            ASSERT_STMT => {
                let expression = match children(node).find(|n| is_expression_kind(n.kind())) {
                    Some(expr) => self.to_expression(&expr)?,
                    None => {
                        self.handler.emit_err(ParserError::unexpected_str("expression in assert", node.text().to_string(), span));
                        leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
                    }
                };
                leo_ast::AssertStatement { variant: leo_ast::AssertVariant::Assert(expression), span, id }.into()
            }
            ASSERT_EQ_STMT => {
                let exprs: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();
                let e0 = if let Some(expr) = exprs.first() {
                    self.to_expression(expr)?
                } else {
                    self.handler.emit_err(ParserError::unexpected_str("first expression in assert_eq", node.text().to_string(), span));
                    leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
                };
                let e1 = if let Some(expr) = exprs.get(1) {
                    self.to_expression(expr)?
                } else {
                    self.handler.emit_err(ParserError::unexpected_str("second expression in assert_eq", node.text().to_string(), span));
                    leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
                };
                leo_ast::AssertStatement { variant: leo_ast::AssertVariant::AssertEq(e0, e1), span, id }.into()
            }
            ASSERT_NEQ_STMT => {
                let exprs: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();
                let e0 = if let Some(expr) = exprs.first() {
                    self.to_expression(expr)?
                } else {
                    self.handler.emit_err(ParserError::unexpected_str("first expression in assert_neq", node.text().to_string(), span));
                    leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
                };
                let e1 = if let Some(expr) = exprs.get(1) {
                    self.to_expression(expr)?
                } else {
                    self.handler.emit_err(ParserError::unexpected_str("second expression in assert_neq", node.text().to_string(), span));
                    leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
                };
                leo_ast::AssertStatement { variant: leo_ast::AssertVariant::AssertNeq(e0, e1), span, id }.into()
            }
            // For ROOT nodes that wrap a statement (from parse_statement_entry)
            ROOT => {
                if let Some(inner) = children(node).find(|n| is_statement_kind(n.kind())) {
                    self.to_statement(&inner)?
                } else {
                    self.handler.emit_err(ParserError::unexpected_str("valid statement", node.text().to_string(), span));
                    leo_ast::ExpressionStatement {
                        expression: leo_ast::ErrExpression { span, id: self.builder.next_id() }.into(),
                        span,
                        id,
                    }
                    .into()
                }
            }
            // Error recovery: emit error and return empty expression statement for ERROR nodes
            ERROR => {
                self.handler.emit_err(ParserError::unexpected_str(
                    "valid statement",
                    node.text().to_string(),
                    span,
                ));
                // Return an expression statement with an ErrExpression
                leo_ast::ExpressionStatement {
                    expression: leo_ast::ErrExpression { span, id: self.builder.next_id() }.into(),
                    span,
                    id,
                }
                .into()
            }
            kind => panic!("unexpected statement kind: {:?}", kind),
        };

        Ok(stmt)
    }

    /// Convert a BLOCK node to a Block.
    fn to_block(&self, node: &SyntaxNode) -> Result<leo_ast::Block> {
        debug_assert_eq!(node.kind(), BLOCK);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let statements = children(node)
            .filter(|n| is_statement_kind(n.kind()))
            .map(|n| self.to_statement(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::Block { statements, span, id })
    }

    /// Convert a LET_STMT node to a DefinitionStatement.
    fn let_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), LET_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Find the pattern
        let place = match children(node).find(|n| matches!(n.kind(), IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN)) {
            Some(pattern_node) => self.pattern_to_definition_place(&pattern_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("pattern in let statement", node.text().to_string(), span));
                leo_ast::DefinitionPlace::Single(leo_ast::Identifier {
                    name: Symbol::intern("_error"),
                    span,
                    id: self.builder.next_id(),
                })
            }
        };

        // Find type annotation if present
        let type_ = children(node).find(|n| is_type_kind(n.kind())).map(|n| self.to_type(&n)).transpose()?;

        // Find the value expression
        let value = match children(node).find(|n| is_expression_kind(n.kind())) {
            Some(expr_node) => self.to_expression(&expr_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("value in let statement", node.text().to_string(), span));
                leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
            }
        };

        Ok(leo_ast::DefinitionStatement { place, type_, value, span, id }.into())
    }

    /// Convert a pattern node to a DefinitionPlace.
    fn pattern_to_definition_place(&self, node: &SyntaxNode) -> Result<leo_ast::DefinitionPlace> {
        let span = self.to_span(node);
        match node.kind() {
            IDENT_PATTERN => {
                let ident = match tokens(node).find(|t| t.kind() == IDENT) {
                    Some(token) => self.to_identifier(&token),
                    None => {
                        self.handler.emit_err(ParserError::unexpected_str("identifier in pattern", node.text().to_string(), span));
                        leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
                    }
                };
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
            TUPLE_PATTERN => {
                let names = children(node)
                    .filter(|n| matches!(n.kind(), IDENT_PATTERN | WILDCARD_PATTERN))
                    .map(|n| {
                        if n.kind() == WILDCARD_PATTERN {
                            // Use a placeholder identifier for wildcard
                            let span = self.to_span(&n);
                            leo_ast::Identifier { name: Symbol::intern("_"), span, id: self.builder.next_id() }
                        } else {
                            match tokens(&n).find(|t| t.kind() == IDENT) {
                                Some(token) => self.to_identifier(&token),
                                None => {
                                    let span = self.to_span(&n);
                                    self.handler.emit_err(ParserError::unexpected_str("identifier in pattern", n.text().to_string(), span));
                                    leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
                                }
                            }
                        }
                    })
                    .collect();
                Ok(leo_ast::DefinitionPlace::Multiple(names))
            }
            WILDCARD_PATTERN => {
                let ident = leo_ast::Identifier { name: Symbol::intern("_"), span, id: self.builder.next_id() };
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
            ERROR => {
                self.handler.emit_err(ParserError::unexpected_str("valid pattern", node.text().to_string(), span));
                let ident = leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() };
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
            kind => {
                self.handler.emit_err(ParserError::unexpected_str("valid pattern", format!("{:?}", kind), span));
                let ident = leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() };
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
        }
    }

    /// Convert a CONST_STMT node to a ConstDeclaration.
    fn const_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), CONST_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get name
        let place = match tokens(node).find(|t| t.kind() == IDENT) {
            Some(token) => self.to_identifier(&token),
            None => {
                self.handler.emit_err(ParserError::unexpected_str("name in const declaration", node.text().to_string(), span));
                leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
            }
        };

        // Get type
        let type_ = match children(node).find(|n| is_type_kind(n.kind())) {
            Some(type_node) => self.to_type(&type_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("type in const declaration", node.text().to_string(), span));
                leo_ast::Type::Err
            }
        };

        // Get value
        let value = match children(node).find(|n| is_expression_kind(n.kind())) {
            Some(value_node) => self.to_expression(&value_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("value in const declaration", node.text().to_string(), span));
                leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
            }
        };

        Ok(leo_ast::ConstDeclaration { place, type_, value, span, id }.into())
    }

    /// Convert a RETURN_STMT node to a ReturnStatement.
    fn return_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), RETURN_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get optional expression
        let expression = children(node)
            .find(|n| is_expression_kind(n.kind()))
            .map(|n| self.to_expression(&n))
            .transpose()?
            .unwrap_or_else(|| leo_ast::UnitExpression { span, id: self.builder.next_id() }.into());

        Ok(leo_ast::ReturnStatement { expression, span, id }.into())
    }

    /// Convert an EXPR_STMT node to an ExpressionStatement.
    fn expr_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), EXPR_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let expression = match children(node).find(|n| is_expression_kind(n.kind())) {
            Some(expr_node) => self.to_expression(&expr_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("expression in expression statement", node.text().to_string(), span));
                leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
            }
        };

        Ok(leo_ast::ExpressionStatement { expression, span, id }.into())
    }

    /// Convert an ASSIGN_STMT node to an AssignStatement.
    fn assign_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), ASSIGN_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get expressions (left and right of operator)
        let expr_children: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();

        let left = if let Some(left_node) = expr_children.first() {
            self.to_expression(left_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("left side in assignment", node.text().to_string(), span));
            return Ok(leo_ast::ExpressionStatement {
                expression: leo_ast::ErrExpression { span, id: self.builder.next_id() }.into(),
                span,
                id,
            }.into());
        };

        let right_expr = if let Some(right_node) = expr_children.get(1) {
            self.to_expression(right_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("right side in assignment", node.text().to_string(), span));
            leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
        };

        // Get operator
        let op_token = match tokens(node).find(|t| is_assign_op(t.kind())) {
            Some(token) => token,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("operator in assignment", node.text().to_string(), span));
                return Ok(leo_ast::AssignStatement { place: left, value: right_expr, span, id }.into());
            }
        };

        let value = if op_token.kind() == EQ {
            // Simple assignment
            right_expr
        } else {
            // Compound assignment - translate to binary op + assignment
            let binary_op = match op_token.kind() {
                PLUS_EQ => leo_ast::BinaryOperation::Add,
                MINUS_EQ => leo_ast::BinaryOperation::Sub,
                STAR_EQ => leo_ast::BinaryOperation::Mul,
                SLASH_EQ => leo_ast::BinaryOperation::Div,
                PERCENT_EQ => leo_ast::BinaryOperation::Rem,
                STAR2_EQ => leo_ast::BinaryOperation::Pow,
                AMP_EQ => leo_ast::BinaryOperation::BitwiseAnd,
                PIPE_EQ => leo_ast::BinaryOperation::BitwiseOr,
                CARET_EQ => leo_ast::BinaryOperation::Xor,
                SHL_EQ => leo_ast::BinaryOperation::Shl,
                SHR_EQ => leo_ast::BinaryOperation::Shr,
                AMP2_EQ => leo_ast::BinaryOperation::And,
                PIPE2_EQ => leo_ast::BinaryOperation::Or,
                _ => {
                    self.handler.emit_err(ParserError::unexpected_str("compound assignment operator", format!("{:?}", op_token.kind()), span));
                    return Ok(leo_ast::AssignStatement { place: left, value: right_expr, span, id }.into());
                }
            };
            leo_ast::BinaryExpression {
                left: left.clone(),
                right: right_expr,
                op: binary_op,
                span,
                id: self.builder.next_id(),
            }
            .into()
        };

        Ok(leo_ast::AssignStatement { place: left, value, span, id }.into())
    }

    /// Convert an IF_STMT node to a ConditionalStatement.
    fn if_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), IF_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get condition expression
        let condition = match children(node).find(|n| is_expression_kind(n.kind())) {
            Some(condition_node) => self.to_expression(&condition_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("condition in if statement", node.text().to_string(), span));
                leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
            }
        };

        // Get then block (first BLOCK child)
        let blocks: Vec<_> = children(node).filter(|n| n.kind() == BLOCK).collect();
        let then = if let Some(then_block) = blocks.first() {
            self.to_block(then_block)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("then block in if statement", node.text().to_string(), span));
            leo_ast::Block { span, statements: Vec::new(), id: self.builder.next_id() }
        };

        // Check for else clause - can be another IF_STMT or a BLOCK
        let otherwise = children(node)
            .skip_while(|n| n.kind() != BLOCK)
            .skip(1)  // Skip the then block
            .find(|n| n.kind() == BLOCK || n.kind() == IF_STMT)
            .map(|n| self.to_statement(&n))
            .transpose()?
            .map(Box::new);

        Ok(leo_ast::ConditionalStatement { condition, then, otherwise, span, id }.into())
    }

    /// Convert a FOR_STMT node to an IterationStatement.
    fn for_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), FOR_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get loop variable
        let variable = match tokens(node).find(|t| t.kind() == IDENT) {
            Some(token) => self.to_identifier(&token),
            None => {
                self.handler.emit_err(ParserError::unexpected_str("variable in for statement", node.text().to_string(), span));
                leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
            }
        };

        // Get optional type annotation
        let type_ = children(node).find(|n| is_type_kind(n.kind())).map(|n| self.to_type(&n)).transpose()?;

        // Get range expressions (before and after ..)
        let expr_children: Vec<_> = children(node).filter(|n| is_expression_kind(n.kind())).collect();

        let start = if let Some(start_node) = expr_children.first() {
            self.to_expression(start_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("start expression in for statement", node.text().to_string(), span));
            leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
        };

        let stop = if let Some(stop_node) = expr_children.get(1) {
            self.to_expression(stop_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("stop expression in for statement", node.text().to_string(), span));
            leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
        };

        // Get body block
        let block = match children(node).find(|n| n.kind() == BLOCK) {
            Some(block_node) => self.to_block(&block_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("block in for statement", node.text().to_string(), span));
                leo_ast::Block { span, statements: Vec::new(), id: self.builder.next_id() }
            }
        };

        Ok(leo_ast::IterationStatement { variable, type_, start, stop, inclusive: false, block, span, id }.into())
    }

    // =========================================================================
    // Item/Program Conversions
    // =========================================================================

    /// Convert a syntax node to a module.
    fn to_module(&self, node: &SyntaxNode, program_name: Symbol, path: Vec<Symbol>) -> Result<leo_ast::Module> {
        // Module nodes are ROOT nodes containing items (functions, structs, consts)
        let mut functions = Vec::new();
        let mut composites = Vec::new();
        let mut consts = Vec::new();

        for child in children(node) {
            match child.kind() {
                FUNCTION_DEF => {
                    let func = self.to_function(&child)?;
                    functions.push((func.identifier.name, func));
                }
                STRUCT_DEF | RECORD_DEF => {
                    let composite = self.to_composite(&child)?;
                    composites.push((composite.identifier.name, composite));
                }
                GLOBAL_CONST => {
                    let global_const = self.to_global_const(&child)?;
                    consts.push((global_const.place.name, global_const));
                }
                PROGRAM_DECL => {
                    // Process items inside program decl
                    for item in children(&child) {
                        match item.kind() {
                            FUNCTION_DEF => {
                                let func = self.to_function(&item)?;
                                functions.push((func.identifier.name, func));
                            }
                            STRUCT_DEF | RECORD_DEF => {
                                let composite = self.to_composite(&item)?;
                                composites.push((composite.identifier.name, composite));
                            }
                            GLOBAL_CONST => {
                                let global_const = self.to_global_const(&item)?;
                                consts.push((global_const.place.name, global_const));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // Sort functions: transitions first
        functions.sort_by_key(|func| if func.1.variant.is_transition() { 0u8 } else { 1u8 });

        Ok(leo_ast::Module { program_name, path, consts, composites, functions })
    }

    /// Convert a syntax node to a program (main file).
    fn to_main(&self, node: &SyntaxNode) -> Result<leo_ast::Program> {
        // The main file contains imports and a program declaration
        let mut imports = indexmap::IndexMap::new();
        let mut functions = Vec::new();
        let mut composites = Vec::new();
        let mut consts = Vec::new();
        let mut mappings = Vec::new();
        let mut storage_variables = Vec::new();
        let mut program_name = None;
        let mut network = None;

        for child in children(node) {
            match child.kind() {
                IMPORT => {
                    let (name, span) = self.import_to_name(&child)?;
                    imports.insert(name, span);
                }
                PROGRAM_DECL => {
                    // Extract program name and network
                    let (pname, pnetwork) = self.program_decl_to_name(&child)?;
                    program_name = Some(pname);
                    network = Some(pnetwork);

                    // Process items inside program decl
                    for item in children(&child) {
                        match item.kind() {
                            FUNCTION_DEF => {
                                let func = self.to_function(&item)?;
                                functions.push((func.identifier.name, func));
                            }
                            STRUCT_DEF | RECORD_DEF => {
                                let composite = self.to_composite(&item)?;
                                composites.push((composite.identifier.name, composite));
                            }
                            GLOBAL_CONST => {
                                let global_const = self.to_global_const(&item)?;
                                consts.push((global_const.place.name, global_const));
                            }
                            MAPPING_DEF => {
                                let mapping = self.to_mapping(&item)?;
                                mappings.push((mapping.identifier.name, mapping));
                            }
                            STORAGE_DEF => {
                                let storage = self.to_storage(&item)?;
                                storage_variables.push((storage.identifier.name, storage));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        let program_name = program_name.expect("program should have name");
        let network = network.expect("program should have network");

        // Sort functions: transitions first
        functions.sort_by_key(|func| if func.1.variant.is_transition() { 0u8 } else { 1u8 });

        let program_scope = leo_ast::ProgramScope {
            program_id: leo_ast::ProgramId { name: program_name, network },
            consts,
            composites,
            mappings,
            storage_variables,
            functions,
            constructor: None,
            span: self.to_span(node),
        };

        Ok(leo_ast::Program {
            imports,
            modules: indexmap::IndexMap::new(),
            stubs: indexmap::IndexMap::new(),
            program_scopes: vec![(program_name.name, program_scope)].into_iter().collect(),
        })
    }

    /// Extract name and span from an IMPORT node.
    fn import_to_name(&self, node: &SyntaxNode) -> Result<(Symbol, Span)> {
        debug_assert_eq!(node.kind(), IMPORT);
        let span = self.to_span(node);

        // Import format: import name.aleo;
        // Find the IDENT token
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("import should have name");
        let name = Symbol::intern(name_token.text());

        Ok((name, span))
    }

    /// Extract program name and network from a PROGRAM_DECL node.
    fn program_decl_to_name(&self, node: &SyntaxNode) -> Result<(leo_ast::Identifier, leo_ast::Identifier)> {
        debug_assert_eq!(node.kind(), PROGRAM_DECL);
        let _span = self.to_span(node);

        // Program format: program name.aleo { ... }
        // Find IDENT (name) and KW_ALEO (network)
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("program should have name");
        let program_name = self.to_identifier(&name_token);

        let aleo_token = tokens(node).find(|t| t.kind() == KW_ALEO).expect("program should have .aleo");
        let network = leo_ast::Identifier {
            name: Symbol::intern("aleo"),
            span: self.token_span(&aleo_token),
            id: self.builder.next_id(),
        };

        Ok((program_name, network))
    }

    /// Convert a FUNCTION_DEF node to a Function.
    fn to_function(&self, node: &SyntaxNode) -> Result<leo_ast::Function> {
        debug_assert!(matches!(node.kind(), FUNCTION_DEF | CONSTRUCTOR_DEF));
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Collect annotations
        let annotations = children(node)
            .filter(|n| n.kind() == ANNOTATION)
            .map(|n| self.to_annotation(&n))
            .collect::<Result<Vec<_>>>()?;

        // Check for async
        let is_async = tokens(node).any(|t| t.kind() == KW_ASYNC);

        // Determine variant
        let variant = if tokens(node).any(|t| t.kind() == KW_TRANSITION) {
            if is_async { leo_ast::Variant::AsyncTransition } else { leo_ast::Variant::Transition }
        } else if tokens(node).any(|t| t.kind() == KW_FUNCTION) {
            if is_async { leo_ast::Variant::AsyncFunction } else { leo_ast::Variant::Function }
        } else if tokens(node).any(|t| t.kind() == KW_INLINE) {
            leo_ast::Variant::Inline
        } else if tokens(node).any(|t| t.kind() == KW_SCRIPT) {
            leo_ast::Variant::Script
        } else if tokens(node).any(|t| t.kind() == KW_CONSTRUCTOR) {
            leo_ast::Variant::Function // Constructor is treated as function
        } else {
            leo_ast::Variant::Function
        };

        // Get function name
        let name_token = tokens(node).filter(|t| t.kind() == IDENT).next().expect("function should have name");
        let identifier = self.to_identifier(&name_token);

        // Get const parameters if any
        let const_parameters = children(node)
            .find(|n| n.kind() == CONST_PARAM_LIST)
            .map(|n| self.to_const_parameters(&n))
            .transpose()?
            .unwrap_or_default();

        // Get input parameters
        let input = children(node)
            .find(|n| n.kind() == PARAM_LIST)
            .map(|n| self.param_list_to_inputs(&n))
            .transpose()?
            .unwrap_or_default();

        // Get return type
        let output_type = children(node)
            .find(|n| is_type_kind(n.kind()))
            .map(|n| self.to_type(&n))
            .transpose()?
            .unwrap_or(leo_ast::Type::Unit);

        // Get block
        let block = children(node)
            .find(|n| n.kind() == BLOCK)
            .map(|n| self.to_block(&n))
            .transpose()?
            .unwrap_or_else(|| leo_ast::Block { statements: Vec::new(), span, id: self.builder.next_id() });

        Ok(leo_ast::Function {
            annotations,
            variant,
            identifier,
            const_parameters,
            input,
            output: Vec::new(), // Output declarations (filled in later if needed)
            output_type,
            block,
            span,
            id,
        })
    }

    /// Convert an ANNOTATION node to an Annotation.
    fn to_annotation(&self, node: &SyntaxNode) -> Result<leo_ast::Annotation> {
        debug_assert_eq!(node.kind(), ANNOTATION);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("annotation should have name");
        let identifier = self.to_identifier(&name_token);

        // TODO: Parse annotation arguments if needed
        Ok(leo_ast::Annotation { identifier, map: indexmap::IndexMap::new(), span, id })
    }

    /// Convert a PARAM_LIST node to function inputs.
    fn param_list_to_inputs(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::Input>> {
        debug_assert_eq!(node.kind(), PARAM_LIST);

        children(node).filter(|n| n.kind() == PARAM).map(|n| self.param_to_input(&n)).collect()
    }

    /// Convert a PARAM node to an Input.
    fn param_to_input(&self, node: &SyntaxNode) -> Result<leo_ast::Input> {
        debug_assert_eq!(node.kind(), PARAM);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Check for mode (visibility)
        let mode = if tokens(node).any(|t| t.kind() == KW_PUBLIC) {
            leo_ast::Mode::Public
        } else if tokens(node).any(|t| t.kind() == KW_PRIVATE) {
            leo_ast::Mode::Private
        } else if tokens(node).any(|t| t.kind() == KW_CONSTANT) {
            leo_ast::Mode::Constant
        } else {
            leo_ast::Mode::None
        };

        // Get name
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("param should have name");
        let identifier = self.to_identifier(&name_token);

        // Get type
        let type_node = children(node).find(|n| is_type_kind(n.kind())).expect("param should have type");
        let type_ = self.to_type(&type_node)?;

        Ok(leo_ast::Input { identifier, mode, type_, span, id })
    }

    /// Convert a const parameter list.
    fn to_const_parameters(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::ConstParameter>> {
        debug_assert_eq!(node.kind(), CONST_PARAM_LIST);

        children(node)
            .filter(|n| n.kind() == CONST_PARAM)
            .map(|n| {
                let span = self.to_span(&n);
                let id = self.builder.next_id();

                let name_token = tokens(&n).find(|t| t.kind() == IDENT).expect("const param should have name");
                let identifier = self.to_identifier(&name_token);

                let type_node =
                    children(&n).find(|n| is_type_kind(n.kind())).expect("const param should have type");
                let type_ = self.to_type(&type_node)?;

                Ok(leo_ast::ConstParameter { identifier, type_, span, id })
            })
            .collect()
    }

    /// Convert a STRUCT_DEF or RECORD_DEF node to a Composite.
    fn to_composite(&self, node: &SyntaxNode) -> Result<leo_ast::Composite> {
        debug_assert!(matches!(node.kind(), STRUCT_DEF | RECORD_DEF));
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let is_record = node.kind() == RECORD_DEF;

        // Get name
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("composite should have name");
        let identifier = self.to_identifier(&name_token);

        // Get members
        let members = children(node)
            .filter(|n| n.kind() == STRUCT_MEMBER)
            .map(|n| self.struct_member_to_member(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::Composite { identifier, const_parameters: Vec::new(), members, is_record, span, id })
    }

    /// Convert a STRUCT_MEMBER node to a Member.
    fn struct_member_to_member(&self, node: &SyntaxNode) -> Result<leo_ast::Member> {
        debug_assert_eq!(node.kind(), STRUCT_MEMBER);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Check for mode
        let mode = if tokens(node).any(|t| t.kind() == KW_PUBLIC) {
            leo_ast::Mode::Public
        } else if tokens(node).any(|t| t.kind() == KW_PRIVATE) {
            leo_ast::Mode::Private
        } else {
            leo_ast::Mode::None
        };

        // Get name
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("member should have name");
        let identifier = self.to_identifier(&name_token);

        // Get type
        let type_node = children(node).find(|n| is_type_kind(n.kind())).expect("member should have type");
        let type_ = self.to_type(&type_node)?;

        Ok(leo_ast::Member { mode, identifier, type_, span, id })
    }

    /// Convert a GLOBAL_CONST node to a ConstDeclaration.
    fn to_global_const(&self, node: &SyntaxNode) -> Result<leo_ast::ConstDeclaration> {
        debug_assert_eq!(node.kind(), GLOBAL_CONST);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get name
        let name_token = tokens(node).find(|t| t.kind() == IDENT).expect("global const should have name");
        let place = self.to_identifier(&name_token);

        // Get type
        let type_node =
            children(node).find(|n| is_type_kind(n.kind())).expect("global const should have type");
        let type_ = self.to_type(&type_node)?;

        // Get value
        let expr_node =
            children(node).find(|n| is_expression_kind(n.kind())).expect("global const should have value");
        let value = self.to_expression(&expr_node)?;

        Ok(leo_ast::ConstDeclaration { place, type_, value, span, id })
    }

    /// Convert a MAPPING_DEF node to a Mapping.
    fn to_mapping(&self, node: &SyntaxNode) -> Result<leo_ast::Mapping> {
        debug_assert_eq!(node.kind(), MAPPING_DEF);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get name
        let identifier = match tokens(node).find(|t| t.kind() == IDENT) {
            Some(token) => self.to_identifier(&token),
            None => {
                self.handler.emit_err(ParserError::unexpected_str("name in mapping", node.text().to_string(), span));
                leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
            }
        };

        // Get key and value types
        let type_nodes: Vec<_> = children(node).filter(|n| is_type_kind(n.kind())).collect();

        let key_type = if let Some(key_node) = type_nodes.first() {
            self.to_type(key_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("key type in mapping", node.text().to_string(), span));
            leo_ast::Type::Err
        };

        let value_type = if let Some(value_node) = type_nodes.get(1) {
            self.to_type(value_node)?
        } else {
            self.handler.emit_err(ParserError::unexpected_str("value type in mapping", node.text().to_string(), span));
            leo_ast::Type::Err
        };

        Ok(leo_ast::Mapping { identifier, key_type, value_type, span, id })
    }

    /// Convert a STORAGE_DEF node to a StorageVariable.
    fn to_storage(&self, node: &SyntaxNode) -> Result<leo_ast::StorageVariable> {
        debug_assert_eq!(node.kind(), STORAGE_DEF);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get name
        let name = match tokens(node).find(|t| t.kind() == IDENT) {
            Some(token) => self.to_identifier(&token),
            None => {
                self.handler.emit_err(ParserError::unexpected_str("name in storage", node.text().to_string(), span));
                leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
            }
        };

        // Get type
        let type_ = match children(node).find(|n| is_type_kind(n.kind())) {
            Some(type_node) => self.to_type(&type_node)?,
            None => {
                self.handler.emit_err(ParserError::unexpected_str("type in storage", node.text().to_string(), span));
                leo_ast::Type::Err
            }
        };

        Ok(leo_ast::StorageVariable { identifier: name, type_, span, id })
    }
}

// =============================================================================
// Public Parse Functions
// =============================================================================

/// Helper to safely create a span from rowan error range, clamping to source bounds.
fn safe_error_span(error: &leo_parser_rowan::ParseError, start_pos: u32, source_len: u32) -> Span {
    let end = start_pos + source_len;
    let lo = (u32::from(error.range.start()) + start_pos).min(end);
    let hi = (u32::from(error.range.end()) + start_pos).min(end).max(lo);
    Span::new(lo, hi)
}

/// Parses a single expression from source code.
pub fn parse_expression(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Expression> {
    let parse = leo_parser_rowan::parse_expression_entry(source);
    let source_len = source.len() as u32;

    // Report parse errors to the handler
    for error in parse.errors() {
        let span = safe_error_span(error, start_pos, source_len);
        handler.emit_err(ParserError::custom(&error.message, span));
    }

    let conversion_context = ConversionContext::new(&handler, node_builder, start_pos);
    conversion_context.to_expression(&parse.syntax())
}

/// Parses a single statement from source code.
pub fn parse_statement(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Statement> {
    let parse = leo_parser_rowan::parse_statement_entry(source);
    let source_len = source.len() as u32;

    // Report parse errors to the handler
    for error in parse.errors() {
        let span = safe_error_span(error, start_pos, source_len);
        handler.emit_err(ParserError::custom(&error.message, span));
    }

    let conversion_context = ConversionContext::new(&handler, node_builder, start_pos);
    conversion_context.to_statement(&parse.syntax())
}

/// Parses a module (non-main source file) into a Module AST.
pub fn parse_module(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    program_name: Symbol,
    path: Vec<Symbol>,
    _network: NetworkName,
) -> Result<leo_ast::Module> {
    let parse = leo_parser_rowan::parse_module_entry(source);
    let source_len = source.len() as u32;

    // Report parse errors to the handler
    for error in parse.errors() {
        let span = safe_error_span(error, start_pos, source_len);
        handler.emit_err(ParserError::custom(&error.message, span));
    }

    let conversion_context = ConversionContext::new(&handler, node_builder, start_pos);
    conversion_context.to_module(&parse.syntax(), program_name, path)
}

/// Parses a complete program with its modules into a Program AST.
pub fn parse(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    _network: NetworkName,
) -> Result<leo_ast::Program> {
    // Parse main program file
    let parse = leo_parser_rowan::parse_file(&source.src);
    let source_len = source.src.len() as u32;

    // Report parse errors to the handler
    for error in parse.errors() {
        let span = safe_error_span(error, source.absolute_start, source_len);
        handler.emit_err(ParserError::custom(&error.message, span));
    }

    // Create context with the main file's start position
    let main_context = ConversionContext::new(&handler, node_builder, source.absolute_start);
    let mut program = main_context.to_main(&parse.syntax())?;
    let program_name = *program.program_scopes.first().unwrap().0;

    // Determine the root directory of the main file (for module resolution)
    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    for module in modules {
        let module_parse = leo_parser_rowan::parse_module_entry(&module.src);
        let module_len = module.src.len() as u32;

        // Report parse errors for each module
        for error in module_parse.errors() {
            let span = safe_error_span(error, module.absolute_start, module_len);
            handler.emit_err(ParserError::custom(&error.message, span));
        }

        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            // Ensure no module uses a keyword in its name
            for segment in &key {
                if symbol_is_keyword(*segment) {
                    return Err(ParserError::keyword_used_as_module_name(key.iter().format("::"), segment).into());
                }
            }

            // Create context with this module's start position
            let module_context = ConversionContext::new(&handler, node_builder, module.absolute_start);
            let module_ast = module_context.to_module(&module_parse.syntax(), program_name, key.clone())?;
            program.modules.insert(key, module_ast);
        }
    }

    Ok(program)
}

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    network: NetworkName,
) -> Result<leo_ast::Ast> {
    Ok(leo_ast::Ast::new(parse(handler, node_builder, source, modules, network)?))
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Get non-trivia children of a node.
fn children(node: &SyntaxNode) -> impl Iterator<Item = SyntaxNode> + '_ {
    node.children().filter(|n| !n.kind().is_trivia())
}

/// Get non-trivia tokens from a node.
fn tokens(node: &SyntaxNode) -> impl Iterator<Item = SyntaxToken> + '_ {
    node.children_with_tokens().filter_map(|elem| elem.into_token()).filter(|t| !t.kind().is_trivia())
}

/// Check if a SyntaxKind is a type node kind.
fn is_type_kind(kind: SyntaxKind) -> bool {
    matches!(kind, TYPE_PATH | TYPE_ARRAY | TYPE_TUPLE | TYPE_OPTIONAL | TYPE_FUTURE)
}

/// Check if a SyntaxKind is an expression node kind.
fn is_expression_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        LITERAL
            | BINARY_EXPR
            | UNARY_EXPR
            | CALL_EXPR
            | FIELD_EXPR
            | INDEX_EXPR
            | CAST_EXPR
            | TERNARY_EXPR
            | ARRAY_EXPR
            | TUPLE_EXPR
            | STRUCT_EXPR
            | PATH_EXPR
            | PAREN_EXPR
            | UNIT_EXPR
            | METHOD_CALL_EXPR
            | REPEAT_EXPR
            | ASYNC_EXPR
            | ASSOC_FN_EXPR
            | ASSOC_CONST_EXPR
            | LOCATOR_EXPR
            | TUPLE_ACCESS_EXPR
            | INTRINSIC_EXPR
    )
}

/// Check if a SyntaxKind is a statement node kind.
fn is_statement_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        LET_STMT
            | CONST_STMT
            | RETURN_STMT
            | EXPR_STMT
            | ASSIGN_STMT
            | IF_STMT
            | FOR_STMT
            | BLOCK
            | ASSERT_STMT
            | ASSERT_EQ_STMT
            | ASSERT_NEQ_STMT
    )
}

/// Check if a SyntaxKind is an assignment operator.
fn is_assign_op(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        EQ | PLUS_EQ
            | MINUS_EQ
            | STAR_EQ
            | SLASH_EQ
            | PERCENT_EQ
            | STAR2_EQ
            | AMP_EQ
            | PIPE_EQ
            | CARET_EQ
            | SHL_EQ
            | SHR_EQ
            | AMP2_EQ
            | PIPE2_EQ
    )
}

/// Convert a type keyword to a primitive Type, if applicable.
fn keyword_to_primitive_type(kind: SyntaxKind) -> Option<leo_ast::Type> {
    let ty = match kind {
        KW_ADDRESS => leo_ast::Type::Address,
        KW_BOOL => leo_ast::Type::Boolean,
        KW_FIELD => leo_ast::Type::Field,
        KW_GROUP => leo_ast::Type::Group,
        KW_SCALAR => leo_ast::Type::Scalar,
        KW_SIGNATURE => leo_ast::Type::Signature,
        KW_STRING => leo_ast::Type::String,
        KW_U8 => leo_ast::Type::Integer(leo_ast::IntegerType::U8),
        KW_U16 => leo_ast::Type::Integer(leo_ast::IntegerType::U16),
        KW_U32 => leo_ast::Type::Integer(leo_ast::IntegerType::U32),
        KW_U64 => leo_ast::Type::Integer(leo_ast::IntegerType::U64),
        KW_U128 => leo_ast::Type::Integer(leo_ast::IntegerType::U128),
        KW_I8 => leo_ast::Type::Integer(leo_ast::IntegerType::I8),
        KW_I16 => leo_ast::Type::Integer(leo_ast::IntegerType::I16),
        KW_I32 => leo_ast::Type::Integer(leo_ast::IntegerType::I32),
        KW_I64 => leo_ast::Type::Integer(leo_ast::IntegerType::I64),
        KW_I128 => leo_ast::Type::Integer(leo_ast::IntegerType::I128),
        _ => return None,
    };
    Some(ty)
}

/// Convert a SyntaxKind operator to BinaryOperation.
fn token_to_binary_op(kind: SyntaxKind) -> leo_ast::BinaryOperation {
    match kind {
        EQ2 => leo_ast::BinaryOperation::Eq,
        BANG_EQ => leo_ast::BinaryOperation::Neq,
        LT => leo_ast::BinaryOperation::Lt,
        LT_EQ => leo_ast::BinaryOperation::Lte,
        GT => leo_ast::BinaryOperation::Gt,
        GT_EQ => leo_ast::BinaryOperation::Gte,
        PLUS => leo_ast::BinaryOperation::Add,
        MINUS => leo_ast::BinaryOperation::Sub,
        STAR => leo_ast::BinaryOperation::Mul,
        SLASH => leo_ast::BinaryOperation::Div,
        PERCENT => leo_ast::BinaryOperation::Rem,
        PIPE2 => leo_ast::BinaryOperation::Or,
        AMP2 => leo_ast::BinaryOperation::And,
        PIPE => leo_ast::BinaryOperation::BitwiseOr,
        AMP => leo_ast::BinaryOperation::BitwiseAnd,
        STAR2 => leo_ast::BinaryOperation::Pow,
        SHL => leo_ast::BinaryOperation::Shl,
        SHR => leo_ast::BinaryOperation::Shr,
        CARET => leo_ast::BinaryOperation::Xor,
        _ => panic!("unexpected binary operator: {:?}", kind),
    }
}

fn symbol_is_keyword(symbol: Symbol) -> bool {
    matches!(
        symbol,
        sym::address
            | sym::aleo
            | sym::As
            | sym::assert
            | sym::assert_eq
            | sym::assert_neq
            | sym::Async
            | sym::block
            | sym::bool
            | sym::Const
            | sym::constant
            | sym::constructor
            | sym::Else
            | sym::False
            | sym::field
            | sym::Fn
            | sym::For
            | sym::function
            | sym::Future
            | sym::group
            | sym::i8
            | sym::i16
            | sym::i32
            | sym::i64
            | sym::i128
            | sym::If
            | sym::import
            | sym::In
            | sym::inline
            | sym::Let
            | sym::leo
            | sym::mapping
            | sym::storage
            | sym::network
            | sym::private
            | sym::program
            | sym::public
            | sym::record
            | sym::Return
            | sym::scalar
            | sym::script
            | sym::SelfLower
            | sym::signature
            | sym::string
            | sym::Struct
            | sym::transition
            | sym::True
            | sym::u8
            | sym::u16
            | sym::u32
            | sym::u64
            | sym::u128
    )
}

/// Computes a module key from a `FileName`, optionally relative to a root directory.
fn compute_module_key(name: &FileName, root_dir: Option<&std::path::Path>) -> Option<Vec<Symbol>> {
    let path = match name {
        FileName::Custom(name) => std::path::Path::new(name).to_path_buf(),
        FileName::Real(path) => {
            let root = root_dir?;
            path.strip_prefix(root).ok()?.to_path_buf()
        }
    };

    let mut key: Vec<Symbol> =
        path.components().map(|comp| Symbol::intern(&comp.as_os_str().to_string_lossy())).collect();

    if let Some(last) = path.file_name()
        && let Some(stem) = std::path::Path::new(last).file_stem()
    {
        key.pop();
        key.push(Symbol::intern(&stem.to_string_lossy()));
    }

    Some(key)
}

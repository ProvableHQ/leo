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
use snarkvm::prelude::{Address, Signature, TestnetV0};

use leo_ast::{NetworkName, NodeBuilder};
use leo_errors::{Handler, ParserError, Result};
use leo_parser_rowan::{SyntaxElement, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken, TextRange};
use leo_span::{
    Span,
    Symbol,
    source_map::{FileName, SourceFile},
    sym,
};

/// Type parameters and const arguments extracted from a `CONST_ARG_LIST` node.
type ConstArgList = (Vec<(leo_ast::Type, Span)>, Vec<leo_ast::Expression>);

// =============================================================================
// ConversionContext
// =============================================================================

/// Context for converting rowan syntax nodes to Leo AST nodes.
struct ConversionContext<'a> {
    handler: &'a Handler,
    builder: &'a NodeBuilder,
    /// The absolute start position to offset spans by.
    start_pos: u32,
    /// When true, suppress `unexpected_str` errors during conversion.
    ///
    /// These errors are always downstream of parse/lex errors (the conversion
    /// only fails when the CST contains ERROR nodes from parse recovery), so
    /// reporting them would be duplicative.
    suppress_cascade: bool,
}

impl<'a> ConversionContext<'a> {
    /// Create a new conversion context.
    fn new(handler: &'a Handler, builder: &'a NodeBuilder, start_pos: u32, suppress_cascade: bool) -> Self {
        Self { handler, builder, start_pos, suppress_cascade }
    }

    /// Emit an `unexpected_str` error, unless cascade suppression is active.
    fn emit_unexpected_str(&self, expected: &str, found: impl std::fmt::Display, span: Span) {
        if !self.suppress_cascade {
            self.handler.emit_err(ParserError::unexpected_str(expected, found, span));
        }
    }

    // =========================================================================
    // Utility Methods
    // =========================================================================

    /// Convert a rowan TextRange to a leo_span::Span.
    fn to_span(&self, node: &SyntaxNode) -> Span {
        let range = node.text_range();
        Span::new(u32::from(range.start()) + self.start_pos, u32::from(range.end()) + self.start_pos)
    }

    /// Convert a token's text range to a Span.
    fn token_span(&self, token: &SyntaxToken) -> Span {
        let range = token.text_range();
        Span::new(u32::from(range.start()) + self.start_pos, u32::from(range.end()) + self.start_pos)
    }

    /// Like `to_span` but starts at the first non-trivia direct token,
    /// excluding leading whitespace/comments from the span.
    fn non_trivia_span(&self, node: &SyntaxNode) -> Span {
        let start = first_non_trivia_token(node).map(|t| t.text_range().start()).unwrap_or(node.text_range().start());
        let end = node.text_range().end();
        Span::new(u32::from(start) + self.start_pos, u32::from(end) + self.start_pos)
    }

    /// Span that excludes both leading and trailing trivia. Suitable for
    /// leaf-like nodes (type paths, single tokens) where trailing whitespace
    /// is not significant.
    fn trimmed_span(&self, node: &SyntaxNode) -> Span {
        let start = first_non_trivia_token(node).map(|t| t.text_range().start()).unwrap_or(node.text_range().start());
        let end = last_non_trivia_token(node).map(|t| t.text_range().end()).unwrap_or(node.text_range().end());
        Span::new(u32::from(start) + self.start_pos, u32::from(end) + self.start_pos)
    }

    /// Span that excludes leading and trailing trivia by scanning all
    /// descendant tokens (deep traversal). Use for expression and statement
    /// nodes whose trailing trivia may be nested inside child nodes.
    fn content_span(&self, node: &SyntaxNode) -> Span {
        let mut first = node.text_range().start();
        let mut last = node.text_range().end();
        let mut found_first = false;
        for elem in node.descendants_with_tokens() {
            if let Some(t) = elem.as_token()
                && !t.kind().is_trivia()
            {
                if !found_first {
                    first = t.text_range().start();
                    found_first = true;
                }
                last = t.text_range().end();
            }
        }
        Span::new(u32::from(first) + self.start_pos, u32::from(last) + self.start_pos)
    }

    /// Extend span to include leading annotations, if any.
    fn span_including_annotations(&self, node: &SyntaxNode, span: Span) -> Span {
        children(node)
            .find(|n| n.kind() == ANNOTATION)
            .map(|ann| Span::new(self.trimmed_span(&ann).lo, span.hi))
            .unwrap_or(span)
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

    /// Create a placeholder identifier for error recovery.
    fn error_identifier(&self, span: Span) -> leo_ast::Identifier {
        leo_ast::Identifier { name: Symbol::intern("_error"), span, id: self.builder.next_id() }
    }

    /// Create a placeholder expression for error recovery.
    fn error_expression(&self, span: Span) -> leo_ast::Expression {
        leo_ast::ErrExpression { span, id: self.builder.next_id() }.into()
    }

    /// Create an `IntrinsicExpression` with no type parameters.
    fn intrinsic_expression(
        &self,
        name: Symbol,
        arguments: Vec<leo_ast::Expression>,
        span: Span,
    ) -> leo_ast::Expression {
        leo_ast::IntrinsicExpression { name, type_parameters: Vec::new(), arguments, span, id: self.builder.next_id() }
            .into()
    }

    /// Create an empty block for error recovery.
    fn error_block(&self, span: Span) -> leo_ast::Block {
        leo_ast::Block { statements: Vec::new(), span, id: self.builder.next_id() }
    }

    /// Emit an error if the literal text has a hex, octal, or binary prefix,
    /// which is not allowed for non-integer types (field, group, scalar).
    fn validate_hexbin_literal(&self, text: &str, suffix_len: u32, span: Span) {
        if text.starts_with("0x") || text.starts_with("0o") || text.starts_with("0b") {
            self.handler.emit_err(ParserError::hexbin_literal_nonintegers(Span::new(span.lo, span.hi - suffix_len)));
        }
    }

    /// Find an IDENT token in `node` or emit an error and return a placeholder.
    fn require_ident(&self, node: &SyntaxNode, label: &str) -> leo_ast::Identifier {
        let span = self.to_span(node);
        match tokens(node).find(|t| t.kind() == IDENT) {
            Some(token) => self.to_identifier(&token),
            None => {
                self.emit_unexpected_str(label, node.text(), span);
                self.error_identifier(span)
            }
        }
    }

    /// Find a type child node or emit an error and return `Type::Err`.
    fn require_type(&self, node: &SyntaxNode, label: &str) -> Result<leo_ast::Type> {
        match children(node).find(|n| n.kind().is_type()) {
            Some(type_node) => self.to_type(&type_node),
            None => {
                self.emit_unexpected_str(label, node.text(), self.to_span(node));
                Ok(leo_ast::Type::Err)
            }
        }
    }

    /// Find an expression child node or emit an error and return `ErrExpression`.
    fn require_expression(&self, node: &SyntaxNode, label: &str) -> Result<leo_ast::Expression> {
        match children(node).find(|n| n.kind().is_expression()) {
            Some(expr_node) => self.to_expression(&expr_node),
            None => {
                let span = self.to_span(node);
                self.emit_unexpected_str(label, node.text(), span);
                Ok(self.error_expression(span))
            }
        }
    }

    /// Validate an identifier, checking for double underscores and length.
    fn validate_identifier(&self, ident: &leo_ast::Identifier) {
        const MAX_IDENTIFIER_LEN: usize = 31;
        let text = ident.name.to_string();
        if text.len() > MAX_IDENTIFIER_LEN {
            self.handler.emit_err(ParserError::identifier_too_long(&text, text.len(), MAX_IDENTIFIER_LEN, ident.span));
        }
        if text.contains("__") {
            self.handler.emit_err(ParserError::identifier_cannot_contain_double_underscore(&text, ident.span));
        }
    }

    /// Validate an identifier in a definition position (struct field, variable,
    /// function name, etc.). In addition to the general identifier checks, this
    /// rejects identifiers that start with `_` or that are reserved keywords.
    fn validate_definition_identifier(&self, ident: &leo_ast::Identifier) {
        // Skip validation for error-recovery placeholders.
        if ident.name == Symbol::intern("_error") {
            return;
        }
        self.validate_identifier(ident);
        let text = ident.name.to_string();
        if text.starts_with('_') {
            self.handler.emit_err(ParserError::identifier_cannot_start_with_underscore(ident.span));
        }
        if symbol_is_keyword(ident.name) {
            self.emit_unexpected_str("an identifier", &text, ident.span);
        }
    }

    // =========================================================================
    // Type Conversions
    // =========================================================================

    /// Convert a type syntax node to a Leo AST Type.
    fn to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        let ty = match node.kind() {
            TYPE_PRIMITIVE => self.type_primitive_to_type(node)?,
            TYPE_LOCATOR => self.type_locator_to_type(node)?,
            TYPE_PATH => self.type_path_to_type(node)?,
            TYPE_ARRAY => self.type_array_to_type(node)?,
            TYPE_VECTOR => self.type_vector_to_type(node)?,
            TYPE_TUPLE => self.type_tuple_to_type(node)?,
            TYPE_OPTIONAL => self.type_optional_to_type(node)?,
            TYPE_FINAL => self.type_final_to_type(node)?,
            TYPE_MAPPING => self.type_mapping_to_type(node)?,
            ERROR => {
                // Parse errors already emitted by emit_parse_errors().
                leo_ast::Type::Err
            }
            kind => panic!("unexpected type node kind: {:?}", kind),
        };
        Ok(ty)
    }

    /// Convert a TYPE_PRIMITIVE node to a Type.
    fn type_primitive_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_PRIMITIVE);
        let prim = tokens(node)
            .next()
            .and_then(|t| keyword_to_primitive_type(t.kind()))
            .expect("TYPE_PRIMITIVE should contain a type keyword");
        Ok(prim)
    }

    /// Convert a TYPE_LOCATOR node to a Type.
    ///
    /// TYPE_LOCATOR represents `program.aleo/Type` or `program.aleo`.
    fn type_locator_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_LOCATOR);

        // Extract program and type name from IDENT tokens.
        let mut idents = tokens(node).filter(|t| t.kind() == IDENT);
        if let (Some(program_token), Some(name_token)) = (idents.next(), idents.next()) {
            let program = self.to_identifier(&program_token);
            let type_name = self.to_identifier(&name_token);
            let span = Span::new(program.span.lo, type_name.span.hi);
            let path = leo_ast::Path::new(Some(program), Vec::new(), type_name, span, self.builder.next_id());

            // Extract const arguments from CONST_ARG_LIST child node
            let (_type_parameters, const_arguments) = self.extract_const_arg_list(node)?;

            return Ok(leo_ast::CompositeType { path, const_arguments }.into());
        }

        // program.aleo without /Type — single IDENT before `.aleo`
        let text = node.text().to_string();
        if text.ends_with(".aleo") {
            let program_str = text.trim_end_matches(".aleo").trim();
            let span = self.content_span(node);

            let program = leo_ast::Identifier { name: Symbol::intern(program_str), span, id: self.builder.next_id() };

            // Just the program name, no type - this is a program ID reference
            let path = leo_ast::Path::new(Some(program), Vec::new(), program, span, self.builder.next_id());
            return Ok(leo_ast::CompositeType { path, const_arguments: Vec::new() }.into());
        }

        panic!("TYPE_LOCATOR should contain .aleo: {:?}", text);
    }

    /// Convert a TYPE_PATH node to a Type.
    ///
    /// TYPE_PATH represents named/composite types: `Foo`, `Foo::Bar`, `Foo::[N]`.
    fn type_path_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_PATH);

        // Regular path: collect identifiers and const generic args
        let mut path_components = Vec::new();

        // Collect IDENT tokens that are direct children of TYPE_PATH (not inside CONST_ARG_LIST)
        for token in tokens(node) {
            match token.kind() {
                IDENT => {
                    path_components.push(self.to_identifier(&token));
                }
                // Skip punctuation
                COLON_COLON | L_BRACKET | R_BRACKET | LT | GT | COMMA | INTEGER => {}
                kind if kind.is_trivia() => {}
                kind => panic!("unexpected token in TYPE_PATH: {:?}", kind),
            }
        }

        // Extract const arguments from CONST_ARG_LIST child node
        let (_type_parameters, const_arguments) = self.extract_const_arg_list(node)?;

        // The last component is the type name, rest are path segments.
        // Path span covers only the identifier tokens, not the const arg list.
        let name = path_components.pop().expect("TYPE_PATH should have at least one identifier");
        let path_span =
            if let Some(first) = path_components.first() { Span::new(first.span.lo, name.span.hi) } else { name.span };
        let path = leo_ast::Path::new(None, path_components, name, path_span, self.builder.next_id());
        Ok(leo_ast::CompositeType { path, const_arguments }.into())
    }

    /// Convert a TYPE_ARRAY node to an ArrayType.
    fn type_array_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_ARRAY);

        let element_node = children(node).find(|n| n.kind().is_type()).expect("array type should have element type");
        let element_type = self.to_type(&element_node)?;
        let length_expr = self.array_length_to_expression(node)?;
        Ok(leo_ast::ArrayType { element_type: Box::new(element_type), length: Box::new(length_expr) }.into())
    }

    /// Convert a TYPE_VECTOR node to a VectorType.
    fn type_vector_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_VECTOR);

        let element_node = children(node).find(|n| n.kind().is_type()).expect("vector type should have element type");
        let element_type = self.to_type(&element_node)?;
        Ok(leo_ast::VectorType { element_type: Box::new(element_type) }.into())
    }

    /// Extract the array length expression from a TYPE_ARRAY node.
    fn array_length_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        match children(node).find(|n| n.kind() == ARRAY_LENGTH) {
            Some(length_node) => self.require_expression(&length_node, "array length"),
            None => {
                // Error recovery: TYPE_ARRAY without ARRAY_LENGTH (e.g. `[T, N]` typo).
                let span = self.to_span(node);
                self.emit_unexpected_str("array length", node.text(), span);
                Ok(self.error_expression(span))
            }
        }
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
        let span = self.to_span(node);

        let type_nodes: Vec<_> = children(node).filter(|n| n.kind().is_type()).collect();

        if type_nodes.is_empty() {
            // Unit type: ()
            return Ok(leo_ast::Type::Unit);
        }

        let elements = type_nodes.iter().map(|n| self.to_type(n)).collect::<Result<Vec<_>>>()?;

        if elements.len() == 1 {
            // Single-element tuple type is invalid - emit error
            self.handler.emit_err(ParserError::tuple_must_have_at_least_two_elements("type", span));
            // Return the single element for error recovery
            return Ok(elements.into_iter().next().unwrap());
        }

        Ok(leo_ast::TupleType::new(elements).into())
    }

    /// Convert a TYPE_OPTIONAL node to an OptionalType.
    fn type_optional_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_OPTIONAL);

        let inner_node = children(node).find(|n| n.kind().is_type()).expect("optional type should have inner type");

        let inner = self.to_type(&inner_node)?;
        Ok(leo_ast::Type::Optional(leo_ast::OptionalType { inner: Box::new(inner) }))
    }

    /// Convert a TYPE_FINAL node to a FutureType.
    fn type_final_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_FINAL);

        // Collect any type children (for Future<fn(T) -> R> syntax)
        let type_nodes: Vec<_> = children(node).filter(|n| n.kind().is_type()).collect();

        if type_nodes.is_empty() {
            // Simple Future with no explicit signature
            return Ok(leo_ast::FutureType::default().into());
        }

        // Future with explicit signature: Future<fn(T1, T2) -> R>
        let types = type_nodes.iter().map(|n| self.to_type(n)).collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::FutureType::new(types, None, true).into())
    }

    fn type_mapping_to_type(&self, node: &SyntaxNode) -> Result<leo_ast::Type> {
        debug_assert_eq!(node.kind(), TYPE_MAPPING);
        let mut type_nodes = children(node).filter(|n| n.kind().is_type());
        let key = type_nodes.next().map(|n| self.to_type(&n)).transpose()?.unwrap_or(leo_ast::Type::Err);
        let value = type_nodes.next().map(|n| self.to_type(&n)).transpose()?.unwrap_or(leo_ast::Type::Err);
        Ok(leo_ast::Type::Mapping(leo_ast::MappingType { key: Box::new(key), value: Box::new(value) }))
    }

    // =========================================================================
    // Expression Conversions
    // =========================================================================

    /// Convert a syntax node to an expression.
    fn to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);

        let expr = match node.kind() {
            LITERAL_FIELD => self.suffixed_literal_to_expression(node, "field", leo_ast::Literal::field)?,
            LITERAL_GROUP => self.suffixed_literal_to_expression(node, "group", leo_ast::Literal::group)?,
            LITERAL_SCALAR => self.suffixed_literal_to_expression(node, "scalar", leo_ast::Literal::scalar)?,
            LITERAL_INT => self.int_literal_to_expression(node)?,
            LITERAL_STRING => self.string_literal_to_expression(node)?,
            LITERAL_ADDRESS => self.address_literal_to_expression(node)?,
            LITERAL_BOOL => self.bool_literal_to_expression(node)?,
            LITERAL_NONE => leo_ast::Literal::none(span, self.builder.next_id()).into(),
            BINARY_EXPR => self.binary_expr_to_expression(node)?,
            UNARY_EXPR => self.unary_expr_to_expression(node)?,
            CALL_EXPR => self.call_expr_to_expression(node)?,
            METHOD_CALL_EXPR => self.method_call_expr_to_expression(node)?,
            FIELD_EXPR => self.field_expr_to_expression(node)?,
            TUPLE_ACCESS_EXPR => self.tuple_access_expr_to_expression(node)?,
            INDEX_EXPR => self.index_expr_to_expression(node)?,
            CAST_EXPR => self.cast_expr_to_expression(node)?,
            TERNARY_EXPR => self.ternary_expr_to_expression(node)?,
            ARRAY_EXPR => self.array_expr_to_expression(node)?,
            REPEAT_EXPR => self.repeat_expr_to_expression(node)?,
            TUPLE_EXPR => self.tuple_expr_to_expression(node)?,
            STRUCT_EXPR => self.struct_expr_to_expression(node)?,
            STRUCT_LOCATOR_EXPR => self.struct_locator_expr_to_expression(node)?,
            PATH_EXPR => self.path_expr_to_expression(node)?,
            PATH_LOCATOR_EXPR => self.path_locator_expr_to_expression(node)?,
            PROGRAM_REF_EXPR => self.program_ref_expr_to_expression(node)?,
            SELF_EXPR => self.keyword_expr_to_path(node, sym::SelfLower)?,
            BLOCK_KW_EXPR => self.keyword_expr_to_path(node, sym::block)?,
            NETWORK_KW_EXPR => self.keyword_expr_to_path(node, sym::network)?,
            PAREN_EXPR => {
                // Parenthesized expression - just unwrap
                if let Some(inner) = children(node).find(|n| n.kind().is_expression()) {
                    self.to_expression(&inner)?
                } else {
                    // No inner expression found - likely parse error
                    self.emit_unexpected_str("expression in parentheses", node.text(), span);
                    self.error_expression(span)
                }
            }
            // Final expression block
            FINAL_EXPR => self.final_expr_to_expression(node)?,
            // For ROOT nodes that wrap an expression (from parse_expression_entry)
            ROOT => {
                if let Some(inner) = children(node).find(|n| n.kind().is_expression()) {
                    self.to_expression(&inner)?
                } else {
                    // Parse errors already emitted by emit_parse_errors().
                    self.error_expression(span)
                }
            }
            // Error recovery: return ErrExpression for ERROR nodes.
            // Parse errors already emitted by emit_parse_errors().
            ERROR => self.error_expression(span),
            kind => panic!("unexpected expression kind: {:?}", kind),
        };

        Ok(expr)
    }

    /// Convert a suffixed literal node (field, group, scalar) to an expression.
    fn suffixed_literal_to_expression(
        &self,
        node: &SyntaxNode,
        suffix: &str,
        ctor: fn(String, Span, leo_ast::NodeID) -> leo_ast::Literal,
    ) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let token = tokens(node).next().expect("literal node should have a token");
        let text = token.text();
        self.validate_hexbin_literal(text, suffix.len() as u32, span);
        let value = text.strip_suffix(suffix).unwrap();
        Ok(ctor(value.to_string(), span, id).into())
    }

    /// Convert a LITERAL_INT node to an expression.
    fn int_literal_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let token = tokens(node).next().expect("LITERAL_INT should have a token");
        self.integer_token_to_expression(&token)
    }

    /// Convert a LITERAL_STRING node to an expression.
    fn string_literal_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let token = tokens(node).next().expect("LITERAL_STRING should have a token");
        Ok(leo_ast::Literal::string(token.text().to_string(), span, id).into())
    }

    /// Convert a LITERAL_ADDRESS node to an expression.
    fn address_literal_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let token = tokens(node).next().expect("LITERAL_ADDRESS should have a token");
        let text = token.text();
        // Validate address literal (skip program addresses like "program.aleo")
        if !text.contains(".aleo") && text.parse::<Address<TestnetV0>>().is_err() {
            self.handler.emit_err(ParserError::invalid_address_lit(text, span));
        }
        Ok(leo_ast::Literal::address(text.to_string(), span, id).into())
    }

    /// Convert a LITERAL_BOOL node to an expression.
    fn bool_literal_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let token = tokens(node).next().expect("LITERAL_BOOL should have a token");
        let value = token.kind() == KW_TRUE;
        Ok(leo_ast::Literal::boolean(value, span, id).into())
    }

    /// Convert a BINARY_EXPR node to a BinaryExpression.
    fn binary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), BINARY_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let mut operands = children(node).filter(|n| n.kind().is_expression() || n.kind().is_type());

        // Find the operator token
        let op_token = match tokens(node).find(|t| t.kind().is_operator() || t.kind() == KW_AS) {
            Some(token) => token,
            None => {
                self.emit_unexpected_str("operator in binary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        let op = token_to_binary_op(op_token.kind());

        // Get left operand
        let left = match operands.next() {
            Some(left_node) => self.to_expression(&left_node)?,
            None => {
                self.emit_unexpected_str("left operand in binary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        // KW_AS should be CAST_EXPR, not binary.
        if op_token.kind() == KW_AS {
            self.emit_unexpected_str("cast expression", "binary AS expression", span);
            return Ok(self.error_expression(span));
        }

        // Get right operand
        let right = match operands.next() {
            Some(right_node) => self.to_expression(&right_node)?,
            None => {
                self.emit_unexpected_str("right operand in binary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        Ok(leo_ast::BinaryExpression { left, right, op, span, id }.into())
    }

    /// Convert a UNARY_EXPR node to a UnaryExpression.
    fn unary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), UNARY_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // Get the operator
        let Some(op_token) = tokens(node).find(|t| matches!(t.kind(), BANG | MINUS)) else {
            self.emit_unexpected_str("operator in unary expression", node.text(), span);
            return Ok(self.error_expression(span));
        };

        let op = if op_token.kind() == BANG { leo_ast::UnaryOperation::Not } else { leo_ast::UnaryOperation::Negate };

        // Get the operand
        let Some(operand) = children(node).find(|n| n.kind().is_expression()) else {
            self.emit_unexpected_str("operand in unary expression", node.text(), span);
            return Ok(self.error_expression(span));
        };

        let mut receiver = self.to_expression(&operand)?;

        // Fold negation into numeric literals
        if op == leo_ast::UnaryOperation::Negate
            && let leo_ast::Expression::Literal(leo_ast::Literal {
                variant:
                    leo_ast::LiteralVariant::Integer(_, ref mut string)
                    | leo_ast::LiteralVariant::Field(ref mut string)
                    | leo_ast::LiteralVariant::Group(ref mut string)
                    | leo_ast::LiteralVariant::Scalar(ref mut string),
                span: ref mut lit_span,
                ..
            }) = receiver
            && !string.starts_with('-')
        {
            string.insert(0, '-');
            *lit_span = span;
            return Ok(receiver);
        }

        Ok(leo_ast::UnaryExpression { receiver, op, span, id }.into())
    }

    /// Extract type parameters and const arguments from a CONST_ARG_LIST child, if present.
    ///
    /// In the rowan CST, `CONST_ARG_LIST` children are either type nodes (for
    /// intrinsic type parameters like `Deserialize::[u32]`) or expression nodes
    /// (for const generic arguments like `Foo::[N]`).
    fn extract_const_arg_list(&self, node: &SyntaxNode) -> Result<ConstArgList> {
        let mut type_parameters = Vec::new();
        let mut const_arguments = Vec::new();
        if let Some(arg_list) = children(node).find(|n| n.kind() == CONST_ARG_LIST) {
            for child in children(&arg_list) {
                if child.kind().is_type() {
                    let span = self.content_span(&child);
                    let ty = self.to_type(&child)?;
                    type_parameters.push((ty, span));
                } else if child.kind().is_expression() {
                    let expr = self.to_expression(&child)?;
                    const_arguments.push(expr);
                }
            }
        }
        Ok((type_parameters, const_arguments))
    }

    /// Convert a CALL_EXPR node to a CallExpression.
    fn call_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), CALL_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // The first child should be the function being called (PATH_EXPR or PATH_LOCATOR_EXPR).
        let mut child_iter = children(node);
        let callee_node = child_iter.next().expect("call expr should have callee");

        let function = match callee_node.kind() {
            PATH_LOCATOR_EXPR => self.locator_tokens_to_path(&callee_node)?,
            _ => self.path_expr_to_path(&callee_node)?,
        };

        // Collect arguments (remaining expression children)
        let arguments = children(node)
            .skip(1)  // Skip the callee
            .filter(|n| n.kind().is_expression())
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        // Extract type parameters and const arguments from CONST_ARG_LIST.
        // In the rowan CST, CONST_ARG_LIST is a child of the PATH_EXPR callee node.
        let (type_parameters, const_arguments) = self.extract_const_arg_list(&callee_node)?;

        // If the path has exactly one qualifier (e.g. `group::to_x_coordinate`),
        // try to canonicalize to an intrinsic. Non-intrinsic qualified calls
        // fall through to the normal CallExpression below.
        if function.user_program().is_none() && function.qualifier().len() == 1 {
            let module = function.qualifier()[0].name;
            let name = function.identifier().name;
            if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(module, name) {
                return Ok(
                    leo_ast::IntrinsicExpression { name: intrinsic_name, type_parameters, arguments, span, id }.into()
                );
            }
        }

        Ok(leo_ast::CallExpression { function, const_arguments, arguments, span, id }.into())
    }

    /// Convert a METHOD_CALL_EXPR node to the appropriate expression.
    ///
    /// Structure: `receiver DOT method_name L_PAREN args R_PAREN`
    fn method_call_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), METHOD_CALL_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // First expression child is the receiver.
        let mut expr_children = children(node).filter(|n| n.kind().is_expression());
        let receiver = match expr_children.next() {
            Some(receiver_node) => self.to_expression(&receiver_node)?,
            None => {
                self.emit_unexpected_str("receiver in method call", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        // Get the method name (IDENT or keyword token after DOT).
        let method_name = match find_name_after_dot(node) {
            Some(method_token) => self.to_identifier(&method_token),
            None => {
                self.emit_unexpected_str("method name in method call", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        // Remaining expression children are the arguments.
        let mut args: Vec<_> = expr_children.map(|n| self.to_expression(&n)).collect::<Result<Vec<_>>>()?;

        // Check for known methods that map to unary/binary operations or intrinsics
        if args.is_empty() {
            if let Some(op) = leo_ast::UnaryOperation::from_symbol(method_name.name) {
                return Ok(leo_ast::UnaryExpression { span, op, receiver, id }.into());
            }
        } else if args.len() == 1
            && let Some(op) = leo_ast::BinaryOperation::from_symbol(method_name.name)
        {
            return Ok(leo_ast::BinaryExpression { span, op, left: receiver, right: args.pop().unwrap(), id }.into());
        }

        // Check for known intrinsic method calls.
        // Ordering follows the lossless parser (conversions.rs):
        // 1. Specific intrinsics (signature, Future, Optional)
        // 2. Unresolved `.get()`/`.set()` (deferred to type checker)
        // 3. Vector/Mapping methods
        let method = method_name.name;
        let all_args = || std::iter::once(receiver.clone()).chain(args.clone()).collect::<Vec<_>>();

        // Known module-specific intrinsics matched by name and arg count.
        let intrinsic_name = match args.len() {
            2 => leo_ast::Intrinsic::convert_path_symbols(sym::signature, method),
            0 => leo_ast::Intrinsic::convert_path_symbols(sym::Final, method)
                .or_else(|| leo_ast::Intrinsic::convert_path_symbols(sym::Optional, method)),
            1 => leo_ast::Intrinsic::convert_path_symbols(sym::Optional, method),
            _ => None,
        };
        if let Some(intrinsic_name) = intrinsic_name {
            return Ok(self.intrinsic_expression(intrinsic_name, all_args(), span));
        }

        // Unresolved `.get()` / `.set()` — the receiver type is unknown at
        // parse time, so defer resolution to the type checker.
        if method == sym::get && args.len() == 1 {
            return Ok(self.intrinsic_expression(Symbol::intern("__unresolved_get"), all_args(), span));
        }
        if method == sym::set && args.len() == 2 {
            return Ok(self.intrinsic_expression(Symbol::intern("__unresolved_set"), all_args(), span));
        }

        // Remaining Vector/Mapping method intrinsics.
        for module in [sym::Vector, sym::Mapping] {
            if let Some(intrinsic_name) = leo_ast::Intrinsic::convert_path_symbols(module, method) {
                return Ok(self.intrinsic_expression(intrinsic_name, all_args(), span));
            }
        }

        // Unknown method call - emit error
        self.handler.emit_err(ParserError::invalid_method_call(receiver, method_name, args.len(), span));
        Ok(self.error_expression(span))
    }

    /// Convert a TUPLE_ACCESS_EXPR node to a TupleAccess expression.
    fn tuple_access_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), TUPLE_ACCESS_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let inner = if let Some(inner_node) = children(node).find(|n| n.kind().is_expression()) {
            self.to_expression(&inner_node)?
        } else {
            self.emit_unexpected_str("expression in tuple access", node.text(), span);
            return Ok(self.error_expression(span));
        };

        let index_token = match tokens(node).find(|t| t.kind() == INTEGER) {
            Some(token) => token,
            None => {
                self.emit_unexpected_str("tuple index", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        let index_text = index_token.text().replace('_', "");
        let index: usize = match index_text.parse() {
            Ok(idx) => idx,
            Err(_) => {
                self.emit_unexpected_str("valid tuple index", index_text, span);
                return Ok(self.error_expression(span));
            }
        };
        Ok(leo_ast::TupleAccess { tuple: inner, index: index.into(), span, id }.into())
    }

    /// Convert a FIELD_EXPR node to a MemberAccess expression.
    fn field_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), FIELD_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // Get the inner expression and its CST kind (used for special-access dispatch).
        let (inner, first_child_kind) = match children(node).find(|n| n.kind().is_expression()) {
            Some(n) => {
                let kind = n.kind();
                (self.to_expression(&n)?, kind)
            }
            None => {
                self.emit_unexpected_str("expression in field access", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        // Get the field name (token after DOT).
        // Field names can be identifiers or keywords (e.g. `self.address`).
        let field_token = match find_name_after_dot(node) {
            Some(token) => token,
            None => {
                self.emit_unexpected_str("field name in field access", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        // Check for `name.aleo` program ID references. The rowan parser
        // creates FIELD_EXPR for these, but the reference parser treats
        // them as address literals (program addresses).
        if field_token.kind() == KW_ALEO
            && let leo_ast::Expression::Path(ref path) = inner
            && path.user_program().is_none()
            && path.qualifier().is_empty()
        {
            let full_name = format!("{}.aleo", path.identifier().name);
            return Ok(leo_ast::Literal::address(full_name, span, id).into());
        }

        // Check for special accesses: self.caller, block.height, etc.
        let field_name = Symbol::intern(field_token.text());

        let special = match (first_child_kind, field_name) {
            (SELF_EXPR, sym::address) => Some(sym::_self_address),
            (SELF_EXPR, sym::caller) => Some(sym::_self_caller),
            (SELF_EXPR, sym::checksum) => Some(sym::_self_checksum),
            (SELF_EXPR, sym::edition) => Some(sym::_self_edition),
            (SELF_EXPR, sym::id) => Some(sym::_self_id),
            (SELF_EXPR, sym::program_owner) => Some(sym::_self_program_owner),
            (SELF_EXPR, sym::signer) => Some(sym::_self_signer),
            (BLOCK_KW_EXPR, sym::height) => Some(sym::_block_height),
            (BLOCK_KW_EXPR, sym::timestamp) => Some(sym::_block_timestamp),
            (NETWORK_KW_EXPR, sym::id) => Some(sym::_network_id),
            (SELF_EXPR | BLOCK_KW_EXPR | NETWORK_KW_EXPR, _) => {
                self.handler.emit_err(ParserError::custom("Unsupported special access", span));
                return Ok(self.error_expression(span));
            }
            _ => None,
        };

        if let Some(intrinsic_name) = special {
            return Ok(self.intrinsic_expression(intrinsic_name, Vec::new(), span));
        }

        // Field token may be an identifier or keyword (e.g. `self.address`).
        let name = leo_ast::Identifier {
            name: Symbol::intern(field_token.text()),
            span: self.token_span(&field_token),
            id: self.builder.next_id(),
        };
        Ok(leo_ast::MemberAccess { inner, name, span, id }.into())
    }

    /// Convert an INDEX_EXPR node to an ArrayAccess expression.
    fn index_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), INDEX_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let mut exprs = children(node).filter(|n| n.kind().is_expression());

        let array = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("array in index expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        let index = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("index in index expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        Ok(leo_ast::ArrayAccess { array, index, span, id }.into())
    }

    /// Convert a CAST_EXPR node to a CastExpression.
    fn cast_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), CAST_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // Get the expression being cast
        let Some(expr_node) = children(node).find(|n| n.kind().is_expression()) else {
            self.emit_unexpected_str("expression in cast", node.text(), span);
            return Ok(self.error_expression(span));
        };
        let expression = self.to_expression(&expr_node)?;

        // Get the target type
        let Some(type_node) = children(node).find(|n| n.kind().is_type()) else {
            self.emit_unexpected_str("type in cast expression", node.text(), span);
            return Ok(self.error_expression(span));
        };
        let type_ = self.to_type(&type_node)?;

        Ok(leo_ast::CastExpression { expression, type_, span, id }.into())
    }

    /// Convert a TERNARY_EXPR node to a TernaryExpression.
    fn ternary_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), TERNARY_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let mut exprs = children(node).filter(|n| n.kind().is_expression());

        let condition = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("condition in ternary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        let if_true = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("true branch in ternary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        let if_false = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("false branch in ternary expression", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        Ok(leo_ast::TernaryExpression { condition, if_true, if_false, span, id }.into())
    }

    /// Convert an ARRAY_EXPR node to an ArrayExpression.
    fn array_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), ARRAY_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let elements = children(node)
            .filter(|n| n.kind().is_expression())
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::ArrayExpression { elements, span, id }.into())
    }

    /// Convert a REPEAT_EXPR node to a RepeatExpression.
    fn repeat_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), REPEAT_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let mut exprs = children(node).filter(|n| n.kind().is_expression());
        let expr = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("expression in repeat", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };
        let count = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("repeat count", node.text(), span);
                return Ok(self.error_expression(span));
            }
        };

        Ok(leo_ast::RepeatExpression { expr, count, span, id }.into())
    }

    /// Convert a TUPLE_EXPR node to a TupleExpression or UnitExpression.
    fn tuple_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), TUPLE_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let elements: Vec<_> = children(node)
            .filter(|n| n.kind().is_expression())
            .map(|n| self.to_expression(&n))
            .collect::<Result<Vec<_>>>()?;

        match elements.len() {
            0 => {
                // Empty tuple is invalid - emit error
                self.handler.emit_err(ParserError::tuple_must_have_at_least_two_elements("expression", span));
                // Return unit expression for error recovery
                Ok(leo_ast::UnitExpression { span, id }.into())
            }
            1 => {
                // Single-element tuple is invalid - emit error
                self.handler.emit_err(ParserError::tuple_must_have_at_least_two_elements("expression", span));
                // Return the single element for error recovery
                Ok(elements.into_iter().next().unwrap())
            }
            _ => Ok(leo_ast::TupleExpression { elements, span, id }.into()),
        }
    }

    /// Build a `CompositeExpression` from a path, collecting field initializers
    /// and const arguments from the node.
    fn composite_expression_from_path(
        &self,
        node: &SyntaxNode,
        path: leo_ast::Path,
        span: Span,
        id: leo_ast::NodeID,
    ) -> Result<leo_ast::Expression> {
        let members = children(node)
            .filter(|n| matches!(n.kind(), STRUCT_FIELD_INIT | STRUCT_FIELD_SHORTHAND))
            .map(|n| self.struct_field_init_to_member(&n))
            .collect::<Result<Vec<_>>>()?;
        let (_type_parameters, const_arguments) = self.extract_const_arg_list(node)?;
        Ok(leo_ast::CompositeExpression { path, const_arguments, members, span, id }.into())
    }

    /// Convert a STRUCT_EXPR node to a CompositeExpression.
    fn struct_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), STRUCT_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let path = self.struct_expr_to_path(node)?;
        self.composite_expression_from_path(node, path, span, id)
    }

    /// Extract a Path from a STRUCT_EXPR node's name tokens.
    fn struct_expr_to_path(&self, node: &SyntaxNode) -> Result<leo_ast::Path> {
        let fallback_span = self.content_span(node);

        // Collect IDENT tokens before L_BRACE, deriving span from identifiers.
        let mut path_components = Vec::new();
        for token in tokens(node) {
            if token.kind() == L_BRACE {
                break;
            }
            if token.kind() == IDENT {
                path_components.push(self.to_identifier(&token));
            }
        }

        let path_span = match (path_components.first(), path_components.last()) {
            (Some(first), Some(last)) => Span::new(first.span.lo, last.span.hi),
            _ => fallback_span,
        };

        let name = match path_components.pop() {
            Some(name) => name,
            None => {
                self.emit_unexpected_str("type name in struct expression", node.text(), fallback_span);
                self.error_identifier(fallback_span)
            }
        };
        Ok(leo_ast::Path::new(None, path_components, name, path_span, self.builder.next_id()))
    }

    /// Convert a STRUCT_LOCATOR_EXPR node to an Expression.
    fn struct_locator_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let path = self.locator_tokens_to_path(node)?;
        self.composite_expression_from_path(node, path, span, id)
    }

    /// Convert a PATH_LOCATOR_EXPR node to an Expression.
    fn path_locator_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        let path = self.locator_tokens_to_path(node)?;
        Ok(leo_ast::Expression::Path(path))
    }

    /// Extract program and type name from a locator node's IDENT tokens.
    ///
    /// Locator nodes have the structure: `IDENT DOT KW_ALEO SLASH IDENT [COLON_COLON CONST_ARG_LIST]`.
    /// The first IDENT is the program name, the second (after SLASH) is the type/function name.
    fn locator_tokens_to_path(&self, node: &SyntaxNode) -> Result<leo_ast::Path> {
        let mut idents = tokens(node).filter(|t| t.kind() == IDENT);
        let program_token = idents.next().expect("locator should have program IDENT");
        let name_token = idents.next().expect("locator should have name IDENT");

        let program = self.to_identifier(&program_token);
        let name = self.to_identifier(&name_token);
        let path_span = Span::new(program.span.lo, name.span.hi);

        Ok(leo_ast::Path::new(Some(program), Vec::new(), name, path_span, self.builder.next_id()))
    }

    /// Convert a STRUCT_FIELD_INIT or STRUCT_FIELD_SHORTHAND node to a CompositeFieldInitializer.
    fn struct_field_init_to_member(&self, node: &SyntaxNode) -> Result<leo_ast::CompositeFieldInitializer> {
        debug_assert!(matches!(node.kind(), STRUCT_FIELD_INIT | STRUCT_FIELD_SHORTHAND));
        let span = self.content_span(node);
        let id = self.builder.next_id();

        let Some(ident_token) = tokens(node).find(|t| t.kind() == IDENT) else {
            self.emit_unexpected_str("identifier in struct field", node.text(), span);
            return Ok(leo_ast::CompositeFieldInitializer {
                identifier: self.error_identifier(span),
                expression: None,
                span,
                id,
            });
        };
        let identifier = self.to_identifier(&ident_token);

        let expression = if node.kind() == STRUCT_FIELD_INIT {
            children(node).find(|n| n.kind().is_expression()).map(|n| self.to_expression(&n)).transpose()?
        } else {
            None
        };

        Ok(leo_ast::CompositeFieldInitializer { identifier, expression, span, id })
    }

    /// Convert a PROGRAM_REF_EXPR node (`name.aleo`) to an address literal.
    fn program_ref_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), PROGRAM_REF_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();
        let text: String = tokens(node).map(|t| t.text().to_string()).collect();
        Ok(leo_ast::Literal::address(text, span, id).into())
    }

    /// Convert a PATH_EXPR node to an Expression.
    fn path_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), PATH_EXPR);

        let path = self.path_expr_to_path(node)?;
        let span = self.trimmed_span(node);
        let id = self.builder.next_id();

        // Detect `group::GEN` → IntrinsicExpression.
        if path.user_program().is_none()
            && path.qualifier().len() == 1
            && path.qualifier()[0].name == sym::group
            && path.identifier().name == sym::GEN
        {
            return Ok(self.intrinsic_expression(sym::_group_gen, Vec::new(), span));
        }

        // Detect signature literals (identifiers starting with `sign1` that
        // parse as valid `Signature<TestnetV0>`).
        if path.user_program().is_none() && path.qualifier().is_empty() {
            let name_text = path.identifier().name.to_string();
            if name_text.starts_with("sign1") && name_text.parse::<Signature<TestnetV0>>().is_ok() {
                return Ok(leo_ast::Literal::signature(name_text, span, id).into());
            }
            // Reject standalone `_ident` in expression context -- these are only
            // valid as the start of intrinsic calls (e.g. `_self_caller()`).
            if name_text.starts_with('_') {
                self.handler.emit_err(ParserError::identifier_cannot_start_with_underscore(span));
                return Ok(self.error_expression(span));
            }
        }

        Ok(leo_ast::Expression::Path(path))
    }

    /// Convert a keyword expression (SELF_EXPR, BLOCK_KW_EXPR, NETWORK_KW_EXPR) to a Path.
    fn keyword_expr_to_path(&self, node: &SyntaxNode, name: Symbol) -> Result<leo_ast::Expression> {
        let span = self.trimmed_span(node);
        let ident = leo_ast::Identifier { name, span, id: self.builder.next_id() };
        let path = leo_ast::Path::new(None, Vec::new(), ident, span, self.builder.next_id());
        Ok(leo_ast::Expression::Path(path))
    }

    /// Convert a FINAL_EXPR node to an Expression.
    fn final_expr_to_expression(&self, node: &SyntaxNode) -> Result<leo_ast::Expression> {
        debug_assert_eq!(node.kind(), FINAL_EXPR);
        let span = self.content_span(node);
        let id = self.builder.next_id();

        // Find the block inside the final expression
        if let Some(block_node) = children(node).find(|n| n.kind() == BLOCK) {
            let block = self.to_block(&block_node)?;
            Ok(leo_ast::AsyncExpression { block, span, id }.into())
        } else {
            // No block found - emit error
            self.emit_unexpected_str("block in final expression", node.text(), span);
            Ok(self.error_expression(span))
        }
    }

    /// Convert a PATH_EXPR node to a Path.
    ///
    /// Note: The lexer produces single IDENT tokens for associated function paths
    /// like `group::to_x_coordinate` or `signature::verify` (via the `PathSpecial`
    /// regex). These coalesced tokens must be split on `::` to build correct path
    /// components. This is a fundamental lexer constraint — `group` and `signature`
    /// are type keywords that must also work as associated function path prefixes.
    fn path_expr_to_path(&self, node: &SyntaxNode) -> Result<leo_ast::Path> {
        let span = self.trimmed_span(node);

        // Regular path: collect identifiers
        let mut path_components = Vec::new();
        for token in tokens(node) {
            match token.kind() {
                IDENT => {
                    let text = token.text();
                    // The lexer produces single IDENT tokens for associated function
                    // paths like "group::to_x_coordinate" or "signature::verify".
                    // Split these on "::" to build the correct path components.
                    if text.contains("::") {
                        let token_span = self.token_span(&token);
                        let mut offset = token_span.lo;
                        for (i, segment) in text.split("::").enumerate() {
                            if i > 0 {
                                offset += 2; // skip "::"
                            }
                            let seg_span = Span::new(offset, offset + segment.len() as u32);
                            path_components.push(leo_ast::Identifier {
                                name: Symbol::intern(segment),
                                span: seg_span,
                                id: self.builder.next_id(),
                            });
                            offset += segment.len() as u32;
                        }
                    } else {
                        path_components.push(self.to_identifier(&token));
                    }
                }
                kind => {
                    if let Some(name) = keyword_to_path_symbol(kind) {
                        path_components.push(leo_ast::Identifier {
                            name,
                            span: self.token_span(&token),
                            id: self.builder.next_id(),
                        });
                    }
                }
            }
        }

        let name = match path_components.pop() {
            Some(name) => name,
            None => {
                self.emit_unexpected_str("identifier in path", node.text(), span);
                self.error_identifier(span)
            }
        };
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
            ASSIGN_STMT => self.simple_assign_to_statement(node)?,
            COMPOUND_ASSIGN_STMT => self.compound_assign_to_statement(node)?,
            IF_STMT => self.if_stmt_to_statement(node)?,
            FOR_STMT | FOR_INCLUSIVE_STMT => self.for_stmt_to_statement(node)?,
            BLOCK => self.to_block(node)?.into(),
            ASSERT_STMT => {
                let expression = self.require_expression(node, "expression in assert")?;
                leo_ast::AssertStatement { variant: leo_ast::AssertVariant::Assert(expression), span, id }.into()
            }
            ASSERT_EQ_STMT => {
                self.assert_binary_to_statement(node, "assert_eq", span, id, leo_ast::AssertVariant::AssertEq)?
            }
            ASSERT_NEQ_STMT => {
                self.assert_binary_to_statement(node, "assert_neq", span, id, leo_ast::AssertVariant::AssertNeq)?
            }
            // For ROOT nodes that wrap a statement (from parse_statement_entry)
            ROOT => {
                if let Some(inner) = children(node).find(|n| n.kind().is_statement()) {
                    self.to_statement(&inner)?
                } else {
                    // Parse errors already emitted by emit_parse_errors().
                    leo_ast::ExpressionStatement { expression: self.error_expression(span), span, id }.into()
                }
            }
            // Error recovery for ERROR nodes.
            // Parse errors already emitted by emit_parse_errors().
            ERROR => leo_ast::ExpressionStatement { expression: self.error_expression(span), span, id }.into(),
            kind => panic!("unexpected statement kind: {:?}", kind),
        };

        Ok(stmt)
    }

    /// Convert an ASSERT_EQ_STMT or ASSERT_NEQ_STMT node to an AssertStatement.
    fn assert_binary_to_statement(
        &self,
        node: &SyntaxNode,
        label: &str,
        span: Span,
        id: leo_ast::NodeID,
        make_variant: fn(leo_ast::Expression, leo_ast::Expression) -> leo_ast::AssertVariant,
    ) -> Result<leo_ast::Statement> {
        let mut exprs = children(node).filter(|n| n.kind().is_expression());
        let e0 = match exprs.next() {
            Some(expr) => self.to_expression(&expr)?,
            None => {
                self.emit_unexpected_str(&format!("first expression in {label}"), node.text(), span);
                self.error_expression(span)
            }
        };
        let e1 = match exprs.next() {
            Some(expr) => self.to_expression(&expr)?,
            None => {
                self.emit_unexpected_str(&format!("second expression in {label}"), node.text(), span);
                self.error_expression(span)
            }
        };
        Ok(leo_ast::AssertStatement { variant: make_variant(e0, e1), span, id }.into())
    }

    /// Convert a BLOCK node to a Block.
    fn to_block(&self, node: &SyntaxNode) -> Result<leo_ast::Block> {
        debug_assert_eq!(node.kind(), BLOCK);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let statements = children(node)
            .filter(|n| n.kind().is_statement())
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
        let place = match children(node).find(|n| matches!(n.kind(), IDENT_PATTERN | TUPLE_PATTERN | WILDCARD_PATTERN))
        {
            Some(pattern_node) => self.pattern_to_definition_place(&pattern_node)?,
            None => {
                self.emit_unexpected_str("pattern in let statement", node.text(), span);
                leo_ast::DefinitionPlace::Single(self.error_identifier(span))
            }
        };

        // Find type annotation if present
        let type_ = children(node).find(|n| n.kind().is_type()).map(|n| self.to_type(&n)).transpose()?;

        let value = self.require_expression(node, "value in let statement")?;

        Ok(leo_ast::DefinitionStatement { place, type_, value, span, id }.into())
    }

    /// Convert a pattern node to a DefinitionPlace.
    fn pattern_to_definition_place(&self, node: &SyntaxNode) -> Result<leo_ast::DefinitionPlace> {
        let span = self.to_span(node);
        match node.kind() {
            IDENT_PATTERN => {
                let ident = self.require_ident(node, "identifier in pattern");
                self.validate_definition_identifier(&ident);
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
                            let ident = self.require_ident(&n, "identifier in pattern");
                            self.validate_definition_identifier(&ident);
                            ident
                        }
                    })
                    .collect();
                Ok(leo_ast::DefinitionPlace::Multiple(names))
            }
            WILDCARD_PATTERN => {
                let ident = leo_ast::Identifier { name: Symbol::intern("_"), span, id: self.builder.next_id() };
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
            _ => {
                self.emit_unexpected_str("valid pattern", node.text(), span);
                let ident = self.error_identifier(span);
                Ok(leo_ast::DefinitionPlace::Single(ident))
            }
        }
    }

    /// Convert a CONST_STMT node to a ConstDeclaration.
    fn const_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), CONST_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let place = self.require_ident(node, "name in const declaration");

        let type_ = self.require_type(node, "type in const declaration")?;

        let value = self.require_expression(node, "value in const declaration")?;

        Ok(leo_ast::ConstDeclaration { place, type_, value, span, id }.into())
    }

    /// Convert a RETURN_STMT node to a ReturnStatement.
    fn return_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), RETURN_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        // Get optional expression
        let expression = children(node)
            .find(|n| n.kind().is_expression())
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

        let expression = self.require_expression(node, "expression in expression statement")?;

        Ok(leo_ast::ExpressionStatement { expression, span, id }.into())
    }

    /// Convert a simple ASSIGN_STMT (`x = expr;`) to a Statement.
    fn simple_assign_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), ASSIGN_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let mut exprs = children(node).filter(|n| n.kind().is_expression());

        let place = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("left side in assignment", node.text(), span);
                return Ok(leo_ast::ExpressionStatement { expression: self.error_expression(span), span, id }.into());
            }
        };

        let value = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("right side in assignment", node.text(), span);
                self.error_expression(span)
            }
        };

        Ok(leo_ast::AssignStatement { place, value, span, id }.into())
    }

    /// Convert a COMPOUND_ASSIGN_STMT (`x += expr;`) to a Statement.
    ///
    /// Desugars `x op= rhs` into `x = x op rhs`.
    fn compound_assign_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), COMPOUND_ASSIGN_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let mut exprs = children(node).filter(|n| n.kind().is_expression());

        let left = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("left side in compound assignment", node.text(), span);
                return Ok(leo_ast::ExpressionStatement { expression: self.error_expression(span), span, id }.into());
            }
        };

        let right = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("right side in compound assignment", node.text(), span);
                self.error_expression(span)
            }
        };

        let op_token =
            tokens(node).find(|t| is_assign_op(t.kind())).expect("COMPOUND_ASSIGN_STMT should have operator");

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
            k => panic!("unexpected compound assignment operator: {k:?}"),
        };

        let value =
            leo_ast::BinaryExpression { left: left.clone(), right, op: binary_op, span, id: self.builder.next_id() }
                .into();

        Ok(leo_ast::AssignStatement { place: left, value, span, id }.into())
    }

    /// Convert an IF_STMT node to a ConditionalStatement.
    fn if_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert_eq!(node.kind(), IF_STMT);
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let condition = self.require_expression(node, "condition in if statement")?;

        // Single-pass: first BLOCK or IF_STMT child is the then-block,
        // second (if any) is the else clause.
        let mut block_or_if = children(node).filter(|n| n.kind() == BLOCK || n.kind() == IF_STMT);

        let then = match block_or_if.next() {
            Some(n) if n.kind() == BLOCK => self.to_block(&n)?,
            _ => {
                self.emit_unexpected_str("then block in if statement", node.text(), span);
                self.error_block(span)
            }
        };

        let otherwise = block_or_if.next().map(|n| self.to_statement(&n)).transpose()?.map(Box::new);

        Ok(leo_ast::ConditionalStatement { condition, then, otherwise, span, id }.into())
    }

    /// Convert a FOR_STMT or FOR_INCLUSIVE_STMT node to an IterationStatement.
    fn for_stmt_to_statement(&self, node: &SyntaxNode) -> Result<leo_ast::Statement> {
        debug_assert!(matches!(node.kind(), FOR_STMT | FOR_INCLUSIVE_STMT));
        let span = self.to_span(node);
        let id = self.builder.next_id();

        let variable = self.require_ident(node, "variable in for statement");

        // Get optional type annotation
        let type_ = children(node).find(|n| n.kind().is_type()).map(|n| self.to_type(&n)).transpose()?;

        // Get range expressions (before and after ..)
        let mut exprs = children(node).filter(|n| n.kind().is_expression());

        let start = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("start expression in for statement", node.text(), span);
                self.error_expression(span)
            }
        };

        let stop = match exprs.next() {
            Some(n) => self.to_expression(&n)?,
            None => {
                self.emit_unexpected_str("stop expression in for statement", node.text(), span);
                self.error_expression(span)
            }
        };

        // Get body block
        let block = match children(node).find(|n| n.kind() == BLOCK) {
            Some(block_node) => self.to_block(&block_node)?,
            None => {
                self.emit_unexpected_str("block in for statement", node.text(), span);
                self.error_block(span)
            }
        };

        let inclusive = node.kind() == FOR_INCLUSIVE_STMT;

        Ok(leo_ast::IterationStatement { variable, type_, start, stop, inclusive, block, span, id }.into())
    }

    // =========================================================================
    // Item/Program Conversions
    // =========================================================================

    /// Collect a single program item (function, struct/record, const) into the given vectors.
    fn collect_program_item(
        &self,
        item: &SyntaxNode,
        is_in_program_block: bool,
        functions: &mut Vec<(Symbol, leo_ast::Function)>,
        composites: &mut Vec<(Symbol, leo_ast::Composite)>,
        consts: &mut Vec<(Symbol, leo_ast::ConstDeclaration)>,
    ) -> Result<()> {
        match item.kind() {
            FUNCTION_DEF | FINAL_FN_DEF | SCRIPT_DEF => {
                let func = self.to_function(item, is_in_program_block)?;
                functions.push((func.identifier.name, func));
            }
            STRUCT_DEF | RECORD_DEF => {
                let composite = self.to_composite(item)?;
                composites.push((composite.identifier.name, composite));
            }
            GLOBAL_CONST => {
                let global_const = self.to_global_const(item)?;
                consts.push((global_const.place.name, global_const));
            }
            _ => {}
        }
        Ok(())
    }

    /// Convert a syntax node to a module.
    fn to_module(&self, node: &SyntaxNode, program_name: Symbol, path: Vec<Symbol>) -> Result<leo_ast::Module> {
        // Module nodes are ROOT nodes containing items (functions, structs, consts)
        let mut functions = Vec::new();
        let mut composites = Vec::new();
        let mut consts = Vec::new();

        for child in children(node) {
            if child.kind() == PROGRAM_DECL {
                for item in children(&child) {
                    self.collect_program_item(&item, true, &mut functions, &mut composites, &mut consts)?;
                }
            } else {
                self.collect_program_item(&child, false, &mut functions, &mut composites, &mut consts)?;
            }
        }

        // Sort functions: entry points first
        functions.sort_by_key(|func| if func.1.variant.is_entry() { 0u8 } else { 1u8 });

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
        let mut constructors = Vec::new();
        let mut program_name = None;
        let mut network = None;

        for child in children(node) {
            match child.kind() {
                IMPORT => {
                    let (name, span) = self.import_to_name(&child)?;
                    imports.insert(name, span);
                }
                PROGRAM_DECL => {
                    if program_name.is_some() {
                        self.handler.emit_err(ParserError::multiple_program_declarations(self.non_trivia_span(&child)));
                        continue;
                    }
                    // Extract program name and network
                    let (pname, pnetwork) = self.program_decl_to_name(&child)?;
                    program_name = Some(pname);
                    network = Some(pnetwork);

                    // Process items inside program decl
                    for item in children(&child) {
                        self.collect_program_item(&item, true, &mut functions, &mut composites, &mut consts)?;
                        match item.kind() {
                            MAPPING_DEF => {
                                let mapping = self.to_mapping(&item)?;
                                mappings.push((mapping.identifier.name, mapping));
                            }
                            STORAGE_DEF => {
                                let storage = self.to_storage(&item)?;
                                storage_variables.push((storage.identifier.name, storage));
                            }
                            CONSTRUCTOR_DEF => {
                                constructors.push(self.to_constructor(&item)?);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    self.collect_program_item(&child, false, &mut functions, &mut composites, &mut consts)?;
                }
            }
        }

        if let Some(extra) = constructors.get(1) {
            return Err(ParserError::custom("A program can only have one constructor.", extra.span).into());
        }

        let span = self.to_span(node);
        let (Some(program_name), Some(network)) = (program_name, network) else {
            return Err(ParserError::missing_program_scope(span).into());
        };

        // Sort functions: entry points first
        functions.sort_by_key(|func| if func.1.variant.is_entry() { 0u8 } else { 1u8 });

        let program_scope = leo_ast::ProgramScope {
            program_id: leo_ast::ProgramId { name: program_name, network },
            consts,
            composites,
            mappings,
            storage_variables,
            functions,
            constructor: constructors.pop(),
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
        let name = match tokens(node).find(|t| t.kind() == IDENT) {
            Some(name_token) => Symbol::intern(name_token.text()),
            None => {
                self.emit_unexpected_str("import name", node.text(), span);
                Symbol::intern("_error")
            }
        };

        // Validate the network suffix is `.aleo`.
        if tokens(node).all(|t| t.kind() != KW_ALEO)
            && let Some(net_token) = find_invalid_network(node)
        {
            self.handler.emit_err(ParserError::invalid_network(self.token_span(&net_token)));
        }

        Ok((name, span))
    }

    /// Extract program name and network from a PROGRAM_DECL node.
    fn program_decl_to_name(&self, node: &SyntaxNode) -> Result<(leo_ast::Identifier, leo_ast::Identifier)> {
        debug_assert_eq!(node.kind(), PROGRAM_DECL);
        let span = self.to_span(node);

        // Program format: program name.aleo { ... }
        let program_name = self.require_ident(node, "program name");

        let network = match tokens(node).find(|t| t.kind() == KW_ALEO) {
            Some(aleo_token) => leo_ast::Identifier {
                name: Symbol::intern("aleo"),
                span: self.token_span(&aleo_token),
                id: self.builder.next_id(),
            },
            None => {
                // Check for an invalid network identifier (e.g. `program test.eth`).
                if let Some(net_token) = find_invalid_network(node) {
                    self.handler.emit_err(ParserError::invalid_network(self.token_span(&net_token)));
                } else {
                    self.emit_unexpected_str(".aleo network", node.text(), span);
                }
                leo_ast::Identifier { name: Symbol::intern("aleo"), span, id: self.builder.next_id() }
            }
        };

        Ok((program_name, network))
    }

    /// Collect all ANNOTATION children from a node.
    fn collect_annotations(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::Annotation>> {
        children(node).filter(|n| n.kind() == ANNOTATION).map(|n| self.to_annotation(&n)).collect()
    }

    /// Find a BLOCK child or produce an error block for recovery.
    fn require_block(&self, node: &SyntaxNode, span: Span) -> Result<leo_ast::Block> {
        Ok(children(node)
            .find(|n| n.kind() == BLOCK)
            .map(|n| self.to_block(&n))
            .transpose()?
            .unwrap_or_else(|| self.error_block(span)))
    }

    /// Convert a FUNCTION_DEF / FINAL_FN_DEF / SCRIPT_DEF node to a Function.
    fn to_function(&self, node: &SyntaxNode, is_in_program_block: bool) -> Result<leo_ast::Function> {
        debug_assert!(matches!(node.kind(), FUNCTION_DEF | FINAL_FN_DEF | SCRIPT_DEF | CONSTRUCTOR_DEF));
        let span = self.span_including_annotations(node, self.non_trivia_span(node));
        let id = self.builder.next_id();

        let annotations = self.collect_annotations(node)?;

        // Determine variant
        let variant = if is_in_program_block {
            leo_ast::Variant::EntryPoint
        } else {
            match node.kind() {
                SCRIPT_DEF => leo_ast::Variant::Script,
                FINAL_FN_DEF => leo_ast::Variant::FinalFn,
                _ => leo_ast::Variant::Fn,
            }
        };

        let identifier = self.require_ident(node, "function name");
        self.validate_identifier(&identifier);

        let const_parameters = self.extract_const_parameters(node)?;

        // Get input parameters
        let input = children(node)
            .find(|n| n.kind() == PARAM_LIST)
            .map(|n| self.param_list_to_inputs(&n))
            .transpose()?
            .unwrap_or_default();

        // Get return type and build output declarations.
        //
        // Two structures are possible:
        // - Single return: FUNCTION_DEF > ... ARROW [KW_PUBLIC|KW_PRIVATE|KW_CONSTANT]? TYPE_* BLOCK
        // - Tuple return:  FUNCTION_DEF > ... ARROW RETURN_TYPE(L_PAREN [vis TYPE_*]+ R_PAREN) BLOCK
        let (output, output_type) = if let Some(return_type_node) = children(node).find(|n| n.kind() == RETURN_TYPE) {
            // Tuple return type.
            self.return_type_to_outputs(&return_type_node)?
        } else if let Some(type_node) = children(node).find(|n| n.kind().is_type()) {
            // Single return type (direct child of FUNCTION_DEF).
            let type_ = self.to_type(&type_node)?;
            // Check for visibility keyword before the type node.
            let (mode, mode_start) = self.return_mode_before(node, &type_node);
            let type_span = self.content_span(&type_node);
            let output_span = match mode_start {
                Some(start) => Span::new(start, type_span.hi),
                None => type_span,
            };
            let output =
                vec![leo_ast::Output { mode, type_: type_.clone(), span: output_span, id: self.builder.next_id() }];
            (output, type_)
        } else {
            (Vec::new(), leo_ast::Type::Unit)
        };

        let block = self.require_block(node, span)?;

        Ok(leo_ast::Function {
            annotations,
            variant,
            identifier,
            const_parameters,
            input,
            output,
            output_type,
            block,
            span,
            id,
        })
    }

    /// Extract the visibility mode keyword that precedes a type node within a parent.
    ///
    /// Scans tokens of the parent, looking for a visibility keyword that
    /// appears immediately before the type node's text range.
    /// Returns the mode and optionally the mode token's offset-adjusted span start.
    fn return_mode_before(&self, parent: &SyntaxNode, type_node: &SyntaxNode) -> (leo_ast::Mode, Option<u32>) {
        let type_start = type_node.text_range().start();
        let mut mode = leo_ast::Mode::None;
        let mut mode_start = None;
        for token in tokens(parent) {
            let token_end = token.text_range().end();
            if token_end > type_start {
                break;
            }
            if let Some(m) = token_kind_to_mode(token.kind()) {
                mode = m;
                mode_start = Some(u32::from(token.text_range().start()) + self.start_pos);
            }
        }
        (mode, mode_start)
    }

    /// Convert a RETURN_TYPE node (tuple return) to (Vec<Output>, Type).
    fn return_type_to_outputs(&self, node: &SyntaxNode) -> Result<(Vec<leo_ast::Output>, leo_ast::Type)> {
        debug_assert_eq!(node.kind(), RETURN_TYPE);

        // RETURN_TYPE contains: L_PAREN [vis? TYPE_*]+ R_PAREN
        // Iterate children, tracking the last-seen visibility keyword.
        let mut outputs = Vec::new();
        let mut current_mode = leo_ast::Mode::None;
        let mut current_mode_start: Option<u32> = None;

        for child in node.children_with_tokens() {
            match &child {
                SyntaxElement::Token(token) if !token.kind().is_trivia() => {
                    if let Some(m) = token_kind_to_mode(token.kind()) {
                        current_mode = m;
                        current_mode_start = Some(u32::from(token.text_range().start()) + self.start_pos);
                    }
                }
                SyntaxElement::Node(child_node) if child_node.kind().is_type() => {
                    let type_ = self.to_type(child_node)?;
                    let type_span = self.content_span(child_node);
                    let output_span = match current_mode_start.take() {
                        Some(start) => Span::new(start, type_span.hi),
                        None => type_span,
                    };
                    outputs.push(leo_ast::Output {
                        mode: current_mode,
                        type_,
                        span: output_span,
                        id: self.builder.next_id(),
                    });
                    current_mode = leo_ast::Mode::None;
                }
                _ => {}
            }
        }

        let output_type = match outputs.len() {
            0 => leo_ast::Type::Unit,
            1 => outputs[0].type_.clone(),
            _ => leo_ast::TupleType::new(outputs.iter().map(|o| o.type_.clone()).collect()).into(),
        };

        Ok((outputs, output_type))
    }

    /// Convert an ANNOTATION node to an Annotation.
    fn to_annotation(&self, node: &SyntaxNode) -> Result<leo_ast::Annotation> {
        debug_assert_eq!(node.kind(), ANNOTATION);
        let span = self.trimmed_span(node);
        let id = self.builder.next_id();

        // Annotation names can be identifiers or keywords (e.g. @program, @test).
        // The name is the first IDENT or keyword token after `@`.
        let name_token =
            tokens(node).find(|t| t.kind() == IDENT || t.kind().is_keyword()).expect("annotation should have name");
        let name = Symbol::intern(name_token.text());
        let name_span = self.token_span(&name_token);
        let identifier = leo_ast::Identifier { name, span: name_span, id: self.builder.next_id() };

        // Parse annotation key-value pairs from ANNOTATION_PAIR child nodes.
        let map = children(node)
            .filter(|n| n.kind() == ANNOTATION_PAIR)
            .filter_map(|pair| {
                let key =
                    tokens(&pair).find(|t| t.kind() == IDENT || t.kind() == KW_ADDRESS || t.kind() == KW_MAPPING)?;
                let val = tokens(&pair).find(|t| t.kind() == STRING)?;
                let text = val.text();
                Some((Symbol::intern(key.text()), text[1..text.len() - 1].to_string()))
            })
            .collect();

        Ok(leo_ast::Annotation { identifier, map, span, id })
    }

    /// Convert a PARAM_LIST node to function inputs.
    fn param_list_to_inputs(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::Input>> {
        debug_assert_eq!(node.kind(), PARAM_LIST);

        children(node)
            .filter(|n| matches!(n.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT))
            .map(|n| self.param_to_input(&n))
            .collect()
    }

    /// Convert a PARAM node to an Input.
    fn param_to_input(&self, node: &SyntaxNode) -> Result<leo_ast::Input> {
        debug_assert!(matches!(node.kind(), PARAM | PARAM_PUBLIC | PARAM_PRIVATE | PARAM_CONSTANT));
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let mode = node_kind_to_mode(node.kind());

        let identifier = self.require_ident(node, "parameter name");
        self.validate_identifier(&identifier);

        let type_ = self.require_type(node, "parameter type")?;

        Ok(leo_ast::Input { identifier, mode, type_, span, id })
    }

    /// Convert a const parameter list.
    fn to_const_parameters(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::ConstParameter>> {
        debug_assert_eq!(node.kind(), CONST_PARAM_LIST);

        children(node)
            .filter(|n| n.kind() == CONST_PARAM)
            .map(|n| {
                let span = self.non_trivia_span(&n);
                let id = self.builder.next_id();

                let identifier = self.require_ident(&n, "const parameter name");

                let type_ = self.require_type(&n, "const parameter type")?;

                Ok(leo_ast::ConstParameter { identifier, type_, span, id })
            })
            .collect()
    }

    /// Extract optional const parameters from a node with a CONST_PARAM_LIST child.
    fn extract_const_parameters(&self, node: &SyntaxNode) -> Result<Vec<leo_ast::ConstParameter>> {
        children(node)
            .find(|n| n.kind() == CONST_PARAM_LIST)
            .map(|n| self.to_const_parameters(&n))
            .transpose()
            .map(|opt| opt.unwrap_or_default())
    }

    /// Convert a STRUCT_DEF or RECORD_DEF node to a Composite.
    fn to_composite(&self, node: &SyntaxNode) -> Result<leo_ast::Composite> {
        debug_assert!(matches!(node.kind(), STRUCT_DEF | RECORD_DEF));
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let is_record = node.kind() == RECORD_DEF;

        let identifier = self.require_ident(node, "struct/record name");
        self.validate_identifier(&identifier);

        let const_parameters = self.extract_const_parameters(node)?;

        // Get members
        let members = children(node)
            .filter(|n| {
                matches!(
                    n.kind(),
                    STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT
                )
            })
            .map(|n| self.struct_member_to_member(&n))
            .collect::<Result<Vec<_>>>()?;

        Ok(leo_ast::Composite { identifier, const_parameters, members, is_record, span, id })
    }

    /// Convert a STRUCT_MEMBER node to a Member.
    fn struct_member_to_member(&self, node: &SyntaxNode) -> Result<leo_ast::Member> {
        debug_assert!(matches!(
            node.kind(),
            STRUCT_MEMBER | STRUCT_MEMBER_PUBLIC | STRUCT_MEMBER_PRIVATE | STRUCT_MEMBER_CONSTANT
        ));
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let mode = node_kind_to_mode(node.kind());

        let identifier = self.require_ident(node, "member name");
        self.validate_identifier(&identifier);

        let type_ = self.require_type(node, "member type")?;

        Ok(leo_ast::Member { mode, identifier, type_, span, id })
    }

    /// Convert a GLOBAL_CONST node to a ConstDeclaration.
    fn to_global_const(&self, node: &SyntaxNode) -> Result<leo_ast::ConstDeclaration> {
        debug_assert_eq!(node.kind(), GLOBAL_CONST);
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let place = self.require_ident(node, "const name");
        self.validate_definition_identifier(&place);

        let type_ = self.require_type(node, "const type")?;

        let value = self.require_expression(node, "const value")?;

        Ok(leo_ast::ConstDeclaration { place, type_, value, span, id })
    }

    /// Convert a MAPPING_DEF node to a Mapping.
    fn to_mapping(&self, node: &SyntaxNode) -> Result<leo_ast::Mapping> {
        debug_assert_eq!(node.kind(), MAPPING_DEF);
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let identifier = self.require_ident(node, "name in mapping");

        // Get key and value types
        let mut type_nodes = children(node).filter(|n| n.kind().is_type());

        let key_type = match type_nodes.next() {
            Some(key_node) => self.to_type(&key_node)?,
            None => {
                self.emit_unexpected_str("key type in mapping", node.text(), span);
                leo_ast::Type::Err
            }
        };

        let value_type = match type_nodes.next() {
            Some(value_node) => self.to_type(&value_node)?,
            None => {
                self.emit_unexpected_str("value type in mapping", node.text(), span);
                leo_ast::Type::Err
            }
        };

        Ok(leo_ast::Mapping { identifier, key_type, value_type, span, id })
    }

    /// Convert a STORAGE_DEF node to a StorageVariable.
    fn to_storage(&self, node: &SyntaxNode) -> Result<leo_ast::StorageVariable> {
        debug_assert_eq!(node.kind(), STORAGE_DEF);
        let span = self.non_trivia_span(node);
        let id = self.builder.next_id();

        let name = self.require_ident(node, "name in storage");

        let type_ = self.require_type(node, "type in storage")?;

        Ok(leo_ast::StorageVariable { identifier: name, type_, span, id })
    }

    /// Convert a CONSTRUCTOR_DEF node to a Constructor.
    fn to_constructor(&self, node: &SyntaxNode) -> Result<leo_ast::Constructor> {
        debug_assert_eq!(node.kind(), CONSTRUCTOR_DEF);
        let span = self.span_including_annotations(node, self.non_trivia_span(node));
        let id = self.builder.next_id();

        let annotations = self.collect_annotations(node)?;
        let block = self.require_block(node, span)?;

        Ok(leo_ast::Constructor { annotations, block, span, id })
    }
}

// =============================================================================
// Public Parse Functions
// =============================================================================

/// Create a span from a rowan `TextRange`, clamping to source bounds and ensuring `hi >= lo`.
fn clamped_span(range: TextRange, start_pos: u32, source_len: u32) -> Span {
    let end = start_pos + source_len;
    let lo = (u32::from(range.start()) + start_pos).min(end);
    let hi = (u32::from(range.end()) + start_pos).min(end).max(lo);
    Span::new(lo, hi)
}

/// Emit lexer errors to the handler with appropriate error types.
fn emit_lex_errors(handler: &Handler, lex_errors: &[leo_parser_rowan::LexError], start_pos: u32, source_len: u32) {
    use leo_parser_rowan::LexErrorKind;
    for error in lex_errors {
        let span = clamped_span(error.range, start_pos, source_len);

        match &error.kind {
            LexErrorKind::InvalidDigit { digit, radix, token } => {
                handler.emit_err(ParserError::wrong_digit_for_radix_span(*digit, *radix, token, span));
            }
            LexErrorKind::CouldNotLex { content } => {
                handler.emit_err(ParserError::could_not_lex_span(content, span));
            }
            LexErrorKind::BidiOverride => {
                handler.emit_err(ParserError::lexer_bidi_override_span(span));
            }
        }
    }
}

/// Emit parse errors to the handler, using structured error types when available.
/// Duplicate errors at the same location are filtered out to prevent cascading errors.
fn emit_parse_errors(
    handler: &Handler,
    errors: &[leo_parser_rowan::ParseError],
    start_pos: u32,
    source_len: u32,
    lex_errors: &[leo_parser_rowan::LexError],
) {
    use std::collections::HashSet;

    let has_lex_errors = !lex_errors.is_empty();

    // Collect lex error byte ranges so we can skip overlapping parse errors.
    let lex_ranges: Vec<(u32, u32)> = lex_errors
        .iter()
        .map(|e| {
            let lo = u32::from(e.range.start()).saturating_add(start_pos);
            let hi = u32::from(e.range.end()).saturating_add(start_pos);
            (lo, hi)
        })
        .collect();

    // Track emitted error ranges to prevent duplicate errors at the same location
    let mut emitted_ranges: HashSet<(u32, u32)> = HashSet::new();
    let mut count = 0;
    let max_errors = 10;

    for error in errors {
        if count >= max_errors {
            break;
        }

        let span = clamped_span(error.range, start_pos, source_len);
        let range_key = (span.lo, span.hi);

        // Skip if we already emitted an error at this exact range
        if emitted_ranges.contains(&range_key) {
            continue;
        }

        // When there are lex errors, skip parse errors at EOF since
        // they are secondary effects of the lex failure.
        if has_lex_errors && span.lo == span.hi && span.hi == start_pos + source_len {
            continue;
        }

        // Skip parse errors that overlap with lex error ranges — these
        // are secondary effects of the lex failure already reported.
        if lex_ranges.iter().any(|&(lo, hi)| span.lo < hi && span.hi > lo) {
            continue;
        }

        emitted_ranges.insert(range_key);

        // Detect EOF errors: the found token is empty or "end of file", or the
        // span sits at/past the end of source.
        let is_eof_error = match &error.found {
            Some(f) => f.is_empty() || f == "end of file",
            None => false,
        } || (span.lo == span.hi && span.hi >= start_pos + source_len);

        if is_eof_error {
            handler.emit_err(ParserError::unexpected_eof(span));
            count += 1;
            continue;
        }

        // Use ParserError::unexpected if we have structured found/expected info
        if let Some(found) = &error.found {
            if error.expected.is_empty() {
                // No structured expected tokens — use the message as a custom error.
                // This covers errors from `error()` like "expected field name".
                handler.emit_err(ParserError::custom(&error.message, span));
            } else {
                let expected_str = error.expected.join(", ");
                handler.emit_err(ParserError::unexpected(found, expected_str, span));
            }
            count += 1;
            continue;
        }

        // Fall back to custom error for unstructured errors
        handler.emit_err(ParserError::custom(&error.message, span));
        count += 1;
    }
}

/// Emit lex and parse errors, then create a `ConversionContext`.
fn conversion_context<'a>(
    handler: &'a Handler,
    node_builder: &'a NodeBuilder,
    lex_errors: &[leo_parser_rowan::LexError],
    parse_errors: &[leo_parser_rowan::ParseError],
    start_pos: u32,
    source_len: u32,
) -> ConversionContext<'a> {
    emit_lex_errors(handler, lex_errors, start_pos, source_len);
    emit_parse_errors(handler, parse_errors, start_pos, source_len, lex_errors);
    let has_errors = !parse_errors.is_empty() || !lex_errors.is_empty();
    ConversionContext::new(handler, node_builder, start_pos, has_errors)
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
    let ctx =
        conversion_context(&handler, node_builder, parse.lex_errors(), parse.errors(), start_pos, source.len() as u32);
    ctx.to_expression(&parse.syntax())
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
    let ctx =
        conversion_context(&handler, node_builder, parse.lex_errors(), parse.errors(), start_pos, source.len() as u32);
    ctx.to_statement(&parse.syntax())
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
    let ctx =
        conversion_context(&handler, node_builder, parse.lex_errors(), parse.errors(), start_pos, source.len() as u32);
    ctx.to_module(&parse.syntax(), program_name, path)
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
    let main_context = conversion_context(
        &handler,
        node_builder,
        parse.lex_errors(),
        parse.errors(),
        source.absolute_start,
        source.src.len() as u32,
    );
    let mut program = main_context.to_main(&parse.syntax())?;
    let program_name = *program.program_scopes.first().unwrap().0;

    // Determine the root directory of the main file (for module resolution)
    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    for module in modules {
        let module_parse = leo_parser_rowan::parse_module_entry(&module.src);
        let module_context = conversion_context(
            &handler,
            node_builder,
            module_parse.lex_errors(),
            module_parse.errors(),
            module.absolute_start,
            module.src.len() as u32,
        );

        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            for segment in &key {
                if symbol_is_keyword(*segment) {
                    return Err(ParserError::keyword_used_as_module_name(key.iter().format("::"), segment).into());
                }
            }
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

/// Find the first IDENT or keyword token after the DOT in a node.
fn find_name_after_dot(node: &SyntaxNode) -> Option<SyntaxToken> {
    let dot_end = tokens(node).find(|t| t.kind() == DOT)?.text_range().end();
    tokens(node).filter(|t| t.text_range().start() >= dot_end).find(|t| t.kind() == IDENT || t.kind().is_keyword())
}

/// First non-trivia direct token of a node.
fn first_non_trivia_token(node: &SyntaxNode) -> Option<SyntaxToken> {
    node.children_with_tokens().find_map(|e| e.into_token().filter(|t| !t.kind().is_trivia()))
}

/// Last non-trivia direct token of a node.
fn last_non_trivia_token(node: &SyntaxNode) -> Option<SyntaxToken> {
    node.children_with_tokens().filter_map(|e| e.into_token().filter(|t| !t.kind().is_trivia())).last()
}

/// Find an invalid network identifier (IDENT after DOT) in a node's tokens.
fn find_invalid_network(node: &SyntaxNode) -> Option<SyntaxToken> {
    let mut saw_dot = false;
    tokens(node).find(|t| {
        if t.kind() == DOT {
            saw_dot = true;
            return false;
        }
        saw_dot && t.kind() == IDENT
    })
}

/// Convert a visibility keyword token kind to a `Mode`.
fn token_kind_to_mode(kind: SyntaxKind) -> Option<leo_ast::Mode> {
    match kind {
        KW_PUBLIC => Some(leo_ast::Mode::Public),
        KW_PRIVATE => Some(leo_ast::Mode::Private),
        KW_CONSTANT => Some(leo_ast::Mode::Constant),
        _ => None,
    }
}

/// Convert a parameter or struct member node kind to a `Mode`.
fn node_kind_to_mode(kind: SyntaxKind) -> leo_ast::Mode {
    match kind {
        PARAM_PUBLIC | STRUCT_MEMBER_PUBLIC => leo_ast::Mode::Public,
        PARAM_PRIVATE | STRUCT_MEMBER_PRIVATE => leo_ast::Mode::Private,
        PARAM_CONSTANT | STRUCT_MEMBER_CONSTANT => leo_ast::Mode::Constant,
        _ => leo_ast::Mode::None,
    }
}

/// Convert a keyword token kind to the corresponding path symbol, if applicable.
fn keyword_to_path_symbol(kind: SyntaxKind) -> Option<Symbol> {
    match kind {
        KW_SELF => Some(sym::SelfLower),
        KW_BLOCK => Some(sym::block),
        KW_NETWORK => Some(sym::network),
        KW_FINAL_UPPER => Some(sym::Final),
        _ => None,
    }
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
            | sym::block
            | sym::bool
            | sym::Const
            | sym::constant
            | sym::constructor
            | sym::Else
            | sym::False
            | sym::field
            | sym::FnUpper
            | sym::Fn
            | sym::For
            | sym::Final
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

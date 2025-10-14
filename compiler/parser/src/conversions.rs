// Copyright (C) 2019-2025 Provable Inc.
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

use indexmap::IndexMap;

use snarkvm::prelude::{Address, TestnetV0};

use leo_ast::{Expression, NodeBuilder};
use leo_errors::{Handler, ParserError, Result, TypeCheckerError};
use leo_parser_lossless::{
    ExpressionKind,
    IntegerLiteralKind,
    IntegerTypeKind,
    LiteralKind,
    StatementKind,
    SyntaxKind,
    SyntaxNode,
    TypeKind,
};
use leo_span::{Span, Symbol, sym};

fn to_identifier(node: &SyntaxNode<'_>, builder: &NodeBuilder) -> leo_ast::Identifier {
    let name = Symbol::intern(node.text);
    leo_ast::Identifier { name, span: node.span, id: builder.next_id() }
}

fn path_to_parts(node: &SyntaxNode<'_>, builder: &NodeBuilder) -> Vec<leo_ast::Identifier> {
    let mut identifiers = Vec::new();
    let mut i = node.span.lo;
    for text in node.text.split("::") {
        let end = i + text.len() as u32;
        let span = leo_span::Span { lo: i, hi: end };
        let name = Symbol::intern(text);
        identifiers.push(leo_ast::Identifier { name, span, id: builder.next_id() });
        // Account for the "::".
        i = end + 2;
    }
    identifiers
}

fn to_mode(node: &SyntaxNode<'_>) -> leo_ast::Mode {
    match node.text {
        "constant" => leo_ast::Mode::Constant,
        "private" => leo_ast::Mode::Private,
        "public" => leo_ast::Mode::Public,
        _ => leo_ast::Mode::None,
    }
}

fn to_type(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Type> {
    let SyntaxKind::Type(type_kind) = node.kind else { todo!() };

    let type_ = match type_kind {
        TypeKind::Address => leo_ast::Type::Address,
        TypeKind::Array => {
            let [_l, type_, _s, length, _r] = &node.children[..] else {
                // This "Can't happen" panic, like others in this file, will not be triggered unless
                // there is an error in grammar.lalrpop.
                panic!("Can't happen");
            };
            let element_type = to_type(type_, builder, handler)?;
            let length = to_expression(length, builder, handler)?;
            leo_ast::ArrayType { element_type: Box::new(element_type), length: Box::new(length) }.into()
        }
        TypeKind::Boolean => leo_ast::Type::Boolean,
        TypeKind::Composite => {
            let name = &node.children[0];
            if let Some((program, name_str)) = name.text.split_once(".aleo/") {
                // This is a locator.
                let name_id = leo_ast::Identifier {
                    name: Symbol::intern(name_str),
                    span: leo_span::Span {
                        lo: name.span.lo + program.len() as u32 + 5,
                        hi: name.span.lo + name.text.len() as u32,
                    },
                    id: builder.next_id(),
                };
                leo_ast::CompositeType {
                    path: leo_ast::Path::new(Vec::new(), name_id, false, None, name_id.span, builder.next_id()),
                    const_arguments: Vec::new(),
                    program: Some(Symbol::intern(program)),
                }
                .into()
            } else {
                // It's a path.
                let mut path_components = path_to_parts(name, builder);
                let mut const_arguments = Vec::new();
                if let Some(arg_list) = node.children.get(1) {
                    const_arguments = arg_list
                        .children
                        .iter()
                        .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                        .map(|child| to_expression(child, builder, handler))
                        .collect::<Result<Vec<_>>>()?;
                }
                let identifier = path_components.pop().unwrap();
                let path = leo_ast::Path::new(path_components, identifier, false, None, name.span, builder.next_id());
                leo_ast::CompositeType { path, const_arguments, program: None }.into()
            }
        }
        TypeKind::Field => leo_ast::Type::Field,
        TypeKind::Future => {
            if node.children.len() == 1 {
                leo_ast::FutureType::default().into()
            } else {
                let types = node
                    .children
                    .iter()
                    .filter(|child| matches!(child.kind, SyntaxKind::Type(..)))
                    .map(|child| to_type(child, builder, handler))
                    .collect::<Result<Vec<_>>>()?;
                leo_ast::FutureType::new(types, None, true).into()
            }
        }
        TypeKind::Group => leo_ast::Type::Group,
        TypeKind::Identifier => todo!(),
        TypeKind::Integer(int_type_kind) => {
            let int_type = match int_type_kind {
                IntegerTypeKind::U8 => leo_ast::IntegerType::U8,
                IntegerTypeKind::U16 => leo_ast::IntegerType::U16,
                IntegerTypeKind::U32 => leo_ast::IntegerType::U32,
                IntegerTypeKind::U64 => leo_ast::IntegerType::U64,
                IntegerTypeKind::U128 => leo_ast::IntegerType::U128,
                IntegerTypeKind::I8 => leo_ast::IntegerType::I8,
                IntegerTypeKind::I16 => leo_ast::IntegerType::I16,
                IntegerTypeKind::I32 => leo_ast::IntegerType::I32,
                IntegerTypeKind::I64 => leo_ast::IntegerType::I64,
                IntegerTypeKind::I128 => leo_ast::IntegerType::I128,
            };
            leo_ast::Type::Integer(int_type)
        }
        TypeKind::Mapping => {
            todo!()
        }
        TypeKind::Optional => {
            let [inner_type, _q] = &node.children[..] else {
                // This "Can't happen" panic, like others in this file, will not be triggered unless
                // there is an error in grammar.lalrpop.
                panic!("Can't happen");
            };
            leo_ast::Type::Optional(leo_ast::OptionalType { inner: Box::new(to_type(inner_type, builder, handler)?) })
        }
        TypeKind::Scalar => leo_ast::Type::Scalar,
        TypeKind::Signature => leo_ast::Type::Signature,
        TypeKind::String => leo_ast::Type::String,
        TypeKind::Tuple => {
            let elements = node
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Type(..)))
                .map(|child| to_type(child, builder, handler))
                .collect::<Result<Vec<_>>>()?;
            leo_ast::TupleType::new(elements).into()
        }
        TypeKind::Numeric => leo_ast::Type::Numeric,
        TypeKind::Unit => leo_ast::Type::Unit,
    };

    Ok(type_)
}

fn to_block(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Block> {
    assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::Block));

    let statements = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Statement(..)))
        .map(|child| to_statement(child, builder, handler))
        .collect::<Result<Vec<_>>>()?;
    Ok(leo_ast::Block { statements, span: node.span, id: builder.next_id() })
}

pub fn to_statement(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Statement> {
    let SyntaxKind::Statement(statement_kind) = node.kind else { todo!() };

    let span = node.span;
    let id = builder.next_id();

    let value = match statement_kind {
        StatementKind::Assert => {
            let [_a, _left, expr, _right, _s] = &node.children[..] else {
                panic!("Can't happen");
            };

            let expr = to_expression(expr, builder, handler)?;
            let variant = leo_ast::AssertVariant::Assert(expr);

            leo_ast::AssertStatement { variant, span, id }.into()
        }
        StatementKind::AssertEq => {
            let [_a, _left, e0, _c, e1, _right, _s] = &node.children[..] else {
                panic!("Can't happen");
            };

            let e0 = to_expression(e0, builder, handler)?;
            let e1 = to_expression(e1, builder, handler)?;
            let variant = leo_ast::AssertVariant::AssertEq(e0, e1);

            leo_ast::AssertStatement { variant, span, id }.into()
        }
        StatementKind::AssertNeq => {
            let [_a, _left, e0, _c, e1, _right, _s] = &node.children[..] else {
                panic!("Can't happen");
            };

            let e0 = to_expression(e0, builder, handler)?;
            let e1 = to_expression(e1, builder, handler)?;
            let variant = leo_ast::AssertVariant::AssertNeq(e0, e1);

            leo_ast::AssertStatement { variant, span, id }.into()
        }
        StatementKind::Assign => {
            let [lhs, a, rhs, _s] = &node.children[..] else {
                panic!("Can't happen");
            };

            let left = to_expression(lhs, builder, handler)?;
            let right = to_expression(rhs, builder, handler)?;
            if a.text == "=" {
                // Just a regular assignment.
                leo_ast::AssignStatement { place: left, value: right, span, id }.into()
            } else {
                // We have to translate it into a binary operation and assignment.
                let op = match a.text {
                    "+=" => leo_ast::BinaryOperation::Add,
                    "-=" => leo_ast::BinaryOperation::Sub,
                    "*=" => leo_ast::BinaryOperation::Mul,
                    "/=" => leo_ast::BinaryOperation::Div,
                    "%=" => leo_ast::BinaryOperation::Rem,
                    "**=" => leo_ast::BinaryOperation::Pow,
                    "<<=" => leo_ast::BinaryOperation::Shl,
                    ">>=" => leo_ast::BinaryOperation::Shr,
                    "&=" => leo_ast::BinaryOperation::BitwiseAnd,
                    "|=" => leo_ast::BinaryOperation::BitwiseOr,
                    "^=" => leo_ast::BinaryOperation::Xor,
                    "&&=" => leo_ast::BinaryOperation::And,
                    "||=" => leo_ast::BinaryOperation::Or,
                    _ => panic!("Can't happen"),
                };

                let binary_expr = leo_ast::BinaryExpression { left: left.clone(), right, op, span, id };

                leo_ast::AssignStatement { place: left, value: binary_expr.into(), span, id }.into()
            }
        }
        StatementKind::Block => to_block(node, builder, handler)?.into(),
        StatementKind::Conditional => {
            match &node.children[..] {
                [_if, c, block] => {
                    // No else.
                    let condition = to_expression(c, builder, handler)?;
                    let then = to_block(block, builder, handler)?;
                    leo_ast::ConditionalStatement { condition, then, otherwise: None, span, id }.into()
                }
                [_if, c, block, _else, otherwise] => {
                    // An else clause.
                    let condition = to_expression(c, builder, handler)?;
                    let then = to_block(block, builder, handler)?;
                    let otherwise = to_statement(otherwise, builder, handler)?;
                    leo_ast::ConditionalStatement { condition, then, otherwise: Some(Box::new(otherwise)), span, id }
                        .into()
                }

                _ => panic!("Can't happen"),
            }
        }
        StatementKind::Const => {
            let [_const, name, _c, type_, _a, rhs, _s] = &node.children[..] else {
                panic!("Can't happen");
            };

            let place = to_identifier(name, builder);
            let type_ = to_type(type_, builder, handler)?;
            let value = to_expression(rhs, builder, handler)?;

            leo_ast::ConstDeclaration { place, type_, value, span, id }.into()
        }
        StatementKind::Definition => {
            match &node.children[..] {
                [_let, name, _c, type_, _assign, e, _s] => {
                    // Singe place, type.
                    let name = to_identifier(name, builder);
                    let place = leo_ast::DefinitionPlace::Single(name);
                    let value = to_expression(e, builder, handler)?;
                    let type_ = Some(to_type(type_, builder, handler)?);
                    leo_ast::DefinitionStatement { place, type_, value, span, id }.into()
                }
                [_let, name, _assign, e, _s] => {
                    // Single place, no type.
                    let name = to_identifier(name, builder);
                    let place = leo_ast::DefinitionPlace::Single(name);
                    let value = to_expression(e, builder, handler)?;
                    leo_ast::DefinitionStatement { place, type_: None, value, span, id }.into()
                }
                children => {
                    // Multiple place, with or without type.
                    let right_paren_index = children.iter().position(|child| child.text == ")").unwrap();

                    // The items between parens.
                    let names = children[2..right_paren_index].iter()
                        // The ones that aren't commas - the identifiers.
                        .filter(|child| child.text != ",")
                        .map(|child| to_identifier(child, builder))
                    .collect();

                    // Get the type, if there is one.
                    let type_ = children
                        .iter()
                        .find(|child| matches!(child.kind, SyntaxKind::Type(..)))
                        .map(|child| to_type(child, builder, handler))
                        .transpose()?;

                    let expr = &children[children.len() - 2];
                    let value = to_expression(expr, builder, handler)?;
                    let place = leo_ast::DefinitionPlace::Multiple(names);
                    leo_ast::DefinitionStatement { place, type_, value, span, id }.into()
                }
            }
        }
        StatementKind::Expression => {
            let expression = to_expression(&node.children[0], builder, handler)?;
            leo_ast::ExpressionStatement { expression, span, id }.into()
        }
        StatementKind::Iteration => {
            match &node.children[..] {
                [_f, i, _n, low, _d, hi, block] => {
                    // No type.
                    leo_ast::IterationStatement {
                        variable: to_identifier(i, builder),
                        type_: None,
                        start: to_expression(low, builder, handler)?,
                        stop: to_expression(hi, builder, handler)?,
                        inclusive: false,
                        block: to_block(block, builder, handler)?,
                        span,
                        id,
                    }
                    .into()
                }
                [_f, i, _c, type_, _n, low, _d, hi, block] => {
                    // With type.
                    leo_ast::IterationStatement {
                        variable: to_identifier(i, builder),
                        type_: Some(to_type(type_, builder, handler)?),
                        start: to_expression(low, builder, handler)?,
                        stop: to_expression(hi, builder, handler)?,
                        inclusive: false,
                        block: to_block(block, builder, handler)?,
                        span,
                        id,
                    }
                    .into()
                }
                _ => panic!("Can't happen"),
            }
        }
        StatementKind::Return => {
            match &node.children[..] {
                [_r, e, _s] => {
                    // With expression.
                    let expression = to_expression(e, builder, handler)?;
                    leo_ast::ReturnStatement { expression, span, id }.into()
                }
                [_r, _s] => {
                    // No expression.
                    leo_ast::ReturnStatement {
                        expression: leo_ast::UnitExpression { span, id: builder.next_id() }.into(),
                        span,
                        id,
                    }
                    .into()
                }
                _ => panic!("Can't happen"),
            }
        }
    };

    Ok(value)
}

pub fn to_expression(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<Expression> {
    let SyntaxKind::Expression(expression_kind) = node.kind else { panic!("Can't happen") };

    let span = node.span;
    let id = builder.next_id();
    let text = || node.text.to_string();

    let value = match expression_kind {
        ExpressionKind::ArrayAccess => {
            let [array, _left, index, _right] = &node.children[..] else {
                panic!("Can't happen");
            };

            let array = to_expression(array, builder, handler)?;
            let index = to_expression(index, builder, handler)?;

            leo_ast::ArrayAccess { array, index, span, id }.into()
        }
        ExpressionKind::AssociatedConstant => {
            let mut components = path_to_parts(&node.children[0], builder);
            let name = components.pop().unwrap();
            let variant = components.pop().unwrap();
            leo_ast::AssociatedConstantExpression { ty: leo_ast::Type::Identifier(variant), name, span, id }.into()
        }
        ExpressionKind::AssociatedFunctionCall => {
            let mut components = path_to_parts(&node.children[0], builder);
            let name = components.pop().unwrap();
            let variant = components.pop().unwrap();
            let arguments = node
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                .map(|child| to_expression(child, builder, handler))
                .collect::<Result<Vec<_>>>()?;

            leo_ast::AssociatedFunctionExpression { variant, name, arguments, span, id }.into()
        }
        ExpressionKind::Async => {
            let [_a, block] = &node.children[..] else {
                panic!("Can't happen");
            };
            leo_ast::AsyncExpression { block: to_block(block, builder, handler)?, span, id }.into()
        }
        ExpressionKind::Array => {
            let elements = node
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                .map(|child| to_expression(child, builder, handler))
                .collect::<Result<Vec<_>>>()?;
            leo_ast::ArrayExpression { elements, span, id }.into()
        }
        ExpressionKind::Binary => {
            let [lhs, op, rhs] = &node.children[..] else {
                panic!("Can't happen");
            };
            // Matches against strings like this are linear searches
            // which could potentially be improved to constant time by
            // storing the logos token in the LALRPOP token and matching
            // against it here.
            let op = match op.text {
                "==" => leo_ast::BinaryOperation::Eq,
                "!=" => leo_ast::BinaryOperation::Neq,
                "<" => leo_ast::BinaryOperation::Lt,
                "<=" => leo_ast::BinaryOperation::Lte,
                ">" => leo_ast::BinaryOperation::Gt,
                ">=" => leo_ast::BinaryOperation::Gte,
                "+" => leo_ast::BinaryOperation::Add,
                "-" => leo_ast::BinaryOperation::Sub,
                "*" => leo_ast::BinaryOperation::Mul,
                "/" => leo_ast::BinaryOperation::Div,
                "%" => leo_ast::BinaryOperation::Rem,
                "||" => leo_ast::BinaryOperation::Or,
                "&&" => leo_ast::BinaryOperation::And,
                "|" => leo_ast::BinaryOperation::BitwiseOr,
                "&" => leo_ast::BinaryOperation::BitwiseAnd,
                "**" => leo_ast::BinaryOperation::Pow,
                "<<" => leo_ast::BinaryOperation::Shl,
                ">>" => leo_ast::BinaryOperation::Shr,
                "^" => leo_ast::BinaryOperation::Xor,
                _ => panic!("Can't happen"),
            };
            let left = to_expression(lhs, builder, handler)?;
            let right = to_expression(rhs, builder, handler)?;

            leo_ast::BinaryExpression { left, right, op, span, id }.into()
        }
        ExpressionKind::Call => {
            let name = &node.children[0];

            let arguments = node
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                .map(|child| to_expression(child, builder, handler))
                .collect::<Result<Vec<_>>>()?;

            let (function, program) = if let Some((first, second)) = name.text.split_once(".aleo/") {
                // This is a locator.
                let symbol = Symbol::intern(second);
                let lo = node.span.lo + first.len() as u32 + ".aleo/".len() as u32;
                let second_span = Span { lo, hi: lo + second.len() as u32 };
                let identifier = leo_ast::Identifier { name: symbol, span: second_span, id: builder.next_id() };
                let function = leo_ast::Path::new(Vec::new(), identifier, false, None, span, builder.next_id());
                (function, Some(Symbol::intern(first)))
            } else {
                // It's a path.
                let mut components = path_to_parts(name, builder);
                let identifier = components.pop().unwrap();
                let function = leo_ast::Path::new(components, identifier, false, None, name.span, builder.next_id());
                (function, None)
            };

            let mut const_arguments = Vec::new();
            if let Some(argument_list) =
                node.children.iter().find(|child| matches!(child.kind, SyntaxKind::ConstArgumentList))
            {
                const_arguments = argument_list
                    .children
                    .iter()
                    .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                    .map(|child| to_expression(child, builder, handler))
                    .collect::<Result<Vec<_>>>()?;
            }

            leo_ast::CallExpression { function, const_arguments, arguments, program, span, id }.into()
        }
        ExpressionKind::Cast => {
            let [expression, _as, type_] = &node.children[..] else {
                panic!("Can't happen");
            };

            let expression = to_expression(expression, builder, handler)?;
            let type_ = to_type(type_, builder, handler)?;

            leo_ast::CastExpression { expression, type_, span, id }.into()
        }
        ExpressionKind::Path => {
            // We need to find the spans of the individual path components, since the
            // lossless tree just has the span of the entire path.
            let mut identifiers = path_to_parts(&node.children[0], builder);
            let identifier = identifiers.pop().unwrap();
            leo_ast::Path::new(identifiers, identifier, false, None, span, id).into()
        }
        ExpressionKind::Literal(literal_kind) => match literal_kind {
            LiteralKind::Address => {
                let t = text();
                if !t.contains(".aleo") && t.parse::<Address<TestnetV0>>().is_err() {
                    // We do this check here rather than in `leo-parser-lossless` simply
                    // to avoid a dependency on snarkvm for `leo-parser-lossless`.
                    handler.emit_err(ParserError::invalid_address_lit(&t, span));
                }
                leo_ast::Literal::address(t, span, id).into()
            }
            LiteralKind::Boolean => match node.text {
                "true" => leo_ast::Literal::boolean(true, span, id).into(),
                "false" => leo_ast::Literal::boolean(false, span, id).into(),
                _ => panic!("Can't happen"),
            },
            LiteralKind::Field => leo_ast::Literal::field(text(), span, id).into(),
            LiteralKind::Group => leo_ast::Literal::group(text(), span, id).into(),
            LiteralKind::Integer(integer_literal_kind) => {
                let integer_type = match integer_literal_kind {
                    IntegerLiteralKind::U8 => leo_ast::IntegerType::U8,
                    IntegerLiteralKind::U16 => leo_ast::IntegerType::U16,
                    IntegerLiteralKind::U32 => leo_ast::IntegerType::U32,
                    IntegerLiteralKind::U64 => leo_ast::IntegerType::U64,
                    IntegerLiteralKind::U128 => leo_ast::IntegerType::U128,
                    IntegerLiteralKind::I8 => leo_ast::IntegerType::I8,
                    IntegerLiteralKind::I16 => leo_ast::IntegerType::I16,
                    IntegerLiteralKind::I32 => leo_ast::IntegerType::I32,
                    IntegerLiteralKind::I64 => leo_ast::IntegerType::I64,
                    IntegerLiteralKind::I128 => leo_ast::IntegerType::I128,
                };
                leo_ast::Literal::integer(integer_type, text(), span, id).into()
            }
            LiteralKind::None => leo_ast::Literal::none(span, id).into(),
            LiteralKind::Scalar => leo_ast::Literal::scalar(text(), span, id).into(),
            LiteralKind::Unsuffixed => leo_ast::Literal::unsuffixed(text(), span, id).into(),
            LiteralKind::String => leo_ast::Literal::string(text(), span, id).into(),
        },
        ExpressionKind::Locator => {
            let text = node.children[0].text;

            // Parse the locator string in format "some_program.aleo/some_name"
            if let Some((program_part, name_part)) = text.split_once(".aleo/") {
                // Create the program identifier
                let program_name_symbol = Symbol::intern(program_part);
                let program_name_span = Span { lo: node.span.lo, hi: node.span.lo + program_part.len() as u32 };
                let program_name =
                    leo_ast::Identifier { name: program_name_symbol, span: program_name_span, id: builder.next_id() };

                // Create the network identifier (always "aleo")
                let network_start = node.span.lo + program_part.len() as u32 + 1; // +1 for the dot
                let network_span = Span {
                    lo: network_start,
                    hi: network_start + 4, // "aleo" is 4 characters
                };
                let network =
                    leo_ast::Identifier { name: Symbol::intern("aleo"), span: network_span, id: builder.next_id() };

                // Create the program ID
                let program_id = leo_ast::ProgramId { name: program_name, network };

                // Create the resource name
                let name_symbol = Symbol::intern(name_part);

                leo_ast::LocatorExpression { program: program_id, name: name_symbol, span, id }.into()
            } else {
                // Invalid locator format - this should have been caught by the parser
                handler.emit_err(ParserError::custom("Invalid locator format", span));
                leo_ast::ErrExpression { span, id }.into()
            }
        }
        ExpressionKind::MemberAccess => {
            let [struct_, _dot, name] = &node.children[..] else {
                panic!("Can't happen.");
            };
            let inner = to_expression(struct_, builder, handler)?;
            let name = to_identifier(name, builder);

            leo_ast::MemberAccess { inner, name, span, id }.into()
        }
        ExpressionKind::MethodCall => {
            let [expr, _dot, name, ..] = &node.children[..] else {
                panic!("Can't happen");
            };

            let name = to_identifier(name, builder);
            let receiver = to_expression(expr, builder, handler)?;

            let mut args = node.children[3..]
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                .map(|child| to_expression(child, builder, handler))
                .collect::<Result<Vec<_>>>()?;

            if let (true, Some(op)) = (args.is_empty(), leo_ast::UnaryOperation::from_symbol(name.name)) {
                // Found an unary operator and the argument list is empty.
                leo_ast::UnaryExpression { span, op, receiver, id }.into()
            } else if let (1, Some(op)) = (args.len(), leo_ast::BinaryOperation::from_symbol(name.name)) {
                // Found a binary operator and the argument list contains a single argument.
                leo_ast::BinaryExpression { span, op, left: receiver, right: args.pop().unwrap(), id }.into()
            } else if let (2, Some(leo_ast::CoreFunction::SignatureVerify)) =
                (args.len(), leo_ast::CoreFunction::from_symbols(sym::signature, name.name))
            {
                leo_ast::AssociatedFunctionExpression {
                    variant: leo_ast::Identifier::new(sym::signature, builder.next_id()),
                    name,
                    arguments: std::iter::once(receiver).chain(args).collect(),
                    span,
                    id,
                }
                .into()
            } else if let (0, Some(leo_ast::CoreFunction::FutureAwait)) =
                (args.len(), leo_ast::CoreFunction::from_symbols(sym::Future, name.name))
            {
                leo_ast::AssociatedFunctionExpression {
                    variant: leo_ast::Identifier::new(sym::Future, builder.next_id()),
                    name,
                    arguments: vec![receiver],
                    span,
                    id: builder.next_id(),
                }
                .into()
            } else if let (0, Some(leo_ast::CoreFunction::OptionalUnwrap)) =
                (args.len(), leo_ast::CoreFunction::from_symbols(sym::Optional, name.name))
            {
                leo_ast::AssociatedFunctionExpression {
                    variant: leo_ast::Identifier::new(sym::Optional, builder.next_id()),
                    name,
                    arguments: vec![receiver],
                    span,
                    id: builder.next_id(),
                }
                .into()
            } else if let (1, Some(leo_ast::CoreFunction::OptionalUnwrapOr)) =
                (args.len(), leo_ast::CoreFunction::from_symbols(sym::Optional, name.name))
            {
                leo_ast::AssociatedFunctionExpression {
                    variant: leo_ast::Identifier::new(sym::Optional, builder.next_id()),
                    name,
                    arguments: std::iter::once(receiver).chain(args).collect(),
                    span,
                    id: builder.next_id(),
                }
                .into()
            } else {
                // Attempt to parse the method call as a mapping operation.
                match (args.len(), leo_ast::CoreFunction::from_symbols(sym::Mapping, name.name)) {
                    (1, Some(leo_ast::CoreFunction::MappingGet))
                    | (2, Some(leo_ast::CoreFunction::MappingGetOrUse))
                    | (2, Some(leo_ast::CoreFunction::MappingSet))
                    | (1, Some(leo_ast::CoreFunction::MappingRemove))
                    | (1, Some(leo_ast::CoreFunction::MappingContains)) => {
                        // Found an instance of `<mapping>.get`, `<mapping>.get_or_use`, `<mapping>.set`, `<mapping>.remove`, or `<mapping>.contains`.
                        leo_ast::AssociatedFunctionExpression {
                            variant: leo_ast::Identifier::new(sym::Mapping, builder.next_id()),
                            name,
                            arguments: std::iter::once(receiver).chain(args).collect(),
                            span,
                            id: builder.next_id(),
                        }
                        .into()
                    }
                    _ => {
                        // Either an invalid unary/binary operator, or more arguments given.
                        handler.emit_err(ParserError::invalid_method_call(receiver, name, args.len(), span));
                        leo_ast::ErrExpression { span, id: builder.next_id() }.into()
                    }
                }
            }
        }
        ExpressionKind::Parenthesized => {
            let [_left, expr, _right] = &node.children[..] else {
                panic!("Can't happen");
            };
            to_expression(expr, builder, handler)?
        }
        ExpressionKind::Repeat => {
            let [_left, expr, _s, count, _right] = &node.children[..] else {
                panic!("Can't happen");
            };
            let expr = to_expression(expr, builder, handler)?;
            let count = to_expression(count, builder, handler)?;
            leo_ast::RepeatExpression { expr, count, span, id }.into()
        }
        ExpressionKind::SpecialAccess => {
            let [qualifier, _dot, name] = &node.children[..] else {
                panic!("Can't happen");
            };

            let inner = to_identifier(qualifier, builder).into();
            let name = to_identifier(name, builder);

            leo_ast::MemberAccess { inner, name, span, id }.into()
        }
        ExpressionKind::Struct => {
            let name = &node.children[0];
            let mut members = Vec::new();
            for initializer in node.children.iter().filter(|node| node.kind == SyntaxKind::StructMemberInitializer) {
                let (init_name, expression) = match &initializer.children[..] {
                    [init_name] => (init_name, None),
                    [init_name, _c, expr] => (init_name, Some(to_expression(expr, builder, handler)?)),
                    _ => panic!("Can't happen"),
                };
                let init_name = to_identifier(init_name, builder);

                members.push(leo_ast::StructVariableInitializer {
                    identifier: init_name,
                    expression,
                    span: initializer.span,
                    id: builder.next_id(),
                });
            }

            let mut const_arguments = Vec::new();
            let maybe_const_params = &node.children[1];
            if maybe_const_params.kind == SyntaxKind::ConstArgumentList {
                for argument in &maybe_const_params.children {
                    if matches!(argument.kind, SyntaxKind::Expression(..)) {
                        let expr = to_expression(argument, builder, handler)?;
                        const_arguments.push(expr);
                    }
                }
            }

            let mut identifiers = path_to_parts(name, builder);
            let identifier = identifiers.pop().unwrap();
            let path = leo_ast::Path::new(identifiers, identifier, false, None, name.span, builder.next_id());

            leo_ast::StructExpression { path, const_arguments, members, span, id }.into()
        }
        ExpressionKind::Ternary => {
            let [cond, _q, if_, _c, then] = &node.children[..] else {
                panic!("Can't happen");
            };
            let condition = to_expression(cond, builder, handler)?;
            let if_true = to_expression(if_, builder, handler)?;
            let if_false = to_expression(then, builder, handler)?;
            leo_ast::TernaryExpression { condition, if_true, if_false, span, id }.into()
        }
        ExpressionKind::Tuple => {
            let elements = node
                .children
                .iter()
                .filter(|expr| matches!(expr.kind, SyntaxKind::Expression(..)))
                .map(|expr| to_expression(expr, builder, handler))
                .collect::<Result<Vec<_>>>()?;
            leo_ast::TupleExpression { elements, span, id }.into()
        }
        ExpressionKind::TupleAccess => {
            let [expr, _dot, integer] = &node.children[..] else {
                panic!("Can't happen");
            };

            let tuple = to_expression(expr, builder, handler)?;
            let integer_text = integer.text.replace("_", "");
            let value: usize = integer_text.parse().expect("Integer should parse.");
            let index = value.into();

            leo_ast::TupleAccess { tuple, index, span, id }.into()
        }
        ExpressionKind::Unary => {
            let [op, operand] = &node.children[..] else {
                panic!("Can't happen");
            };
            let mut operand_expression = to_expression(operand, builder, handler)?;
            let op_variant = match op.text {
                "!" => leo_ast::UnaryOperation::Not,
                "-" => leo_ast::UnaryOperation::Negate,
                _ => panic!("Can't happen"),
            };
            if op_variant == leo_ast::UnaryOperation::Negate {
                use leo_ast::LiteralVariant::*;
                if let Expression::Literal(leo_ast::Literal {
                    variant: Integer(_, string) | Field(string) | Group(string) | Scalar(string),
                    span,
                    ..
                }) = &mut operand_expression
                {
                    if !string.starts_with('-') {
                        // The operation was a negation and the literal was not already negative, so fold it in
                        // and discard the unary expression.
                        string.insert(0, '-');
                        *span = op.span + operand.span;
                        return Ok(operand_expression);
                    }
                }
            }
            leo_ast::UnaryExpression { receiver: operand_expression, op: op_variant, span, id }.into()
        }
        ExpressionKind::Unit => leo_ast::UnitExpression { span, id }.into(),
    };

    Ok(value)
}

fn to_const_parameters(
    node: &SyntaxNode<'_>,
    builder: &NodeBuilder,
    handler: &Handler,
) -> Result<Vec<leo_ast::ConstParameter>> {
    assert_eq!(node.kind, SyntaxKind::ConstParameterList);

    node.children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::ConstParameter))
        .map(|child| {
            let [id, _c, type_] = &child.children[..] else {
                panic!("Can't happen");
            };

            Ok(leo_ast::ConstParameter {
                identifier: to_identifier(id, builder),
                type_: to_type(type_, builder, handler)?,
                span: child.span,
                id: builder.next_id(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn to_annotation(node: &SyntaxNode<'_>, builder: &NodeBuilder) -> Result<leo_ast::Annotation> {
    assert_eq!(node.kind, SyntaxKind::Annotation);
    let name = to_identifier(&node.children[1], builder);

    let mut map = IndexMap::new();
    node.children.get(2).inspect(|list| {
        for member in list.children.iter() {
            if member.kind != SyntaxKind::AnnotationMember {
                continue;
            }

            let [key, _assign, value] = &member.children[..] else {
                panic!("Can't happen");
            };
            let key = Symbol::intern(key.text);
            // Get rid of the delimiting double quotes on the string.
            let value = value.text[1..value.text.len() - 1].to_string();
            map.insert(key, value);
        }
    });
    Ok(leo_ast::Annotation { identifier: name, map, span: node.span, id: builder.next_id() })
}

fn to_function(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Function> {
    assert_eq!(node.kind, SyntaxKind::Function);

    let annotations = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Annotation))
        .map(|child| to_annotation(child, builder))
        .collect::<Result<Vec<_>>>()?;
    let async_index = annotations.len();
    let is_async = node.children[async_index].text == "async";

    let function_variant_index = if is_async { async_index + 1 } else { async_index };

    // The behavior here matches the old parser - "inline" and "script" may be marked async,
    // but async is ignored. Presumably we should fix this but it's theoretically a breaking change.
    let variant = match (is_async, node.children[function_variant_index].text) {
        (true, "function") => leo_ast::Variant::AsyncFunction,
        (false, "function") => leo_ast::Variant::Function,
        (_, "inline") => leo_ast::Variant::Inline,
        (_, "script") => leo_ast::Variant::Script,
        (true, "transition") => leo_ast::Variant::AsyncTransition,
        (false, "transition") => leo_ast::Variant::Transition,
        _ => panic!("Can't happen"),
    };

    let name = &node.children[function_variant_index + 1];
    let id = to_identifier(name, builder);

    let mut const_parameters = Vec::new();
    if let Some(const_param_list) =
        node.children.iter().find(|child| matches!(child.kind, SyntaxKind::ConstParameterList))
    {
        const_parameters = to_const_parameters(const_param_list, builder, handler)?;
    }

    let parameter_list = node.children.iter().find(|child| matches!(child.kind, SyntaxKind::ParameterList)).unwrap();
    let input = parameter_list
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Parameter))
        .map(|child| {
            let mode = to_mode(&child.children[0]);
            let index = if mode == leo_ast::Mode::None { 0 } else { 1 };
            let [name, _c, type_] = &child.children[index..] else {
                panic!("Can't happen");
            };
            Ok(leo_ast::Input {
                identifier: to_identifier(name, builder),
                mode,
                type_: to_type(type_, builder, handler)?,
                span: child.span,
                id: builder.next_id(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let [.., maybe_outputs, block] = &node.children[..] else {
        panic!("Can't happen");
    };
    let block = to_block(block, builder, handler)?;

    let to_output = |node: &SyntaxNode<'_>| -> Result<leo_ast::Output> {
        let mode = to_mode(&node.children[0]);
        let type_ = node.children.last().unwrap();

        Ok(leo_ast::Output { mode, type_: to_type(type_, builder, handler)?, span: node.span, id: builder.next_id() })
    };

    let output = match maybe_outputs.kind {
        SyntaxKind::FunctionOutput => {
            let output = to_output(maybe_outputs)?;
            vec![output]
        }
        SyntaxKind::FunctionOutputs => maybe_outputs
            .children
            .iter()
            .filter(|child| matches!(child.kind, SyntaxKind::FunctionOutput))
            .map(|child| to_output(child))
            .collect::<Result<Vec<_>>>()?,
        _ => Vec::new(),
    };

    Ok(leo_ast::Function::new(
        annotations,
        variant,
        id,
        const_parameters,
        input,
        output,
        block,
        node.span,
        builder.next_id(),
    ))
}

fn to_composite(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Composite> {
    assert_eq!(node.kind, SyntaxKind::StructDeclaration);

    let [struct_or_record, i, .., members] = &node.children[..] else {
        panic!("Can't happen");
    };

    let members = members
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::StructMemberDeclaration))
        .map(|child| {
            let (mode, ident, type_) = match &child.children[..] {
                [ident, _c, type_] => (leo_ast::Mode::None, ident, type_),
                [privacy, ident, _c, type_] => (to_mode(privacy), ident, type_),
                _ => panic!("Can't happen"),
            };

            Ok(leo_ast::Member {
                mode,
                identifier: to_identifier(ident, builder),
                type_: to_type(type_, builder, handler)?,
                span: child.span,
                id: builder.next_id(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut const_parameters = Vec::new();
    if let Some(const_param_list) =
        node.children.iter().find(|child| matches!(child.kind, SyntaxKind::ConstParameterList))
    {
        const_parameters = to_const_parameters(const_param_list, builder, handler)?;
    }

    Ok(leo_ast::Composite {
        identifier: to_identifier(i, builder),
        const_parameters,
        members,
        external: None,
        is_record: struct_or_record.text == "record",
        span: node.span,
        id: builder.next_id(),
    })
}

fn to_global_const(
    node: &SyntaxNode<'_>,
    builder: &NodeBuilder,
    handler: &Handler,
) -> Result<leo_ast::ConstDeclaration> {
    assert_eq!(node.kind, SyntaxKind::GlobalConst);

    let [_l, ident, _colon, type_, _a, expr, _s] = &node.children[..] else {
        panic!("Can't happen");
    };

    Ok(leo_ast::ConstDeclaration {
        place: to_identifier(ident, builder),
        type_: to_type(type_, builder, handler)?,
        value: to_expression(expr, builder, handler)?,
        span: node.span,
        id: builder.next_id(),
    })
}

fn to_constructor(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Constructor> {
    assert_eq!(node.kind, SyntaxKind::Constructor);
    let annotations = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Annotation))
        .map(|child| to_annotation(child, builder))
        .collect::<Result<Vec<_>>>()?;
    let block = to_block(node.children.last().unwrap(), builder, handler)?;

    Ok(leo_ast::Constructor { annotations, block, span: node.span, id: builder.next_id() })
}

fn to_mapping(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Mapping> {
    assert_eq!(node.kind, SyntaxKind::Mapping);

    let [_mapping, name, _colon, key_type, _arrow, value_type, _s] = &node.children[..] else {
        panic!("Can't happen");
    };

    Ok(leo_ast::Mapping {
        identifier: to_identifier(name, builder),
        key_type: to_type(key_type, builder, handler)?,
        value_type: to_type(value_type, builder, handler)?,
        span: node.span,
        id: builder.next_id(),
    })
}

pub fn to_module(
    node: &SyntaxNode<'_>,
    builder: &NodeBuilder,
    program_name: Symbol,
    path: Vec<Symbol>,
    handler: &Handler,
) -> Result<leo_ast::Module> {
    assert_eq!(node.kind, SyntaxKind::ModuleContents);

    let mut functions = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Function))
        .map(|child| {
            let function = to_function(child, builder, handler)?;
            Ok((function.identifier.name, function))
        })
        .collect::<Result<Vec<_>>>()?;
    // Passes like type checking expect transitions to come first.
    // Irrelevant for modules at the moment.
    functions.sort_by_key(|func| if func.1.variant.is_transition() { 0u8 } else { 1u8 });

    let structs = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::StructDeclaration))
        .map(|child| {
            let composite = to_composite(child, builder, handler)?;
            Ok((composite.identifier.name, composite))
        })
        .collect::<Result<Vec<_>>>()?;

    let consts = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::GlobalConst))
        .map(|child| {
            let global_const = to_global_const(child, builder, handler)?;
            Ok((global_const.place.name, global_const))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(leo_ast::Module { program_name, path, consts, structs, functions })
}

pub fn to_main(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Program> {
    assert_eq!(node.kind, SyntaxKind::MainContents);

    let imports = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Import))
        .map(|child| {
            let name = Symbol::intern(child.children[1].text.strip_suffix(".aleo").unwrap());
            (name, (leo_ast::Program::default(), child.span))
        })
        .collect::<IndexMap<_, _>>();

    let program_node = node.children.last().unwrap();

    let mut functions = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Function))
        .map(|child| {
            let function = to_function(child, builder, handler)?;
            Ok((function.identifier.name, function))
        })
        .collect::<Result<Vec<_>>>()?;
    // Passes like type checking expect transitions to come first.
    functions.sort_by_key(|func| if func.1.variant.is_transition() { 0u8 } else { 1u8 });

    let structs = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::StructDeclaration))
        .map(|child| {
            let composite = to_composite(child, builder, handler)?;
            Ok((composite.identifier.name, composite))
        })
        .collect::<Result<Vec<_>>>()?;

    let consts = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::GlobalConst))
        .map(|child| {
            let global_const = to_global_const(child, builder, handler)?;
            Ok((global_const.place.name, global_const))
        })
        .collect::<Result<Vec<_>>>()?;

    let mappings = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Mapping))
        .map(|child| {
            let mapping = to_mapping(child, builder, handler)?;
            Ok((mapping.identifier.name, mapping))
        })
        .collect::<Result<Vec<_>>>()?;

    // This follows the behavior of the old parser - if multiple constructors are
    // present, we silently throw out all but the last. Probably this should be
    // changed but it would theoretically be a breaking change.
    let mut constructors = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Constructor))
        .map(|child| to_constructor(child, builder, handler))
        .collect::<Result<Vec<_>>>()?;

    if let Some(extra) = constructors.get(1) {
        return Err(TypeCheckerError::custom("A program can only have one constructor.", extra.span).into());
    }

    let program_id_node = &program_node.children[1];
    let program_name_text = program_id_node.text.strip_suffix(".aleo").unwrap();
    let program_name_symbol = Symbol::intern(program_name_text);
    let hi = program_id_node.span.lo + program_name_text.len() as u32;
    let program_id = leo_ast::ProgramId {
        name: leo_ast::Identifier {
            name: program_name_symbol,
            span: Span { lo: program_id_node.span.lo, hi },
            id: builder.next_id(),
        },
        network: leo_ast::Identifier { name: sym::aleo, span: Span { lo: hi + 1, hi: hi + 5 }, id: builder.next_id() },
    };
    let program_scope = leo_ast::ProgramScope {
        program_id,
        consts,
        structs,
        mappings,
        functions,
        constructor: constructors.pop(),
        span: node.span,
    };
    Ok(leo_ast::Program {
        modules: Default::default(),
        imports,
        stubs: Default::default(),
        program_scopes: std::iter::once((program_name_symbol, program_scope)).collect(),
    })
}

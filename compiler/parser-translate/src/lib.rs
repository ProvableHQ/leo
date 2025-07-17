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
use itertools::Itertools as _;

use leo_ast::{Expression, NetworkName, NodeBuilder};
use leo_errors::{Handler, ParserError, Result};
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
use leo_span::{
    Span,
    Symbol,
    source_map::{FileName, SourceFile},
    sym,
};

#[cfg(test)]
mod test;

fn to_identifier(node: &SyntaxNode<'_>, builder: &NodeBuilder) -> leo_ast::Identifier {
    let name = Symbol::intern(node.text);
    leo_ast::Identifier { name, span: node.span, id: builder.next_id() }
}

fn path_to_parts(node: &SyntaxNode<'_>, builder: &NodeBuilder) -> Vec<leo_ast::Identifier> {
    let mut identifiers = Vec::new();
    let mut i = node.span.lo;
    for text in node.text.split("::") {
        let len = text.len() as u32;
        let end = i + len;
        let this_span = leo_span::Span { lo: i, hi: end };
        // Account for the "::".
        i += len + 2;
        let name = Symbol::intern(text);
        identifiers.push(leo_ast::Identifier { name, span: this_span, id: builder.next_id() })
    }
    identifiers
}

fn to_mode(node: &SyntaxNode<'_>) -> leo_ast::Mode {
    match node.text {
        "constant" => leo_ast::Mode::Constant,
        "private" => leo_ast::Mode::Public,
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
                panic!("Can't happen");
            };
            let element_type = to_type(type_, builder, handler)?;
            let length = to_expression(length, builder, handler)?;
            leo_ast::ArrayType { element_type: Box::new(element_type), length: Box::new(length) }.into()
        }
        TypeKind::Boolean => leo_ast::Type::Boolean,
        TypeKind::Composite => {
            let name = &node.children[0];
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
            let path = leo_ast::Path::new(path_components, identifier, None, name.span, builder.next_id());
            leo_ast::CompositeType { path, const_arguments, program: None }.into()
        }
        TypeKind::Field => leo_ast::Type::Field,
        TypeKind::Future => leo_ast::Type::Future(Default::default()),
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

fn to_statement(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Statement> {
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
            let [_const, name, _c, type_, rhs, _s] = &node.children[..] else {
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

fn to_expression(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<Expression> {
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
                let function = leo_ast::Path::new(Vec::new(), identifier, None, span, builder.next_id());
                (function, Some(Symbol::intern(first)))
            } else {
                // It's a path.
                let mut components = path_to_parts(name, builder);
                let identifier = components.pop().unwrap();
                let function = leo_ast::Path::new(components, identifier, None, name.span, builder.next_id());
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
            leo_ast::Path::new(identifiers, identifier, None, span, id).into()
        }
        ExpressionKind::Literal(literal_kind) => match literal_kind {
            LiteralKind::Address => leo_ast::Literal::address(text(), span, id).into(),
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
            LiteralKind::Scalar => leo_ast::Literal::scalar(text(), span, id).into(),
            LiteralKind::Unsuffixed => leo_ast::Literal::unsuffixed(text(), span, id).into(),
            LiteralKind::String => todo!(),
        },
        ExpressionKind::Locator => todo!(),
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
            let path = leo_ast::Path::new(identifiers, identifier, None, name.span, builder.next_id());

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
            let value: usize = integer.text.parse().expect("Integer should parse.");
            let index = value.into();

            leo_ast::TupleAccess { tuple, index, span, id }.into()
        }
        ExpressionKind::Unary => {
            let [op, operand] = &node.children[..] else {
                panic!("Can't happen");
            };
            let operand_expression = to_expression(operand, builder, handler)?;
            let op_variant = match op.text {
                "!" => leo_ast::UnaryOperation::Not,
                "-" => leo_ast::UnaryOperation::Negate,
                _ => panic!("Can't happen"),
            };
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
        for (key, _assign, value) in list.children.iter()
            // Skip the open paren.
            .skip(1)
            // Group them 3 at a time, for each member.
            .tuples()
        {
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
        let mode = match node.children[0].text {
            "private" => leo_ast::Mode::Private,
            "public" => leo_ast::Mode::Public,
            _ => leo_ast::Mode::None,
        };

        let type_ = node.children.last().unwrap();

        Ok(leo_ast::Output { mode, type_: to_type(type_, builder, handler)?, span: node.span, id: builder.next_id() })
    };
    let (output, output_type) = match maybe_outputs.kind {
        SyntaxKind::FunctionOutput => {
            let output = to_output(maybe_outputs)?;
            let output_type = output.type_.clone();
            (vec![output], output_type)
        }
        SyntaxKind::FunctionOutputs => {
            let output = maybe_outputs
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::FunctionOutput))
                .map(|child| to_output(child))
                .collect::<Result<Vec<_>>>()?;
            let output_type =
                leo_ast::TupleType { elements: output.iter().map(|output| output.type_.clone()).collect() }.into();
            (output, output_type)
        }
        _ => panic!("Can't happen"),
    };

    Ok(leo_ast::Function {
        annotations: Vec::new(),
        variant,
        identifier: id,
        const_parameters,
        input,
        output,
        output_type,
        block,
        span: node.span,
        id: builder.next_id(),
    })
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

    let [_l, ident, _colon, type_, expr, _s] = &node.children[..] else {
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

fn to_module(
    node: &SyntaxNode<'_>,
    builder: &NodeBuilder,
    program_name: Symbol,
    path: Vec<Symbol>,
    handler: &Handler,
) -> Result<leo_ast::Module> {
    assert_eq!(node.kind, SyntaxKind::ModuleContents);

    let functions = node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Function))
        .map(|child| {
            let function = to_function(child, builder, handler)?;
            Ok((function.identifier.name, function))
        })
        .collect::<Result<Vec<_>>>()?;

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

fn to_main(node: &SyntaxNode<'_>, builder: &NodeBuilder, handler: &Handler) -> Result<leo_ast::Program> {
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

    let functions = program_node
        .children
        .iter()
        .filter(|child| matches!(child.kind, SyntaxKind::Function))
        .map(|child| {
            let function = to_function(child, builder, handler)?;
            Ok((function.identifier.name, function))
        })
        .collect::<Result<Vec<_>>>()?;

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

pub fn parse_expression(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Expression> {
    let node = leo_parser_lossless::parse_expression(handler.clone(), source, start_pos)?;
    to_expression(&node, node_builder, &handler)
}

pub fn parse_statement(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Statement> {
    let node = leo_parser_lossless::parse_statement(handler.clone(), source, start_pos)?;
    to_statement(&node, node_builder, &handler)
}

pub fn parse_module(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    program_name: Symbol,
    path: Vec<Symbol>,
    _network: NetworkName,
) -> Result<leo_ast::Module> {
    let node_module = leo_parser_lossless::parse_module(handler.clone(), source, start_pos)?;
    to_module(&node_module, node_builder, program_name, path, &handler)
}

pub fn parse(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    _network: NetworkName,
) -> Result<leo_ast::Program> {
    let program_node = leo_parser_lossless::parse_main(handler.clone(), &source.src, source.absolute_start)?;
    let mut program = to_main(&program_node, node_builder, &handler)?;
    let program_name = *program.program_scopes.first().unwrap().0;

    // Determine the root directory of the main file (for module resolution)
    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    for module in modules {
        let node_module = leo_parser_lossless::parse_module(handler.clone(), &module.src, module.absolute_start)?;
        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            // Ensure no module uses a keyword in its name
            for segment in &key {
                if symbol_is_keyword(*segment) {
                    return Err(ParserError::keyword_used_as_module_name(key.iter().format("::"), segment).into());
                }
            }

            let module_ast = to_module(&node_module, node_builder, program_name, key.clone(), &handler)?;
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

fn symbol_is_keyword(symbol: Symbol) -> bool {
    matches!(
        symbol,
        sym::address |
        sym::aleo |
        sym::As |
        sym::assert |
        sym::assert_eq |
        sym::assert_neq |
        sym::Async |   // if you need it
        sym::block |
        sym::bool |
        sym::Const |
        sym::constant |
        sym::constructor |
        sym::Else |
        sym::False |
        sym::field |
        sym::Fn |
        sym::For |
        sym::function |
        sym::Future |
        sym::group |
        sym::i8 |
        sym::i16 |
        sym::i32 |
        sym::i64 |
        sym::i128 |
        sym::If |
        sym::import |
        sym::In |
        sym::inline |
        sym::Let |
        sym::leo |
        sym::mapping |
        sym::network |
        sym::private |
        sym::program |
        sym::public |
        sym::record |
        sym::Return |
        sym::scalar |
        sym::script |
        sym::SelfLower |
        sym::signature |
        sym::string |
        sym::Struct |
        sym::transition |
        sym::True |
        sym::u8 |
        sym::u16 |
        sym::u32 |
        sym::u64 |
        sym::u128
    )
}

/// Computes a module key from a `FileName`, optionally relative to a root directory.
///
/// This function converts a file path like `src/foo/bar.leo` into a `Vec<Symbol>` key
/// like `["foo", "bar"]`, suitable for inserting into the program's module map.
///
/// # Arguments
/// * `name` - The filename of the module, either real (from disk) or synthetic (custom).
/// * `root_dir` - The root directory to strip from the path, if any.
///
/// # Returns
/// * `Some(Vec<Symbol>)` - The computed module key.
/// * `None` - If the path can't be stripped or processed.
fn compute_module_key(name: &FileName, root_dir: Option<&std::path::Path>) -> Option<Vec<Symbol>> {
    // Normalize the path depending on whether it's a custom or real file
    let path = match name {
        FileName::Custom(name) => std::path::Path::new(name).to_path_buf(),
        FileName::Real(path) => {
            let root = root_dir?;
            path.strip_prefix(root).ok()?.to_path_buf()
        }
    };

    // Convert path components (e.g., "foo/bar") into symbols: ["foo", "bar"]
    let mut key: Vec<Symbol> =
        path.components().map(|comp| Symbol::intern(&comp.as_os_str().to_string_lossy())).collect();

    // Strip the file extension from the last component (e.g., "bar.leo" → "bar")
    if let Some(last) = path.file_name() {
        if let Some(stem) = std::path::Path::new(last).file_stem() {
            key.pop(); // Remove "bar.leo"
            key.push(Symbol::intern(&stem.to_string_lossy())); // Add "bar"
        }
    }

    Some(key)
}

// #[test]
// fn test_expression() {
//     create_session_if_not_set_then(|_| {
//         let node_builder = NodeBuilder::new(0);
//         const TEXT: &str = "abc";
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let x = leo_parser_lossless::grammar::ExprParser::new().parse(&mut lexer).unwrap();
//         let expr = to_expression(&x, &node_builder);
//         println!("{x:?}");
//         println!("{expr:#?}");
//     });
// }

// #[test]
// fn test_function() {
//     create_session_if_not_set_then(|_| {
//         let node_builder = NodeBuilder::new(0);
//         const TEXT: &str = "@something @abc(def = \"ghi\")
//         async function its_name::[X: u32, Y: field](a: field, b: field) -> field {
//             let x: u32 = 5;
//             return a + b + Y;
//         }
//         ";
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let x = leo_parser_lossless::grammar::FunctionDeclarationParser::new().parse(&mut lexer).unwrap();
//         let func = to_function(&x, &node_builder);
//         println!("{x:#?}");
//         println!("{func:#?}");
//     });
// }

// #[test]
// fn test_struct() {
//     create_session_if_not_set_then(|_| {
//         let node_builder = NodeBuilder::new(0);
//         const TEXT: &str = "struct X {
//             x: field,
//             y: bool,
//             z: [Abc; 2],
//         }
//         ";
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let x = leo_parser_lossless::grammar::StructDeclarationParser::new().parse(&mut lexer).unwrap();
//         let comp = to_composite(&x, &node_builder);
//         println!("{x:#?}");
//         println!("{comp:#?}");
//     });
// }

// #[test]
// fn test_constructor() {
//     create_session_if_not_set_then(|_| {
//         let node_builder = NodeBuilder::new(0);
//         const TEXT: &str = "@some_annotation
//         async constructor() {
//             assert_eq(1u32, 2u32);
//         }
//         ";
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let x = leo_parser_lossless::grammar::ConstructorDeclarationParser::new().parse(&mut lexer).unwrap();
//         let constructor = to_constructor(&x, &node_builder);
//         println!("{x:#?}");
//         println!("{constructor:#?}");
//     });
// }

// #[test]
// fn test_mapping() {
//     create_session_if_not_set_then(|_| {
//         let node_builder = NodeBuilder::new(0);
//         const TEXT: &str = "mapping abcdef: [some_type; 2] => what;";
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let x = leo_parser_lossless::grammar::MappingDeclarationParser::new().parse(&mut lexer).unwrap();
//         let constructor = to_mapping(&x, &node_builder);
//         println!("{x:#?}");
//         println!("{constructor:#?}");
//     });
// }

// #[test]
// fn test_something() {
//     const TEXT: &str = "

// program test.aleo {
//     async transition main() -> bool {

//     }

//     async function finalize_main() {

//     }

//     function main() -> bool {

//     }
//     async function main(a: foo, b: bar) -> baz {

//     }
// }
// ";
//     create_session_if_not_set_then(|_| {
//         let mut lexer = leo_parser_lossless::tokens::Lexer::new(TEXT, 0);
//         let _x = leo_parser_lossless::grammar::MainContentsParser::new().parse(&mut lexer).unwrap();
//         // println!("{x:#?}");
//     })
// }

// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use super::*;
use leo_errors::{ParserError, Result};

use leo_span::{sym, Symbol};
use snarkvm::console::{account::Address, network::Testnet3};

const INT_TYPES: &[Token] = &[
    Token::I8,
    Token::I16,
    Token::I32,
    Token::I64,
    Token::I128,
    Token::U8,
    Token::U16,
    Token::U32,
    Token::U64,
    Token::U128,
    Token::Field,
    Token::Group,
    Token::Scalar,
];

impl ParserContext<'_> {
    /// Returns an [`Expression`] AST node if the next token is an expression.
    /// Includes struct init expressions.
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        // Store current parser state.
        let prior_fuzzy_state = self.disallow_struct_construction;

        // Allow struct init expressions.
        self.disallow_struct_construction = false;

        // Parse expression.
        let result = self.parse_conditional_expression();

        // Restore prior parser state.
        self.disallow_struct_construction = prior_fuzzy_state;

        result
    }

    /// Returns an [`Expression`] AST node if the next tokens represent
    /// a ternary expression. May or may not include struct init expressions.
    ///
    /// Otherwise, tries to parse the next token using [`parse_boolean_or_expression`].
    pub(super) fn parse_conditional_expression(&mut self) -> Result<Expression> {
        // Try to parse the next expression. Try BinaryOperation::Or.
        let mut expr = self.parse_boolean_or_expression()?;

        // Parse the rest of the ternary expression.
        if self.eat(&Token::Question) {
            let if_true = self.parse_expression()?;
            self.expect(&Token::Colon)?;
            let if_false = self.parse_expression()?;
            expr = Expression::Ternary(TernaryExpression {
                span: expr.span() + if_false.span(),
                condition: Box::new(expr),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
                id: self.node_builder.next_id(),
            });
        }
        Ok(expr)
    }

    /// Constructs a binary expression `left op right`.
    fn bin_expr(node_builder: &NodeBuilder, left: Expression, right: Expression, op: BinaryOperation) -> Expression {
        Expression::Binary(BinaryExpression {
            span: left.span() + right.span(),
            op,
            left: Box::new(left),
            right: Box::new(right),
            id: node_builder.next_id(),
        })
    }

    /// Parses a left-associative binary expression `<left> token <right>` using `f` for left/right.
    /// The `token` is translated to `op` in the AST.
    fn parse_bin_expr(
        &mut self,
        tokens: &[Token],
        mut f: impl FnMut(&mut Self) -> Result<Expression>,
    ) -> Result<Expression> {
        let mut expr = f(self)?;
        while let Some(op) = self.eat_bin_op(tokens) {
            expr = Self::bin_expr(self.node_builder, expr, f(self)?, op);
        }
        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent
    /// a binary OR expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_boolean_and_expression`].
    fn parse_boolean_or_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::Or], Self::parse_boolean_and_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary AND expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_equality_expression`].
    fn parse_boolean_and_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::And], Self::parse_equality_expression)
    }

    /// Eats one of binary operators matching any in `tokens`.
    fn eat_bin_op(&mut self, tokens: &[Token]) -> Option<BinaryOperation> {
        self.eat_any(tokens).then(|| match &self.prev_token.token {
            Token::Eq => BinaryOperation::Eq,
            Token::NotEq => BinaryOperation::Neq,
            Token::Lt => BinaryOperation::Lt,
            Token::LtEq => BinaryOperation::Lte,
            Token::Gt => BinaryOperation::Gt,
            Token::GtEq => BinaryOperation::Gte,
            Token::Add => BinaryOperation::Add,
            Token::Sub => BinaryOperation::Sub,
            Token::Mul => BinaryOperation::Mul,
            Token::Div => BinaryOperation::Div,
            Token::Rem => BinaryOperation::Rem,
            Token::Or => BinaryOperation::Or,
            Token::And => BinaryOperation::And,
            Token::BitOr => BinaryOperation::BitwiseOr,
            Token::BitAnd => BinaryOperation::BitwiseAnd,
            Token::Pow => BinaryOperation::Pow,
            Token::Shl => BinaryOperation::Shl,
            Token::Shr => BinaryOperation::Shr,
            Token::BitXor => BinaryOperation::Xor,
            _ => unreachable!("`eat_bin_op` shouldn't produce this"),
        })
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary relational expression: less than, less than or equals, greater than, greater than or equals.
    ///
    /// Otherwise, tries to parse the next token using [`parse_additive_expression`].
    fn parse_ordering_expression(&mut self) -> Result<Expression> {
        let mut expr = self.parse_bitwise_exclusive_or_expression()?;
        if let Some(op) = self.eat_bin_op(&[Token::Lt, Token::LtEq, Token::Gt, Token::GtEq]) {
            let right = self.parse_bitwise_exclusive_or_expression()?;
            expr = Self::bin_expr(self.node_builder, expr, right, op);
        }
        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary equals or not equals expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_ordering_expression`].
    fn parse_equality_expression(&mut self) -> Result<Expression> {
        let mut expr = self.parse_ordering_expression()?;
        if let Some(op) = self.eat_bin_op(&[Token::Eq, Token::NotEq]) {
            let right = self.parse_ordering_expression()?;
            expr = Self::bin_expr(self.node_builder, expr, right, op);
        }
        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// bitwise exclusive or expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_bitwise_inclusive_or_expression`].
    fn parse_bitwise_exclusive_or_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::BitXor], Self::parse_bitwise_inclusive_or_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// bitwise inclusive or expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_bitwise_and_expression`].
    fn parse_bitwise_inclusive_or_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::BitOr], Self::parse_bitwise_and_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// bitwise and expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_shift_expression`].
    fn parse_bitwise_and_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::BitAnd], Self::parse_shift_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// shift left or a shift right expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_additive_expression`].
    fn parse_shift_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::Shl, Token::Shr], Self::parse_additive_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary addition or subtraction expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_mul_div_pow_expression`].
    fn parse_additive_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::Add, Token::Sub], Self::parse_multiplicative_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary multiplication, division, or a remainder expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_exponential_expression`].
    fn parse_multiplicative_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::Mul, Token::Div, Token::Rem], Self::parse_exponential_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary exponentiation expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_cast_expression`].
    fn parse_exponential_expression(&mut self) -> Result<Expression> {
        self.parse_bin_expr(&[Token::Pow], Self::parse_cast_expression)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// cast expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_unary_expression`].
    fn parse_cast_expression(&mut self) -> Result<Expression> {
        let mut expr = self.parse_unary_expression()?;
        if self.eat(&Token::As) {
            let (type_, end_span) = self.parse_primitive_type()?;
            let span = expr.span() + end_span;
            expr = Expression::Cast(CastExpression {
                expression: Box::new(expr),
                type_,
                span,
                id: self.node_builder.next_id(),
            });
        }

        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// unary not, negate, or bitwise not expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_postfix_expression`].
    pub(super) fn parse_unary_expression(&mut self) -> Result<Expression> {
        let mut ops = Vec::new();
        while self.eat_any(&[Token::Not, Token::Sub]) {
            let operation = match self.prev_token.token {
                Token::Not => UnaryOperation::Not,
                Token::Sub => UnaryOperation::Negate,
                _ => unreachable!("parse_unary_expression_ shouldn't produce this"),
            };
            ops.push((operation, self.prev_token.span));
        }

        let mut inner = self.parse_postfix_expression()?;

        // If the last operation is a negation and the inner expression is a literal, then construct a negative literal.
        if let Some((UnaryOperation::Negate, _)) = ops.last() {
            match inner {
                Expression::Literal(Literal::Integer(integer_type, string, span, id)) => {
                    // Remove the negation from the operations.
                    // Note that this unwrap is safe because there is at least one operation in `ops`.
                    let (_, op_span) = ops.pop().unwrap();
                    // Construct a negative integer literal.
                    inner =
                        Expression::Literal(Literal::Integer(integer_type, format!("-{string}"), op_span + span, id));
                }
                Expression::Literal(Literal::Field(string, span, id)) => {
                    // Remove the negation from the operations.
                    // Note that
                    let (_, op_span) = ops.pop().unwrap();
                    // Construct a negative field literal.
                    inner = Expression::Literal(Literal::Field(format!("-{string}"), op_span + span, id));
                }
                Expression::Literal(Literal::Group(group_literal)) => {
                    // Remove the negation from the operations.
                    let (_, op_span) = ops.pop().unwrap();
                    // Construct a negative group literal.
                    // Note that we only handle the case where the group literal is a single integral value.
                    inner = Expression::Literal(Literal::Group(Box::new(match *group_literal {
                        GroupLiteral::Single(string, span, id) => {
                            GroupLiteral::Single(format!("-{string}"), op_span + span, id)
                        }
                        GroupLiteral::Tuple(tuple) => GroupLiteral::Tuple(tuple),
                    })));
                }
                Expression::Literal(Literal::Scalar(string, span, id)) => {
                    // Remove the negation from the operations.
                    let (_, op_span) = ops.pop().unwrap();
                    // Construct a negative scalar literal.
                    inner = Expression::Literal(Literal::Scalar(format!("-{string}"), op_span + span, id));
                }
                _ => (), // Do nothing.
            }
        }

        // Apply the operations in reverse order, constructing a unary expression.
        for (op, op_span) in ops.into_iter().rev() {
            inner = Expression::Unary(UnaryExpression {
                span: op_span + inner.span(),
                op,
                receiver: Box::new(inner),
                id: self.node_builder.next_id(),
            });
        }

        Ok(inner)
    }

    // TODO: Parse method call expressions directly and later put them into a canonical form.
    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// method call expression.
    fn parse_method_call_expression(&mut self, receiver: Expression, method: Identifier) -> Result<Expression> {
        // Parse the argument list.
        let (mut args, _, span) = self.parse_expr_tuple()?;
        let span = receiver.span() + span;

        if let (true, Some(op)) = (args.is_empty(), UnaryOperation::from_symbol(method.name)) {
            // Found an unary operator and the argument list is empty.
            Ok(Expression::Unary(UnaryExpression {
                span,
                op,
                receiver: Box::new(receiver),
                id: self.node_builder.next_id(),
            }))
        } else if let (1, Some(op)) = (args.len(), BinaryOperation::from_symbol(method.name)) {
            // Found a binary operator and the argument list contains a single argument.
            Ok(Expression::Binary(BinaryExpression {
                span,
                op,
                left: Box::new(receiver),
                right: Box::new(args.swap_remove(0)),
                id: self.node_builder.next_id(),
            }))
        } else if let (2, Some(CoreFunction::SignatureVerify)) =
            (args.len(), CoreFunction::from_symbols(sym::signature, method.name))
        {
            Ok(Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                ty: Type::Identifier(Identifier::new(sym::signature, self.node_builder.next_id())),
                name: method,
                arguments: {
                    let mut arguments = vec![receiver];
                    arguments.extend(args);
                    arguments
                },
                span,
                id: self.node_builder.next_id(),
            })))
        } else {
            // Attempt to parse the method call as a mapping operation.
            match (args.len(), CoreFunction::from_symbols(sym::Mapping, method.name)) {
                (1, Some(CoreFunction::MappingGet))
                | (2, Some(CoreFunction::MappingGetOrUse))
                | (2, Some(CoreFunction::MappingSet))
                | (1, Some(CoreFunction::MappingRemove))
                | (1, Some(CoreFunction::MappingContains)) => {
                    // Found an instance of `<mapping>.get`, `<mapping>.get_or_use`, `<mapping>.set`, `<mapping>.remove`, or `<mapping>.contains`.
                    Ok(Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: Type::Identifier(Identifier::new(sym::Mapping, self.node_builder.next_id())),
                        name: method,
                        arguments: {
                            let mut arguments = vec![receiver];
                            arguments.extend(args);
                            arguments
                        },
                        span,
                        id: self.node_builder.next_id(),
                    })))
                }
                _ => {
                    // Either an invalid unary/binary operator, or more arguments given.
                    self.emit_err(ParserError::invalid_method_call(receiver, method, args.len(), span));
                    Ok(Expression::Err(ErrExpression { span, id: self.node_builder.next_id() }))
                }
            }
        }
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// static access expression.
    fn parse_associated_access_expression(&mut self, module_name: Expression) -> Result<Expression> {
        // Parse struct name expression into struct type.
        let type_ = if let Expression::Identifier(ident) = module_name {
            Type::Identifier(ident)
        } else {
            return Err(ParserError::invalid_associated_access(&module_name, module_name.span()).into());
        };

        // Parse the struct member name (can be variable or function name).
        let member_name = self.expect_identifier()?;

        // Check if there are arguments.
        Ok(Expression::Access(if self.check(&Token::LeftParen) {
            // Parse the arguments
            let (args, _, end) = self.parse_expr_tuple()?;

            // Return the struct function.
            AccessExpression::AssociatedFunction(AssociatedFunction {
                span: module_name.span() + end,
                ty: type_,
                name: member_name,
                arguments: args,
                id: self.node_builder.next_id(),
            })
        } else {
            // Return the struct constant.
            AccessExpression::AssociatedConstant(AssociatedConstant {
                span: module_name.span() + member_name.span(),
                ty: type_,
                name: member_name,
                id: self.node_builder.next_id(),
            })
        }))
    }

    /// Parses a tuple of `Expression` AST nodes.
    pub(crate) fn parse_expr_tuple(&mut self) -> Result<(Vec<Expression>, bool, Span)> {
        self.parse_paren_comma_list(|p| p.parse_expression().map(Some))
    }

    /// Returns an [`Expression`] AST node if the next tokens represent an
    /// array access, struct member access, function call, or static function call expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_primary_expression`].
    fn parse_postfix_expression(&mut self) -> Result<Expression> {
        // We don't directly parse named-type's and Identifier's here as
        // the ABNF states. Rather the primary expression already
        // handle those. The ABNF is more specific for language reasons.
        let mut expr = self.parse_primary_expression()?;
        loop {
            if self.eat(&Token::Dot) {
                if self.check_int() {
                    // Eat a tuple member access.
                    let (index, span) = self.eat_whole_number()?;
                    expr = Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(expr),
                        index,
                        span,
                        id: self.node_builder.next_id(),
                    }))
                } else if self.eat(&Token::Leo) {
                    // Eat an external function call.
                    self.eat(&Token::Div); // todo: Make `/` a more general token.

                    // Parse function name.
                    let name = self.expect_identifier()?;

                    // Parse the function call.
                    let (arguments, _, span) = self.parse_paren_comma_list(|p| p.parse_expression().map(Some))?;
                    expr = Expression::Call(CallExpression {
                        span: expr.span() + span,
                        function: Box::new(Expression::Identifier(name)),
                        external: Some(Box::new(expr)),
                        arguments,
                        id: self.node_builder.next_id(),
                    });
                } else {
                    // Parse identifier name.
                    let name = self.expect_identifier()?;

                    if self.check(&Token::LeftParen) {
                        // Eat a method call on a type
                        expr = self.parse_method_call_expression(expr, name)?
                    } else {
                        // Eat a struct member access.
                        expr = Expression::Access(AccessExpression::Member(MemberAccess {
                            span: expr.span() + name.span(),
                            inner: Box::new(expr),
                            name,
                            id: self.node_builder.next_id(),
                        }))
                    }
                }
            } else if self.eat(&Token::DoubleColon) {
                // Eat a core struct constant or core struct function call.
                expr = self.parse_associated_access_expression(expr)?;
            } else if self.eat(&Token::LeftSquare) {
                // Eat an array access.
                let index = self.parse_expression()?;
                // Eat the closing bracket.
                let span = self.expect(&Token::RightSquare)?;
                expr = Expression::Access(AccessExpression::Array(ArrayAccess {
                    span: expr.span() + span,
                    array: Box::new(expr),
                    index: Box::new(index),
                    id: self.node_builder.next_id(),
                }))
            } else if self.check(&Token::LeftParen) {
                // Check that the expression is an identifier.
                if !matches!(expr, Expression::Identifier(_)) {
                    self.emit_err(ParserError::unexpected(expr.to_string(), "an identifier", expr.span()))
                }
                // Parse a function call that's by itself.
                let (arguments, _, span) = self.parse_paren_comma_list(|p| p.parse_expression().map(Some))?;
                expr = Expression::Call(CallExpression {
                    span: expr.span() + span,
                    function: Box::new(expr),
                    external: None,
                    arguments,
                    id: self.node_builder.next_id(),
                });
            }
            // Check if next token is a dot to see if we are calling recursive method.
            if !(self.check(&Token::Dot) || self.check(&Token::LeftSquare)) {
                break;
            }
        }
        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// tuple initialization expression or an affine group literal.
    fn parse_tuple_expression(&mut self) -> Result<Expression> {
        if let Some(gt) = self.eat_group_partial().transpose()? {
            return Ok(Expression::Literal(Literal::Group(Box::new(GroupLiteral::Tuple(gt)))));
        }

        let (mut elements, trailing, span) = self.parse_expr_tuple()?;

        match elements.len() {
            // If the tuple expression is empty, return a `UnitExpression`.
            0 => Ok(Expression::Unit(UnitExpression { span, id: self.node_builder.next_id() })),
            1 => match trailing {
                // If there is one element in the tuple but no trailing comma, e.g `(foo)`, return the element.
                false => Ok(elements.swap_remove(0)),
                // If there is one element in the tuple and a trailing comma, e.g `(foo,)`, emit an error since tuples must have at least two elements.
                true => Err(ParserError::tuple_must_have_at_least_two_elements("expression", span).into()),
            },
            // Otherwise, return a tuple expression.
            // Note: This is the only place where `TupleExpression` is constructed in the parser.
            _ => Ok(Expression::Tuple(TupleExpression { elements, span, id: self.node_builder.next_id() })),
        }
    }

    /// Returns an [`Expression`] AST node if the next tokens represent an array initialization expression.
    fn parse_array_expression(&mut self) -> Result<Expression> {
        let (elements, _, span) = self.parse_bracket_comma_list(|p| p.parse_expression().map(Some))?;

        match elements.is_empty() {
            // If the array expression is empty, return an error.
            true => Err(ParserError::array_must_have_at_least_one_element("expression", span).into()),
            // Otherwise, return an array expression.
            // Note: This is the only place where `ArrayExpression` is constructed in the parser.
            false => Ok(Expression::Array(ArrayExpression { elements, span, id: self.node_builder.next_id() })),
        }
    }

    /// Returns a reference to the next token if it is a [`GroupCoordinate`], or [None] if
    /// the next token is not a [`GroupCoordinate`].
    fn peek_group_coordinate(&self, dist: &mut usize) -> Option<GroupCoordinate> {
        let (advanced, gc) = self.look_ahead(*dist, |t0| match &t0.token {
            Token::Add => Some((1, GroupCoordinate::SignHigh)),
            Token::Sub => self.look_ahead(*dist + 1, |t1| match &t1.token {
                Token::Integer(value) => Some((2, GroupCoordinate::Number(format!("-{value}"), t1.span))),
                _ => Some((1, GroupCoordinate::SignLow)),
            }),
            Token::Underscore => Some((1, GroupCoordinate::Inferred)),
            Token::Integer(value) => Some((1, GroupCoordinate::Number(value.clone(), t0.span))),
            _ => None,
        })?;
        *dist += advanced;
        Some(gc)
    }

    /// Removes the next two tokens if they are a pair of [`GroupCoordinate`] and returns them,
    /// or [None] if the next token is not a [`GroupCoordinate`].
    fn eat_group_partial(&mut self) -> Option<Result<GroupTuple>> {
        assert!(self.check(&Token::LeftParen)); // `(`.

        // Peek at first gc.
        let start_span = &self.token.span;
        let mut dist = 1; // 0th is `(` so 1st is first gc's start.
        let first_gc = self.peek_group_coordinate(&mut dist)?;

        let check_ahead = |d, token: &_| self.look_ahead(d, |t| (&t.token == token).then_some(t.span));

        // Peek at `,`.
        check_ahead(dist, &Token::Comma)?;
        dist += 1; // Standing at `,` so advance one for next gc's start.

        // Peek at second gc.
        let second_gc = self.peek_group_coordinate(&mut dist)?;

        // Peek at `)`.
        let right_paren_span = check_ahead(dist, &Token::RightParen)?;
        dist += 1; // Standing at `)` so advance one for 'group'.

        // Peek at `group`.
        let end_span = check_ahead(dist, &Token::Group)?;
        dist += 1; // Standing at `)` so advance one for 'group'.

        let gt =
            GroupTuple { span: start_span + &end_span, x: first_gc, y: second_gc, id: self.node_builder.next_id() };

        // Eat everything so that this isn't just peeking.
        for _ in 0..dist {
            self.bump();
        }

        if let Err(e) = assert_no_whitespace(right_paren_span, end_span, &format!("({},{})", gt.x, gt.y), "group") {
            return Some(Err(e));
        }

        Some(Ok(gt))
    }

    fn parse_struct_member(&mut self) -> Result<StructVariableInitializer> {
        let identifier = if self.allow_identifier_underscores && self.eat(&Token::Underscore) {
            // Allow `_nonce` for struct records.
            let identifier_without_underscore = self.expect_identifier()?;
            Identifier::new(
                Symbol::intern(&format!("_{}", identifier_without_underscore.name)),
                self.node_builder.next_id(),
            )
        } else {
            self.expect_identifier()?
        };

        let (expression, span) = if self.eat(&Token::Colon) {
            // Parse individual struct variable declarations.
            let expression = self.parse_expression()?;
            let span = identifier.span + expression.span();
            (Some(expression), span)
        } else {
            (None, identifier.span)
        };

        Ok(StructVariableInitializer { identifier, expression, id: self.node_builder.next_id(), span })
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// struct initialization expression.
    /// let foo = Foo { x: 1u8 };
    pub fn parse_struct_init_expression(&mut self, identifier: Identifier) -> Result<Expression> {
        let (members, _, end) =
            self.parse_list(Delimiter::Brace, Some(Token::Comma), |p| p.parse_struct_member().map(Some))?;

        Ok(Expression::Struct(StructExpression {
            span: identifier.span + end,
            name: identifier,
            members,
            id: self.node_builder.next_id(),
        }))
    }

    /// Returns an [`Expression`] AST node if the next token is a primary expression:
    /// - Literals: field, group, unsigned integer, signed integer, boolean, address
    /// - Aggregate types: array, tuple
    /// - Identifiers: variables, keywords
    /// - self
    ///
    /// Returns an expression error if the token cannot be matched.
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        if let Token::LeftParen = self.token.token {
            return self.parse_tuple_expression();
        } else if let Token::LeftSquare = self.token.token {
            return self.parse_array_expression();
        }

        let SpannedToken { token, span } = self.token.clone();
        self.bump();

        Ok(match token {
            Token::Integer(value) => {
                let suffix_span = self.token.span;
                let full_span = span + suffix_span;
                let assert_no_whitespace = |x| assert_no_whitespace(span, suffix_span, &value, x);
                match self.eat_any(INT_TYPES).then_some(&self.prev_token.token) {
                    // Literal followed by `field`, e.g., `42field`.
                    Some(Token::Field) => {
                        assert_no_whitespace("field")?;
                        Expression::Literal(Literal::Field(value, full_span, self.node_builder.next_id()))
                    }
                    // Literal followed by `group`, e.g., `42group`.
                    Some(Token::Group) => {
                        assert_no_whitespace("group")?;
                        Expression::Literal(Literal::Group(Box::new(GroupLiteral::Single(
                            value,
                            full_span,
                            self.node_builder.next_id(),
                        ))))
                    }
                    // Literal followed by `scalar` e.g., `42scalar`.
                    Some(Token::Scalar) => {
                        assert_no_whitespace("scalar")?;
                        Expression::Literal(Literal::Scalar(value, full_span, self.node_builder.next_id()))
                    }
                    // Literal followed by other type suffix, e.g., `42u8`.
                    Some(suffix) => {
                        assert_no_whitespace(&suffix.to_string())?;
                        let int_ty = Self::token_to_int_type(suffix).expect("unknown int type token");
                        Expression::Literal(Literal::Integer(int_ty, value, full_span, self.node_builder.next_id()))
                    }
                    None => return Err(ParserError::implicit_values_not_allowed(value, span).into()),
                }
            }
            Token::True => Expression::Literal(Literal::Boolean(true, span, self.node_builder.next_id())),
            Token::False => Expression::Literal(Literal::Boolean(false, span, self.node_builder.next_id())),
            Token::AddressLit(address_string) => {
                if address_string.parse::<Address<Testnet3>>().is_err() {
                    self.emit_err(ParserError::invalid_address_lit(&address_string, span));
                }
                Expression::Literal(Literal::Address(address_string, span, self.node_builder.next_id()))
            }
            Token::StaticString(value) => {
                Expression::Literal(Literal::String(value, span, self.node_builder.next_id()))
            }
            Token::Identifier(name) => {
                let ident = Identifier { name, span, id: self.node_builder.next_id() };
                if !self.disallow_struct_construction && self.check(&Token::LeftCurly) {
                    // Parse struct and records inits as struct expressions.
                    // Enforce struct or record type later at type checking.
                    self.parse_struct_init_expression(ident)?
                } else {
                    Expression::Identifier(ident)
                }
            }
            Token::SelfLower => {
                Expression::Identifier(Identifier { name: sym::SelfLower, span, id: self.node_builder.next_id() })
            }
            Token::Block => {
                Expression::Identifier(Identifier { name: sym::block, span, id: self.node_builder.next_id() })
            }
            t if crate::type_::TYPE_TOKENS.contains(&t) => Expression::Identifier(Identifier {
                name: t.keyword_to_symbol().unwrap(),
                span,
                id: self.node_builder.next_id(),
            }),
            token => {
                return Err(ParserError::unexpected_str(token, "expression", span).into());
            }
        })
    }
}

fn assert_no_whitespace(left_span: Span, right_span: Span, left: &str, right: &str) -> Result<()> {
    if left_span.hi != right_span.lo {
        let error_span = Span::new(left_span.hi, right_span.lo); // The span between them.
        return Err(ParserError::unexpected_whitespace(left, right, error_span).into());
    }

    Ok(())
}

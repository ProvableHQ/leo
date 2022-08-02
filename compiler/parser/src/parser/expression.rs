// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_span::Symbol;
use snarkvm_console::{account::Address, network::Testnet3};

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
    /// Includes circuit init expressions.
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        // Store current parser state.
        let prior_fuzzy_state = self.disallow_circuit_construction;

        // Allow circuit init expressions.
        self.disallow_circuit_construction = false;

        // Parse expression.
        let result = self.parse_conditional_expression();

        // Restore prior parser state.
        self.disallow_circuit_construction = prior_fuzzy_state;

        result
    }

    /// Returns an [`Expression`] AST node if the next tokens represent
    /// a ternary expression. May or may not include circuit init expressions.
    ///
    /// Otherwise, tries to parse the next token using [`parse_boolean_or_expression`].
    pub(super) fn parse_conditional_expression(&mut self) -> Result<Expression> {
        // Try to parse the next expression. Try BinaryOperation::Or.
        let expr = self.parse_boolean_or_expression()?;

        // Parse the rest of the ternary expression.
        if self.eat(&Token::Question) {
            let if_true = self.parse_expression()?;
            self.expect(&Token::Colon)?;
            let if_false = self.parse_expression()?;
            let span = expr.span() + if_false.span();
            let kind = ExpressionKind::Ternary(TernaryExpression {
                condition: Box::new(expr),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            });
            return Ok(Expression { kind, span });
        }
        Ok(expr)
    }

    /// Creates a binary expression for `left op right`.
    fn mk_binary(left: Expression, right: Expression, op: BinaryOperation) -> Expression {
        let span = left.span() + right.span();
        let kind = ExpressionKind::Binary(BinaryExpression {
            op,
            left: Box::new(left),
            right: Box::new(right),
        });
        Expression { kind, span }
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
            expr = Self::mk_binary(expr, f(self)?, op);
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
            expr = Self::mk_binary(expr, right, op);
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
            expr = Self::mk_binary(expr, right, op);
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
        self.parse_bin_expr(
            &[Token::Mul, Token::Div, Token::Rem],
            Self::parse_exponential_expression,
        )
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// binary exponentiation expression.
    ///
    /// Otherwise, tries to parse the next token using [`parse_unary_expression`].
    fn parse_exponential_expression(&mut self) -> Result<Expression> {
        let mut expr = self.parse_unary_expression()?;

        if let Some(op) = self.eat_bin_op(&[Token::Pow]) {
            let right = self.parse_exponential_expression()?;
            expr = Self::mk_binary(expr, right, op);
        }

        Ok(expr)
    }

    /// Creates an unary expression for `op arg`.
    fn mk_unary(span: Span, op: UnaryOperation, receiver: Expression) -> Expression {
        let receiver = Box::new(receiver);
        let kind = ExpressionKind::Unary(UnaryExpression { op, receiver });
        Expression { span, kind }
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
        let expr = ops
            .into_iter()
            .rev()
            .fold(self.parse_postfix_expression()?, |inner, (op, op_span)| {
                Self::mk_unary(op_span + inner.span(), op, inner)
            });
        Ok(expr)
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// method call expression.
    fn parse_method_call_expression(&mut self, receiver: Expression, method: Identifier) -> Result<Expression> {
        // Parse the argument list.
        let (mut args, _, span) = self.parse_expr_tuple()?;
        let span = receiver.span() + span;

        if let (true, Some(op)) = (args.is_empty(), UnaryOperation::from_symbol(method.name)) {
            // Found an unary operator and the argument list is empty.
            Ok(Self::mk_unary(span, op, receiver))
        } else if let (1, Some(op)) = (args.len(), BinaryOperation::from_symbol(method.name)) {
            // Found a binary operator and the argument list contains a single argument.
            Ok(Self::mk_binary(receiver, args.swap_remove(0), op))
        } else {
            // Either an invalid unary/binary operator, or more arguments given.
            self.emit_err(ParserError::invalid_method_call(receiver, method, span));
            let kind = ExpressionKind::Err(ErrExpression);
            Ok(Expression { span, kind })
        }
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// static access expression.
    fn parse_associated_access_expression(&mut self, circuit_name: Expression) -> Result<Expression> {
        // Parse circuit name expression into circuit type.
        let circuit_type = if let ExpressionKind::Identifier(ident) = circuit_name.kind {
            Type::Identifier(ident)
        } else {
            return Err(ParserError::invalid_associated_access(&circuit_name, circuit_name.span()).into());
        };

        // Parse the circuit member name (can be variable or function name).
        let member_name = self.expect_identifier()?;

        // Check if there are arguments.
        let ae = if self.check(&Token::LeftParen) {
            // Parse the arguments
            let (args, ..) = self.parse_expr_tuple()?;

            // Return the circuit function.
            AccessExpression::AssociatedFunction(AssociatedFunction {
                ty: circuit_type,
                name: member_name,
                args,
            })
        } else {
            // Return the circuit constant.
            AccessExpression::AssociatedConstant(AssociatedConstant {
                ty: circuit_type,
                name: member_name,
            })
        };
        Ok(self.mk_access_expr(circuit_name.span(), ae))
    }

    /// Parses a tuple of `Expression` AST nodes.
    pub(crate) fn parse_expr_tuple(&mut self) -> Result<(Vec<Expression>, bool, Span)> {
        self.parse_paren_comma_list(|p| p.parse_expression().map(Some))
    }

    /// Returns an [`Expression`] AST node if the next tokens represent an
    /// array access, circuit member access, function call, or static function call expression.
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
                    let index = self.eat_integer()?;
                    expr = self.mk_access_expr(
                        expr.span(),
                        AccessExpression::Tuple(TupleAccess {
                            tuple: Box::new(expr),
                            index,
                        }),
                    );
                } else {
                    // Parse accessed field name.
                    let name = self.expect_identifier()?;

                    if self.check(&Token::LeftParen) {
                        // Eat a method call on a type
                        expr = self.parse_method_call_expression(expr, name)?
                    } else {
                        // Eat a circuit member access.
                        expr = self.mk_access_expr(
                            expr.span(),
                            AccessExpression::Member(MemberAccess {
                                inner: Box::new(expr),
                                name,
                            }),
                        );
                    }
                }
            } else if self.eat(&Token::DoubleColon) {
                // Eat a core circuit constant or core circuit function call.
                expr = self.parse_associated_access_expression(expr)?;
            } else if self.check(&Token::LeftParen) {
                // Parse a function call that's by itself.
                let (arguments, _, span) = self.parse_paren_comma_list(|p| p.parse_expression().map(Some))?;
                let span = expr.span() + span;
                let kind = ExpressionKind::Call(CallExpression {
                    function: Box::new(expr),
                    arguments,
                });
                expr = Expression { kind, span };
            }
            // Check if next token is a dot to see if we are calling recursive method.
            if !self.check(&Token::Dot) {
                break;
            }
        }
        Ok(expr)
    }

    /// Creates an access expression.
    fn mk_access_expr(&self, lo: Span, ae: AccessExpression) -> Expression {
        let span = lo + self.prev_token.span;
        let kind = ExpressionKind::Access(ae);
        Expression { kind, span }
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// tuple initialization expression or an affine group literal.
    fn parse_tuple_expression(&mut self) -> Result<Expression> {
        if let Some(gt) = self.eat_group_partial().transpose()? {
            let span = gt.span;
            let kind = ExpressionKind::Literal(Literal::Group(Box::new(GroupLiteral::Tuple(gt))));
            return Ok(Expression { kind, span });
        }

        let (mut elements, trailing, span) = self.parse_expr_tuple()?;

        if !trailing && elements.len() == 1 {
            Ok(elements.swap_remove(0))
        } else {
            let kind = ExpressionKind::Tuple(TupleExpression { elements });
            Ok(Expression { kind, span })
        }
    }

    /// Returns a reference to the next token if it is a [`GroupCoordinate`], or [None] if
    /// the next token is not a [`GroupCoordinate`].
    fn peek_group_coordinate(&self, dist: &mut usize) -> Option<GroupCoordinate> {
        let (advanced, gc) = self.look_ahead(*dist, |t0| match &t0.token {
            Token::Add => Some((1, GroupCoordinate::SignHigh)),
            Token::Sub => self.look_ahead(*dist + 1, |t1| match &t1.token {
                Token::Integer(value) => Some((2, GroupCoordinate::Number(format!("-{}", value), t1.span))),
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

        let check_ahead = |d, token: &_| self.look_ahead(d, |t| (&t.token == token).then(|| t.span));

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

        let gt = GroupTuple {
            span: start_span + &end_span,
            x: first_gc,
            y: second_gc,
        };

        // Eat everything so that this isn't just peeking.
        for _ in 0..dist {
            self.bump();
        }

        if let Err(e) = assert_no_whitespace(right_paren_span, end_span, &format!("({},{})", gt.x, gt.y), "group") {
            return Some(Err(e));
        }

        Some(Ok(gt))
    }

    fn parse_circuit_member(&mut self) -> Result<CircuitVariableInitializer> {
        let identifier = if self.allow_identifier_underscores && self.eat(&Token::Underscore) {
            // Allow `_nonce` for circuit records.
            let identifier_without_underscore = self.expect_identifier()?;
            Identifier::new(Symbol::intern(&format!("_{}", identifier_without_underscore.name)))
        } else {
            self.expect_identifier()?
        };

        let expression = if self.eat(&Token::Colon) {
            // Parse individual circuit variable declarations.
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(CircuitVariableInitializer { identifier, expression })
    }

    /// Returns an [`Expression`] AST node if the next tokens represent a
    /// circuit initialization expression.
    /// let foo = Foo { x: 1u8 };
    pub fn parse_circuit_init_expression(&mut self, identifier: Identifier) -> Result<Expression> {
        let (members, _, end) = self.parse_list(Delimiter::Brace, Some(Token::Comma), |p| {
            p.parse_circuit_member().map(Some)
        })?;

        let span = identifier.span + end;
        let kind = ExpressionKind::Circuit(CircuitExpression {
            name: identifier,
            members,
        });
        Ok(Expression { kind, span })
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
        }

        let SpannedToken { token, span } = self.token.clone();
        self.bump();

        Ok(match token {
            Token::Integer(value) => {
                let suffix_span = self.token.span;
                let full_span = span + suffix_span;
                let assert_no_whitespace = |x| assert_no_whitespace(span, suffix_span, &value, x);
                let lit = match self.eat_any(INT_TYPES).then(|| &self.prev_token.token) {
                    // Literal followed by `field`, e.g., `42field`.
                    Some(Token::Field) => {
                        assert_no_whitespace("field")?;
                        Literal::Field(value, full_span)
                    }
                    // Literal followed by `group`, e.g., `42group`.
                    Some(Token::Group) => {
                        assert_no_whitespace("group")?;
                        Literal::Group(Box::new(GroupLiteral::Single(value, full_span)))
                    }
                    // Literal followed by `scalar` e.g., `42scalar`.
                    Some(Token::Scalar) => {
                        assert_no_whitespace("scalar")?;
                        Literal::Scalar(value, full_span)
                    }
                    // Literal followed by other type suffix, e.g., `42u8`.
                    Some(suffix) => {
                        assert_no_whitespace(&suffix.to_string())?;
                        match suffix {
                            Token::I8 => Literal::I8(value, full_span),
                            Token::I16 => Literal::I16(value, full_span),
                            Token::I32 => Literal::I32(value, full_span),
                            Token::I64 => Literal::I64(value, full_span),
                            Token::I128 => Literal::I128(value, full_span),
                            Token::U8 => Literal::U8(value, full_span),
                            Token::U16 => Literal::U16(value, full_span),
                            Token::U32 => Literal::U32(value, full_span),
                            Token::U64 => Literal::U64(value, full_span),
                            Token::U128 => Literal::U128(value, full_span),
                            _ => return Err(ParserError::unexpected_token("Expected integer type suffix", span).into()),
                        }
                    }
                    None => return Err(ParserError::implicit_values_not_allowed(value, span).into()),
                };
                Self::mk_lit_expr(full_span, lit)
            }
            Token::True => Self::mk_lit_expr(span, Literal::Boolean(true, span)),
            Token::False => Self::mk_lit_expr(span, Literal::Boolean(false, span)),
            Token::AddressLit(address_string) => {
                if address_string.parse::<Address<Testnet3>>().is_err() {
                    self.emit_err(ParserError::invalid_address_lit(&address_string, span));
                }
                Self::mk_lit_expr(span, Literal::Address(address_string, span))
            }
            Token::StaticString(value) => Self::mk_lit_expr(span, Literal::String(value, span)),
            Token::Identifier(name) => {
                let ident = Identifier { name, span };
                if !self.disallow_circuit_construction && self.check(&Token::LeftCurly) {
                    // Parse circuit and records inits as circuit expressions.
                    // Enforce circuit or record type later at type checking.
                    self.parse_circuit_init_expression(ident)?
                } else {
                    Self::mk_ident_expr(ident)
                }
            }
            t if crate::type_::TYPE_TOKENS.contains(&t) => Self::mk_ident_expr(Identifier {
                name: t.keyword_to_symbol().unwrap(),
                span,
            }),
            token => {
                return Err(ParserError::unexpected_str(token, "expression", span).into());
            }
        })
    }

    /// Creates an literal expression for `lit`.
    fn mk_lit_expr(span: Span, lit: Literal) -> Expression {
        let kind = ExpressionKind::Literal(lit);
        Expression { span, kind }
    }

    /// Creates an identifier expression for `ident`.
    fn mk_ident_expr(ident: Identifier) -> Expression {
        Expression {
            kind: ExpressionKind::Identifier(ident),
            span: ident.span,
        }
    }
}

fn assert_no_whitespace(left_span: Span, right_span: Span, left: &str, right: &str) -> Result<()> {
    if left_span.hi != right_span.lo {
        let error_span = Span::new(left_span.hi, right_span.lo); // The span between them.
        return Err(ParserError::unexpected_whitespace(left, right, error_span).into());
    }

    Ok(())
}

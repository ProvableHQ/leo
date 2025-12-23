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

use crate::CompilerState;

use leo_ast::{BinaryOperation, Expression, Identifier, LiteralVariant, Node as _, Path, UnaryOperation};
use leo_span::Symbol;

use std::collections::HashMap;

/// An atomic expression - path or literal.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Atom {
    Path(Vec<Symbol>),
    Literal(LiteralVariant),
}

/// An expression that can be mapped to a variable, and eliminated if it appears again.
///
/// For now we are rather conservative in the types of expressions we allow.
/// We define this separate type rather than using `Expression` largely for ease
/// of hashing and comparison while ignoring superfluous information like node ids and
/// spans. It also makes explicit in the type that subexpressions must be atoms.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Expr {
    Atom(Atom),
    Array(Vec<Atom>),
    ArrayAccess { array: Atom, index: Atom },
    Binary { op: BinaryOperation, left: Atom, right: Atom },
    Repeat { value: Atom, count: Atom },
    Ternary { condition: Atom, if_true: Atom, if_false: Atom },
    Unary { op: UnaryOperation, receiver: Atom },
}

impl From<Atom> for Expr {
    fn from(value: Atom) -> Self {
        Expr::Atom(value)
    }
}

#[derive(Default, Debug)]
pub struct Scope {
    pub expressions: HashMap<Expr, Symbol>,
}

pub struct CommonSubexpressionEliminatingVisitor<'a> {
    pub state: &'a mut CompilerState,

    pub scopes: Vec<Scope>,
}

impl CommonSubexpressionEliminatingVisitor<'_> {
    pub fn in_scope<T>(&mut self, func: impl FnOnce(&mut Self) -> T) -> T {
        self.scopes.push(Default::default());
        let result = func(self);
        self.scopes.pop();
        result
    }

    /// Turn `expression` into an `Atom` if possible, looking it up in the expression
    /// tables when it's a path. Also changes `expression` into the found value.
    fn try_atom(&self, expression: &mut Expression) -> Option<Atom> {
        // Get the ID of the expression.
        let id = expression.id();
        // Modify the expression in place if it's a path that can be replaced.
        let value = match expression {
            Expression::Literal(literal) => Atom::Literal(literal.variant.clone()),
            Expression::Path(path) => {
                let atom_path =
                    Atom::Path(path.qualifier().iter().map(|id| id.name).chain([path.identifier().name]).collect());
                let expr = Expr::Atom(atom_path);
                if let Some(name) = self.scopes.iter().rev().find_map(|scope| scope.expressions.get(&expr)) {
                    // Get the type of the expression.
                    let type_ = self.state.type_table.get(&id)?;
                    // Construct a new path for this identifier.
                    let p = Path::from(Identifier::new(*name, self.state.node_builder.next_id())).to_local();
                    // Assign the type of the path.
                    self.state.type_table.insert(p.id(), type_);
                    // This path is mapped to some name already, so replace it.
                    *path = p;
                    Atom::Path(vec![*name])
                } else {
                    let Expr::Atom(atom_path) = expr else { unreachable!() };
                    atom_path
                }
            }

            Expression::ArrayAccess(_)
            | Expression::Intrinsic(_)
            | Expression::Async(_)
            | Expression::Array(_)
            | Expression::Binary(_)
            | Expression::Call(_)
            | Expression::Cast(_)
            | Expression::Err(_)
            | Expression::Locator(_)
            | Expression::MemberAccess(_)
            | Expression::Repeat(_)
            | Expression::Composite(_)
            | Expression::Ternary(_)
            | Expression::Tuple(_)
            | Expression::TupleAccess(_)
            | Expression::Unary(_)
            | Expression::Unit(_) => return None,
        };

        Some(value)
    }

    /// Reconstruct the expression, looking it up in the table of expressions to try to replace it with a
    /// variable.
    ///
    /// - `place` If this expression is the right hand side of a definition, `place` is the left hand side,
    ///
    /// Returns (transformed expression, place_not_needed). `place_not_needed` is true iff it has been mapped to
    /// another path, and thus its definition is no longer needed.
    pub fn try_expr(&mut self, mut expression: Expression, place: Option<Symbol>) -> Option<(Expression, bool)> {
        let span = expression.span();
        let expr: Expr = match &mut expression {
            Expression::ArrayAccess(array_access) => {
                let array = self.try_atom(&mut array_access.array)?;
                let index = self.try_atom(&mut array_access.index)?;
                Expr::ArrayAccess { array, index }
            }
            Expression::Array(array_expression) => {
                let atoms = array_expression
                    .elements
                    .iter_mut()
                    .map(|elt| self.try_atom(elt))
                    .collect::<Option<Vec<Atom>>>()?;
                Expr::Array(atoms)
            }
            Expression::Binary(binary_expression) => {
                let left = self.try_atom(&mut binary_expression.left)?;
                let right = self.try_atom(&mut binary_expression.right)?;
                let (left, right) = if matches!(
                    binary_expression.op,
                    BinaryOperation::Add
                        | BinaryOperation::AddWrapped
                        | BinaryOperation::BitwiseAnd
                        | BinaryOperation::BitwiseOr
                        | BinaryOperation::Eq
                        | BinaryOperation::Neq
                        | BinaryOperation::Mul
                ) && right < left
                {
                    // If it's a commutative op, order the operands in a deterministic order.
                    (right, left)
                } else {
                    (left, right)
                };
                Expr::Binary { op: binary_expression.op, left, right }
            }
            Expression::Literal(literal) => Atom::Literal(literal.variant.clone()).into(),
            Expression::Path(path) => {
                Atom::Path(path.qualifier().iter().map(|id| id.name).chain([path.identifier().name]).collect()).into()
            }
            Expression::Repeat(repeat_expression) => {
                let value = self.try_atom(&mut repeat_expression.expr)?;
                let count = self.try_atom(&mut repeat_expression.count)?;
                Expr::Repeat { value, count }
            }
            Expression::Ternary(ternary_expression) => {
                let condition = self.try_atom(&mut ternary_expression.condition)?;
                let if_true = self.try_atom(&mut ternary_expression.if_true)?;
                let if_false = self.try_atom(&mut ternary_expression.if_false)?;
                Expr::Ternary { condition, if_true, if_false }
            }
            Expression::Unary(unary) => {
                let receiver = self.try_atom(&mut unary.receiver)?;
                Expr::Unary { op: unary.op, receiver }
            }

            Expression::Intrinsic(intrinsic) => {
                for arg in &mut intrinsic.arguments {
                    if !matches!(arg, Expression::Locator(_)) {
                        self.try_atom(arg)?;
                    }
                }
                return Some((expression, false));
            }

            Expression::Call(call) => {
                // Don't worry about the const expressions.
                for arg in &mut call.arguments {
                    self.try_atom(arg)?;
                }
                return Some((expression, false));
            }

            Expression::Cast(cast) => {
                self.try_atom(&mut cast.expression)?;
                return Some((expression, false));
            }

            Expression::MemberAccess(member_access) => {
                self.try_atom(&mut member_access.inner)?;
                return Some((expression, false));
            }

            Expression::Composite(composite_expression) => {
                for initializer in &mut composite_expression.members {
                    if let Some(expr) = initializer.expression.as_mut() {
                        self.try_atom(expr)?;
                    }
                }
                return Some((expression, false));
            }

            Expression::Tuple(tuple_expression) => {
                // Tuple expressions only exist in return statements at this point in
                // compilation, so we need only visit each member.
                tuple_expression.elements = tuple_expression
                    .elements
                    .drain(..)
                    .map(|expr| self.try_expr(expr, None).map(|x| x.0))
                    .collect::<Option<Vec<_>>>()?;
                return Some((expression, false));
            }

            Expression::TupleAccess(_) => panic!("Tuple access expressions should not exist in this pass."),

            Expression::Locator(_) | Expression::Async(_) | Expression::Err(_) | Expression::Unit(_) => {
                return Some((expression, false));
            }
        };

        for map in self.scopes.iter().rev() {
            if let Some(name) = map.expressions.get(&expr).cloned() {
                // We already have a symbol whose value is this expression.
                let identifier = Identifier { name, span, id: self.state.node_builder.next_id() };
                // Get the type of the expression.
                let type_ = self.state.type_table.get(&expression.id())?.clone();
                // Assign the type of the new expression.
                self.state.type_table.insert(identifier.id, type_.clone());
                if let Some(place) = place {
                    // We were defining a new variable, whose right hand side is already defined, so map
                    // this variable to the previous variable.
                    self.scopes.last_mut().unwrap().expressions.insert(Atom::Path(vec![place]).into(), name);
                    return Some((identifier.into(), true));
                }
                return Some((identifier.into(), false));
            }
        }

        if let Some(place) = place {
            // No variable yet refers to this expression, so map the expression to the variable.
            self.scopes.last_mut().unwrap().expressions.insert(expr, place);
        }

        Some((expression, false))
    }
}

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

use crate::expression_can_be_discarded;

use super::{
    SsaConstPropagationVisitor,
    composite_sra::{atom_fields_from_composite, atom_for_member},
    visitor::{is_one_literal, is_zero_literal, same_ssa_atom},
};

use leo_ast::{
    const_eval::{self, Value},
    *,
};
const VALUE_ERROR: &str = "A non-future value should always be able to be converted into an expression";

impl AstReconstructor for SsaConstPropagationVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Option<Value>;

    /// Reconstruct a path expression. If the path refers to a variable that has
    /// a constant value, replace it with that constant.
    fn reconstruct_path(&mut self, input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // In SSA form, paths should refer to local variables (or composite members).
        // Check if this variable has a constant value.
        let identifier_name = input.identifier().name;

        if let Some(constant_value) = self.constants.get(&identifier_name).cloned() {
            // Replace the path with the constant value.
            let span = input.span();
            let id = input.id();
            let (new_expr, _) = self.value_to_expression(&constant_value, span, id).expect(VALUE_ERROR);
            self.changed = true;
            (new_expr, Some(constant_value))
        } else {
            // No constant value for this variable, keep the path as is.
            (input.into(), None)
        }
    }

    /// Reconstruct a member access. If the inner expression is a local path whose
    /// binding was built from a composite literal with atom-valued fields, forward
    /// the access directly to the stored atom. This is scalar replacement of
    /// aggregates for short-lived struct values — the struct itself is left for
    /// dead-code elimination to remove once all field accesses are forwarded.
    fn reconstruct_member_access(
        &mut self,
        mut input: MemberAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (inner, _) = self.reconstruct_expression(input.inner, &());
        input.inner = inner;

        if let Some(atom) = atom_for_member(&self.atom_fielded_composites, &input.inner, input.name.name) {
            self.changed = true;
            // Recompute the atom's Value so downstream folding sees the forwarded
            // constant (literals evaluate directly; paths look up tracked constants).
            let opt_value = match &atom {
                Expression::Literal(lit) => {
                    let ty = self.state.type_table.get(&lit.id());
                    const_eval::literal_to_value(lit, &ty).ok()
                }
                Expression::Path(p) => p.try_local_symbol().and_then(|s| self.constants.get(&s).cloned()),
                // Unreachable: `reconstruct_definition` only populates
                // `atom_fielded_composites` with fields passing `is_atom`,
                // which restricts to `Path`/`Literal`.
                _ => unreachable!("atom_fielded_composites fields must be Path or Literal"),
            };
            return (atom, opt_value);
        }

        (input.into(), None)
    }

    /// Reconstruct a literal expression and convert it to a Value.
    fn reconstruct_literal(&mut self, mut input: Literal, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let type_info = self.state.type_table.get(&input.id());

        // If this is an optional, then unwrap it first.
        let type_info = type_info.as_ref().map(|ty| match ty {
            Type::Optional(opt) => *opt.inner.clone(),
            _ => ty.clone(),
        });

        if let Ok(value) = const_eval::literal_to_value(&input, &type_info) {
            match input.variant {
                LiteralVariant::Address(ref s) if s.ends_with("aleo") => {
                    // Do not fold program names as the VM needs to handle them directly
                    (input.into(), None)
                }

                // If we know the type of an unsuffixed literal, might as well change it to a suffixed literal.
                LiteralVariant::Unsuffixed(s) => {
                    match type_info.expect("Expected type information to be available") {
                        Type::Integer(ty) => input.variant = LiteralVariant::Integer(ty, s),
                        Type::Field => input.variant = LiteralVariant::Field(s),
                        Type::Group => input.variant = LiteralVariant::Group(s),
                        Type::Scalar => input.variant = LiteralVariant::Scalar(s),
                        _ => panic!("Type checking should have prevented this."),
                    }
                    (input.into(), Some(value))
                }
                _ => (input.into(), Some(value)),
            }
        } else {
            (input.into(), None)
        }
    }

    /// Reconstruct a binary expression. First try full constant folding (both
    /// operands known), then apply algebraic identity / absorbing-element rules
    /// when only one operand is a known literal.
    fn reconstruct_binary(
        &mut self,
        input: BinaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let input_id = input.id();

        let (left, lhs_opt_value) = self.reconstruct_expression(input.left, &());
        let (right, rhs_opt_value) = self.reconstruct_expression(input.right, &());

        // Full constant folding: both operands are known constants.
        if let (Some(lhs_value), Some(rhs_value)) = (&lhs_opt_value, &rhs_opt_value) {
            match const_eval::evaluate_binary(
                span,
                input.op,
                lhs_value,
                rhs_value,
                &self.state.type_table.get(&input_id),
            ) {
                Ok(new_value) => {
                    let (new_expr, _) = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    self.changed = true;
                    return (new_expr, Some(new_value));
                }
                Err(err) => {
                    self.emit_err(err);
                    return (BinaryExpression { left, right, ..input }.into(), None);
                }
            }
        }

        // Algebraic simplification: at least one operand is a literal.
        let left_is_zero = matches!(&left, Expression::Literal(lit) if is_zero_literal(lit));
        let right_is_zero = matches!(&right, Expression::Literal(lit) if is_zero_literal(lit));
        let left_is_one = matches!(&left, Expression::Literal(lit) if is_one_literal(lit));
        let right_is_one = matches!(&right, Expression::Literal(lit) if is_one_literal(lit));
        let left_is_bool =
            |val: bool| matches!(&left, Expression::Literal(lit) if lit.variant == LiteralVariant::Boolean(val));
        let right_is_bool =
            |val: bool| matches!(&right, Expression::Literal(lit) if lit.variant == LiteralVariant::Boolean(val));

        match input.op {
            // x + 0 = 0 + x = x
            BinaryOperation::Add | BinaryOperation::AddWrapped => {
                if right_is_zero {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_zero {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
            }
            // x - 0 = x
            BinaryOperation::Sub | BinaryOperation::SubWrapped if right_is_zero => {
                self.changed = true;
                return (left, lhs_opt_value);
            }

            // x * 1 = x, x * 0 = 0
            BinaryOperation::Mul | BinaryOperation::MulWrapped => {
                if right_is_one {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_one {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                // Absorbing element: x * 0 = 0 (only for integers and field, not group or scalar)
                let result_ty = self.state.type_table.get(&input_id);
                if matches!(&result_ty, Some(Type::Integer(_) | Type::Field)) {
                    if right_is_zero {
                        self.changed = true;
                        return (right, rhs_opt_value);
                    }
                    if left_is_zero {
                        self.changed = true;
                        return (left, lhs_opt_value);
                    }
                }
            }

            // x / 1 = x
            BinaryOperation::Div | BinaryOperation::DivWrapped if right_is_one => {
                self.changed = true;
                return (left, lhs_opt_value);
            }

            // x && true = x, x && false = false
            BinaryOperation::And => {
                if right_is_bool(true) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_bool(true) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if right_is_bool(false) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if left_is_bool(false) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
            }

            // x || false = x, x || true = true
            BinaryOperation::Or => {
                if right_is_bool(false) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_bool(false) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if right_is_bool(true) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if left_is_bool(true) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
            }

            BinaryOperation::BitwiseAnd => {
                // x & false = false (boolean), x & 0 = 0 (integer)
                if right_is_zero {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if left_is_zero {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                // x & true = x (boolean)
                if right_is_bool(true) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_bool(true) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
            }
            BinaryOperation::BitwiseOr => {
                // x | 0 = x (integer), x | false = x (boolean)
                if right_is_zero {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_zero {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                // x | true = true (boolean)
                if right_is_bool(true) {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
                if left_is_bool(true) {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
            }
            BinaryOperation::Xor => {
                // x ^ 0 = x (integer), x ^ false = x (boolean)
                if right_is_zero {
                    self.changed = true;
                    return (left, lhs_opt_value);
                }
                if left_is_zero {
                    self.changed = true;
                    return (right, rhs_opt_value);
                }
            }

            // Self-comparison: x == x -> true, x != x -> false
            BinaryOperation::Eq if same_ssa_atom(&left, &right) => {
                self.changed = true;
                let id = self.state.node_builder.next_id();
                self.state.type_table.insert(id, Type::Boolean);
                let lit = Literal::boolean(true, span, id);
                return (lit.into(), Some(Value::from(true)));
            }
            BinaryOperation::Neq if same_ssa_atom(&left, &right) => {
                self.changed = true;
                let id = self.state.node_builder.next_id();
                self.state.type_table.insert(id, Type::Boolean);
                let lit = Literal::boolean(false, span, id);
                return (lit.into(), Some(Value::from(false)));
            }

            _ => {}
        }

        (BinaryExpression { left, right, ..input }.into(), None)
    }

    /// Reconstruct a unary expression. First try constant folding, then apply
    /// algebraic rules (double-not, double-negate).
    fn reconstruct_unary(&mut self, input: UnaryExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let input_id = input.id();
        let span = input.span;
        let (receiver, opt_value) = self.reconstruct_expression(input.receiver, &());

        if let Some(value) = opt_value {
            // We were able to evaluate the operand, so we can evaluate the expression.
            match const_eval::evaluate_unary(span, input.op, &value, &self.state.type_table.get(&input_id)) {
                Ok(new_value) => {
                    let (new_expr, _) = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    self.changed = true;
                    return (new_expr, Some(new_value));
                }
                Err(err) => self.emit_err(err),
            }
        }

        // Double-not: !!x -> x
        if input.op == UnaryOperation::Not
            && let Expression::Unary(ref inner) = receiver
            && inner.op == UnaryOperation::Not
        {
            self.changed = true;
            return (inner.receiver.clone(), None);
        }

        // Double-negate: -(-x) -> x
        if input.op == UnaryOperation::Negate
            && let Expression::Unary(ref inner) = receiver
            && inner.op == UnaryOperation::Negate
        {
            self.changed = true;
            return (inner.receiver.clone(), None);
        }

        (UnaryExpression { receiver, ..input }.into(), None)
    }

    /// Reconstruct a ternary expression and fold a constant condition only after both arms are reconstructed.
    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let ternary_span = input.span();
        let (cond, cond_value) = self.reconstruct_expression(input.condition, &());
        let (if_true, if_true_value) = self.reconstruct_expression(input.if_true, &());
        let (if_false, if_false_value) = self.reconstruct_expression(input.if_false, &());

        match cond_value.and_then(|v| v.try_into().ok()) {
            Some(true) if expression_can_be_discarded(&if_false, self.state) => {
                self.changed = true;
                (if_true, if_true_value)
            }
            Some(false) if expression_can_be_discarded(&if_true, self.state) => {
                self.changed = true;
                (if_false, if_false_value)
            }
            _ => {
                // Boolean branch folding: collapse a bool-literal-branched ternary.
                // Commonly arises after composite forwarding erases an `is_some`-style
                // flag that was selected across a ternary.
                let if_true_bool = if_true_value.as_ref().and_then(|v| bool::try_from(v.clone()).ok());
                let if_false_bool = if_false_value.as_ref().and_then(|v| bool::try_from(v.clone()).ok());
                match (if_true_bool, if_false_bool) {
                    // `cond ? true : false` -> `cond`.
                    (Some(true), Some(false)) => {
                        self.changed = true;
                        return (cond, None);
                    }
                    // `cond ? false : true` -> `!cond`.
                    (Some(false), Some(true)) => {
                        self.changed = true;
                        let id = self.state.node_builder.next_id();
                        self.state.type_table.insert(id, Type::Boolean);
                        let not_cond =
                            UnaryExpression { op: UnaryOperation::Not, receiver: cond, span: ternary_span, id };
                        return (not_cond.into(), None);
                    }
                    // `cond ? b : b` -> `b` for the same bool literal `b`. The
                    // condition is an SSA atom at this point, so dropping it is
                    // side-effect-free.
                    (Some(a), Some(b)) if a == b => {
                        self.changed = true;
                        return (if_true, if_true_value);
                    }
                    _ => {}
                }

                // Identical arms: cond ? x : x -> x (for any type, when both
                // arms are the same SSA variable). Safe because the condition
                // is side-effect-free in SSA form.
                if same_ssa_atom(&if_true, &if_false) {
                    self.changed = true;
                    return (if_true, if_true_value);
                }

                (TernaryExpression { condition: cond, if_true, if_false, ..input }.into(), None)
            }
        }
    }

    /// Reconstruct an array access expression and fold it if array and index are constants.
    fn reconstruct_array_access(
        &mut self,
        input: ArrayAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();

        let (array, array_opt) = self.reconstruct_expression(input.array, &());
        let (index, index_opt) = self.reconstruct_expression(input.index, &());

        if let Some(index_value) = index_opt
            && let Some(array_value) = array_opt
        {
            let result_value =
                array_value.array_index(index_value.as_u32().unwrap() as usize).expect("We already checked bounds.");
            self.changed = true;
            let (new_expr, _) = self.value_to_expression(&result_value, span, id).expect(VALUE_ERROR);
            return (new_expr, Some(result_value.clone()));
        }

        (ArrayAccess { array, index, ..input }.into(), None)
    }

    /// Reconstruct an array expression and fold it if all elements are constants.
    fn reconstruct_array(
        &mut self,
        mut input: ArrayExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        let mut elements_changed = false;
        input.elements.iter_mut().for_each(|element| {
            let old_element = element.clone();
            let (new_element, value_opt) = self.reconstruct_expression(std::mem::take(element), &());
            // Check if the element actually changed (not just its structure, but if it's a different expression)
            if old_element.id() != new_element.id() {
                elements_changed = true;
            }
            if let Some(value) = value_opt {
                values.push(value);
            }
            *element = new_element;
        });
        // Only set changed if elements actually changed. Don't set changed just because
        // we can evaluate the array - that would cause an infinite loop since the array
        // expression structure doesn't change.
        if elements_changed {
            self.changed = true;
        }

        if values.len() == input.elements.len() {
            (input.into(), Some(Value::make_array(values.into_iter())))
        } else {
            (input.into(), None)
        }
    }

    /// Reconstruct a tuple expression and fold it if all elements are constants.
    fn reconstruct_tuple(
        &mut self,
        mut input: TupleExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::with_capacity(input.elements.len());
        let mut elements_changed = false;
        for expr in input.elements.iter_mut() {
            let old_expr = expr.clone();
            let (new_expr, opt_value) = self.reconstruct_expression(std::mem::take(expr), &());
            // Check if the element actually changed
            if old_expr.id() != new_expr.id() {
                elements_changed = true;
            }
            *expr = new_expr;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        // Only set changed if elements actually changed. Don't set changed just because
        // we can evaluate the tuple - that would cause an infinite loop since the tuple
        // expression structure doesn't change.
        if elements_changed {
            self.changed = true;
        }

        let opt_value = if values.len() == input.elements.len() { Some(Value::make_tuple(values)) } else { None };

        (input.into(), opt_value)
    }

    /* Statements */
    /// Reconstruct a definition statement. If the RHS evaluates to a constant, track it
    /// in the constants map for propagation. Additionally, when the RHS is a composite
    /// literal whose fields are all atoms (paths or literals), record the field-to-atom
    /// mapping so that subsequent `x.field` accesses can be forwarded without
    /// rematerializing the struct.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the RHS expression first.
        let (new_value, opt_value) = self.reconstruct_expression(input.value, &());

        if let Some(value) = opt_value {
            match &input.place {
                DefinitionPlace::Single(identifier) => {
                    self.constants.insert(identifier.name, value);
                }
                DefinitionPlace::Multiple(identifiers) => {
                    for (i, id) in identifiers.iter().enumerate() {
                        if let Some(v) = value.tuple_index(i) {
                            self.constants.insert(id.name, v);
                        }
                    }
                }
            }
        } else if let (DefinitionPlace::Single(identifier), Expression::Composite(composite)) =
            (&input.place, &new_value)
            && let Some(fields) = atom_fields_from_composite(composite)
        {
            self.atom_fielded_composites.insert(identifier.name, fields);
        }

        input.value = new_value;

        (input.into(), None)
    }

    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("there should be no assignments at this stage");
    }

    /// Reconstruct a conditional statement and fold it if the condition is a constant.
    ///
    /// Constant conditions reach this pass when inlining substitutes literal arguments into a
    /// finalize-context body, or when the iterative-inline weave const-specializes a duplicated
    /// continuation. Both after the pre-flattening const propagation has already run. The
    /// condition is an SSA atom at this point, so dropping it is side-effect-free, and the
    /// untaken branch is statically dead.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, condition_value) = self.reconstruct_expression(conditional.condition, &());

        match condition_value.and_then(|v| v.try_into().ok()) {
            Some(true) => {
                self.changed = true;
                (Statement::Block(self.reconstruct_block(conditional.then).0), None)
            }
            Some(false) => {
                self.changed = true;
                match conditional.otherwise {
                    Some(otherwise) => self.reconstruct_statement(*otherwise),
                    None => (Statement::dummy(), None),
                }
            }
            _ => {
                let then = self.reconstruct_block(conditional.then).0;
                let otherwise = conditional.otherwise.map(|s| Box::new(self.reconstruct_statement(*s).0));
                (ConditionalStatement { condition, then, otherwise, ..conditional }.into(), None)
            }
        }
    }
}

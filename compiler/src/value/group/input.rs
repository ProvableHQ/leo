// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! Methods to enforce constraints on input group values in a Leo program.

use crate::{ConstrainedValue, GroupType};
use leo_asg::GroupValue;
use leo_ast::InputValue;
use leo_errors::{CompilerError, LeoError, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};

pub(crate) fn allocate_group<F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    option: Option<GroupValue>,
    span: &Span,
) -> Result<G, LeoError> {
    G::alloc(
        cs.ns(|| format!("`{}: group` {}:{}", name, span.line_start, span.col_start)),
        || option.ok_or(SynthesisError::AssignmentMissing),
    )
    .map_err(|_| {
        LeoError::from(CompilerError::group_value_missing_group(
            format!("{}: group", name),
            span,
        ))
    })
}

pub(crate) fn group_from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
    // Check that the parameter value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Group(string) = input {
                Some(string)
            } else {
                return Err(LeoError::from(CompilerError::group_value_missing_group(
                    input.to_string(),
                    span,
                )));
            }
        }
        None => None,
    };

    let group = allocate_group(
        cs,
        name,
        option.map(|x| match *x {
            leo_ast::GroupValue::Single(s, _) => GroupValue::Single(s),
            leo_ast::GroupValue::Tuple(tuple) => GroupValue::Tuple((&tuple.x).into(), (&tuple.y).into()),
        }),
        span,
    )?;

    Ok(ConstrainedValue::Group(group))
}

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

//! Stores all defined names in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::GlobalConst;

// TODO remove
// use snarkvm_models::curves::PrimeField;

// impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
//     pub fn store_global_const(&mut self, global_const: &GlobalConst, value: ConstrainedValue<'a, F, G>) {
//         let variable = global_const.variable.borrow();

//         self.store(variable.id, value);
//     }
// }

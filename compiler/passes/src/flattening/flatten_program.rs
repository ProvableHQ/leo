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

use leo_ast::*;

use crate::Flattener;

impl<'a> ProgramReconstructor for Flattener<'a> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        /* let f_inputs = if input.is_main() {
            let (non_consts, consts): (Vec<_>, Vec<_>) = input
                .input
                .iter()
                .partition(|fi| matches!(fi, FunctionInput::Variable(v) if v.mode() != ParamMode::Const));

            if let Some(main) = self.symbol_table.functions.borrow_mut().get_mut(&input.identifier.name) {
                main.input = &non_consts;
            } else {
                // self.handler
                todo!();
            }

            non_consts
        } else {
            input.input.clone()
        }; */

        Function {
            identifier: input.identifier,
            input: input.input.clone(),
            output: input.output,
            core_mapping: input.core_mapping.clone(),
            block: self.reconstruct_block(input.block),
            span: input.span,
        }
    }
}

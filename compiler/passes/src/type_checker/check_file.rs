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

use crate::{Declaration, TypeChecker, VariableSymbol};

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_function(&mut self, input: &'a Function) -> VisitResult {
        self.symbol_table.clear_variables();
        self.parent = Some(input.name());
        input.input.iter().for_each(|i| {
            let input_var = i.get_variable();

            if let Err(err) = self.symbol_table.insert_variable(
                input_var.identifier.name,
                VariableSymbol {
                    type_: &input_var.type_,
                    span: input_var.span(),
                    declaration: Declaration::Input(input_var.mode()),
                },
            ) {
                self.handler.emit_err(err);
            }
        });

        VisitResult::VisitChildren
    }
}

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

use crate::{DeadCodeEliminator, VariableSymbol, VariableTracker, VariableType};

use leo_ast::{
    AssignStatement,
    Block,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionVisitor as _,
    IterationStatement,
    Node,
    Statement,
    StatementReconstructor,
    StatementVisitor,
    Type,
};
use leo_span::Symbol;

impl StatementReconstructor for DeadCodeEliminator<'_> {
    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_definition(&mut self, _: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let is_used = match &input.place {
            Expression::Identifier(id) => self.symbol_table.symbol_is_used(id.name),
            Expression::Tuple(tuple) => tuple.elements.iter().any(|expr| {
                let Expression::Identifier(id) = expr else {
                    panic!("Invalid lhs of an assignment");
                };
                self.symbol_table.symbol_is_used(id.name)
            }),
            _ => panic!("Invalid lhs of an assignment."),
        };
        if input.value.side_effect_free() && !is_used {
            self.changed = true;
            (Statement::dummy(), Default::default())
        } else {
            // We can't get rid of this assignment.
            (Statement::Assign(Box::new(input)), Default::default())
        }
    }

    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            input.statements = input
                .statements
                .into_iter()
                .map(|stmt| slf.reconstruct_statement(stmt).0)
                .filter(|stmt| !stmt.is_empty())
                .collect();
            (input, Default::default())
        })
    }
}

impl StatementVisitor for VariableTracker<'_> {
    fn visit_block(&mut self, input: &Block) {
        self.in_scope(input.id(), |slf| input.statements.iter().for_each(|stmt| slf.visit_statement(stmt)));
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        // Visit the rhs.
        self.visit_expression(&input.value, &Default::default());

        // Add the symbol(s) on the lhs.
        let empty_symbol = Symbol::intern("");
        let mut insert = |symbol| {
            // We don't actually need any particular information about the variable,
            // so just insert something.
            self.symbol_table
                .insert_variable(empty_symbol, symbol, VariableSymbol {
                    type_: Type::Err,
                    span: Default::default(),
                    declaration: VariableType::Const,
                })
                .expect("Variable insertion failed");
        };
        match &input.place {
            Expression::Identifier(id) => insert(id.name),
            Expression::Tuple(tuple) => {
                for expr in tuple.elements.iter() {
                    let Expression::Identifier(id) = expr else {
                        panic!("Invalid lhs of an assignment.");
                    };
                    insert(id.name);
                }
            }
            _ => panic!("Invalid lhs of an assignment."),
        }
    }

    fn visit_definition(&mut self, _input: &DefinitionStatement) {
        panic!("`DefinitionStatement`s should not exist at this phase of compilation.");
    }

    fn visit_iteration(&mut self, _input: &IterationStatement) {
        panic!("`IterationStatement`s should not exist at this phase of compilation.");
    }
}

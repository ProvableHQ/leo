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

use crate::{Pass, PathResolution, SymbolTable, SymbolTableCreation, TypeChecking, TypeCheckingInput};

use leo_ast::{ArrayType, CompositeType, ProgramReconstructor as _, Type};
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::IndexMap;
use itertools::Itertools;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct OptionLowering;

impl Pass for OptionLowering {
    type Input = TypeCheckingInput;
    type Output = ();

    const NAME: &str = "OptionLowering";

    fn do_pass(input: TypeCheckingInput, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = OptionLoweringVisitor {
            state,
            program: Symbol::intern(""),
            module: vec![],
            function: None,
            new_structs: IndexMap::new(),
            reconstructed_structs: IndexMap::new(),
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        // We need to recreate the symbol table and run type checking again because this pass may introduce new structs
        // and modify existing ones.
        visitor.state.symbol_table = SymbolTable::default();
        println!("{}", state.ast.ast);
        PathResolution::do_pass((), state)?;
        SymbolTableCreation::do_pass((), state)?;
        TypeChecking::do_pass(input.clone(), state)?;

        Ok(())
    }
}

pub fn make_optional_struct_symbol(ty: &Type) -> Symbol {
    // Step 1: Extract a usable type name
    fn display_type(ty: &Type) -> String {
        match ty {
            Type::Address
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::Signature
            | Type::Boolean
            | Type::Integer(..) => format!("{ty}"),
            Type::Array(ArrayType { element_type, length }) => {
                format!("[{}; {length}]", display_type(element_type))
            }
            Type::Composite(CompositeType { path, .. }) => {
                format!("::{}", path.absolute_path().iter().format("::"))
            }

            Type::Tuple(_)
            | Type::Optional(_)
            | Type::Mapping(_)
            | Type::Numeric
            | Type::Identifier(_)
            | Type::Future(_)
            | Type::String
            | Type::Err
            | Type::Unit => {
                panic!("unexpected inner type in optional struct name")
            }
        }
    }

    // Step 3: Build symbol that ends with `?`.
    Symbol::intern(&format!("\"{}?\"", display_type(ty)))
}

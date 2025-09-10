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

use leo_ast::{CompositeType, ProgramReconstructor as _, Type};
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
            modified_structs: IndexMap::new(),
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;

        println!("after: {}", visitor.state.ast.ast);

        // We need to recreate the symbol table and run type checking again because this pass introduces new structs
        // to the program.
        visitor.state.symbol_table = SymbolTable::default();
        PathResolution::do_pass((), state)?;
        SymbolTableCreation::do_pass((), state)?;
        TypeChecking::do_pass(input.clone(), state)?;

        Ok(())
    }
}

pub fn sanitize_name(name: &str) -> String {
    use regex::Regex;

    // Replace any character that is NOT alphanumeric or underscore with underscore
    let re_non_alnum = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
    let temp = re_non_alnum.replace_all(name, "_");

    // Optionally collapse multiple underscores into one
    let re_multi_underscore = Regex::new(r"_+").unwrap();
    let sanitized = re_multi_underscore.replace_all(&temp, "_");

    // Optionally trim leading/trailing underscores
    sanitized.trim_matches('_').to_string()
}

pub fn visit_type(input: &Type) -> String {
    match input {
        Type::Address
        | Type::Field
        | Type::Group
        | Type::Scalar
        | Type::Signature
        | Type::String
        | Type::Future(..)
        | Type::Identifier(..)
        | Type::Boolean
        | Type::Integer(..)
        | Type::Array(_) => format!("{input}"),
        Type::Composite(CompositeType { path, .. }) => path.absolute_path().iter().format("::").to_string(),
        Type::Optional(_) => {
            panic!("Optional types are not supported at this phase of compilation")
        }
        Type::Mapping(_) => {
            panic!("Mapping types are not supported at this phase of compilation")
        }
        Type::Tuple(_) => {
            panic!("Tuple types should not be visited at this phase of compilation")
        }
        Type::Numeric => panic!("`Numeric` types should not exist at this phase of compilation"),
        Type::Err => panic!("Error types should not exist at this phase of compilation"),
        Type::Unit => panic!("Unit types are not supported at this phase of compilation"),
    }
}

pub fn optional_struct_name(ty: &Type) -> Symbol {
    Symbol::intern(&sanitize_name(&format!("Op__{}", visit_type(ty))))
}

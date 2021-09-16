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

// Currently does not support crate `leo-compiler` because it has a dependency in its tree
// which is not wasm compatible. All compiler passes (such as TypeInference)

use leo_ast::AstPass;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(method, catch)]
pub fn parse(program: &str) -> Result<String, JsValue> {
    Ok(parse_program(program).map_err(|err| JsValue::from_str(&err.to_string()))?)
}

/// Parse the program and pass the Canonicalization phase;
/// Asg is useless without compiler passes, so we need to add them once the compatibility problem in
/// snarkvm is solved.
fn parse_program(program: &str) -> leo_errors::Result<String> {
    let ast = leo_parser::parse_ast("", program)?;
    let ast = leo_ast_passes::Canonicalizer::do_pass(ast.into_repr())?.to_json_string()?;

    Ok(ast)
}

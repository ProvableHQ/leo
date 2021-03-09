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

use crate::ast::Ast;
use leo_asg::{new_alloc_context, new_context, Asg as LeoAsg};
use leo_parser::parse_ast;

use std::path::Path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Asg(LeoAsg<'static>);

#[wasm_bindgen]
impl Asg {
    #[wasm_bindgen(constructor)]
    pub fn from(ast: &Ast) -> Self {
        let arena = new_alloc_context();
        let asg = LeoAsg::new(new_context(&arena), &ast.0, &mut leo_imports::ImportParser::default()).unwrap();
        Self(asg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn asg_test() {
        // let expected = include_str!("../.resources/basic/expected_ast.json");

        let filepath = "../.resources/basic/main.leo";
        let program_name = "basic";
        let program_string = include_str!("../.resources/basic/main.leo");

        let ast = Ast::new(filepath, program_name, program_string);
        let candidate = Asg::from(&ast);

        // let expected = JsValue::from_str(expected);
        // let candidate = JsValue::from_serde(&candidate).unwrap();
        //
        // assert_eq!(expected, candidate);
    }
}

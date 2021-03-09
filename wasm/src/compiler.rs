// // Copyright (C) 2019-2021 Aleo Systems Inc.
// // This file is part of the Leo library.
//
// // The Leo library is free software: you can redistribute it and/or modify
// // it under the terms of the GNU General Public License as published by
// // the Free Software Foundation, either version 3 of the License, or
// // (at your option) any later version.
//
// // The Leo library is distributed in the hope that it will be useful,
// // but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// // GNU General Public License for more details.
//
// // You should have received a copy of the GNU General Public License
// // along with the Leo library. If not, see <https://www.gnu.org/licenses/>.
//
// use crate::ast::Ast;
//
// use std::path::Path;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// pub struct Compiler(String);
//
// #[wasm_bindgen]
// impl Compiler {
//     #[wasm_bindgen(constructor)]
//     pub fn new(filepath: &str, program_name: &str, program_string: &str) -> Self {
//         let ast = Ast::new(filepath, program_name, program_string).unwrap();
//         Self(ast.to_string())
//     }
//
//     #[wasm_bindgen]
//     pub fn to_string(&self) -> String {
//         self.0.clone()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use wasm_bindgen_test::*;
//
//     #[wasm_bindgen_test]
//     fn ast_test() {}
// }

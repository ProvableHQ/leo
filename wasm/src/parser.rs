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
// use leo_ast::Ast as LeoAst;
// use leo_parser::parse_ast;
//
// use std::path::Path;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// pub struct Parser(pub(crate) String);
//
// #[wasm_bindgen]
// impl Parser {
//     #[wasm_bindgen(constructor)]
//     pub fn new(filepath: &str, program_string: &str) -> Self {
//         let grammar = LeoGrammar::new(&Path::new(filepath), &program_string).unwrap();
//         Self(grammar.to_json_string().unwrap())
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
//     fn grammar_test() {
//         let expected = include_str!("../.resources/basic/expected_grammar.json");
//
//         let filepath = "../.resources/basic/main.leo";
//         let program_string = include_str!("../.resources/basic/main.leo");
//
//         let candidate = Parser::new(filepath, program_string).to_string();
//
//         let expected = JsValue::from_str(expected);
//         let candidate = JsValue::from_serde(&candidate).unwrap();
//
//         assert_eq!(expected, candidate);
//     }
// }

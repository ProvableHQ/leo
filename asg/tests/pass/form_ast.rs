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

use crate::{load_asg, make_test_context};
use leo_parser::parse_ast;

#[test]
fn test_basic() {
    let program_string = include_str!("./circuits/pedersen_mock.leo");
    let asg = load_asg(program_string).unwrap();
    let reformed_ast = leo_asg::reform_ast(&asg);
    println!("{}", reformed_ast);
    // panic!();
}

#[test]
fn test_function_rename() {
    let program_string = r#"
    function iteration() -> u32 {
        let a = 0u32;
    
        for i in 0..10 {
            a += 1;
        }
    
        return a
    }
    
    function main() {
        const total = iteration() + iteration();
    
        console.assert(total == 20);
    }
    "#;
    let asg = load_asg(program_string).unwrap();
    let reformed_ast = leo_asg::reform_ast(&asg);
    println!("{}", reformed_ast);
    // panic!();
}

#[test]
fn test_imports() {
    let import_name = "test-import".to_string();
    let context = make_test_context();
    let mut imports = crate::mocked_resolver(&context);
    let test_import = r#"
    circuit Point {
      x: u32
      y: u32
    }
    
    function foo() -> u32 {
      return 1u32
    }
  "#;
    imports
        .packages
        .insert(import_name.clone(), load_asg(test_import).unwrap());
    let program_string = r#"
        import test-import.foo;

        function main() {
            console.assert(foo() == 1u32);
        }
    "#;

    let test_import_ast = parse_ast(&import_name, test_import).unwrap();
    println!("{}", serde_json::to_string(test_import_ast.as_repr()).unwrap());

    let test_ast = parse_ast("test.leo", program_string).unwrap();
    println!("{}", serde_json::to_string(test_ast.as_repr()).unwrap());

    let asg = crate::load_asg_imports(&context, program_string, &mut imports).unwrap();
    let reformed_ast = leo_asg::reform_ast(&asg);
    println!("{}", serde_json::to_string(&reformed_ast).unwrap());
    // panic!();
}

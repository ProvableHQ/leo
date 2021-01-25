// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::load_asg;

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
        let mut a = 0u32;
    
        for i in 0..10 {
            a += 1;
        }
    
        return a
    }
    
    function main() {
        let total = iteration() + iteration();
    
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
    let mut imports = crate::mocked_resolver();
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
        .insert("test-import".to_string(), load_asg(test_import).unwrap());
    let program_string = r#"
        import test-import.foo;

        function main() {
            console.assert(foo() == 1u32);
        }
    "#;
    println!(
        "{}",
        serde_json::to_string(&crate::load_ast("test-import.leo", test_import).unwrap()).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&crate::load_ast("test.leo", program_string).unwrap()).unwrap()
    );
    let asg = crate::load_asg_imports(program_string, &mut imports).unwrap();
    let reformed_ast = leo_asg::reform_ast(&asg);
    println!("{}", serde_json::to_string(&reformed_ast).unwrap());
    // panic!();
}

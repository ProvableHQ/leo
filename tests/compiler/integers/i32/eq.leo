/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i32, b: i32, c: bool) -> bool {
        return (a == b) == c;
    }
}

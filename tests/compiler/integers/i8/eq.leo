/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i8, b: i8, c: bool) -> bool {
        return (a == b) == c;
    }
}

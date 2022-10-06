/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i16, b: i16, c: bool) -> bool {
        return (a == b) == c;
    }
}

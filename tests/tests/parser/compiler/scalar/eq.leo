/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: scalar) -> bool {
        let c: bool = a == b;
        let d: bool = a != b;
        return d;
    }}

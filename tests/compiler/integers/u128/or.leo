/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u128, b: u128, c: u128) -> bool {
        return a | b == c;
    }
}

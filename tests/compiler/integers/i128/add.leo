/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128, b: i128, c: i128) -> bool {
        return a + b == c;
    }
}

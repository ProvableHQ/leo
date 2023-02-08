/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i32, b: i32, c: i32) -> bool {
        return a & b == c;
    }
}

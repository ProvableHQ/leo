/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i32) -> i32 {
        let b: i32 = -2147483647i32;
        return b - a; // -2147483648i32
    }
}

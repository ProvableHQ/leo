/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: i32, b: i32, c: i32) -> bool {
        let r: i32 = s ? a : b;
    
        return r == c;
    }
}

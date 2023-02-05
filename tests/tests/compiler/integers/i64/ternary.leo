/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: i64, b: i64, c: i64) -> bool {
        let r: i64 = s ? a : b;
    
        return r == c;
    }
}

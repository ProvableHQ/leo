/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // TODO: This test passes, but constant propogation should detect an overflow.
    
    transition main(y: bool) -> i64 {
        let a: i64 = -9223372036854775807i64;
    
        return a - 2i64; // This line attempts to return -9223372036854775809i64 however, the smallest i64 value is -9223372036854775808i64.
    }}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // TODO: This test passes, but constant propogation should detect an overflow.
    
    transition main(y: bool) -> i32 {
        let a: i32 = -2147483647i32;
    
        return a - 2i32; // This line attempts to return -2147483649i32 however, the smallest i32 value is -2147483648i32.
    }}

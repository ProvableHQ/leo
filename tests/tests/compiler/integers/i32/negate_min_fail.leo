/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // TODO: This test passes, but constant propogation should detect an overflow.
    
    transition main(y: bool) -> i32 {
        let a: i32 = -2147483648i32;
    
        return -a; // This line attempts to return 2147483648i32 however, the largest i32 value is 2147483647i32.
    }
}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // TODO: This test passes, but constant propogation should detect an overflow.
    
    transition main(y: bool) -> i8 {
        let a: i8 = -128i8;
    
        return -a; // This line attempts to return 128i8 however, the largest i8 value is 127i8.
    }
}

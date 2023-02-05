/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    // TODO: This test passes, but constant propogation should detect an overflow.
    
    transition main(y: bool) -> i16 {
        let a: i16 = -32768i16;
    
        return -a; // This line attempts to return 32768i16 however, the largest i16 value is 32767i16.
    }
}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(x: u8, y: bool) -> u8 {
        return y ? x : 2u8;
    }
}

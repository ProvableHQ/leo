/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u8 = 255u8;
        return y == true;
    }
}

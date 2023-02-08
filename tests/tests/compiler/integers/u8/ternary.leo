/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: u8, b: u8, c: u8) -> bool {
        let r: u8 = s ? a : b;
    
        return r == c;
    }
}

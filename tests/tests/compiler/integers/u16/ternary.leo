/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: u16, b: u16, c: u16) -> bool {
        let r: u16 = s ? a : b;
    
        return r == c;
    }
}

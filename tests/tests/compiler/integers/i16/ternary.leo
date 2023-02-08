/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: i16, b: i16, c: i16) -> bool {
        let r: i16 = s ? a : b;
    
        return r == c;
    }
}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(s: bool, a: i8, b: i8, c: i8) -> bool {
        let r: i8 = s ? a : b;
    
        return r == c;
    }
}

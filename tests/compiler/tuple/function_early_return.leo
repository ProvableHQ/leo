/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u8, b: u8) -> (u8, u8) {
        if (a == b) {
            return (a, b);
        }
        let c: u8 = a + b;
        let d: u8 = a - b;
        return (c, d);
    }
}

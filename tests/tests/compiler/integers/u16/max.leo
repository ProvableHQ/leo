/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: u16 = 65535u16;
        return y == true;
    }
}

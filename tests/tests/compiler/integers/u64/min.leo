/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: u64 = 0u64;
        return y == true;
    }
}

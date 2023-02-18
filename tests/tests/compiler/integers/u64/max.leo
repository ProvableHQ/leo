/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: u64 = 18446744073709551615u64;
        return y == true;
    }
}

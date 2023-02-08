/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u64 = 18446744073709551615u64;
        return y == true;
    }
}

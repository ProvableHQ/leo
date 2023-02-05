/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u128, b: u128) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

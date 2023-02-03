/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u64, b: u64) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

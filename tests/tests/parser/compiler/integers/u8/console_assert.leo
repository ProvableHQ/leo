/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u8, b: u8) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u16, b: u16) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

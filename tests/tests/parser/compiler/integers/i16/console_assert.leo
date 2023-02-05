/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

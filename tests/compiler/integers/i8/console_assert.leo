/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

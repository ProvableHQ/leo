/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i32, b: i32) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

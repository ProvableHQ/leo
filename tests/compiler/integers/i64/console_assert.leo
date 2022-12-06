/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i32, b: i32) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

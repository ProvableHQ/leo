/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}

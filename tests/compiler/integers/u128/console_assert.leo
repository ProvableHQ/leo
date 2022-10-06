/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: u128, b: u128) -> bool {
        let ret: bool = a == b;
        console.assert(ret);
        return ret;
    }
}

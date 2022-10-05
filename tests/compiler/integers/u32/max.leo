/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u32 = 4294967295u32;
        return y == true;
    }
}

/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u8 = 255u8;
        return y == true;
    }
}

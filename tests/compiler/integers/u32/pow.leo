/*
namespace: Compile
expectation: Pass
input_file: inputs/pow.in

# This test might take to long to fully compile.
# If necessary we could move it to disabled_tests.
*/

program test.aleo {    
    transition main(a: u32, b: u32, c: u32) -> bool {
        return a ** 2u8 == a ** 2u16 && a ** b == c;
    }
}

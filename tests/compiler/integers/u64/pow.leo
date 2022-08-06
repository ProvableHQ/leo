/*
namespace: Compile
expectation: Pass
input_file: inputs/pow.in

# The exponent must be u8, u16, or u32
*/

@program
function main(a: u64, b: u64, c: u64) -> bool {
    return a ** 2u8 == a ** 2u16 && a ** 2u32 == c;
}

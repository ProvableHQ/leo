/*
namespace: Compile
expectation: Fail
input_file: inputs/pow.in

# The exponent must be u8, u16, or u32
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a ** b == c;
}

/*
namespace: Compile
expectation: Pass
input_file: inputs/pow.in
*/

@program
function main(a: u8, b: u8, c: u8) -> bool {
    return a ** b == a ** 2u16 && a ** 2u32 == c;
}

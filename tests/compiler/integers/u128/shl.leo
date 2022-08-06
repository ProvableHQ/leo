/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: u128, b: u128, c: u128) -> bool {
    return a << 2u8 == a << 2u16 && a << 2u32 == c;
}

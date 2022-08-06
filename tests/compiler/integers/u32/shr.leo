/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: u32, b: u32, c: u32) -> bool {
    return a >> 2u8 == a >> 2u16 && a >> b == c;
}

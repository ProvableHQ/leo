/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i8, b: i8, c: i8) -> bool {
    return a << 2u8 == a << 2u16 && a << 2u32 == c;
}

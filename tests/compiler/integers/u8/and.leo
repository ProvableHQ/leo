/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: u8, b: u8, c: u8) -> bool {
    return a & b == c;
}

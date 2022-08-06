/*
namespace: Compile
expectation: Pass
input_file: inputs/ternary_explicit_and_implicit.in
*/

@program
function main(x: u8, y: bool) -> u8 {
    return y ? x : 2u8;
}

/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main() -> bool {
    let x: i8 = -128i8;
    return x == -128i8;
}
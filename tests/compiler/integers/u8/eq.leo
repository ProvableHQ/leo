/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

function main(a: u8, b: u8, c: bool) -> bool {
    return (a == b) == c;
}

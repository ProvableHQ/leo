/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

function main(a: u128, b: u128, c: bool) -> bool {
    return (a == b) == c;
}

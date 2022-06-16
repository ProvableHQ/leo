/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a & b == c;
}

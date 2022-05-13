/*
namespace: Compile
expectation: Pass
input_file: inputs/pow.in

# This test might take to long to fully compile.
# If necessary we could move it to disabled_tests.
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a ** b == c;
}

/*
namespace: Compile
expectation: Pass
input_file: inputs/pow.in

# This test might take to long to fully compile.
# If necessary we could move it to disabled_tests.
*/

function main(a: u128, b: u128, c: u128) -> bool {
    return a ** b == c;
}

/*
namespace: Compile
expectation: Fail
input_file: inputs/pow.in
*/

function main(a: i128, b: i128, c: i128) -> bool {
    return a ** b == c;
}

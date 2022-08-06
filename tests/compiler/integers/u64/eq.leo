/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

@program
function main(a: u64, b: u64, c: bool) -> bool {
    return (a == b) == c;
}

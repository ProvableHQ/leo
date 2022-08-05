/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

@program
function main(a: group) -> group {
    return 1scalar * a;
}
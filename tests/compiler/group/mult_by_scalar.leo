/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

transition main(a: group) -> group {
    return 1scalar * a;
}
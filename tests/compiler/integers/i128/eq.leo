/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: i128, b: i128, c: bool) -> bool {
    return (a == b) == c;
}

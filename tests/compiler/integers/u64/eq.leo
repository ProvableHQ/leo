/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: u64, b: u64, c: bool) -> bool {
    return (a == b) == c;
}

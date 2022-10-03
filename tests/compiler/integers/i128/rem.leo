/*
namespace: Compile
expectation: Pass
input_file: inputs/div.in
*/

transition main(a: i128, b: i128, c: i128) -> bool {
    return a % b == c;
}

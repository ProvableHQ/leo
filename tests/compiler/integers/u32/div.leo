/*
namespace: Compile
expectation: Pass
input_file: inputs/div.in
*/

transition main(a: u32, b: u32, c: u32) -> bool {
    return a / b == c;
}

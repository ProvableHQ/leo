/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

transition main(a: u8, b: u8) -> u8 {
    return a ^ b;
}

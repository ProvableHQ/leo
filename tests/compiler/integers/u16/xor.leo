/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

transition main(a: u16, b: u16) -> u16 {
    return a ^ b;
}

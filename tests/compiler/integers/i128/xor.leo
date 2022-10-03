/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

transition main(a: i128, b: i128) -> i128 {
    return a ^ b;
}


/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main() -> bool {
    const arr: [u8; (2, 2)] = [1u8; (2, 1)]; // incorrect dimensions
    return false;
}

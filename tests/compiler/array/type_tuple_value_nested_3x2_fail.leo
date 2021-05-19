/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main() {
    const b: [u8; (2, 3)] = [[0; 2]; 3]; // initializer (incorrectly reversed ordering)
}

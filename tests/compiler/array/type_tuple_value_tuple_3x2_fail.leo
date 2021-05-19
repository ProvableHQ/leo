/*
namespace: Compile
expectation: Fail
*/

function main() {
    const b: [u8; (2, 3)] = [0; (3, 2)]; // initializer (incorrectly reversed ordering)
}

/*
namespace: Compile
expectation: Fail
*/

// Variables are immutable by default.
function main() {
    const a = 1u32;
    a = 0;
}
/*
namespace: Compile
expectation: Fail
*/

// Arrays are immutable by default.
function main() {
    const a = [1u32];
    a[0] = 0;
}
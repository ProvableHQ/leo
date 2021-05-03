/*
namespace: Compile
expectation: Pass
*/

function main() {
    let a = [1u32];
    a[0] = 0;

    console.assert(a[0] == 0u32);
}
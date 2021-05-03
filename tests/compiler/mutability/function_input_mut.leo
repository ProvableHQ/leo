/*
namespace: Compile
expectation: Pass
inputs:
 - mut.in: |
    [main]
    a: bool = false;
*/

// Function input are mutable by default.
function main(a: bool) {
    a = true;

    console.assert(a == true);
}
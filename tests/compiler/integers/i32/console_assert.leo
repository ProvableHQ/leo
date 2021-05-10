/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 1;
    b: i32 = 1;
*/

function main(a: i32, b: i32) {
    console.assert(a == b);
}

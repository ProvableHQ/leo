/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 1;
    b: i128 = 1;
*/

function main(a: i128, b: i128) {
    console.assert(a == b);
}

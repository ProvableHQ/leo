/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 1;
    b: i64 = 1;
*/

function main(a: i64, b: i64) {
    console.assert(a == b);
}

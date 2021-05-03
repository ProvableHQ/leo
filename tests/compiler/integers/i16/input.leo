/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 3;
    b: i16 = 3;
*/

function main(a: i16, b: i16) {
    console.assert(a == b);
}
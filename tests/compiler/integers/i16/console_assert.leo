/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 2;
    b: i16 = 2;
*/

function main(a: i16, b: i16) {
    console.assert(a == b);
}
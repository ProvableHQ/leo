/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 1;
    b: i16 = 1;
*/

function main(a: i16, b: i16) {
    console.assert(a == b);
}

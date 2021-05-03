/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 6;
    b: i16 = 3;
    c: i16 = 2;
*/

function main(a: i16, b: i16, c: i16) {
    console.assert(a / b == c);
}
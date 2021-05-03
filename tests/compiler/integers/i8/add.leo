/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 1;
    b: i8 = 2;
    c: i8 = 3;
*/

function main(a: i8, b: i8, c: i8) {
    console.assert(a + b == c);
}
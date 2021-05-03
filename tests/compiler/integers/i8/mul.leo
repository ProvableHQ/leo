/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 2;
    b: i8 = 5;
    c: i8 = 10;
*/

function main(a: i8, b: i8, c: i8) {
    console.assert(a * b == c);
}
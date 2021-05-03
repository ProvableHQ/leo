/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 100;
    b: i8 = 50;
    c: i8 = 50;
*/

function main(a: i8, b: i8, c: i8) {
    console.assert(a - b == c);
}
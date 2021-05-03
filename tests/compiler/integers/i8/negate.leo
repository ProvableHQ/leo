/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 5;
    b: i8 = -5;
*/

function main(a: i8, b: i8) {
    console.assert(-a == b);
}
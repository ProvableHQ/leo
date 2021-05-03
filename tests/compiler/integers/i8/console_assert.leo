/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 1;
    b: i8 = 1;
*/

function main(a: i8, b: i8) {
    console.assert(a == b);
}
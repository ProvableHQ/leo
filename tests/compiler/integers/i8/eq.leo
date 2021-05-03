/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 2;
    b: i8 = 2;
    c: bool = true;
*/

function main(a: i8, b: i8, c: bool) {
    console.assert(a == b == c);
}
/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 4;
    b: i16 = 1;
    c: bool = true;
 - i16_false: |
    [main]
    a: i16 = 1;
    b: i16 = 2;
    c: bool = false;
*/

function main(a: i16, b: i16, c: bool) {
    console.assert(a > b == c);
}
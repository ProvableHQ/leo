/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 3;
    b: i16 = 3;
    c: bool = true;
 - i16_false: |
    [main]
    a: i16 = 4;
    b: i16 = 3;
    c: bool = false;
*/

function main(a: i16, b: i16, c: bool) {
    console.assert(a <= b == c);
}
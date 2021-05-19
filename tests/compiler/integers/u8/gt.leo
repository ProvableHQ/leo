/*
namespace: Compile
expectation: Pass
inputs:
 - u8_g.in: |
    [main]
    a: u8 = 3;
    b: u8 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u8_f.in: |
    [main]
    a: u8 = 1;
    b: u8 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: bool) {
    console.assert(a > b == c);
}

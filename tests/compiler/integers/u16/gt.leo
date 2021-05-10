/*
namespace: Compile
expectation: Pass
inputs:
 - u16_g.in: |
    [main]
    a: u16 = 3;
    b: u16 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u16_f.in: |
    [main]
    a: u16 = 1;
    b: u16 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: bool) {
    console.assert(a > b == c);
}

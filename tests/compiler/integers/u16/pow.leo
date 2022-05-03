/*
namespace: Compile
expectation: Pass
inputs:
 - u16_f.in: |
    [main]
    a: u16 = 2;
    b: u16 = 2;
    c: u16 = 4;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: u16) -> bool {
    return a ** b == c;
}

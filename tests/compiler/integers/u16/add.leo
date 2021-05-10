/*
namespace: Compile
expectation: Pass
inputs:
 - u16.in: |
    [main]
    a: u16 = 2;
    b: u16 = 4;
    c: u16 = 6;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: u16) -> bool {
    return a + b == c;
}

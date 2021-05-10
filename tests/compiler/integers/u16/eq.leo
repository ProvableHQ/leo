/*
namespace: Compile
expectation: Pass
inputs:
 - u16_e.in: |
    [main]
    a: u16 = 2;
    b: u16 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u16_n.in: |
    [main]
    a: u16 = 3;
    b: u16 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: bool) -> bool {
    return a == b == c;
}

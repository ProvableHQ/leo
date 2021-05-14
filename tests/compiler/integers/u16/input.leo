/*
namespace: Compile
expectation: Pass
inputs:
 - u16_g.in: |
    [main]
    a: u16 = 2;
    b: u16 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16) -> bool {
    return a == b;
}

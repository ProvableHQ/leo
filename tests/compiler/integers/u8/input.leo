/*
namespace: Compile
expectation: Pass
inputs:
 - u8_g.in: |
    [main]
    a: u8 = 2;
    b: u8 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8) -> bool {
    return a == b;
}

/*
namespace: Compile
expectation: Pass
inputs:
 - u8.in: |
    [main]
    a: u8 = 2;
    b: u8 = 4;
    c: u8 = 6;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: u8) -> bool {
    return a + b == c;
}

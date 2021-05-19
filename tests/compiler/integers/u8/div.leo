/*
namespace: Compile
expectation: Pass
inputs:
 - u8.in: |
    [main]
    a: u8 = 4;
    b: u8 = 2;
    c: u8 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: u8) -> bool {
    return a / b == c;
}

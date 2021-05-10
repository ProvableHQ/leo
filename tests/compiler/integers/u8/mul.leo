/*
namespace: Compile
expectation: Pass
inputs:
 - mul.in: |
    [main]
    a: u8 = 10;
    b: u8 = 20;
    c: u8 = 30;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: u8) -> bool {
    return a * b == c;
}

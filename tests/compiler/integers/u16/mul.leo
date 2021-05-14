/*
namespace: Compile
expectation: Pass
inputs:
 - mul.in: |
    [main]
    a: u16 = 10;
    b: u16 = 20;
    c: u16 = 30;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: u16) -> bool {
    return a * b == c;
}

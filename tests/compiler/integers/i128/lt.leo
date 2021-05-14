/*
namespace: Compile
expectation: Pass
inputs:
 - i128_l.in: |
    [main]
    a: i128 = 2;
    b: i128 = 4;
    c: bool = true;

    [registers]
    r0: bool = false;
 - i128_e.in: |
    [main]
    a: i128 = 2;
    b: i128 = 2;
    c: bool = false;

    [registers]
    r0: bool = false;
*/

function main(a: i128, b: i128, c: bool) -> bool{
    return a < b == c;
}

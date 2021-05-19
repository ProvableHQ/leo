/*
namespace: Compile
expectation: Pass
inputs:
 - u128_g.in: |
    [main]
    a: u128 = 3;
    b: u128 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u128_e.in: |
    [main]
    a: u128 = 2;
    b: u128 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u128_f.in: |
    [main]
    a: u128 = 1;
    b: u128 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: bool) -> bool {
    return a >= b == c;
}

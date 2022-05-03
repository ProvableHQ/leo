/*
namespace: Compile
expectation: Pass
inputs:
 - u128_t.in: |
    [main]
    s: bool = true;
    a: u128 = 4u128;
    b: u128 = 2u128;
    c: u128 = 4u128;

    [registers]
    r0: bool = true;
 - u128_f.in: |
    [main]
    s: bool = false;
    a: u128 = 4u128;
    b: u128 = 2u128;
    c: u128 = 2u128;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u128, b: u128, c: u128) -> bool {
    let r: u128 = s ? a : b;

    return r == c;
}

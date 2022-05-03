/*
namespace: Compile
expectation: Pass
inputs:
 - u128_g.in: |
    [main]
    a: u128 = 3u128;
    b: u128 = 2u128;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u128_f.in: |
    [main]
    a: u128 = 1u128;
    b: u128 = 2u128;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: bool) -> bool {
    let ret: bool = a > b == c;
    console.assert(ret);
    return ret;
}

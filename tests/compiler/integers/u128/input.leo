/*
namespace: Compile
expectation: Pass
inputs:
 - u128_g.in: |
    [main]
    a: u128 = 2;
    b: u128 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128) -> bool {
    return a == b;
}

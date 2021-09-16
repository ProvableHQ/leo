/*
namespace: Compile
expectation: Pass
inputs:
 - ne.in: |
    [main]
    a: u128 = 10;
    b: u128 = 20;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: bool) -> bool {
    return (a != b) == c;
}

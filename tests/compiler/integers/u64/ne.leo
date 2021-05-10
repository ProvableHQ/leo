/*
namespace: Compile
expectation: Pass
inputs:
 - ne.in: |
    [main]
    a: u64 = 10;
    b: u64 = 20;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: bool) -> bool {
    return a != b == c;
}

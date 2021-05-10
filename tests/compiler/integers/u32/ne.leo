/*
namespace: Compile
expectation: Pass
inputs:
 - ne.in: |
    [main]
    a: u32 = 10;
    b: u32 = 20;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32, c: bool) -> bool {
    return a != b == c;
}

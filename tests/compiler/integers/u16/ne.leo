/*
namespace: Compile
expectation: Pass
inputs:
 - ne.in: |
    [main]
    a: u16 = 10;
    b: u16 = 20;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: bool) -> bool {
    return a != b == c;
}

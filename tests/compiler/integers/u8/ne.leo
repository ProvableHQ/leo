/*
namespace: Compile
expectation: Pass
inputs:
 - ne.in: |
    [main]
    a: u8 = 10;
    b: u8 = 20;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: bool) -> bool {
    return (a != b) == c;
}

/*
namespace: Compile
expectation: Pass
inputs:
 - u16.in: |
    [main]
    a: u16 = 2;
    b: u16 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16) -> bool {
    console.assert(a == b);
    return a == b;
}

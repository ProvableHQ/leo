/*
namespace: Compile
expectation: Pass
inputs:
 - u8.in: |
    [main]
    a: u8 = 2;
    b: u8 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8) -> bool {
    console.assert(a == b);
    return a == b;
}

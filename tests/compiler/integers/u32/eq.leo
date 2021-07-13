/*
namespace: Compile
expectation: Pass
inputs:
 - u32_e.in: |
    [main]
    a: u32 = 2;
    b: u32 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u32_n.in: |
    [main]
    a: u32 = 3;
    b: u32 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32, c: bool) -> bool {
    return (a == b) == c;
}

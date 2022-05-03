/*
namespace: Compile
expectation: Pass
inputs:
 - u32_t.in: |
    [main]
    s: bool = true;
    a: u32 = 4u32;
    b: u32 = 2u32;
    c: u32 = 4u32;

    [registers]
    r0: bool = true;
 - u32_f.in: |
    [main]
    s: bool = false;
    a: u32 = 4u32;
    b: u32 = 2u32;
    c: u32 = 2u32;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u32, b: u32, c: u32) -> bool {
    let r: u32 = s ? a : b;

    return r == c;
}

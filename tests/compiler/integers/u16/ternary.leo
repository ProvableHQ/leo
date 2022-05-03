/*
namespace: Compile
expectation: Pass
inputs:
 - u16_t.in: |
    [main]
    s: bool = true;
    a: u16 = 4u16;
    b: u16 = 2u16;
    c: u16 = 4u16;

    [registers]
    r0: bool = true;
 - u16_f.in: |
    [main]
    s: bool = false;
    a: u16 = 4u16;
    b: u16 = 2u16;
    c: u16 = 2u16;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u16, b: u16, c: u16) -> bool {
    let r: u16 = s ? a : b;

    return r == c;
}

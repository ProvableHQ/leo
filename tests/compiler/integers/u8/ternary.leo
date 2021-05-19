/*
namespace: Compile
expectation: Pass
inputs:
 - u8_t.in: |
    [main]
    s: bool = true;
    a: u8 = 4;
    b: u8 = 2;
    c: u8 = 4;

    [registers]
    r0: bool = true;
 - u8_f.in: |
    [main]
    s: bool = false;
    a: u8 = 4;
    b: u8 = 2;
    c: u8 = 2;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u8, b: u8, c: u8) -> bool {
    let r = s ? a : b;

    return r == c;
}
